// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::QListModelProxyCpp;
use crate::{call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use crate::genericrustproxy::GenericRustProxy;
use qtbridge_runtime::QModelItem;
use qtbridge_type_lib::{QByteArray, QHash, QModelIndex, QVariant};

#[doc(hidden)]
pub trait QListModelAdapter: DispatchMetaCall + 'static {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

impl<T> QListModelAdapter for T
where
    T: QListModel + QObjectHolder<ProxyRust = QListModelProxyRust> {

    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_index(self, row, column, parent)
    }

    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        return self.len() as i32;
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let Some(item) = self.get(index.row() as usize)
        else {
            return QVariant::default();
        };
        item.get_role(role)
    }

    fn role_names(&self) -> QHash<i32, QByteArray> {
        let names = T::Item::role_names();
        let mut result = QHash::default();
        names.iter()
            .for_each(|(k, v)| result.insert(k, &QByteArray::from(v)));
        result
    }

    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        if !index.is_valid() {
            return false;
        }
        let Some(mut item) = self.get(index.row() as usize)
            .cloned()
        else {
            return false;
        };
        let updated = item.set_role(role, value);
        if updated {
            self.set_unnotified(index.row() as usize, item);
            let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
            unsafe { &mut *proxy }.base_data_changed(&mut *self, index, index);
        }
        updated
    }

    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let first = first as usize;
        let last = first + count as usize;
        if last > self.len() {
            return false;
        }
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_remove_rows(&mut *self, parent, first as i32, (last - 1) as i32);
        for index in (first..last).rev() {
            self.remove_unnotified(index);
        }
        unsafe { &mut *proxy }.base_end_remove_rows(&mut *self);
        true
    }

    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_sibling(self, row, column, idx)
    }
}

/// A trait representing a list-based model.
///
/// [`QListModel`] provides an interface for list-like data structures
/// that are exposed to Qt through the Model-View concept.
/// <https://doc.qt.io/qt-6/qtquick-modelviewsdata-modelview.html>.
///
/// This trait requires the `qobject` macro to set up the correct Qt proxy.
/// The macro will further generate functionality in the form of the
/// [`QListModelBase`] trait that supplements the [`QListModel`] functionality.
///
/// ## Design
///
/// - The model owns items of associated type `Item` that has to implement
///   the [`QModelItem`] trait. Roles are derived from the [`QModelItem`]
///   implementation.
/// - Mutation methods are provided in an **unnotified** form, meaning
///   they modify the underlying data without emitting Qt model signals.
/// - These methods are used by the automatically implemented [`QListModelBase`]
///   trait to create methods that notify the UI about changes in collections.
///
/// As a minimum you have to implement the methods [`QListModel::len`] and
/// [`QListModel::get`] to create a readable list model. Further methods can be
/// implemented to make the model fully mutable.
///
/// Methods that do not return an [`Option`] or a boolean value must succeed
/// and perform exactly the operation described in the documentation to avoid
/// invalidating the synchronization between any views and the underlying data.
/// No additional structural changes may occur outside the provided functions.
///
/// **Note that default implementations may `panic!`** if the corresponding method is
/// not overridden. It is your responsibility to make sure that these functions are
/// not called from QML.
///
/// ## Example
///
/// ``` ignore
/// use qtbridge::qobject;
/// #[qobject(Base = QListModel)]
/// mod backend {
///     use qtbridge::{QListModel, QListModelBase};
///
///     #[derive(Default)]
///     pub struct Backend {
///         string_list: Vec<String>,
///     }
///     impl QListModel for Backend {
///         type Item = String;
///
///         fn len(&self) -> usize {
///             self.string_list.len()
///         }
///         fn get(&self, index: usize) -> Option<&Self::Item> {
///             self.string_list.get(index)
///         }
///     }
/// }
///
/// ```
///
/// The list model can be used in QML views as follows
/// ``` qml, ignore
/// ListView {
///     model: backend
///     delegate: Text {
///         required property string value
///         text: value
///     }
/// }
/// ```
pub trait QListModel {
    /// The item type stored in the model.
    ///
    /// Items must:
    /// - Implement [`QModelItem`] to integrate with Qt
    /// - Be [`Default`] for creating new items
    /// - Be [`Clone`] for safe data access and copying
    type Item: QModelItem + Default + Clone;

    /// Returns the number of items in the list.
    fn len(&self) -> usize;

