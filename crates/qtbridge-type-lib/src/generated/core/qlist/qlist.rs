// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

/// The QList is a generic struct that provides a dynamic array.
///
/// QList is one of Qt's generic container structs. It stores its items in adjacent memory locations and provides fast index-based access.
///
/// The following types are currently supported as items in a QList:
/// * [i8][crate::QList_i8]
/// * [u8][crate::QList_u8]
/// * [i16][crate::QList_i16]
/// * [u16][crate::QList_u16]
/// * [i32][crate::QList_i32]
/// * [u32][crate::QList_u32]
/// * [i64][crate::QList_i64]
/// * [u64][crate::QList_u64]
/// * [f32][crate::QList_f32]
/// * [f64][crate::QList_f64]
/// * [QByteArray][crate::QList_QByteArray] (also known as [QByteArrayList][crate::QByteArrayList])
/// * [QString][crate::QList_QString] (also known as [QStringList][crate::QStringList])
/// * [QVariant][crate::QList_QVariant] (also known as [QVariantList][crate::QVariantList])
/// * [*mut QObject][crate::QObjectList]
///
/// See also [QList documentation](https://doc.qt.io/qt-6/qlist.html).
#[derive(Debug)]
#[repr(C)]
pub struct QList<T>
where
    Self: QListImpl<T>,
{
    _d: std::mem::MaybeUninit<usize>,
    _ptr: std::mem::MaybeUninit<usize>,
    _size: std::mem::MaybeUninit<usize>,
    phantoms: core::marker::PhantomData<T>,
}
impl<T> QList<T>
where
    Self: QListImpl<T>,
{
    /// Inserts value at the end of the list.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::default();
    /// list.append(1);
    /// list.append(2);
    /// let three: i32 = 3;
    /// list.append(three);
    /// assert_eq!(list, [1, 2, 3]);
    /// ```
    pub fn append(&mut self, value: T) {
        <Self as QListImpl<T>>::append(self, value)
    }
    /// Returns the maximum number of items that can be stored in the list without forcing a reallocation.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::default();
    /// assert_eq!(0, list.capacity());
    /// list.append(1);
    /// assert!(1 <= list.capacity());
    /// list.reserve(100);
    /// assert_eq!(list.capacity(), 100);
    /// ```
    pub fn capacity(&self) -> usize {
        <Self as QListImpl<T>>::capacity(self)
    }
    /// Removes all the elements from the list.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3]);
    /// assert!(!list.is_empty());
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        <Self as QListImpl<T>>::clear(self)
    }
    /// Returns true if the list contains an occurrence of value; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let list = QList::from([10, 20, 30]);
    /// assert!(list.contains(&20));
    /// assert!(!list.contains(&40));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        <Self as QListImpl<T>>::contains(self, value)
    }
    /// Returns true if the list has size 0; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::default();
    /// assert!(list.is_empty());
    ///
    /// list.append(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
    /// Inserts value at the end of the list.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3]);
    /// list.push_back(4);
    /// assert_eq!(list, [1, 2, 3, 4]);
    /// ```
    pub fn push_back(&mut self, value: T) {
        <Self as QListImpl<T>>::push_back(self, value)
    }
    /// Removes 1 element from the list, starting at index position i.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// list.remove(3);
    /// assert_eq!(list, [1, 2, 3, 5, 6, 7, 8, 9]);
    /// ```
    pub fn remove(&mut self, i: isize) {
        <Self as QListImpl<T>>::remove(self, i)
    }
    /// Attempts to allocate memory for at least size elements.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let mut list = QList::<i32>::default();
    /// list.reserve(100);
    /// assert_eq!(list.capacity(), 100);
    /// ```
    pub fn reserve(&mut self, size: usize) {
        <Self as QListImpl<T>>::reserve(self, size)
    }
    /// Returns the number of items in the list as isize.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let list = QList::from([1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(list.len(), 7);
    /// ```
    pub fn size(&self) -> isize {
        <Self as QListImpl<T>>::size(self)
    }
    /// Returns the number of items in the list as usize.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let list = QList::from([1, 2, 3, 4, 5, 6, 7, 8]);
    /// assert_eq!(list.len(), 8);
    /// ```
    pub fn len(&self) -> usize {
        self.size() as usize
    }
    /// Returns a const reference to the first item in the list. This function assumes that the list isn't empty.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let list = QList::from([1, 2, 3]);
    /// assert_eq!(*list.first(), 1);
    /// ```
    pub fn first(&self) -> &T {
        <Self as QListImpl<T>>::first(self)
    }
    /// Returns a const reference to the last item in the list. This function assumes that the list isn't empty.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QList;
    /// let list = QList::from([1, 2, 3]);
    /// assert_eq!(*list.last(), 3);
    pub fn last(&self) -> &T {
        <Self as QListImpl<T>>::last(self)
    }
}
#[doc(hidden)]
pub trait QListImpl<T> {
    fn append(&mut self, value: T);
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn contains(&self, value: &T) -> bool;
    fn push_back(&mut self, value: T);
    fn remove(&mut self, i: isize);
    fn reserve(&mut self, size: usize);
    fn size(&self) -> isize;
    fn first(&self) -> &T;
    fn last(&self) -> &T;
    fn do_drop(&mut self);
}
impl<T> Drop for QList<T>
where
    Self: QListImpl<T>,
{
    fn drop(&mut self) {
        <Self as QListImpl<T>>::do_drop(self)
    }
}
