// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! The bridge's object index and deletion policy.
//!
//! Liveness is not held here: every attached object is kept alive by its
//! proxy, so a Rust value lives exactly as long as its `QObject`, plus any
//! user handles, which are plain `Rc<RefCell<T>>`s whose drops never
//! tear anything down. The registry observes each object through a `Weak`
//! reference and decides when the `QObject`s of Rust-created objects die.
//!
//! Every entry names its [`Owner`]:
//!
//! * [`Owner::RustRegistry`]: Rust-created. Pinned to `CppOwnership` while
//!   Rust holds a handle, so the QML engine cannot delete it. The engine
//!   keeps the JS wrapper of a `CppOwnership` object alive for the whole
//!   object lifetime. [`collect_garbage`] therefore hands ownership to the
//!   engine by setting `JavaScriptOwnership` for objects without Rust
//!   interest (strong count is down to the proxy's own). The engine's
//!   garbage collector deletes them with exact reachability and takes
//!   down the value together with the proxy. An object that re-enters Rust
//!   is changed back to `CppOwnership`. Objects that were never wrapped
//!   with a JS wrapper are deleted directly when Rust interest vanishes.
//! * [`Owner::Engine`]: QML-created. The engine (or a parent) deletes the
//!   `QObject`; the registry never does, and the entry only serves the
//!   proxy lookup.
//!
//! The `CppOwnership` flag guards only against the garbage collector:
//! deletion paths that ignore the ownership flag (parents, components,
//! engine death) can take a `QObject` of either kind. The registry entry
//! is deleted together with the `QObject` but the Rust value then survives
//! through user handles and gets a fresh `QObject` with `Owner::RustRegistry`
//! attached on its next exposure.
//!
//! [`collect_garbage`] is triggered by the garbage collection of the
//! QmlEngine and under allocation pressure (see `register`).
//! A garbage collection cannot be observed with e.g. connecting to a
//! signal, so we use a sentinel that is injected into the QML engine and
//! that should be deleted on the next garbage collector cycle.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Weak;

use qtbridge_type_lib::QObject;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;

        include!("cpp/registry.h");
    }

    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/qml/qqmlapplicationengine/cpp/qqmlapplicationengine.h");
        type QQmlApplicationEngine = qtbridge_type_lib::QQmlApplicationEngine;
    }

    #[namespace = "rust::bridge::registry"]
    unsafe extern "C++" {
        /// Whether any QML engine currently holds a JS wrapper for `obj`.
        #[rust_name = has_live_js_wrapper]
        unsafe fn hasLiveJsWrapper(obj: *const QObject) -> bool;

        #[rust_name = set_cpp_ownership]
        unsafe fn setCppOwnership(obj: *mut QObject);

        #[rust_name = set_javascript_ownership]
        unsafe fn setJavaScriptOwnership(obj: *mut QObject);

        #[rust_name = is_javascript_ownership]
        unsafe fn isJavaScriptOwnership(obj: *mut QObject) -> bool;

        #[rust_name = install_gc_sentinel_impl]
        fn installGcSentinel(engine: Pin<&mut QQmlApplicationEngine>);
    }

    #[namespace = "rust::bridge::registry"]
    extern "Rust" {
        fn collect_garbage();
    }
}

/// Arms the automatic collection trigger on `engine` by creating a sentinel.
///
/// The sentinel is a dummy `QObject` with `JavaScriptOwnership` whose JS
/// wrapper is referenced by nothing: the next garbage collection frees the
/// wrapper and thereby deletes the sentinel, whose `destroyed()` signal runs
/// [`collect_garbage`] and re-arms a new sentinel one event-loop turn later.
/// [`crate::QApp`] arms this automatically; call it manually when driving a raw engine.
pub fn install_gc_sentinel(engine: core::pin::Pin<&mut qtbridge_type_lib::QQmlApplicationEngine>) {
    ffi::install_gc_sentinel_impl(engine);
}

/// ownership indicator:
 #[derive(Clone, PartialEq)]
 pub enum Owner {
    /// The registry: pinned to `CppOwnership` while Rust holds a handle,
    /// changed to `JavaScriptOwnership` by [`collect_garbage`] and then
    /// finally deleted by the QML engine.
    RustRegistry,
    /// The QML engine which deletes its own objects.
    Engine,
}

struct Entry {
    /// Type-erased pointer to the object's `RustProxy`.
    proxy: *const u8,
    /// The attached [`QObject`]. Valid for as long as the entry exists: its
    /// deletion tears down the proxy, whose `on_drop` unregisters the entry.
    qobject: *mut QObject,
    /// Observe Rust usage. RustProxy holds the strong reference and
    /// guarantees liveness.
    value: Weak<dyn Any>,
    /// Initiator of deletion of this entry:
    owner: Owner,
}

/// Entries keyed by the address of the user value.
struct Entries {
    map: HashMap<*const u8, Entry>,
    /// Number of entries with a `shared_owner` for debugging and
    /// collecting under pressure
    owned: usize,
}

impl Drop for Entries {
    fn drop(&mut self) {
        for (_, entry) in self.map.drain() {
            if entry.owner == Owner::RustRegistry {
                QObject::delete(entry.qobject);  // Deletes both proxies
            }
        }
    }
}

thread_local! {
    static REGISTRY: RefCell<Entries> = RefCell::new(Entries { map: HashMap::new(), owned: 0 });
}