    /// Returns a reference to the item at `index`, or `None` if the index
    /// is out of bounds.
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Sets the item at `index`. Reimplement this function but call
    /// [`QListModelBase::set`] to notify Qt about the modification.
    ///
    /// Returns `true` if the value was successfully set, or `false` if the
    /// operation failed (e.g., index out of bounds or value fails
    /// validation by the business logic).
    ///
    /// The default implementation does nothing and returns `false`.
    fn set_unnotified(&mut self, _index: usize, _value: Self::Item) -> bool {
        false
    }

    /// Appends an item to the end of the model. Reimplement this
    /// function but call [`QListModelBase::push`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// The default implementation falls back to [`QListModel::insert_unnotified`],
    /// which in turn panics by default.
    fn push_unnotified(&mut self, value: Self::Item) {
        self.insert_unnotified(self.len(), value);
    }

    /// Inserts `value` at `index`. Reimplement this function but
    /// call [`QListModelBase::insert`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// Panics by default. Implementors must override this method to support
    /// insertion.
    fn insert_unnotified(&mut self, _index: usize, _value: Self::Item) {
        panic!("In order to use insert, implement insert_unnotified")
    }

    /// Removes and returns the last item in the model. Reimplement this
    /// function but call [`QListModelBase::pop`] to notify Qt
    /// about the modification.
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    ///
    /// The default implementation falls back to [`QListModel::remove_unnotified`],
    /// which in turn panics by default.
    fn pop_unnotified(&mut self) -> Option<Self::Item> {
        (self.len() > 0)
            .then(|| self.remove_unnotified(self.len() - 1))
    }

    /// Removes and returns the item at `index`. Reimplement this
    /// function but call [`QListModelBase::remove`] to notify Qt
    /// about the modification.
    ///
    /// The index must be valid and the model has to guarantee the success of
    /// the operation.
    ///
    /// Panics by default. Implementors must override this method to support
    /// removal.
    fn remove_unnotified(&mut self, _index: usize) -> Self::Item {
        panic!("In order to use remove, implement remove_unnotified")
    }

    /// Resets the model's internal storage. Reimplement this function but
    /// call [`QListModelBase::reset`] to notify Qt about the modification.
    ///
    /// Panics by default. Implementors must override this method to support
    /// a model reset.
    ///
    /// After [`QListModel::reset_unnotified`] returns, the internal storage
    /// must reflect the new model state: [`QListModel::len`] and
    /// [`QListModel::get`] must be consistent with the updated storage.
    fn reset_unnotified(&mut self) {
        panic!("In order to use reset, implement reset_unnotified")
    }
}

/// A data-change signaling extension of [`QListModel`].
///
/// `QListModelBase` provides the signaling mutation API for list models.
/// The methods defined in this trait wrap the corresponding
/// `*_unnotified` methods from [`QListModel`] and automatically emit the
/// required Qt model signals (such as `beginInsertRows`, `endInsertRows`,
/// `dataChanged`, etc.). This allows the UI to react to changes in the
/// underlying data.
///
/// This trait is automatically implemented by the `qobject` macro and
/// should not be implemented manually.
///
/// ## Usage
///
/// When modifying data that you made accessible with [`QListModel`], you
/// have to use the functions provided by this trait. Do **not** call the
/// `*_unnotified` methods from [`QListModel`] directly unless you are
/// manually handling Qt model notifications.
///
/// The correctness of this trait depends on implementors of [`QListModel`]
/// ensuring that:
///
/// * The `*_unnotified` methods perform the exact mutation corresponding
///   to the emitted Qt signals.
/// * No additional structural changes occur.
///
/// Violating this contract may result in undefined behavior in Qt views.
pub trait QListModelBase : QListModel + QObjectHolder<ProxyRust = QListModelProxyRust> {
    /// Sets the item at `index` and notifies any attached views about
    /// the change, if the operation is successful.
    ///
    /// This method calls [`QListModel::set_unnotified`].
    ///
    /// Returns `true` if the value was successfully updated,
    /// or `false` if the operation failed (for example, if the index
    /// was out of bounds or validation failed).
    fn set(&mut self, index: usize, value: <Self as QListModel>::Item) -> bool {
        if self.set_unnotified(index, value) {
            let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
            let model_index = unsafe { &*proxy }.base_index(&*self, index as i32, 0, &QModelIndex::default());
            unsafe { &mut *proxy }.base_data_changed(&mut *self, &model_index, &model_index);
            true
        } else {
            false
        }
    }

