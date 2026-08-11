// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! The owner of record for every Rust-created `#[qobject]` instance, and
//! the proxy table for every attached object, Rust- or QML-created.
//!
//! The registry holds one strong reference per attached object, so user
//! handles are plain [`Rc<RefCell<T>>`]s whose drops never tear anything
//! down and T cannot be moved out of its original place; `QObject`s are
//! explicit `CppOwnership` while Rust references them, so the QML engine
//! cannot delete them.
//!
//! The QML engine keeps the JS wrapper of a `CppOwnership` object alive for
//! the whole object lifetime. [`collect_garbage`] therefore frees objects
//! without Rust interest (strong count is down to the registry's own)
//! directly when the engine never wrapped them, and otherwise hands ownership
//! to the engine by setting `JavaScriptOwnership`. The engine's garbage collector
//! deletes them with exact reachability and takes down the reference together
//! with the proxy. An object that re-enters Rust and clones the
//! [`Rc<RefCell<T>>`] from the proxy is changed back to `CppOwnership`.
//!
//! [`collect_garbage`] is triggered by the garbage collection of the
//! QmlEngine and under allocation pressure (see `register`).
//! A garbage collection cannot be observed with e.g. connecting to a
//! signal, so we use a sentinel that is injected into the QML engine and
//! that should be deleted on the next garbage collector cycle.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

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

struct Entry {
    /// Type-erased pointer to the object's `RustProxy`.
    proxy: *const u8,
    /// The attached [`QObject`]. Valid for as long as the entry exists: its
    /// deletion tears down the proxy, whose `on_drop` unregisters the entry.
    qobject: *mut QObject,
    /// Shared reference counter to observe Rust usage and guarantee
    /// liveness. `None` for QML-created objects, which the engine owns.
    shared_owner: Option<Rc<dyn Any>>,
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
        // Thread teardown: free the objects we own; QObjects without a
        // `shared_owner` are the engine's (or a parent's) to delete.
        for (_, entry) in self.map.drain() {
            if let Some(owner) = entry.shared_owner {
                QObject::delete(entry.qobject); // Deletes both proxies
                drop(owner);
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

/// Registers an attached object; `shared_owner` is set for Rust-created
/// objects, which the registry owns.
pub(crate) fn register(
    key: *const u8, proxy: *const u8, qobject: *mut QObject, shared_owner: Option<Rc<dyn Any>>,
) {
    let owned = shared_owner.is_some();
    if owned {
        unsafe { ffi::set_cpp_ownership(qobject) };
    }
    REGISTRY.with_borrow_mut(|entries| {
        let old = entries.map.insert(key, Entry { proxy, qobject, shared_owner });
        debug_assert!(old.is_none(), "Object is already registered");
        entries.owned += owned as usize;
    });
    // If we reach a certain amount of QObjects, we will trigger a collect to
    // clean up stale objects.
    if owned && owned_count() >= COLLECT_THRESHOLD.get() {
        collect_garbage();
    }
}

/// Takes ownership back when a handed-over object re-enters Rust.
pub(crate) fn repin(key: *const u8) {
    REGISTRY.with_borrow(|entries| {
        if let Some(entry) = entries.map.get(&key) {
            if entry.shared_owner.is_some() {
                unsafe { ffi::set_cpp_ownership(entry.qobject) };
            }
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
            entries.owned -= entry.shared_owner.is_some() as usize;
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
        let doomed: Vec<(Rc<dyn Any>, *mut QObject)> = REGISTRY.with_borrow_mut(|entries| {
            let extracted: Vec<_> = entries.map.extract_if(|_key, entry| {
                    let Some(owner) = &entry.shared_owner else {
                        return false;
                    };
                    if Rc::strong_count(owner) > 1
                        || unsafe { ffi::is_javascript_ownership(entry.qobject) } {
                        return false;
                    }
                    if unsafe { ffi::has_live_js_wrapper(entry.qobject) } {
                        // The wrapper of a CppOwned object never dies: hand
                        // the object to the engine, whose exact
                        // reachability decides. Its deletion runs the
                        // proxy teardown like any other.
                        unsafe { ffi::set_javascript_ownership(entry.qobject) };
                        return false;
                    }
                    true
                })
                .map(|(_key, entry)| {
                    let owner = entry.shared_owner
                        .expect("Only owned entries are extracted");
                    (owner, entry.qobject)
                })
                .collect();
            entries.owned -= extracted.len();
            extracted
        });
        if doomed.is_empty() {
            break;
        }
        for (shared_reference, qobject) in doomed {
            // Tears down the proxy pair; its on_drop removes the registry
            // entry.
            QObject::delete(qobject);
            // Ours is the last reference: this frees the Rust object,
            // running a user-provided Drop if there is one.
            drop(shared_reference);
        }
    }
    // Update the threshold on when we automatically collect QObjects.
    // Twice the amount after a fresh sweep seems to be a good spot.
    // The minimum value of 64 avoids collection on every few allocations.
    // TODO: We might have to re-evaluate the values or this simplistic
    // algorithm.
    COLLECT_THRESHOLD.set((owned_count() * 2).max(64));
}
