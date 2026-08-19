// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]


use qtbridge_type_lib::{QGuiApplication, QVariantList};
use qtbridge::{qobject, QObjectHolder};
use qtbridge::qtbridge_type_lib::{QSignalSpy};
use qtbridge::invoke_method;
#[qobject]
pub mod test_object {
    use std::cell::Cell;

    #[derive(Default)]
    pub struct TestObject {
        pub mutable_slot_called: bool,
        pub int_value: i32,
        pub immutable_slot_called: Cell<bool>,
    }

    impl TestObject {
        #[qsignal]
        pub fn signal_no_args(&mut self);

        #[qslot]
        fn mutable_slot(&mut self) {
            self.mutable_slot_called = true;
        }

        #[qslot]
        fn immutable_slot(&self) {
            self.immutable_slot_called.set(true);
        }

        #[qslot]
        pub fn set_int(&mut self, value: i32) {
            self.int_value = value;
        }

        #[qslot]
        pub fn add_ints(&mut self, a: i32, b: i32) {
            self.int_value = a + b;
        }
    }
}

pub use test_object::TestObject;

fn test_invoke_method_alive() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    assert!(qobject_holder.borrow().get_qml_method_invoker().invoke_method("signal_no_args"));
}

fn test_invoke_method_destroyed() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qobject_holder.borrow().detach_qobject();
    assert!(!qml_method_invoker.invoke_method("signal_no_args"));
}

fn test_signal_emitted(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(unsafe { &*qobject_holder.borrow().get_qobject_ptr() }, "signal_no_args");
    qobject_holder.borrow().get_qml_method_invoker().invoke_method("signal_no_args");
    app.process_events();
    app.process_events();
    assert_eq!(spy.count(), 1);
}

fn test_mutable_slot(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("mutable_slot");
    app.process_events();
    app.process_events();
    assert!(qobject_holder.borrow().mutable_slot_called);
}

fn test_immutable_slot(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("immutable_slot");
    app.process_events();
    app.process_events();
    assert!(qobject_holder.borrow().immutable_slot_called.get());
}

fn test_slot_with_parameters(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    assert!(invoker.invoke_method_with_args("add_ints", &QVariantList::from_iter(&[(&15).into(), (&17).into()])));
    app.process_events();
    app.process_events();
    assert_eq!(qobject_holder.borrow().int_value, 32);
}

fn test_immutable_slot_via_macro(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    invoke_method!(invoker, "immutable_slot");
    app.process_events();
    app.process_events();
    assert!(qobject_holder.borrow().immutable_slot_called.get());
}

fn test_slot_with_parameters_via_macro(app: &QGuiApplication) {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    invoke_method!(invoker, "add_ints", 15, 17);
    app.process_events();
    app.process_events();
    assert_eq!(qobject_holder.borrow().int_value, 32);
}

#[cfg(not(miri))]
fn main() {
    let app = QGuiApplication::new();
    test_invoke_method_alive();
    test_invoke_method_destroyed();
    test_signal_emitted(&app);
    test_mutable_slot(&app);
    test_immutable_slot(&app);
    test_slot_with_parameters(&app);
    test_immutable_slot_via_macro(&app);
    test_slot_with_parameters_via_macro(&app);
}

#[cfg(miri)]
fn main() {}
