// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QList, QListImpl, QString};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QList] for type [QString].
pub type QList_QString = QList<QString>;
/// This is an alias for type [QList] for type [QString].
pub type QStringList = QList<QString>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h");
        #[allow(dead_code)]
        type QList_QString = super::QList_QString;
        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        type QString = crate::QString;
    }
    #[namespace = "rust::bridge::qlist_qstring"]
    unsafe extern "C++" {
        # [rust_name = qlist_drop]
        fn QList_Drop(v: &mut QList_QString);
        # [rust_name = qlist_default]
        fn QList_Default() -> QList_QString;
        # [rust_name = qlist_clone]
        fn QList_Clone(v: &QList_QString) -> QList_QString;
        # [rust_name = qlist_eq]
        fn QList_Eq(lhs: &QList_QString, rhs: &QList_QString) -> bool;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QList_QString, value: QString);
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QList_QString);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QList_QString, value: &QString) -> bool;
        # [rust_name = inline_cpp_fn_push_back]
        fn inlineCppFn_push_back(_obj: &mut QList_QString, value: QString);
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QList_QString, i: isize, n: isize);
        # [rust_name = inline_cpp_fn_reserve]
        fn inlineCppFn_reserve(_obj: &mut QList_QString, size: usize);
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QList_QString) -> isize;
        # [rust_name = inline_cpp_fn_first]
        fn inlineCppFn_first(_obj: &QList_QString) -> &QString;
        # [rust_name = inline_cpp_fn_last]
        fn inlineCppFn_last(_obj: &QList_QString) -> &QString;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qstring_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_QString_index(_obj: &QList_QString, index: usize) -> *const QString;
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_array_of_qstring_n_for_qlist_qstring_eq]
        fn inlineCppFn_TraitImpl_PartialEq_array_of_QString_N_for_QList_QString_eq(_obj: &QList_QString, rhs: &[QString]) -> bool;
    }
}
unsafe impl cxx::ExternType for QList_QString {
    type Id = cxx::type_id!("QList_QString");
    type Kind = cxx::kind::Trivial;
}
impl Default for QList_QString {
    fn default() -> Self {
        ffi::qlist_default()
    }
}
impl Clone for QList_QString {
    fn clone(&self) -> Self {
        ffi::qlist_clone(self)
    }
}
impl PartialEq for QList_QString {
    fn eq(&self, other: &Self) -> bool {
        ffi::qlist_eq(self, other)
    }
}
impl From<&[QString]> for QList<QString> {
    fn from(value: &[QString]) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.iter() {
            result.append(item.clone());
        }
        result
    }
}
impl<const N: usize> From<[QString; N]> for QList<QString> {
    fn from(value: [QString; N]) -> Self {
        let mut result = Self::default();
        result.reserve(N);
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&Vec<QString>> for QList<QString> {
    fn from(value: &Vec<QString>) -> Self {
        Self::from(value.as_slice())
    }
}
impl From<Vec<QString>> for QList<QString> {
    fn from(value: Vec<QString>) -> Self {
        let mut result = Self::default();
        result.reserve(value.len());
        for item in value.into_iter() {
            result.append(item);
        }
        result
    }
}
impl From<&QList<QString>> for Vec<QString> {
    fn from(value: &QList<QString>) -> Self {
        let mut v = Vec::with_capacity(value.len());
        for i in 0..value.len() {
            v.push(value[i].clone());
        }
        v
    }
}
impl From<QList<QString>> for Vec<QString> {
    fn from(value: QList<QString>) -> Self {
        Self::from(&value)
    }
}
impl From<&QList<QString>> for Vec<String> {
    fn from(value: &QList<QString>) -> Self {
        let mut v = Vec::with_capacity(value.len());
        for i in 0..value.len() {
            v.push((&value[i]).into());
        }
        v
    }
}
impl From<QList<QString>> for Vec<String> {
    fn from(value: QList<QString>) -> Self {
        Self::from(&value)
    }
}
impl From<&Vec<String>> for QList<QString> {
    fn from(value: &Vec<String>) -> Self {
        let mut v = QList::default();
        v.reserve(value.len());
        for st in value {
            v.append(st.into())
        }
        v
    }
}
impl From<Vec<String>> for QList<QString> {
    fn from(value: Vec<String>) -> Self {
        Self::from(&value)
    }
}
impl std::ops::Index<usize> for QList<QString> {
    type Output = QString;
    fn index(&self, index: usize) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_usize_for_qlist_qstring_index;
        unsafe { cpp(self, index).as_ref() }.expect("Out of bounds access to QList")
    }
}
impl<const N: usize> PartialEq<[QString; N]> for QList<QString> {
    fn eq(&self, other: &[QString; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let cpp = ffi::inline_cpp_fn_trait_impl_partial_eq_array_of_qstring_n_for_qlist_qstring_eq;
        cpp(self, other)
    }
}
impl QListImpl<QString> for QList_QString {
    fn append(&mut self, value: QString) {
        let cpp = ffi::inline_cpp_fn_append;
        cpp(self, value);
    }
    fn clear(&mut self) {
        let cpp = ffi::inline_cpp_fn_clear;
        cpp(self);
    }
    fn contains(&self, value: &QString) -> bool {
        ffi::inline_cpp_fn_contains(self, value)
    }
    fn push_back(&mut self, value: QString) {
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
    fn first(&self) -> &QString {
        let cpp = ffi::inline_cpp_fn_first;
        cpp(self)
    }
    fn last(&self) -> &QString {
        let cpp = ffi::inline_cpp_fn_last;
        cpp(self)
    }
    fn do_drop(&mut self) {
        ffi::qlist_drop(self)
    }
}
