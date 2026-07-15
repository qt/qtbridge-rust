// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QListImpl, QObject};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [*mut QObject].
pub struct QList_ptr_mut_QObject {
    _d: std::mem::MaybeUninit<usize>,
    _ptr: std::mem::MaybeUninit<usize>,
    _size: std::mem::MaybeUninit<usize>,
}
/// This is an alias for type [QList] for type [*mut QObject].
pub type QObjectList = QList_ptr_mut_QObject;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_ptr_mut_qobject.h");
        #[allow(dead_code)]
        type QList_ptr_mut_QObject = super::QList_ptr_mut_QObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = crate::QObject;
    }
    #[namespace = "rust::bridge::qlist_ptr_mut_qobject"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_ptr_mut_QObject);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_ptr_mut_QObject;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_ptr_mut_QObject) -> QList_ptr_mut_QObject;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_ptr_mut_QObject, rhs: &QList_ptr_mut_QObject) -> bool;
        # [rust_name = inline_cpp_fn_append]
        unsafe fn inlineCppFn_append(_obj: &mut QList_ptr_mut_QObject, value: *mut QObject);
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_ptr_mut_QObject);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_ptr_mut_QObject, value: &*mut QObject) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        unsafe fn inlineCppFn_push_back(_obj: &mut QList_ptr_mut_QObject, value: *mut QObject);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_ptr_mut_QObject, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_ptr_mut_QObject, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_ptr_mut_QObject) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_ptr_mut_QObject) -> &*mut QObject;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_ptr_mut_QObject) -> &*mut QObject;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_ptr_mut_qobject_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_ptr_mut_QObject_index(_obj: &QList_ptr_mut_QObject, index: usize) -> *const *mut QObject;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_ptr_mut_qobject_n_for_qlist_ptr_mut_qobject_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_ptr_mut_QObject_N_for_QList_ptr_mut_QObject_eq(_obj: &QList_ptr_mut_QObject, rhs: &[*mut QObject]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_ptr_mut_QObject {
    type Id = cxx::type_id!("QList_ptr_mut_QObject");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_ptr_mut_QObject {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_ptr_mut_QObject {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl Drop for QList_ptr_mut_QObject {
    fn drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
impl PartialEq for QList_ptr_mut_QObject {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[*mut QObject]> for QList_ptr_mut_QObject {
    fn from(value: &[*mut QObject]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(*item);
        }
        result
    }
}
impl<const N: usize> From<[*mut QObject; N]> for QList_ptr_mut_QObject {
    fn from(value: [*mut QObject; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<*mut QObject>> for QList_ptr_mut_QObject {
    fn from(value: &Vec<*mut QObject>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<*mut QObject>> for QList_ptr_mut_QObject {
    fn from(value: Vec<*mut QObject>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList_ptr_mut_QObject> for Vec<*mut QObject> {
    fn from(value: &QList_ptr_mut_QObject) -> Self {
        let mut v = Vec::with_capacity(value.len());
        for i in 0..value.len() {
            v.push(value[i]);
        }
        v
    }
}
impl From<QList_ptr_mut_QObject> for Vec<*mut QObject> {
    fn from(value: QList_ptr_mut_QObject) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList_ptr_mut_QObject {
    type Output = *mut QObject;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_ptr_mut_qobject_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[*mut QObject; N]> for QList_ptr_mut_QObject {
    fn eq(&self, other: &[*mut QObject; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_ptr_mut_qobject_n_for_qlist_ptr_mut_qobject_eq;
        cpp(self, other)
    }
}

impl QList_ptr_mut_QObject {
    pub fn append(&mut self, value: *mut QObject) {
        let cpp = ffi::inline_cpp_fn_append;
        unsafe { cpp(self, value) };
    }
    pub fn len(&self) -> usize {
        let cpp = ffi::inline_cpp_fn_size;
        cpp(self) as usize
    }
}
impl QListImpl<*mut QObject> for QList_ptr_mut_QObject {
    fn append(&mut self, value: *mut QObject) {
        let cpp = ffi::inline_cpp_fn_append;
        unsafe { cpp(self, value) };
    }
    fn clear(&mut self) {
        let cpp = ffi::inline_cpp_fn_clear;
        cpp(self);
    }
    fn contains(&self, value: &*mut QObject) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: *mut QObject) {
        let cpp = ffi::inline_cpp_fn_push_back;
        unsafe { cpp(self, value) };
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
    fn first(&self) -> &*mut QObject {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &*mut QObject {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
