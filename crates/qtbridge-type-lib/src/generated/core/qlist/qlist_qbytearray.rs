// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QByteArray, QList, QListImpl};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [QByteArray].
pub type QList_QByteArray = QList<QByteArray>;
/// This is an alias for type [QList] for type [QByteArray].
pub type QByteArrayList = QList<QByteArray>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qbytearray.h");
        #[allow(dead_code)]
        type QList_QByteArray = super::QList_QByteArray;
        include!("qtbridge-type-lib/src/generated/core/qbytearray/cpp/qbytearray.h");
        type QByteArray = crate::QByteArray;
    }
    #[namespace = "rust::bridge::qlist_qbytearray"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_QByteArray);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_QByteArray;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_QByteArray) -> QList_QByteArray;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_QByteArray, rhs: &QList_QByteArray) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_QByteArray, value: QByteArray);
        # [rust_name = inline_cpp_fn_capacity]
        fn inlineCppFn_capacity(_obj: &QList_QByteArray) -> usize;
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_QByteArray);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_QByteArray, value: &QByteArray) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_QByteArray, value: QByteArray);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_QByteArray, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_QByteArray, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_QByteArray) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_QByteArray) -> &QByteArray;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_QByteArray) -> &QByteArray;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qbyte_array_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_QByteArray_index(_obj: &QList_QByteArray, index: usize) -> *const QByteArray;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_qbyte_array_n_for_qlist_qbyte_array_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_QByteArray_N_for_QList_QByteArray_eq(_obj: &QList_QByteArray, rhs: &[QByteArray]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_QByteArray {
    type Id = cxx::type_id!("QList_QByteArray");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_QByteArray {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_QByteArray {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_QByteArray {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[QByteArray]> for QList<QByteArray> {
    fn from(value: &[QByteArray]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(item.clone());
        }
        result
    }
}
impl<const N: usize> From<[QByteArray; N]> for QList<QByteArray> {
    fn from(value: [QByteArray; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<QByteArray>> for QList<QByteArray> {
    fn from(value: &Vec<QByteArray>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<QByteArray>> for QList<QByteArray> {
    fn from(value: Vec<QByteArray>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<QByteArray>> for Vec<QByteArray> {
    fn from(value: &QList<QByteArray>) -> Self {
        let mut v = Vec::with_capacity(value.len());
        for i in 0..value.len() {
            v.push(value[i].clone());
        }
        v
    }
}
impl From<QList<QByteArray>> for Vec<QByteArray> {
    fn from(value: QList<QByteArray>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<QByteArray> {
    type Output = QByteArray;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qbyte_array_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[QByteArray; N]> for QList<QByteArray> {
    fn eq(&self, other: &[QByteArray; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_qbyte_array_n_for_qlist_qbyte_array_eq;
        cpp(self, other)
    }
}
impl QListImpl<QByteArray> for QList_QByteArray {
    fn append(&mut self, value: QByteArray) {
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
    fn contains(&self, value: &QByteArray) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: QByteArray) {
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
    fn first(&self) -> &QByteArray {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &QByteArray {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
