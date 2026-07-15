// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QListImpl};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [bool].
pub type QList_bool = QList<bool>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_bool.h");
        #[allow(dead_code)]
        type QList_bool = super::QList_bool;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
    }
    #[namespace = "rust::bridge::qlist_bool"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_bool);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_bool;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_bool) -> QList_bool;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_bool, rhs: &QList_bool) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_bool, value: bool);
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_bool);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_bool, value: &bool) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_bool, value: bool);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_bool, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_bool, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_bool) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_bool) -> &bool;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_bool) -> &bool;
        # [rust_name = inline_cpp_fn_trait_impl_from_ref_qlist_bool_for_vec_bool_from]
        fn inlineCppFn_TraitImpl_From_ref_QList_bool_for_Vec_bool_from(src: &QList_bool) -> Vec<bool>;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_bool_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_bool_index(_obj: &QList_bool, index: usize) -> *const bool;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_bool_n_for_qlist_bool_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_bool_N_for_QList_bool_eq(_obj: &QList_bool, rhs: &[bool]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_bool {
    type Id = cxx::type_id!("QList_bool");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_bool {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_bool {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_bool {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[bool]> for QList<bool> {
    fn from(value: &[bool]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(*item);
        }
        result
    }
}
impl<const N: usize> From<[bool; N]> for QList<bool> {
    fn from(value: [bool; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<bool>> for QList<bool> {
    fn from(value: &Vec<bool>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<bool>> for QList<bool> {
    fn from(value: Vec<bool>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<bool>> for Vec<bool> {
    fn from(value: &QList<bool>) -> Self {
        let cpp = ffi::inline_cpp_fn_trait_impl_from_ref_qlist_bool_for_vec_bool_from;
        cpp(value)
    }
}
impl From<QList<bool>> for Vec<bool> {
    fn from(value: QList<bool>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<bool> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_bool_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[bool; N]> for QList<bool> {
    fn eq(&self, other: &[bool; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_bool_n_for_qlist_bool_eq;
        cpp(self, other)
    }
}
impl QListImpl<bool> for QList_bool {
    fn append(&mut self, value: bool) {
        let cpp = ffi::inline_cpp_fn_append;
        cpp(self, value);
    }
    fn clear(&mut self) {
        let cpp = ffi::inline_cpp_fn_clear;
        cpp(self);
    }
    fn contains(&self, value: &bool) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: bool) {
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
    fn first(&self) -> &bool {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &bool {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