thread_local! {
    static COLLECT_THRESHOLD: Cell<usize> = const { Cell::new(64) };
}

fn owned_count() -> usize {
    REGISTRY.with_borrow(|entries| entries.owned)
}

/// Registers an attached object
pub(crate) fn register(
    key: *const u8, proxy: *const u8, qobject: *mut QObject,
    value: Weak<dyn Any>, owner: Owner
) {
    let registry_owned = owner == Owner::RustRegistry;
    if registry_owned {
        unsafe { ffi::set_cpp_ownership(qobject) };
    }
    REGISTRY.with_borrow_mut(|entries| {
        let old = entries.map.insert(key, Entry { proxy, qobject, value, owner });
        debug_assert!(old.is_none(), "Object is already registered");
        entries.owned += registry_owned as usize;
    });
    // If we reach a certain amount of QObjects, we will trigger a collect to
    // clean up stale objects.
    if registry_owned && owned_count() >= COLLECT_THRESHOLD.get() {
        collect_garbage();
    }
}

/// Takes ownership back when a handed-over object re-enters Rust.
pub(crate) fn repin(key: *const u8) {
    REGISTRY.with_borrow(|entries| {
        if let Some(entry) = entries.map.get(&key)
            && entry.owner == Owner::RustRegistry {
            unsafe { ffi::set_cpp_ownership(entry.qobject) };
        }
    });
}

/// Drops the entry for `key`; called from the proxy teardown when the
/// `QObject` is deleted (by [`collect_garbage`] or by the engine). Entries
/// of objects freed by [`collect_garbage`] are already extracted by then,
/// and during registry teardown the map is being drained.
pub(crate) fn unregister(key: *const u8) {
    let _ = REGISTRY.try_with(|entries: &RefCell<Entries>| {
        let mut entries = entries.borrow_mut();
        if let Some(entry) = entries.map.remove(&key) {
            entries.owned -= (entry.owner == Owner::RustRegistry) as usize;
        }
    });
}

/// Returns the type-erased `RustProxy` pointer for `key`, or null when the
/// object has no attached `QObject`.
pub(crate) fn proxy_ptr(key: *const u8) -> *const u8 {
    REGISTRY.with_borrow(|entries| {
        entries.map.get(&key).map_or(std::ptr::null(), |entry| entry.proxy)
    })
}

/// The number of objects the registry currently owns. Useful for leak
/// checks.
pub fn live_count() -> usize {
    owned_count()
}

/// The number of objects the registry currently owns. Useful for leak
/// checks.
pub fn live_proxy_count() -> usize {
    REGISTRY.with_borrow(|entries| entries.map.len())
}


/// Frees every object that is neither referenced from Rust nor reachable from
/// QML.
/// Runs automatically after every garbage collection if using [crate::QApp]
/// and under allocation pressure; call it explicitly for deterministic
/// reclamation points.
pub fn collect_garbage() {
    // Rust interest is the strong count above the registry's own reference.
    // Objects the engine never wrapped are freed directly; wrapped ones are
    // handed over to the engine, whose next garbage collection deletes them
    // unless QML still reaches them.

    // Freeing an object can release its references to other registered
    // objects (e.g. children stored in fields), so iterate to a fixpoint.
    loop {
        // Extract first, act outside the borrow: deleting a QObject
        // re-enters the registry through the proxy teardown's unregister.
        let doomed: Vec<(Weak<dyn Any>, *mut QObject)> = REGISTRY.with_borrow_mut(|entries| {
            let extracted: Vec<_> = entries.map.extract_if(|_key, entry| {
                    if entry.owner == Owner::Engine {
                        return false;
                    }
                    debug_assert!(entry.value.strong_count() >= 1);
                    if entry.value.strong_count() > 1 {
                        return false;
                    }
                    if unsafe { ffi::is_javascript_ownership(entry.qobject) } {
                        return false;
                    }
                    if unsafe { ffi::has_live_js_wrapper(entry.qobject) } {
                        // The wrapper of a CppOwned object never dies and we can
                        // therefore not track QML interest into the object. Hand
                        // the object to the engine in order to check QML interest.
                        // The engine will initiate the teardown.
                        unsafe { ffi::set_javascript_ownership(entry.qobject) };
                        return false;
                    }
                    // No Rust interest (Only strong reference is in the proxy)
                    // No QML interest (No JS Wrapper)
                    true
                })
                .map(|(_key, entry)| {
                    (entry.value, entry.qobject)
                })
                .collect();
            entries.owned -= extracted.len();
            extracted
        });
        if doomed.is_empty() {
            break;
        }
        for (value, qobject) in doomed {
            // Ensure that the Rust object is alive for the whole destructor
            let keep_alive = value.upgrade();
            assert!(keep_alive.is_some());
            // Tears down the proxy pair; its on_drop removes the registry
            // entry.
            QObject::delete(qobject);
            // Ours is the last reference: this frees the Rust object,
            // running a user-provided Drop if there is one.
            drop(keep_alive);
        }
    }
    // Update the threshold on when we automatically collect QObjects.
    // Twice the amount after a fresh sweep seems to be a good spot.
    // The minimum value of 64 avoids collection on every few allocations.
    // TODO: We might have to re-evaluate the values or this simplistic
    // algorithm.
    COLLECT_THRESHOLD.set((owned_count() * 2).max(64));
}
