// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QListImpl};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [f64].
pub type QList_f64 = QList<f64>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_f64.h");
        #[allow(dead_code)]
        type QList_f64 = super::QList_f64;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
    }
    #[namespace = "rust::bridge::qlist_f64"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_f64);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_f64;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_f64) -> QList_f64;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_f64, rhs: &QList_f64) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_f64, value: f64);
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_f64);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_f64, value: &f64) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_f64, value: f64);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_f64, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_f64, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_f64) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_f64) -> &f64;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_f64) -> &f64;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qlist_f64_for_vec_f64_from]
        fn inlineCppFn_TraitImpl_From_ref_QList_f64_for_Vec_f64_from(src: &QList_f64) -> Vec<f64>;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_f64_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_f64_index(_obj: &QList_f64, index: usize) -> *const f64;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_f64_n_for_qlist_f64_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_f64_N_for_QList_f64_eq(_obj: &QList_f64, rhs: &[f64]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_f64 {
    type Id = cxx::type_id!("QList_f64");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_f64 {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_f64 {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_f64 {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[f64]> for QList<f64> {
    fn from(value: &[f64]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(*item);
        }
        result
    }
}
impl<const N: usize> From<[f64; N]> for QList<f64> {
    fn from(value: [f64; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<f64>> for QList<f64> {
    fn from(value: &Vec<f64>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<f64>> for QList<f64> {
    fn from(value: Vec<f64>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<f64>> for Vec<f64> {
    fn from(value: &QList<f64>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_qlist_f64_for_vec_f64_from;
        cpp(value)
    }
}
impl From<QList<f64>> for Vec<f64> {
    fn from(value: QList<f64>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<f64> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_f64_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[f64; N]> for QList<f64> {
    fn eq(&self, other: &[f64; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_f64_n_for_qlist_f64_eq;
        cpp(self, other)
    }
}
impl QListImpl<f64> for QList_f64 {
    fn append(&mut self, value: f64) {
        let cpp = ffi::inline_cpp_fn_append;
        cpp(self, value);
    }
    fn clear(&mut self) {
        let cpp = ffi::inline_cpp_fn_clear;
        cpp(self);
    }
    fn contains(&self, value: &f64) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: f64) {
        let cpp = ffi::inline_cpp_fn_push_back;
        cpp(self, value);
    }
    fn remove(&mut self, i: isize) {
        let cpp = ffi::inline_cpp_fn_remove;
        cpp(self, i, 1);
    }
    fn reserve(&mut self, size: usize) {
        let cpp = ffi::inline_cpp_fn_reserve;
        cpp(self, size);
    }
    fn size(&self) -> isize {
        let cpp = ffi::inline_cpp_fn_size;
        cpp(self)
    }
    fn first(&self) -> &f64 {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &f64 {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
