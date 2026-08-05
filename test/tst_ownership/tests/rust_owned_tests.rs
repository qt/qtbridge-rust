// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Rust-side lifetime of registry-owned objects: user handles are plain
//! `Rc<RefCell<T>>`s whose drops never delete anything; `qtbridge::collect_garbage()`
//! frees objects that nothing references anymore.

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

/// Dropping the last user handle deletes nothing — the registry owns the
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

fn main() {
    drop_then_collect_deletes_object_and_qobject();
    collect_spares_objects_with_rust_handles();
}
