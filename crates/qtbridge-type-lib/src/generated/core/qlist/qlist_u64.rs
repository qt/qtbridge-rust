// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QListImpl};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [u64].
pub type QList_u64 = QList<u64>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_u64.h");
        #[allow(dead_code)]
        type QList_u64 = super::QList_u64;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
    }
    #[namespace = "rust::bridge::qlist_u64"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_u64);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_u64;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_u64) -> QList_u64;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_u64, rhs: &QList_u64) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_u64, value: u64);
        # [rust_name = inline_cpp_fn_capacity]
        fn inlineCppFn_capacity(_obj: &QList_u64) -> usize;
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_u64);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_u64, value: &u64) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_u64, value: u64);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_u64, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_u64, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_u64) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_u64) -> &u64;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_u64) -> &u64;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qlist_u64_for_vec_u64_from]
        fn inlineCppFn_TraitImpl_From_ref_QList_u64_for_Vec_u64_from(src: &QList_u64) -> Vec<u64>;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_u64_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u64_index(_obj: &QList_u64, index: usize) -> *const u64;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_u64_n_for_qlist_u64_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_u64_N_for_QList_u64_eq(_obj: &QList_u64, rhs: &[u64]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_u64 {
    type Id = cxx::type_id!("QList_u64");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_u64 {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_u64 {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_u64 {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[u64]> for QList<u64> {
    fn from(value: &[u64]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(*item);
        }
        result
    }
}
impl<const N: usize> From<[u64; N]> for QList<u64> {
    fn from(value: [u64; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<u64>> for QList<u64> {
    fn from(value: &Vec<u64>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<u64>> for QList<u64> {
    fn from(value: Vec<u64>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<u64>> for Vec<u64> {
    fn from(value: &QList<u64>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_qlist_u64_for_vec_u64_from;
        cpp(value)
    }
}
impl From<QList<u64>> for Vec<u64> {
    fn from(value: QList<u64>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<u64> {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_u64_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[u64; N]> for QList<u64> {
    fn eq(&self, other: &[u64; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_u64_n_for_qlist_u64_eq;
        cpp(self, other)
    }
}
impl QListImpl<u64> for QList_u64 {
    fn append(&mut self, value: u64) {
        let cpp = ffi::inline_cpp_fn_append;
        cpp(self, value);
    }
    fn capacity(&self) -> usize {
        let cpp = ffi::inline_cpp_fn_capacity;
        cpp(self)
    }
    fn clear(&mut self) {
        let cpp = ffi::inline_cpp_fn_clear;
        cpp(self);
    }
    fn contains(&self, value: &u64) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: u64) {
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
    fn first(&self) -> &u64 {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &u64 {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
