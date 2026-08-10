// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::QAbstractItemModelProxyCpp;
use crate::{call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use crate::genericrustproxy::GenericRustProxy;
use qtbridge_type_lib::{QHash_i32_QByteArray, QModelIndex, QVariant};

pub trait QAbstractItemModelAdapter: DispatchMetaCall + 'static {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash_i32_QByteArray;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

impl<T> QAbstractItemModelAdapter for T
where
    T: QAbstractItemModel {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::index(self, row, column, parent)
    }
    fn parent(&self, child: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::parent(self, child)
    }
    fn row_count(&self, parent: &QModelIndex) -> i32 {
        <Self as QAbstractItemModel>::row_count(self, parent)
    }
    fn column_count(&self, parent: &QModelIndex) -> i32 {
        <Self as QAbstractItemModel>::column_count(self, parent)
    }
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        <Self as QAbstractItemModel>::data(self, index, role)
    }
    fn role_names(&self) -> QHash_i32_QByteArray {
        <Self as QAbstractItemModel>::role_names(self)
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        <Self as QAbstractItemModel>::set_data(self, index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        <Self as QAbstractItemModel>::remove_rows(self, first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::sibling(self, row, column, idx)
    }
}

pub trait QAbstractItemModel : QObjectHolder<ProxyRust = QAbstractItemModelProxyRust> {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash_i32_QByteArray {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_role_names(self)
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_set_data(&mut *self, index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_remove_rows(&mut *self, first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_sibling(self, row, column, idx)
    }
}

pub trait QAbstractItemModelBase : QAbstractItemModel {
    fn role_names(&self) -> QHash_i32_QByteArray {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_role_names(self)
    }
    fn set_data(
        &mut self,
        index: &QModelIndex,
        value: &QVariant,
        role: i32,
    ) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_set_data(&mut *self, index, value, role)
    }
    fn remove_rows(
        &mut self,
        first: i32,
        count: i32,
        parent: &QModelIndex,
    ) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_remove_rows(&mut *self, first, count, parent)
    }
    fn sibling(
        &self,
        row: i32,
        column: i32,
        idx: &QModelIndex,
    ) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_sibling(self, row, column, idx)
    }
    fn data_changed(
        &mut self,
        top_left: &QModelIndex,
        bottom_right: &QModelIndex,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_data_changed(&mut *self, top_left, bottom_right)
    }
    fn begin_insert_columns(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_insert_columns(&mut *self, parent, first, last)
    }
    fn end_insert_columns(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_insert_columns(&mut *self)
    }
    fn begin_insert_rows(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_insert_rows(&mut *self, parent, first, last)
    }
    fn end_insert_rows(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_insert_rows(&mut *self)
    }
    fn begin_move_columns(
        &mut self,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_move_columns(
            &mut *self,
            source_parent,
            source_first,
            source_last,
            destination_parent,
            destination_child,
        )
    }
    fn end_move_columns(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_move_columns(&mut *self)
    }
    fn begin_move_rows(
        &mut self,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_move_rows(
            &mut *self,
            source_parent,
            source_first,
            source_last,
            destination_parent,
            destination_child,
        )
    }
    fn end_move_rows(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_move_rows(&mut *self)
    }
    fn begin_remove_columns(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_remove_columns(&mut *self, parent, first, last)
    }
    fn end_remove_columns(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_remove_columns(&mut *self)
    }
    fn begin_remove_rows(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_remove_rows(&mut *self, parent, first, last)
    }
    fn end_remove_rows(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_remove_rows(&mut *self)
    }
    fn begin_reset_model(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_begin_reset_model(&mut *self)
    }
    fn end_reset_model(&mut self) {
        let Some(proxy) = self.try_get_rust_proxy_ptr() else { return };
        unsafe { &mut *proxy }.base_end_reset_model(&mut *self)
    }
    fn create_index(
        &self,
        row: i32,
        column: i32,
        ptr: usize,
    ) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_create_index(self, row, column, ptr)
    }
}

impl<T> QAbstractItemModelBase for T
where T: QAbstractItemModel {}

pub type QAbstractItemModelProxyRust = GenericRustProxy<QAbstractItemModelProxyCpp, dyn QAbstractItemModelAdapter>;

impl QAbstractItemModelProxyRust {
    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, index(row, column, parent))
    }
    pub fn parent(&self, child: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, parent(child))
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, row_count(parent))
    }
    pub fn column_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, column_count(parent))
    }
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        call_rust_trait_impl!(self, data(index, role))
    }
    pub fn role_names(&self) -> QHash_i32_QByteArray {
        call_rust_trait_impl!(self, role_names())
    }
    pub fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_rust_trait_impl!(mut self, set_data(index, value, role))
    }
    pub fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_rust_trait_impl!(mut self, remove_rows(first, count, parent))
    }
    pub fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, sibling(row, column, idx))
    }

    pub fn base_role_names(&self, reference: &dyn QAbstractItemModelAdapter) -> QHash_i32_QByteArray {
        call_cpp_impl!(self, reference, base_role_names())
    }
    pub fn base_set_data(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_cpp_impl!(mut self, mut_ref, base_set_data(index, value, role))
    }
    pub fn base_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_cpp_impl!(mut self, mut_ref, base_remove_rows(first, count, parent))
    }
    pub fn base_sibling(&self, reference: &dyn QAbstractItemModelAdapter, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_cpp_impl!(self, reference, base_sibling(row, column, idx))
    }
    pub fn base_data_changed(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, top_left: &QModelIndex, bottom_right: &QModelIndex) {
        call_cpp_impl!(mut self, mut_ref, base_data_changed(top_left, bottom_right))
    }
    pub fn base_begin_insert_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_insert_columns(parent, first, last))
    }
    pub fn base_end_insert_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_insert_columns())
    }
    pub fn base_begin_insert_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_insert_rows(parent, first, last))
    }
    pub fn base_end_insert_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_insert_rows())
    }
    pub fn base_begin_move_columns(
        &mut self,
        mut_ref: &mut dyn QAbstractItemModelAdapter,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        call_cpp_impl!(mut self, mut_ref, base_begin_move_columns(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_move_columns())
    }
    pub fn base_begin_move_rows(
        &mut self,
        mut_ref: &mut dyn QAbstractItemModelAdapter,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        call_cpp_impl!(mut self, mut_ref, base_begin_move_rows(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_move_rows())
    }
    pub fn base_begin_remove_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_remove_columns(parent, first, last))
    }
    pub fn base_end_remove_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_remove_columns())
    }
    pub fn base_begin_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_remove_rows(parent, first, last))
    }
    pub fn base_end_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_remove_rows())
    }
    pub fn base_begin_reset_model(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_begin_reset_model())
    }
    pub fn base_end_reset_model(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_reset_model())
    }
    pub fn base_create_index(&self, reference: &dyn QAbstractItemModelAdapter, row: i32, column: i32, ptr: usize) -> QModelIndex {
        call_cpp_impl!(self, reference, base_create_index(row, column, ptr))
    }
}