    /// Appends `value` to the end of the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QListModel::push_unnotified`].
    fn push(&mut self, value: Self::Item) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        let len = self.len() as i32;
        unsafe { &mut *proxy }.base_begin_insert_rows(&mut *self, &QModelIndex::default(), len, len);
        self.push_unnotified(value);
        unsafe { &mut *proxy }.base_end_insert_rows(&mut *self);
    }

    /// Inserts `value` at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QListModel::insert_unnotified`].
    fn insert(&mut self, index: usize, value: Self::Item) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_insert_rows(&mut *self, &QModelIndex::default(), index as i32, index as i32);
        self.insert_unnotified(index, value);
        unsafe { &mut *proxy }.base_end_insert_rows(&mut *self);
    }

    /// Removes and returns the last item in the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QListModel::pop_unnotified`].
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    fn pop(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        let len = self.len() as i32;
        unsafe { &mut *proxy }.base_begin_remove_rows(&mut *self, &QModelIndex::default(), len - 1, len - 1);
        let value = self.pop_unnotified();
        unsafe { &mut *proxy }.base_end_remove_rows(&mut *self);
        value
    }

    /// Removes and returns the item at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QListModel::remove_unnotified`].
    fn remove(&mut self, index: usize) -> Self::Item {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_remove_rows(&mut *self, &QModelIndex::default(), index as i32, index as i32);
        let value = self.remove_unnotified(index);
        unsafe { &mut *proxy }.base_end_remove_rows(&mut *self);
        value
    }

    /// Resets the entire model and notifies any attached views to resynchronize all data.
    ///
    /// This method calls [`QListModel::reset_unnotified`].
    fn reset(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_reset_model(&mut *self);
        self.reset_unnotified();
        unsafe { &mut *proxy }.base_end_reset_model(&mut *self);
    }
}

impl<T> QListModelBase for T
where T: QListModel + QObjectHolder<ProxyRust = QListModelProxyRust> { }

pub type QListModelProxyRust = GenericRustProxy<QListModelProxyCpp, dyn QListModelAdapter>;

impl QListModelProxyRust {
    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, index(row, column, parent))
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, row_count(parent))
    }
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        call_rust_trait_impl!(self, data(index, role))
    }
    pub fn role_names(&self) -> QHash<i32, QByteArray> {
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

    pub fn base_index(&self, reference: &dyn QListModelAdapter, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_cpp_impl!(self, reference, base_index(row, column, parent))
    }
    pub fn base_role_names(&self, reference: &dyn QListModelAdapter) -> QHash<i32, QByteArray> {
        call_cpp_impl!(self, reference, base_role_names())
    }
    pub fn base_set_data(&mut self, mut_ref: &mut dyn QListModelAdapter, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_cpp_impl!(mut self, mut_ref, base_set_data(index, value, role))
    }
    pub fn base_remove_rows(&mut self, mut_ref: &mut dyn QListModelAdapter, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_cpp_impl!(mut self, mut_ref, base_remove_rows(first, count, parent))
    }
    pub fn base_sibling(&self, reference: &dyn QListModelAdapter, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_cpp_impl!(self, reference, base_sibling(row, column, idx))
    }
    pub fn base_data_changed(&mut self, mut_ref: &mut dyn QListModelAdapter, top_left: &QModelIndex, bottom_right: &QModelIndex) {
        call_cpp_impl!(mut self, mut_ref, base_data_changed(top_left, bottom_right))
    }
    pub fn base_begin_insert_rows(&mut self, mut_ref: &mut dyn QListModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_insert_rows(parent, first, last))
    }
    pub fn base_end_insert_rows(&mut self, mut_ref: &mut dyn QListModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_insert_rows())
    }
    pub fn base_begin_move_rows(&mut self, mut_ref: &mut dyn QListModelAdapter, source_parent: &QModelIndex, source_first: i32, source_last: i32, destination_parent: &QModelIndex, destination_child: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_move_rows(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_rows(&mut self, mut_ref: &mut dyn QListModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_move_rows())
    }
    pub fn base_begin_remove_rows(&mut self, mut_ref: &mut dyn QListModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, mut_ref, base_begin_remove_rows(parent, first, last))
    }
    pub fn base_end_remove_rows(&mut self, mut_ref: &mut dyn QListModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_remove_rows())
    }
    pub fn base_begin_reset_model(&mut self, mut_ref: &mut dyn QListModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_begin_reset_model())
    }
    pub fn base_end_reset_model(&mut self, mut_ref: &mut dyn QListModelAdapter) {
        call_cpp_impl!(mut self, mut_ref, base_end_reset_model())
    }
}
