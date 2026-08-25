// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use std::fmt::Debug;
use qtbridge::{QApp, QObjectHolder, qobject};
use qtbridge::qtbridge_type_lib::{QString, QVariantValue};
use common::{MAX_SAFE_INTEGER, MIN_SAFE_INTEGER, capitalize_first_char, get_type_name};

#[qobject(ConvertToCamelCase)]
pub mod test_object {
    use super::{MIN_SAFE_INTEGER, MAX_SAFE_INTEGER};

    #[derive(Default)]
    pub struct TestObject {
        pub bool_value: bool,
        pub i8_value: i8,
        pub u8_value: u8,
        pub i16_value: i16,
        pub u16_value: u16,
        pub i32_value: i32,
        pub u32_value: u32,
        pub i64_value: i64,
        pub u64_value: u64,
        pub isize_value: isize,
        pub usize_value: usize,
        pub f32_value: f32,
        pub f64_value: f64,
        pub string_value: String,
        pub str_value: String,
    }

    impl TestObject {
        qproperty!("slotBoolValue", Member = bool_value);
        qproperty!("slotI8Value", Member = i8_value);
        qproperty!("slotU8Value", Member = u8_value);
        qproperty!("slotI16Value", Member = i16_value);
        qproperty!("slotU16Value", Member = u16_value);
        qproperty!("slotI32Value", Member = i32_value);
        qproperty!("slotU32Value", Member = u32_value);
        qproperty!("slotI64Value", Member = i64_value);
        qproperty!("slotU64Value", Member = u64_value);
        qproperty!("slotIsizeValue", Member = isize_value);
        qproperty!("slotUsizeValue", Member = usize_value);
        qproperty!("slotF32Value", Member = f32_value);
        qproperty!("slotF64Value", Member = f64_value);
        qproperty!("slotStringValue", Member = string_value);
        qproperty!("slotStrValue", Member = str_value);

        #[qslot]
        fn slot_unit(&self) {
        }
        #[qslot]
        fn slot_bool(&self) -> bool {
            true
        }
        #[qslot]
        fn slot_i8(&self) -> i8 {
            i8::MIN
        }
        #[qslot]
        fn slot_u8(&self) -> u8 {
            u8::MAX
        }
        #[qslot]
        fn slot_i16(&self) -> i16 {
            i16::MIN
        }
        #[qslot]
        fn slot_u16(&self) -> u16 {
            u16::MAX
        }
        #[qslot]
        fn slot_i32(&self) -> i32 {
            i32::MIN
        }
        #[qslot]
        fn slot_u32(&self) -> u32 {
            u32::MAX
        }
        #[qslot]
        fn slot_i64(&self) -> i64 {
            MIN_SAFE_INTEGER
        }
        #[qslot]
        fn slot_u64(&self) -> u64 {
            MAX_SAFE_INTEGER as u64
        }
        #[qslot]
        fn slot_isize(&self) -> isize {
            MIN_SAFE_INTEGER as isize
        }
        #[qslot]
        fn slot_usize(&self) -> usize {
            MAX_SAFE_INTEGER as usize
        }
        #[qslot]
        fn slot_f32(&self) -> f32 {
            0.5
        }
        #[qslot]
        fn slot_f64(&self) -> f64 {
            0.125
        }
        #[qslot]
        fn slot_string(&self) -> String {
            "def".to_owned()
        }

    }
}

pub use test_object::TestObject;

fn get_qml_code_for(slot_type_suffix: &str) -> String {
    format!(r#"
        import QtQuick
        Item {{
            required property var testObject
            Component.onCompleted: {{
                testObject.slot{slot_type_suffix}Value = testObject.slot{slot_type_suffix}();
            }}
        }}
    "#)
}

fn test_slot_return<T, VariantT>(expected: T)
where
    T: Debug + PartialEq + Sized + TryFrom<VariantT>,
    <T as TryFrom<VariantT>>::Error: Debug,
    VariantT: Debug + QVariantValue,
{
    let type_str = get_type_name::<T>();
    // Capitalize the first char of the type name.
    let suffix = capitalize_first_char(&type_str);

    // Patch qml code.
    let qml = get_qml_code_for(&suffix);

    // Instantiate a test object.
    let obj = TestObject::default_with_attached_qobject();

    // Init QApp with QML code for the given signal.
    let mut app = QApp::new();
    let obj_var = obj.borrow().as_qvariant();
    app.add_initial_property("testObject", &obj_var)
       .load_qml(qml.as_bytes());

    // Read the value returned from the slot and stored to the dedicated property.
    let result_var = unsafe { &*obj.borrow()
        .get_qobject_ptr() }
        .property(&format!("slot{suffix}Value"));

    // Check returned value.
    assert!(result_var.is_valid());
    let result: T = result_var.value::<VariantT>()
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(expected, result);

}

fn main() {
    if cfg!(miri) {
        return;
    }
    test_slot_return::<bool, bool>(true);
    test_slot_return::<i8, i8>(i8::MIN);
    test_slot_return::<u8, u8>(u8::MAX);
    test_slot_return::<i16, i16>(i16::MIN);
    test_slot_return::<u16, u16>(u16::MAX);
    test_slot_return::<i32, i32>(i32::MIN);
    test_slot_return::<u32, u32>(u32::MAX);
    test_slot_return::<i64, i64>(MIN_SAFE_INTEGER);
    test_slot_return::<u64, u64>(MAX_SAFE_INTEGER as u64);
    test_slot_return::<isize, i64>(MIN_SAFE_INTEGER as isize);
    test_slot_return::<usize, u64>(MAX_SAFE_INTEGER as usize);
    test_slot_return::<f32, f32>(0.5);
    test_slot_return::<f64, f64>(0.125);
    test_slot_return::<String, QString>("def".into());
}
