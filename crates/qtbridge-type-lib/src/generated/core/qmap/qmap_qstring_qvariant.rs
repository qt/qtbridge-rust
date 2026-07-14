// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QList_QString, QList_QVariant, QMap, QMapImpl, QString, QVariant};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QMap] for types [QString], [QVariant].
pub type QMap_QString_QVariant = QMap<QString, QVariant>;
/// This is an alias for type [QMap] for types [QString], [QVariant].
pub type QVariantMap = QMap<QString, QVariant>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qmap/cpp/qmap_qstring_qvariant.h");
        #[allow(dead_code)]
        type QMap_QString_QVariant = super::QMap_QString_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h");
        type QList_QString = crate::QList_QString;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qvariant.h");
        type QList_QVariant = crate::QList_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        type QString = crate::QString;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = crate::QVariant;
    }
    #[namespace = "rust::bridge::qmap_qstring_qvariant"]
    unsafe extern "C++" {
        # [rust_name = qmap_drop]
        fn QMap_Drop(v: &mut QMap_QString_QVariant);
        # [rust_name = qmap_default]
        fn QMap_Default() -> QMap_QString_QVariant;
        # [rust_name = qmap_clone]
        fn QMap_Clone(v: &QMap_QString_QVariant) -> QMap_QString_QVariant;
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QMap_QString_QVariant);
        # [rust_name = inline_cpp_fn_insert]
        fn inlineCppFn_insert(_obj: &mut QMap_QString_QVariant, key: &QString, value: &QVariant);
        # [rust_name = inline_cpp_fn_is_empty]
        fn inlineCppFn_is_empty(_obj: &QMap_QString_QVariant) -> bool;
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QMap_QString_QVariant, key: &QString) -> i32;
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QMap_QString_QVariant) -> i32;
        # [rust_name = inline_cpp_fn_keys]
        fn inlineCppFn_keys(_obj: &QMap_QString_QVariant) -> QList_QString;
        # [rust_name = inline_cpp_fn_values]
        fn inlineCppFn_values(_obj: &QMap_QString_QVariant) -> QList_QVariant;
        # [rust_name = inline_cpp_fn_value]
        fn inlineCppFn_value(_obj: &QMap_QString_QVariant, key: &QString) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qmap_qstring_qvariant_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QMap_QString_QVariant_for_QVariant_from(value: &QMap_QString_QVariant) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qmap_qstring_qvariant_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QMap_QString_QVariant_try_from(from: &QVariant, result: &mut QMap_QString_QVariant) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_ref_qstring_for_qmap_qstring_qvariant_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_ref_QString_for_QMap_QString_QVariant_index(_obj: &QMap_QString_QVariant, key: &QString) -> *const QVariant;
    }
}
unsafe impl cxx::ExternType for QMap_QString_QVariant {
    type Id = cxx::type_id!("QMap_QString_QVariant");
    type Kind = cxx::kind::Trivial;
}
impl Default for QMap_QString_QVariant {
    fn default() -> Self {
        ffi::qmap_default()
    }
}
impl Clone for QMap_QString_QVariant {
    fn clone(&self) -> Self {
        ffi::qmap_clone(self)
    }
}
impl From<(QString, QVariant)> for QMap<QString, QVariant> {
    fn from(src: (QString, QVariant)) -> Self {
        let mut result = Self::default();
        result.insert(&src.0, &src.1);
        result
    }
}
impl From<&[(QString, QVariant)]> for QMap<QString, QVariant> {
    fn from(src: &[(QString, QVariant)]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(k, v));
        result
    }
}
impl From<(&str, QVariant)> for QMap<QString, QVariant> {
    fn from(src: (&str, QVariant)) -> Self {
        let mut result = Self::default();
        result.insert(&QString::from(src.0), &src.1);
        result
    }
}
impl From<&[(&str, QVariant)]> for QMap<QString, QVariant> {
    fn from(src: &[(&str, QVariant)]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(&QString::from(*k), v));
        result
    }
}
impl<const N: usize> From<[(&str, QVariant); N]> for QMap<QString, QVariant> {
    fn from(src: [(&str, QVariant); N]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(&QString::from(*k), v));
        result
    }
}
impl From<&QMap<QString, QVariant>> for QVariant {
    fn from(value: &QMap<QString, QVariant>) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_qmap_qstring_qvariant_for_qvariant_from(value)
    }
}
impl TryFrom<&QVariant> for QMap<QString, QVariant> {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qmap_qstring_qvariant_try_from;
        let mut result = QMap::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl<const N: usize> From<[(QString, QVariant); N]> for QMap<QString, QVariant> {
    fn from(src: [(QString, QVariant); N]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(k, v));
        result
    }
}
impl std::ops::Index<&QString> for QMap<QString, QVariant> {
    type Output = QVariant;
    fn index(&self, index: &QString) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_ref_qstring_for_qmap_qstring_qvariant_index;
        unsafe { cpp(self, index).as_ref() }.expect("Given key does not exist in QMap")
    }
}
impl QMapImpl<QString, QVariant> for QMap_QString_QVariant {
    type QListK = QList<QString>;
    type QListV = QList_QVariant;

    fn clear(&mut self) {
        ffi::inline_cpp_fn_clear(self)
    }
    fn insert(&mut self, key: &QString, value: &QVariant) {
        let cpp = ffi::inline_cpp_fn_insert;
        cpp(self, key, value)
    }
    fn is_empty(&self) -> bool {
        ffi::inline_cpp_fn_is_empty(self)
    }
    fn remove(&mut self, key: &QString) -> i32 {
        let cpp = ffi::inline_cpp_fn_remove;
        cpp(self, key)
    }
    fn size(&self) -> i32 {
        ffi::inline_cpp_fn_size(self)
    }
    fn keys(&self) -> QList<QString> {
        let cpp = ffi::inline_cpp_fn_keys;
        cpp(self)
    }
    fn values(&self) -> QList_QVariant {
        let cpp = ffi::inline_cpp_fn_values;
        cpp(self)
    }
    fn value(&self, key: &QString) -> QVariant {
        let cpp = ffi::inline_cpp_fn_value;
        cpp(self, key)
    }
    fn do_drop(&mut self) {
        ffi::qmap_drop(self)
    }
}
