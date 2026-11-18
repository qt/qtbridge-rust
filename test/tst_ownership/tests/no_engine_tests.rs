// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Behavior of `#[qobject]` types when no `QApp`, `QGuiApplication` or QML
//! engine exists: plain use, signal emission (including object arguments),
//! the method invoker, and reclamation. Show that `collect_garbage()` needs
//! no application at all.

#[cfg(not(miri))]
#[path = "../../common/mod.rs"]
mod common;

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QmlObject, collect_garbage, qobject};
use qtbridge::qtbridge_runtime::QObjectHolder;
use qtbridge::qtbridge_type_lib::QSignalSpy;

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
    #[qslot]
    pub fn ping(&self) -> i32 {
        42
    }
}

#[derive(Default)]
pub struct Backend {
    pub calls: i32,
}

#[qobject]
impl Backend {
    #[qsignal]
    pub fn plain_signal(&mut self);

    #[qsignal]
    pub fn child_signal(&mut self, child: Rc<RefCell<Child>>);

    #[qslot]
    pub fn bump(&mut self) {
        self.calls += 1;
    }
}

/// Plain Rust use without any application: construction, borrows, slots as
/// methods, signal emission into the void, and reclamation via `collect_garbage()`.
fn plain_lifecycle_works_without_application() {
    let backend = Backend::default_with_attached_qobject();
    backend.borrow_mut().bump();
    assert_eq!(backend.borrow().calls, 1);

    // Emitting with no receivers is a no-op, not a crash.
    backend.borrow_mut().plain_signal();

    let spy = QSignalSpy::new(
        unsafe { &*backend.borrow().get_qobject_ptr() },
        "destroyed"
    );
    drop(backend);
    assert_eq!(spy.count(), 0);
    // collect_garbage() needs no application: nothing references the object and no
    // engine has ever seen it.
    collect_garbage();
    assert_eq!(spy.count(), 1);
}

/// An object argument crossing without any application stays alive until a
/// collection. Keeping alive because a wrapper could still appear once an
/// engine exists.
fn object_signal_argument_without_application_is_reclaimed_by_collect_garbage() {
    let backend = Backend::default_with_attached_qobject();

    let child = Child::default_with_attached_qobject();
    let weak = Rc::downgrade(&child);

    backend.borrow_mut().child_signal(child);

    // The last user handle is gone, but the registry keeps the object.
    assert!(weak.upgrade().is_some());

    collect_garbage();
    assert!(weak.upgrade().is_none());
}

/// The thread handle degrades gracefully without an application.
fn thread_handle_without_application_does_not_crash() {
    let backend = Backend::default_with_attached_qobject();
    let handle = backend.borrow().get_thread_handle();
    // The queued closure cannot be delivered; the call itself must not
    // crash or corrupt the object.
    handle.invoke_mut(|b| b.bump());
    assert_eq!(backend.borrow().calls, 0);
}

/// Even without any application (and no sentinel, no event loop) churning
/// through objects must not grow memory without bound: the allocation-
/// pressure trigger collects on its own.
fn appless_churn_is_bounded_by_allocation_pressure() {
    use qtbridge::qtbridge_runtime::registry_owned_count;

    let baseline = registry_owned_count();
    for _ in 0..1000 {
        // Created, attached, dropped: no handle survives the iteration and
        // this test never calls collect_garbage().
        drop(Child::default_with_attached_qobject());
    }
    assert!(registry_owned_count() < baseline + 128,
        "allocation pressure must keep the appless registry bounded, got {}",
        registry_owned_count());
}

#[cfg(not(miri))]
fn main() {
    common::run_test!(plain_lifecycle_works_without_application);
    common::run_test!(appless_churn_is_bounded_by_allocation_pressure);
    common::run_test!(object_signal_argument_without_application_is_reclaimed_by_collect_garbage);
    common::run_test!(thread_handle_without_application_does_not_crash);
}

#[cfg(miri)]
fn main() {}
