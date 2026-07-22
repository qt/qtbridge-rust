// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QVariant;
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [QVariant].
pub struct QList_QVariant {
    _d: std::mem::MaybeUninit<usize>,
    _ptr: std::mem::MaybeUninit<usize>,
    _size: std::mem::MaybeUninit<usize>,
}
/// This is an alias for type [QList] for type [QVariant].
pub type QVariantList = QList_QVariant;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qvariant.h");
        #[allow(dead_code)]
        type QList_QVariant = super::QList_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = crate::QVariant;
    }
    #[namespace = "rust::bridge::qlist_qvariant"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_QVariant);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_QVariant;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_QVariant) -> QList_QVariant;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_QVariant, rhs: &QList_QVariant) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_QVariant, value: QVariant);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_QVariant, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_QVariant) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_QVariant) -> &QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qlist_qvariant_for_qvariant_from]
        fn inlineCppFn_TraitImpl_From_ref_QList_QVariant_for_QVariant_from(value: &QList_QVariant) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qlist_qvariant_try_from]
        fn inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QList_QVariant_try_from(from: &QVariant, result: &mut QList_QVariant) -> bool;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qvariant_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_QVariant_index(_obj: &QList_QVariant, index: usize) -> *const QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_qvariant_n_for_qlist_qvariant_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_QVariant_N_for_QList_QVariant_eq(_obj: &QList_QVariant, rhs: &[QVariant]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_QVariant {
    type Id = cxx::type_id!("QList_QVariant");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_QVariant {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_QVariant {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl Drop for QList_QVariant {
    fn drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
impl PartialEq for QList_QVariant {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[QVariant]> for QList_QVariant {
    fn from(value: &[QVariant]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(item.clone());
        }
        result
    }
}
impl<const N: usize> From<[QVariant; N]> for QList_QVariant {
    fn from(value: [QVariant; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<QVariant>> for QList_QVariant {
    fn from(value: &Vec<QVariant>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<QVariant>> for QList_QVariant {
    fn from(value: Vec<QVariant>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList_QVariant> for Vec<QVariant> {
    fn from(value: &QList_QVariant) -> Self {
        let mut v = Vec::with_capacity(value.len());
        for i in 0..value.len() {
            v.push(value[i].clone());
        }
        v
    }
}
impl From<QList_QVariant> for Vec<QVariant> {
    fn from(value: QList_QVariant) -> Self {
        Self::from(&value)
    }
}
impl From<&QList_QVariant> for QVariant {
    fn from(value: &QList_QVariant) -> Self {
        ffi::inline_cpp_fn_trait_impl_from_ref_qlist_qvariant_for_qvariant_from(value)
    }
}
impl TryFrom<&QVariant> for QList_QVariant {
    type Error = ();
    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let conv_fn = ffi::inline_cpp_fn_trait_impl_try_from_ref_qvariant_for_qlist_qvariant_try_from;
        let mut result = QList_QVariant::default();
        match conv_fn(value, &mut result) {
            true => Ok(result),
            false => Err(()),
        }
    }
}
impl std::ops::Index<usize> for QList_QVariant {
    type Output = QVariant;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qvariant_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[QVariant; N]> for QList_QVariant {
    fn eq(&self, other: &[QVariant; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_qvariant_n_for_qlist_qvariant_eq;
        cpp(self, other)
    }
}
impl QList_QVariant {
    fn append(&mut self, value: QVariant) {
        let cpp = ffi::inline_cpp_fn_append;
        cpp(self, value);
    }
    pub fn first(&self) -> &QVariant {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    pub fn len(&self) -> usize {
        let cpp = ffi::inline_cpp_fn_size;
        cpp(self) as usize
    }
    fn reserve(&mut self, size: usize) {
        let cpp = ffi::inline_cpp_fn_reserve;
        cpp(self, size);
    }
}
