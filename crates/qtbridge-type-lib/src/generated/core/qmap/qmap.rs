// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QList;
/// The QMap is a generic struct that provides an associative array.
///
/// The following types are currently supported as entries in a QMap:
/// * [(i32, QString)][crate::QMap_i32_QString]
/// * [(QString, QVariant)][crate::QMap_QString_QVariant] (also known as [QVariantMap][crate::QVariantMap])
///
/// See also: [QMap documentation](https://doc.qt.io/qt-6/qmap.html).
#[derive(Debug)]
#[repr(C)]
pub struct QMap<K, V>
where
    Self: QMapImpl<K, V>,
    QList<K>: crate::QListImpl<K>,
    QList<V>: crate::QListImpl<V>,
{
    _d: std::mem::MaybeUninit<usize>,
    phantoms: core::marker::PhantomData<(K, V)>,
}
impl<K, V> QMap<K, V>
where
    Self: QMapImpl<K, V>,
    QList<K>: crate::QListImpl<K>,
    QList<V>: crate::QListImpl<V>,
{
    /// Removes all the items from the map.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::QMap;
    /// let mut map = QMap::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert!(!map.is_empty());
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        <Self as QMapImpl<K, V>>::clear(self)
    }
    /// Inserts a new entry with the given key and value into the map.
    /// If an entry with the same key already exists, its value is replaced with the one provided in the argument.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// map.insert(&91, &"a".into());
    /// assert_eq!(map[&91], "a".into());
    /// ```
    pub fn insert(&mut self, key: &K, value: &V) {
        <Self as QMapImpl<K, V>>::insert(self, key, value)
    }
    /// Returns true if the map contains no items; otherwise returns false.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// assert!(map.is_empty());
    /// map.insert(&92, &"b".into());
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        <Self as QMapImpl<K, V>>::is_empty(self)
    }
    /// Remove the entry with the given key from the map.
    /// Returns 1 if the key existed in the map, and 0 otherwise.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (4, "Four"),
    /// ]);
    /// assert_eq!(map.remove(&2), 1);
    /// assert_eq!(map.keys(), [1, 3, 4])
    /// ```
    pub fn remove(&mut self, key: &K) -> i32 {
        <Self as QMapImpl<K, V>>::remove(self, key)
    }
    /// Returns the number of (key, value) pairs in the map as i32.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// assert_eq!(map.len(), 0);
    /// map.insert(&42, &"Forty two".into());
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn size(&self) -> i32 {
        <Self as QMapImpl<K, V>>::size(self)
    }
    /// Returns the number of (key, value) pairs in the map as usize.
    /// See example of QMap<K, V>::size().
    pub fn len(&self) -> usize {
        self.size() as usize
    }
    /// Returns a list containing all the keys in the map in ascending order.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let map = QMap::<i32, QString>::from([
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (1, "One"),
    /// ]);
    /// assert_eq!(map.keys(), [1, 2, 3]);
    /// ```
    pub fn keys(&self) -> QList<K> {
        <Self as QMapImpl<K, V>>::keys(self)
    }
    /// Returns a list containing all the values in the map, in ascending order of their keys.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let map = QMap::<i32, QString>::from([
    ///     (3, "Three"),
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert_eq!(map.values(), [QString::from("One"), QString::from("Two"), QString::from("Three")]);
    /// ```
    pub fn values(&self) -> QList<V> {
        <Self as QMapImpl<K, V>>::values(self)
    }
    /// Returns the value associated with the specified key.
    /// # Examples
    /// ```ignore
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::from([
    ///     (1, "a"),
    ///     (2, "b"),
    ///     (3, "c"),
    /// ]);
    /// assert_eq!(map.value(&3), "c".into());
    /// ```
    pub fn value(&self, key: &K) -> V {
        <Self as QMapImpl<K, V>>::value(self, key)
    }
}
#[doc(hidden)]
pub trait QMapImpl<K, V>
where
    QList<K>: crate::QListImpl<K>,
    QList<V>: crate::QListImpl<V>,
{
    fn clear(&mut self);
    fn insert(&mut self, key: &K, value: &V);
    fn is_empty(&self) -> bool;
    fn remove(&mut self, key: &K) -> i32;
    fn size(&self) -> i32;
    fn keys(&self) -> QList<K>;
    fn values(&self) -> QList<V>;
    fn value(&self, key: &K) -> V;
    fn do_drop(&mut self);
}
impl<K, V> Drop for QMap<K, V>
where
    Self: QMapImpl<K, V>,
    QList<K>: crate::QListImpl<K>,
    QList<V>: crate::QListImpl<V>,
{
    fn drop(&mut self) {
        <Self as QMapImpl<K, V>>::do_drop(self)
    }
}
