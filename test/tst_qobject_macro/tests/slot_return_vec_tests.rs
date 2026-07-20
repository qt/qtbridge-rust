// // Copyright (C) 2026 The Qt Company Ltd.
// // SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use std::fmt::Debug;
use qtbridge::{QApp, QObjectHolder, qobject};
use qtbridge::qtbridge_type_lib::QVariant;
use crate::common::{capitalize_first_char, get_type_name};

#[qobject(ConvertToCamelCase)]
pub mod test_object {

    #[derive(Default)]
    pub struct TestObject {
        pub bool_return: Vec<bool>,
        pub i8_return: Vec<i8>,
        pub u8_return: Vec<u8>,
        pub i16_return: Vec<i16>,
        pub u16_return: Vec<u16>,
        pub i32_return: Vec<i32>,
        pub u32_return: Vec<u32>,
        pub i64_return: Vec<i64>,
        pub u64_return: Vec<u64>,
        pub isize_return: Vec<isize>,
        pub usize_return: Vec<usize>,
        pub f32_return: Vec<f32>,
        pub f64_return: Vec<f64>,
        pub string_return: Vec<String>,
    }

    impl TestObject {
        #[qslot]
        fn slot_vec_bool(&self) -> Vec<bool> {
            vec![false, true, true, false]
        }
        #[qslot]
        fn slot_vec_i8(&self) -> Vec<i8> {
            vec![-1, 0, 1, 2]
        }
        #[qslot]
        fn slot_vec_u8(&self) -> Vec<u8> {
            vec![3, 4, 5]
        }
        #[qslot]
        fn slot_vec_i16(&self) -> Vec<i16> {
            vec![-6, 7, 8, 9]
        }
        #[qslot]
        fn slot_vec_u16(&self) -> Vec<u16> {
            vec![10, 11, 12]
        }
        #[qslot]
        fn slot_vec_i32(&self) -> Vec<i32> {
            vec![13, -14, 15, -16]
        }
        #[qslot]
        fn slot_vec_u32(&self) -> Vec<u32> {
            vec![17, 18, 19]
        }
        #[qslot]
        fn slot_vec_i64(&self) -> Vec<i64> {
            vec![-20, 21, 22]
        }
        #[qslot]
        fn slot_vec_u64(&self) -> Vec<u64> {
            vec![23, 24, 25]
        }
        #[qslot]
        fn slot_vec_isize(&self) -> Vec<isize> {
            vec![-26, 27, 28]
        }
        #[qslot]
        fn slot_vec_usize(&self) -> Vec<usize> {
            vec![29, 30, 31]
        }
        #[qslot]
        fn slot_vec_f32(&self) -> Vec<f32> {
            vec![0.5, 0.25, 0.125]
        }
        #[qslot]
        fn slot_vec_f64(&self) -> Vec<f64> {
            vec![0.25, 0.125, 0.0625]
        }
        #[qslot]
        fn slot_vec_string(&self) -> Vec<String> {
            ["zero", "um", "dois", "três", "quatro"]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect()
        }

        qproperty!("slotVecBoolReturn", Member = bool_return);
        qproperty!("slotVecI8Return", Member = i8_return);
        qproperty!("slotVecU8Return", Member = u8_return);
        qproperty!("slotVecI16Return", Member = i16_return);
        qproperty!("slotVecU16Return", Member = u16_return);
        qproperty!("slotVecI32Return", Member = i32_return);
        qproperty!("slotVecU32Return", Member = u32_return);
        qproperty!("slotVecI64Return", Member = i64_return);
        qproperty!("slotVecU64Return", Member = u64_return);
        qproperty!("slotVecIsizeReturn", Member = isize_return);
        qproperty!("slotVecUsizeReturn", Member = usize_return);
        qproperty!("slotVecF32Return", Member = f32_return);
        qproperty!("slotVecF64Return", Member = f64_return);
        qproperty!("slotVecStringReturn", Member = string_return);
    }
}

pub use test_object::TestObject;

fn get_qml_code_for(slot_type_suffix: &str) -> String {
    format!(r#"
        import QtQuick
        Item {{
            required property var testObject
            Component.onCompleted: {{
                testObject.slotVec{slot_type_suffix}Return = testObject.slotVec{slot_type_suffix}();
            }}
        }}
    "#)
}

fn test_slot_return_impl<T, VarT>(expected: &[T])
where
    T: Debug + PartialEq + TryFrom<VarT>,
    <T as TryFrom<VarT>>::Error: Debug,
    Vec<T> :FromIterator<T>,
    Vec<VarT>: TryFrom<QVariant>,
    <Vec<VarT> as TryFrom<QVariant>>::Error: Debug,
{
    let type_str = get_type_name::<T>();
    let suffix = capitalize_first_char(&type_str);

    // Patch qml code.
    let qml = get_qml_code_for(&suffix);

    // Create a test object
    let obj = TestObject::default_with_attached_qobject();

    // Init QApp with QML code for the given signal.
    let mut app = QApp::new();
    let obj_var = obj.borrow().as_qvariant();
    app.add_initial_property("testObject", &obj_var)
       .load_qml(qml.as_bytes());

    // Read the value returned from the slot and stored to the dedicated property.
    let result_var = unsafe { &*obj.borrow()
        .get_qobject_ptr() }
        .property(&format!("slotVec{suffix}Return"));

    // Check returned value.
    assert!(result_var.is_valid());
    let result: Vec<T> = <Vec<VarT>>::try_from(result_var)
        .unwrap()
        .into_iter()
        .map(|a| T::try_from(a).unwrap())
        .collect();
    assert_eq!(expected, result);
}

fn main() {
    if cfg!(miri) {
        return;
    }
    test_slot_return_impl::<bool, bool>(&[false, true, true, false]);
    test_slot_return_impl::<i8, i8>(&[-1, 0, 1, 2]);
    test_slot_return_impl::<u8, u8>(&[3, 4, 5]);
    test_slot_return_impl::<i16, i16>(&[-6, 7, 8, 9]);
    test_slot_return_impl::<u16, u16>(&[10, 11, 12]);
    test_slot_return_impl::<i32, i32>(&[13, -14, 15, -16]);
    test_slot_return_impl::<u32, u32>(&[17, 18, 19]);
    test_slot_return_impl::<i64, i64>(&[-20, 21, 22]);
    test_slot_return_impl::<u64, u64>(&[23, 24, 25]);
    test_slot_return_impl::<isize, i64>(&[-26, 27, 28]);
    test_slot_return_impl::<usize, u64>(&[29, 30, 31]);
    test_slot_return_impl::<f32, f32>(&[0.5, 0.25, 0.125]);
    test_slot_return_impl::<f64, f64>(&[0.25, 0.125, 0.0625]);
    test_slot_return_impl::<String, String>(&["zero", "um", "dois", "três", "quatro"].map(ToOwned::to_owned));
}
