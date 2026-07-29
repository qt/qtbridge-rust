// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QByteArray, QJsonArray, QJsonObject, QJsonValue, QList, QMap_QString_QVariant, QMetaType, QObject, QObjectList, QString};
use std::mem::MaybeUninit;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        #[allow(dead_code)]
        type QVariant = super::QVariant;
        include!("qtbridge-type-lib/src/generated/core/qjsonarray/cpp/qjsonarray.h");
        type QJsonArray = crate::QJsonArray;
        include!("qtbridge-type-lib/src/generated/core/qjsonobject/cpp/qjsonobject.h");
        type QJsonObject = crate::QJsonObject;
        include!("qtbridge-type-lib/src/generated/core/qjsonvalue/cpp/qjsonvalue.h");
        type QJsonValue = crate::QJsonValue;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist.h");
        type QList_QByteArray = crate::QList_QByteArray;
        type QList_QString = crate::QList_QString;
        type QList_bool = crate::QList_bool;
        type QList_f32 = crate::QList_f32;
        type QList_f64 = crate::QList_f64;
        type QList_i16 = crate::QList_i16;
        type QList_i32 = crate::QList_i32;
        type QList_i64 = crate::QList_i64;
        type QList_i8 = crate::QList_i8;
        include!("cxx-qt-lib/core/qlist/QList_QObjectMutPtr.h");
        type QList_QObjectMutPtr = crate::QList_QObjectMutPtr;
        type QList_u16 = crate::QList_u16;
        type QList_u32 = crate::QList_u32;
        type QList_u64 = crate::QList_u64;
        type QList_u8 = crate::QList_u8;
        include!("qtbridge-type-lib/src/generated/core/qmap/cpp/qmap_qstring_qvariant.h");
        type QMap_QString_QVariant = crate::QMap_QString_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = crate::QObject;
    }
    #[namespace = "rust::bridge::qvariant"]
    unsafe extern "C++" {
        # [rust_name = qvariant_drop]
        fn QVariant_Drop(v: &mut QVariant);
        # [rust_name = qvariant_default]
        fn QVariant_Default() -> QVariant;
        # [rust_name = qvariant_clone]
        fn QVariant_Clone(v: &QVariant) -> QVariant;
        # [rust_name = inline_cpp_fn_is_valid]
        fn inlineCppFn_is_valid(_obj: &QVariant) -> bool;
        # [rust_name = inline_cpp_fn_meta_type]
        fn inlineCppFn_meta_type(_obj: &QVariant) -> QMetaType;
        # [rust_name = inline_cpp_fn_trait_impl_to_string_for_qvariant_to_string]
        fn inlineCppFn_TraitImpl_ToString_for_QVariant_to_string(_obj: &QVariant) -> String;
        # [rust_name = inline_cpp_fn_trait_impl_from_string_slice_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_string_slice_for_QVariant_from(from: &str) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_string_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_String_for_QVariant_from(from: &Vec<String>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_string_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_String_try_from(from: &QVariant, result: &mut String) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_string_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_String_try_from(from: &QVariant, result: &mut Vec<String>) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_bool_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_bool_for_QVariant_from(value: &bool) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_i8_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_i8_for_QVariant_from(value: &i8) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_u8_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_u8_for_QVariant_from(value: &u8) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_i16_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_i16_for_QVariant_from(value: &i16) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_u16_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_u16_for_QVariant_from(value: &u16) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_i32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_i32_for_QVariant_from(value: &i32) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_u32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_u32_for_QVariant_from(value: &u32) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_i64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_i64_for_QVariant_from(value: &i64) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_u64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_u64_for_QVariant_from(value: &u64) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_isize_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_isize_for_QVariant_from(value: &isize) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_usize_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_usize_for_QVariant_from(value: &usize) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_f32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_f32_for_QVariant_from(value: &f32) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_f64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_f64_for_QVariant_from(value: &f64) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_ptr_mut_qobject_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_ptr_mut_QObject_for_QVariant_from(value: &*mut QObject) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qobject_list_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QObjectList_for_QVariant_from(value: &QList_QObjectMutPtr) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_bool_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_bool_for_QVariant_from(value: &Vec<bool>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_i8_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_i8_for_QVariant_from(value: &Vec<i8>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_u8_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_u8_for_QVariant_from(value: &Vec<u8>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_i16_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_i16_for_QVariant_from(value: &Vec<i16>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_u16_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_u16_for_QVariant_from(value: &Vec<u16>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_i32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_i32_for_QVariant_from(value: &Vec<i32>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_u32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_u32_for_QVariant_from(value: &Vec<u32>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_i64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_i64_for_QVariant_from(value: &Vec<i64>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_u64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_u64_for_QVariant_from(value: &Vec<u64>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_f32_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_f32_for_QVariant_from(value: &Vec<f32>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_f64_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_f64_for_QVariant_from(value: &Vec<f64>) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_qbyte_array_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_QByteArray_for_QVariant_from(list: QList_QByteArray) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_vec_qstring_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_Vec_QString_for_QVariant_from(list: QList_QString) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_bool_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_bool_try_from(from: &QVariant, result: &mut bool) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i8_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i8_try_from(from: &QVariant, result: &mut i8) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u8_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u8_try_from(from: &QVariant, result: &mut u8) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i16_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i16_try_from(from: &QVariant, result: &mut i16) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u16_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u16_try_from(from: &QVariant, result: &mut u16) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i32_try_from(from: &QVariant, result: &mut i32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u32_try_from(from: &QVariant, result: &mut u32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i64_try_from(from: &QVariant, result: &mut i64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u64_try_from(from: &QVariant, result: &mut u64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_isize_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_isize_try_from(from: &QVariant, result: &mut isize) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_usize_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_usize_try_from(from: &QVariant, result: &mut usize) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_f32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_f32_try_from(from: &QVariant, result: &mut f32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_f64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_f64_try_from(from: &QVariant, result: &mut f64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_ptr_mut_qobject_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_ptr_mut_QObject_try_from(from: &QVariant, result: &mut *mut QObject) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qobject_list_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QObjectList_try_from(from: &QVariant, result: &mut QList_QObjectMutPtr) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_bool_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_bool_try_from(from: &QVariant, result: &mut QList_bool) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i8_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i8_try_from(from: &QVariant, result: &mut QList_i8) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u8_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u8_try_from(from: &QVariant, result: &mut QList_u8) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i16_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i16_try_from(from: &QVariant, result: &mut QList_i16) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u16_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u16_try_from(from: &QVariant, result: &mut QList_u16) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i32_try_from(from: &QVariant, result: &mut QList_i32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u32_try_from(from: &QVariant, result: &mut QList_u32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i64_try_from(from: &QVariant, result: &mut QList_i64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u64_try_from(from: &QVariant, result: &mut QList_u64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_f32_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_f32_try_from(from: &QVariant, result: &mut QList_f32) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_f64_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_f64_try_from(from: &QVariant, result: &mut QList_f64) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qjson_array_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QJsonArray_for_QVariant_from(value: &QJsonArray) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_array_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonArray_try_from(from: &QVariant, result: &mut QJsonArray) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qjson_object_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QJsonObject_for_QVariant_from(value: &QJsonObject) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_object_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonObject_try_from(from: &QVariant, result: &mut QJsonObject) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qjson_value_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QJsonValue_for_QVariant_from(value: &QJsonValue) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_value_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonValue_try_from(from: &QVariant, result: &mut QJsonValue) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qmap_qstring_qvariant_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QMap_QString_QVariant_try_from(from: &QVariant, result: &mut QMap_QString_QVariant) -> bool;
    }
}
/// The QVariant struct acts like an enum for the most common Qt data types.
///
/// QVariant represents dynamically typed value container.
/// It holds a value of an arbitrary supported type and allows
/// runtime type inspection and safe conversion from/to compatible types.
///
/// In the context of qtbridge, `QVariant` is primarily used for data exchange between the Rust backend and the Qml Engine.
/// This kind of data transfer occur in the following areas of implementation internals:
/// * item models
/// * exposing user-defined structures to Qml via signals/slots/properties
///
/// In the user code, you may encounter `QVariant` when using [QQmlApplicationEngine::set_initial_properties][crate::QQmlApplicationEngine::set_initial_properties].
///
/// # Examples
/// ```
/// # use qtbridge_type_lib::QVariant;
/// let var = QVariant::from("123");
/// let converted: i32 = var.try_into()
///     .expect("Conversion failed");
/// assert_eq!(converted, 123);
/// ```
///
/// See also: [QVariant documentation](https://doc.qt.io/qt-6/qvariant.html).
#[repr(C)]
pub struct QVariant {
    _content: MaybeUninit<[u8; 32]>,
}
unsafe impl cxx::ExternType for QVariant {
    type Id = cxx::type_id!("QVariant");
    type Kind = cxx::kind::Trivial;
}
impl Drop for QVariant {
    fn drop(&mut self) {
        ffi::qvariant_drop(self)
    }
}
impl Default for QVariant {
    fn default() -> Self {
        ffi::qvariant_default()
    }
}
impl Clone for QVariant {
    fn clone(&self) -> Self {
        ffi::qvariant_clone(self)
    }
}
impl ToString for QVariant {
    fn to_string(&self) -> String {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_to_string_for_qvariant_to_string;
        conv_fn(self)
    }
}
impl From<&()> for QVariant {
    fn from(_: &()) -> Self {
        QVariant::default()
    }
}
impl From<&str> for QVariant {
    fn from(value: &str) -> Self {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_from_string_slice_for_qvariant_from;
        conv_fn(value)
    }
}
impl From<&String> for QVariant {
    fn from(value: &String) -> Self {
        QVariant::from(value.as_str())
    }
}
impl From<&Vec<String>> for QVariant {
    fn from(value: &Vec<String>) -> Self {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_from_ref_vec_string_for_qvariant_from;
        conv_fn(value)
    }
}
impl TryFrom<&QVariant> for () {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
        match value.is_valid() {
            true => Err(()),
            false => Ok(()),
        }
    }
}
impl TryFrom<&QVariant> for String {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_string_try_from;
        let mut result = String::new();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for Vec<String> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_string_try_from;
        let mut result = Vec::<String>::new();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl From<&bool> for QVariant {
    fn from(value: &bool) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_bool_for_qvariant_from(value)
    }
}
impl From<&i8> for QVariant {
    fn from(value: &i8) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_i8_for_qvariant_from(value)
    }
}
impl From<&u8> for QVariant {
    fn from(value: &u8) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_u8_for_qvariant_from(value)
    }
}
impl From<&i16> for QVariant {
    fn from(value: &i16) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_i16_for_qvariant_from(value)
    }
}
impl From<&u16> for QVariant {
    fn from(value: &u16) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_u16_for_qvariant_from(value)
    }
}
impl From<&i32> for QVariant {
    fn from(value: &i32) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_i32_for_qvariant_from(value)
    }
}
impl From<&u32> for QVariant {
    fn from(value: &u32) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_u32_for_qvariant_from(value)
    }
}
impl From<&i64> for QVariant {
    fn from(value: &i64) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_i64_for_qvariant_from(value)
    }
}
impl From<&u64> for QVariant {
    fn from(value: &u64) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_u64_for_qvariant_from(value)
    }
}
impl From<&isize> for QVariant {
    fn from(value: &isize) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_isize_for_qvariant_from(value)
    }
}
impl From<&usize> for QVariant {
    fn from(value: &usize) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_usize_for_qvariant_from(value)
    }
}
impl From<&f32> for QVariant {
    fn from(value: &f32) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_f32_for_qvariant_from(value)
    }
}
impl From<&f64> for QVariant {
    fn from(value: &f64) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_f64_for_qvariant_from(value)
    }
}
impl From<&*mut QObject> for QVariant {
    fn from(value: &*mut QObject) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_ptr_mut_qobject_for_qvariant_from(value)
    }
}
impl From<&QObjectList> for QVariant {
    fn from(value: &QObjectList) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_qobject_list_for_qvariant_from(value)
    }
}
impl From<bool> for QVariant {
    fn from(value: bool) -> Self {
        QVariant::from(&value)
    }
}
impl From<i8> for QVariant {
    fn from(value: i8) -> Self {
        QVariant::from(&value)
    }
}
impl From<u8> for QVariant {
    fn from(value: u8) -> Self {
        QVariant::from(&value)
    }
}
impl From<i16> for QVariant {
    fn from(value: i16) -> Self {
        QVariant::from(&value)
    }
}
impl From<u16> for QVariant {
    fn from(value: u16) -> Self {
        QVariant::from(&value)
    }
}
impl From<i32> for QVariant {
    fn from(value: i32) -> Self {
        QVariant::from(&value)
    }
}
impl From<u32> for QVariant {
    fn from(value: u32) -> Self {
        QVariant::from(&value)
    }
}
impl From<i64> for QVariant {
    fn from(value: i64) -> Self {
        QVariant::from(&value)
    }
}
impl From<u64> for QVariant {
    fn from(value: u64) -> Self {
        QVariant::from(&value)
    }
}
impl From<f32> for QVariant {
    fn from(value: f32) -> Self {
        QVariant::from(&value)
    }
}
impl From<f64> for QVariant {
    fn from(value: f64) -> Self {
        QVariant::from(&value)
    }
}
impl From<isize> for QVariant {
    fn from(value: isize) -> Self {
        QVariant::from(&value)
    }
}
impl From<usize> for QVariant {
    fn from(value: usize) -> Self {
        QVariant::from(&value)
    }
}
impl From<String> for QVariant {
    fn from(value: String) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<String>> for QVariant {
    fn from(value: Vec<String>) -> Self {
        QVariant::from(&value)
    }
}
impl From<*mut QObject> for QVariant {
    fn from(value: *mut QObject) -> Self {
        QVariant::from(&value)
    }
}
impl From<QObjectList> for QVariant {
    fn from(value: QObjectList) -> Self {
        QVariant::from(&value)
    }
}
impl From<&Vec<bool>> for QVariant {
    fn from(value: &Vec<bool>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_bool_for_qvariant_from(value)
    }
}
impl From<&Vec<i8>> for QVariant {
    fn from(value: &Vec<i8>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_i8_for_qvariant_from(value)
    }
}
impl From<&Vec<u8>> for QVariant {
    fn from(value: &Vec<u8>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_u8_for_qvariant_from(value)
    }
}
impl From<&Vec<i16>> for QVariant {
    fn from(value: &Vec<i16>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_i16_for_qvariant_from(value)
    }
}
impl From<&Vec<u16>> for QVariant {
    fn from(value: &Vec<u16>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_u16_for_qvariant_from(value)
    }
}
impl From<&Vec<i32>> for QVariant {
    fn from(value: &Vec<i32>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_i32_for_qvariant_from(value)
    }
}
impl From<&Vec<u32>> for QVariant {
    fn from(value: &Vec<u32>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_u32_for_qvariant_from(value)
    }
}
impl From<&Vec<i64>> for QVariant {
    fn from(value: &Vec<i64>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_i64_for_qvariant_from(value)
    }
}
impl From<&Vec<u64>> for QVariant {
    fn from(value: &Vec<u64>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_u64_for_qvariant_from(value)
    }
}
impl From<&Vec<f32>> for QVariant {
    fn from(value: &Vec<f32>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_f32_for_qvariant_from(value)
    }
}
impl From<&Vec<f64>> for QVariant {
    fn from(value: &Vec<f64>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_vec_f64_for_qvariant_from(value)
    }
}
impl From<&Vec<QByteArray>> for QVariant {
    fn from(value: &Vec<QByteArray>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_vec_qbyte_array_for_qvariant_from;
        cpp(QList::from(value))
    }
}
impl From<&Vec<QString>> for QVariant {
    fn from(value: &Vec<QString>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_vec_qstring_for_qvariant_from;
        cpp(QList::from(value))
    }
}
impl From<Vec<bool>> for QVariant {
    fn from(value: Vec<bool>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<i8>> for QVariant {
    fn from(value: Vec<i8>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<u8>> for QVariant {
    fn from(value: Vec<u8>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<i16>> for QVariant {
    fn from(value: Vec<i16>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<u16>> for QVariant {
    fn from(value: Vec<u16>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<i32>> for QVariant {
    fn from(value: Vec<i32>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<u32>> for QVariant {
    fn from(value: Vec<u32>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<i64>> for QVariant {
    fn from(value: Vec<i64>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<u64>> for QVariant {
    fn from(value: Vec<u64>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<f32>> for QVariant {
    fn from(value: Vec<f32>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<f64>> for QVariant {
    fn from(value: Vec<f64>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<QByteArray>> for QVariant {
    fn from(value: Vec<QByteArray>) -> Self {
        QVariant::from(&value)
    }
}
impl From<Vec<QString>> for QVariant {
    fn from(value: Vec<QString>) -> Self {
        QVariant::from(&value)
    }
}
impl TryFrom<&QVariant> for bool {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_bool_try_from;
        let mut x = <bool>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for i8 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i8_try_from;
        let mut x = <i8>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for u8 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u8_try_from;
        let mut x = <u8>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for i16 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i16_try_from;
        let mut x = <i16>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for u16 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u16_try_from;
        let mut x = <u16>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for i32 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i32_try_from;
        let mut x = <i32>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for u32 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u32_try_from;
        let mut x = <u32>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for i64 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_i64_try_from;
        let mut x = <i64>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for u64 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_u64_try_from;
        let mut x = <u64>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for isize {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_isize_try_from;
        let mut x = <isize>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for usize {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_usize_try_from;
        let mut x = <usize>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for f32 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_f32_try_from;
        let mut x = <f32>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for f64 {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_f64_try_from;
        let mut x = <f64>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for *mut QObject {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_ptr_mut_qobject_try_from;
        let mut x = <*mut QObject>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for QObjectList {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let convert_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qobject_list_try_from;
        let mut x = <QObjectList>::default();
        match convert_fn(value, &mut x) {
            true => Ok(x),
            false => Err(()),
        }
    }
}
impl TryFrom<&QVariant> for QMap_QString_QVariant {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qmap_qstring_qvariant_try_from;
        let mut result = QMap_QString_QVariant::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}

fn qvariant_to_vec<T, ConvertFn>(src: &QVariant, convert_fn: ConvertFn) -> Result<Vec<T>, ()>
where
    T: Clone + cxx_qt_lib::QListElement,
    ConvertFn: FnOnce(&QVariant, &mut QList<T>) -> bool,
{
    let mut list = QList::default();
    if !convert_fn(src, &mut list) {
        return Err(())
    }

    let vec = list.iter()
        .cloned()
        .collect();
    Ok(vec)
}

impl TryFrom<&QVariant> for Vec<bool> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_bool_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<i8> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i8_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<u8> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u8_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<i16> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i16_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<u16> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u16_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<i32> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i32_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<u32> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u32_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<i64> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_i64_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<u64> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_u64_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<f32> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_f32_try_from)
    }
}
impl TryFrom<&QVariant> for Vec<f64> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        qvariant_to_vec(value, ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_vec_f64_try_from)
    }
}
impl TryFrom<QVariant> for bool {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for i8 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for u8 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for i16 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for u16 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for i32 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for u32 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for i64 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for u64 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for isize {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for usize {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for f32 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for f64 {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for String {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for *mut QObject {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for QObjectList {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<bool> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<i8> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<u8> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<i16> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<u16> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<i32> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<u32> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<i64> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<u64> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<f32> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<f64> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl TryFrom<QVariant> for Vec<String> {
    type Error = ();
    fn try_from(value: QVariant) -> Result<Self, ()> {
        Self::try_from(&value)
    }
}
impl From<&QJsonArray> for QVariant {
    fn from(value: &QJsonArray) -> QVariant {
        ffi::inline_cpp_fn_trait_impl_from_ref_qjson_array_for_qvariant_from(value)
    }
}
impl TryFrom<&QVariant> for QJsonArray {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_array_try_from;
        let mut result = QJsonArray::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl From<&QJsonObject> for QVariant {
    fn from(value: &QJsonObject) -> QVariant {
        ffi::inline_cpp_fn_trait_impl_from_ref_qjson_object_for_qvariant_from(value)
    }
}
impl TryFrom<&QVariant> for QJsonObject {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_object_try_from;
        let mut result = QJsonObject::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl From<&QJsonValue> for QVariant {
    fn from(value: &QJsonValue) -> QVariant {
        ffi::inline_cpp_fn_trait_impl_from_ref_qjson_value_for_qvariant_from(value)
    }
}
impl TryFrom<&QVariant> for QJsonValue {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qjson_value_try_from;
        let mut result = QJsonValue::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl QVariant {
    #[allow(dead_code)]
    /// Returns `true` if this object holds some value or false otherwise.
    ///
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QVariant;
    /// let var_default = QVariant::default();
    /// assert!(!var_default.is_valid());
    /// let var_int = QVariant::from(42);
    /// assert!(var_int.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let cpp = ffi::inline_cpp_fn_is_valid;
        cpp(self)
    }
    #[allow(dead_code)]
    pub fn meta_type(&self) -> QMetaType {
        let cpp = ffi::inline_cpp_fn_meta_type;
        cpp(self)
    }

    pub fn to_cxx_qt(&self) -> cxx_qt_lib::QVariant {
        Self::into_cxx_qt(self.clone())
    }

    pub fn into_cxx_qt(self) -> cxx_qt_lib::QVariant {
        unsafe { std::mem::transmute(self) }
    }

    pub fn from_cxx_qt(src: &cxx_qt_lib::QVariant) -> Self {
        unsafe { std::mem::transmute(src.clone()) }
    }
}
