// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Rust-side lifetime of registry-owned objects: user handles are plain
//! `Rc<RefCell<T>>`s whose drops never delete anything; `qtbridge::collect_garbage()`
//! frees objects that nothing references anymore.

use std::cell::Cell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, collect_garbage, qobject};
use qtbridge::qtbridge_type_lib::QSignalSpy;

#[qobject]
mod test_object {
    #[derive(Default)]
    pub struct TestObject {
    }

    impl TestObject {
    }
}

use test_object::TestObject;

thread_local! { static DROP_COUNT: Cell<u32> = Cell::new(0); }

#[qobject]
mod dropper {
    #[derive(Default)]
    pub struct Dropper {
    }

    impl Dropper {
    }

    impl Drop for Dropper {
        fn drop(&mut self) {
            super::DROP_COUNT.with(|count| count.set(count.get() + 1));
        }
    }
}

use dropper::Dropper;

/// Dropping the last user handle deletes nothing: the registry owns the
/// object; `collect_garbage()` frees it.
fn drop_then_collect_deletes_object_and_qobject() {
    let obj = TestObject::default_with_attached_qobject();
    // One user handle plus the registry's owning reference.
    assert_eq!(2, Rc::strong_count(&obj));

    let weak = Rc::downgrade(&obj);
    let spy = QSignalSpy::new(
        unsafe { &*obj.borrow().get_qobject_ptr() },
        "destroyed"
    );

    drop(obj);
    // Still owned by the registry: pointers in flight cannot dangle.
    assert_eq!(spy.count(), 0);
    assert!(weak.upgrade().is_some());

    collect_garbage();
    assert_eq!(spy.count(), 1);
    assert!(weak.upgrade().is_none());
}

/// `collect_garbage()` spares objects that Rust still references.
fn collect_spares_objects_with_rust_handles() {
    let obj = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(
        unsafe { &*obj.borrow().get_qobject_ptr() },
        "destroyed"
    );

    collect_garbage();
    assert_eq!(spy.count(), 0);
    assert!(!obj.borrow().get_qobject_ptr().is_null());
}

/// Clones are ordinary `Rc` clones; the object stays alive until the last
/// user handle is gone and a collection ran.
fn clone_keeps_object_alive_until_last_handle_and_collect_garbage() {
    let obj = TestObject::default_with_attached_qobject();
    let second = obj.clone();
    assert_eq!(3, Rc::strong_count(&obj)); // two handles + registry

    let spy = QSignalSpy::new(
        unsafe { &*obj.borrow().get_qobject_ptr() },
        "destroyed"
    );

    drop(obj);
    collect_garbage();
    assert_eq!(spy.count(), 0, "a live clone must keep the object");

    drop(second);
    collect_garbage();
    assert_eq!(spy.count(), 1);
}

/// Weak handles expire when the registry releases the object.
fn weak_handle_expires_after_collect_garbage() {
    let obj = TestObject::default_with_attached_qobject();
    let weak = Rc::downgrade(&obj);

    let upgraded = weak.upgrade().expect("object is still alive");
    assert!(Rc::ptr_eq(&obj, &upgraded));

    drop(upgraded);
    drop(obj);
    assert!(weak.upgrade().is_some(), "the registry still owns it");
    collect_garbage();
    assert!(weak.upgrade().is_none());
}

/// A user-provided `Drop` impl runs when the registry releases the object.
fn user_drop_impl_runs_at_collect_garbage() {
    let obj = Dropper::default_with_attached_qobject();
    DROP_COUNT.with(|count| count.set(0));

    drop(obj);
    assert_eq!(DROP_COUNT.with(|count| count.get()), 0);

    collect_garbage();
    assert_eq!(DROP_COUNT.with(|count| count.get()), 1);
}

#[cfg(not(miri))]
fn main() {
    drop_then_collect_deletes_object_and_qobject();
    collect_spares_objects_with_rust_handles();
    clone_keeps_object_alive_until_last_handle_and_collect_garbage();
    weak_handle_expires_after_collect_garbage();
    user_drop_impl_runs_at_collect_garbage();
}

#[cfg(miri)]
fn main() {}
