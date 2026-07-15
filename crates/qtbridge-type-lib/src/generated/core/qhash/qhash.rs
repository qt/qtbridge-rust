// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

/// The QHash is a generic struct that provides a hash-table-based dictionary.
///
/// The following types are currently supported as entries in a QHash:
/// * [(i32, QByteArray)][crate::QHash_i32_QByteArray]
/// * [(QByteArray, QVariant)][crate::QHash_QByteArray_QVariant]
/// * [(QString, QVariant)][crate::QHash_QString_QVariant]
///
/// See also: [QHash documentation](https://doc.qt.io/qt-6/qhash.html).
#[derive(Debug)]
#[repr(C)]
pub struct QHash<K, V>
where
    Self: QHashImpl<K, V>,
{
    _d_ptr: std::mem::MaybeUninit<usize>,
    phantoms: core::marker::PhantomData<(K, V)>,
}
impl<K, V> QHash<K, V>
where
    Self: QHashImpl<K, V>,
{
    /// Removes all items from the QHash and frees up all memory used by it.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QHash;
    /// let mut qhash = QHash::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert!(!qhash.is_empty());
    /// qhash.clear();
    /// assert!(qhash.is_empty());
    /// ```
    pub fn clear(&mut self) {
        <Self as QHashImpl<K, V>>::clear(self)
    }
    /// Returns true if the QHash object contains an item with the key; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::from([
    ///     (10, "ten"),
    ///     (20, "twenty"),
    ///     (30, "thirty"),
    /// ]);
    /// assert!(qhash.contains(&20));
    /// assert!(!qhash.contains(&40));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        <Self as QHashImpl<K, V>>::contains(self, key)
    }
    /// Inserts a key-value pair into the map.
    /// If the map has this key present, the value is updated with the one from the argument.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// qhash.insert(&42, &"abc".into());
    /// assert_eq!(qhash[&42], "abc".into());
    /// ```
    pub fn insert(&mut self, key: &K, value: &V) {
        <Self as QHashImpl<K, V>>::insert(self, key, value)
    }
    /// # Returns true if the QHash object contains no items; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// assert!(qhash.is_empty());
    /// qhash.insert(&93, &"c".into());
    /// assert!(!qhash.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        <Self as QHashImpl<K, V>>::is_empty(self)
    }
    /// Removes the entry that has specified key from the QHash object.
    /// Returns true if the key exists in the QHash object and the item has been removed, and false otherwise.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    ///     (3, "Three"),
    /// ]);
    /// assert!(qhash.remove(&2));
    /// assert!(qhash.contains(&1));
    /// assert!(!qhash.contains(&2));
    /// assert!(qhash.contains(&3));
    /// assert!(!qhash.remove(&5));
    /// ```
    pub fn remove(&mut self, key: &K) -> bool {
        <Self as QHashImpl<K, V>>::remove(self, key)
    }
    /// Returns the number of items in the QHash object as isize.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// assert_eq!(qhash.len(), 0);
    /// qhash.insert(&42, &"Forty two".into());
    /// assert_eq!(qhash.len(), 1);
    /// ```
    pub fn size(&self) -> isize {
        <Self as QHashImpl<K, V>>::size(self)
    }
    /// Returns the number of items in the QHash object as usize.
    pub fn len(&self) -> usize {
        self.size() as usize
    }
    /// Returns a list containing all the keys in the QHash object, in an arbitrary order.
    /// The order is guaranteed to be the same as that used by values().
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::<i32, QByteArray>::from([
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (1, "One"),
    /// ]);
    /// let keys = qhash.keys();
    /// assert!(keys.contains(&1));
    /// assert!(keys.contains(&2));
    /// assert!(keys.contains(&3));
    /// ```
    pub fn keys(&self) -> <Self as QHashImpl<K, V>>::QListK {
        <Self as QHashImpl<K, V>>::keys(self)
    }
    /// Returns a list containing all the values in the QHash object, in an arbitrary order.
    /// The order is guaranteed to be the same as that used by keys().
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::<i32, QByteArray>::from([
    ///     (3, "Three"),
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// let values = qhash.values();
    /// assert!(values.contains(&"One".into()));
    /// assert!(values.contains(&"Two".into()));
    /// assert!(values.contains(&"Three".into()));
    /// ```
    pub fn values(&self) -> <Self as QHashImpl<K, V>>::QListV {
        <Self as QHashImpl<K, V>>::values(self)
    }
    /// Returns the value associated with the key.
    /// If the QHash object contains no item with the key, the function returns default-initialized value.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::from([
    ///     (1, "a"),
    ///     (2, "b"),
    ///     (3, "c"),
    /// ]);
    /// assert_eq!(qhash.value(&3), "c".into());
    /// ```
    pub fn value(&self, key: &K) -> V {
        <Self as QHashImpl<K, V>>::value(self, key)
    }
}
#[doc(hidden)]
pub trait QHashImpl<K, V>
{
    type QListK;
    type QListV;

    fn clear(&mut self);
    fn contains(&self, key: &K) -> bool;
    fn insert(&mut self, key: &K, value: &V);
    fn is_empty(&self) -> bool;
    fn remove(&mut self, key: &K) -> bool;
    fn size(&self) -> isize;
    fn keys(&self) -> Self::QListK;
    fn values(&self) -> Self::QListV;
    fn value(&self, key: &K) -> V;
    fn do_drop(&mut self);
}
impl<K, V> Drop for QHash<K, V>
where
    Self: QHashImpl<K, V>,
{
    fn drop(&mut self) {
        <Self as QHashImpl<K, V>>::do_drop(self)
    }
}
