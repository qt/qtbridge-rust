// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Property getters and setters may carry attributes that are not meta
//! attributes, such as `///` doc comments (`#[doc]`) or `#[inline]`.

use qtbridge::{QObjectHolder, QmlObject, qobject};
use qtbridge_type_lib::QVariant;
use qtbridge_type_lib::core::qobject;

#[derive(Default)]
pub struct Documented {
    value: i32,
}

#[qobject]
impl Documented {
    qproperty!("value", Read = value, Write = set_value, Notify = value_changed);

    #[qsignal]
    fn value_changed(&mut self);

    /// The current value.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Sets the value and notifies QML.
    #[inline]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.value_changed();
    }
}

/// The getter with a doc comment is found and read through the property.
fn qproperty_documented_getter_can_be_read() {
    let obj = Documented::default_with_attached_qobject();
    obj.borrow_mut().value = 42;

    let var = qobject::property(unsafe { &*obj.borrow().get_qobject_ptr() }, "value");
    assert_eq!(var.value::<i32>(), Some(42));
}

/// The setter with a doc comment and `#[inline]` is found and written
/// through the property.
fn qproperty_documented_setter_can_be_written() {
    let obj = Documented::default_with_attached_qobject();

    let qobj_ptr = obj.borrow().get_qobject_ptr();
    unsafe { qobject::set_property(qobj_ptr, "value", &QVariant::from(&7_i32)) };
    assert_eq!(obj.borrow().value, 7);
}

fn main() {
    qproperty_documented_getter_can_be_read();
    qproperty_documented_setter_can_be_written();
}
