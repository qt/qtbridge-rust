// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QListImpl};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [i32].
pub type QList_i32 = QList<i32>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_i32.h");
        #[allow(dead_code)]
        type QList_i32 = super::QList_i32;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
    }
    #[namespace = "rust::bridge::qlist_i32"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_i32);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_i32;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_i32) -> QList_i32;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_i32, rhs: &QList_i32) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_i32, value: i32);
        # [rust_name = inline_cpp_fn_capacity]
        fn inlineCppFn_capacity(_obj: &QList_i32) -> usize;
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_i32);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_i32, value: &i32) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_i32, value: i32);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_i32, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_i32, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_i32) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_i32) -> &i32;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_i32) -> &i32;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qlist_i32_for_vec_i32_from]
        fn inlineCppFn_TraitImpl_From_ref_QList_i32_for_Vec_i32_from(src: &QList_i32) -> Vec<i32>;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_i32_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i32_index(_obj: &QList_i32, index: usize) -> *const i32;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_i32_n_for_qlist_i32_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_i32_N_for_QList_i32_eq(_obj: &QList_i32, rhs: &[i32]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_i32 {
    type Id = cxx::type_id!("QList_i32");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_i32 {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_i32 {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_i32 {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[i32]> for QList<i32> {
    fn from(value: &[i32]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(*item);
        }
        result
    }
}
impl<const N: usize> From<[i32; N]> for QList<i32> {
    fn from(value: [i32; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<i32>> for QList<i32> {
    fn from(value: &Vec<i32>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<i32>> for QList<i32> {
    fn from(value: Vec<i32>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<i32>> for Vec<i32> {
    fn from(value: &QList<i32>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_qlist_i32_for_vec_i32_from;
        cpp(value)
    }
}
impl From<QList<i32>> for Vec<i32> {
    fn from(value: QList<i32>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<i32> {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_i32_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[i32; N]> for QList<i32> {
    fn eq(&self, other: &[i32; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_i32_n_for_qlist_i32_eq;
        cpp(self, other)
    }
}
impl QListImpl<i32> for QList_i32 {
    fn append(&mut self, value: i32) {
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
    fn contains(&self, value: &i32) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: i32) {
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
    fn first(&self) -> &i32 {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &i32 {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
