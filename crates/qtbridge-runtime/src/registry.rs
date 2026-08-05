// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! The owner of record for every Rust-created `#[qobject]` instance.
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
    /// Shared reference counter to observe Rust usage and guarantee
    /// liveness.
    shared_owner: Rc<dyn Any>,
    /// The attached [`QObject`]. Valid for as long as the entry exists: its
    /// deletion tears down the proxy, whose `on_drop` unregisters the entry.
    qobject: *mut QObject,
}

/// Entries keyed by the address of the user value (same as
/// proxy multiton map)
struct Entries(HashMap<*const u8, Entry>);

impl std::ops::Deref for Entries {
    type Target = HashMap<*const u8, Entry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Entries {
    fn drop(&mut self) {
        // Thread-local destruction order is unspecified: freeing objects
        // here would run QObject destructors while the Proxy map might
        // already be deleted. Therefore leak it here to be safe.
        //
        // TODO: This can lead to situations where the users drop is not
        // called. If they use drop to e.g. close a file, they have a
        // problem. Maybe we can improve?
        for (_, entry) in self.drain() {
            std::mem::forget(entry.shared_owner);
        }
    }
}

thread_local! {
    static REGISTRY: RefCell<Entries> = RefCell::new(Entries(HashMap::new()));
}

thread_local! {
    static COLLECT_THRESHOLD: Cell<usize> = const { Cell::new(64) };
}

fn owned_count() -> usize {
    REGISTRY.with_borrow(|entries| entries.len())
}

/// Registers a Rust-created object; called when its `QObject` is attached.
pub(crate) fn register(keep: Rc<dyn Any>, key: *const u8, qobject: *mut QObject) {
    unsafe { ffi::set_cpp_ownership(qobject) };
    REGISTRY.with_borrow_mut(|entries| {
        entries.insert(key, Entry { shared_owner: keep, qobject })
    });
    // If we reach a certain amount of QObjects, we will trigger a collect to
    // clean up stale objects.
    if owned_count() >= COLLECT_THRESHOLD.get() {
        collect_garbage();
    }
}

/// Takes ownership back when a handed-over object re-enters Rust.
pub(crate) fn repin(key: *const u8) {
    REGISTRY.with_borrow_mut(|entries| {
        if let Some(entry) = entries.get_mut(&key) {
            unsafe { ffi::set_cpp_ownership(entry.qobject) };
        }
    });
}

/// Drops the registry's reference for `key`; called from the proxy teardown
/// when the `QObject` is deleted (by [`collect_garbage`] or by the engine).
pub(crate) fn unregister(key: *const u8) {
    REGISTRY.with_borrow_mut(|entries| entries.remove(&key));
}

/// The number of objects the registry currently owns. Useful for leak
/// checks.
pub fn live_count() -> usize {
    owned_count()
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
        // re-enters the registry through the proxy teardown's call to
        // `register`.
        let doomed: Vec<(Rc<dyn Any>, *mut QObject)> = REGISTRY.with_borrow_mut(|entries| {
            entries.extract_if(|_key, entry| {
                    if Rc::strong_count(&entry.shared_owner) > 1
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
                .map(|(_key, entry)| (entry.shared_owner, entry.qobject))
                .collect()
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
