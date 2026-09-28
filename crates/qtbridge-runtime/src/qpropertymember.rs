// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{QMetaType, QObjectMutPtr, QVariant};

use crate::{QMetaTypeCompatible, QObjectHolder, QVariantConvertible, QmlElement};

/// Enables a type to be used as a property.
///
/// Implemented for:
/// - Primitive numeric types and `bool`
/// - [`String`]
/// - [`Vec<T>`] where `T` is one of the above
/// - [`Rc<RefCell<T>>`] where `T` implements [`QObjectHolder`]
/// - [`Vec<Rc<RefCell<T>>>`] where `T` implements [`QmlElement`]
///
/// You will not need to implement this trait yourself; adding support for custom types requires CXX/C++ bindings.
pub trait QPropertyMember: Sized {
    fn qmetatype() -> QMetaType;

    /// Returns a `QVariant` representation of `self` for read operations.
    /// `Owner` is the [`QObjectHolder`] that holds this property; passing it
    /// allows returning views onto its members and borrowing correctly on
    /// access. If the member is passed by value, `owner` can be ignored.
    ///
    /// # Safety
    ///
    /// For `QObject`-backed members the returned `QVariant` carries a raw,
    /// untracked `QObject*`: the caller must consume it and start tracking
    /// its state immediately, before any garbage collector can run. The
    /// metaobject dispatch upholds this by consuming the variant within
    /// the same call stack. See [`from_qvariant`](QPropertyMember::from_qvariant).
    unsafe fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant;

    /// Returns a `QVariant` view of `self` for read operations, with access to
    /// the property's notify signal. Unlike [`to_qvariant`](QPropertyMember::to_qvariant),
    /// this variant can return a live view that emits `notify` when the
    /// underlying data changes.
    ///
    /// The default implementation ignores `notify` and falls back to
    /// [`to_qvariant`](QPropertyMember::to_qvariant).
    ///
    /// # Safety
    ///
    /// Same untracked-pointer contract as [`to_qvariant`](QPropertyMember::to_qvariant).
    unsafe fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        let _ = notify;
        unsafe { self.to_qvariant(owner) }
    }

    /// Converts `value` into the concrete type, used for write operations.
    ///
    /// # Safety
    ///
    /// For `QObject`-backed members this reads a raw `QObject*` out of the
    /// variant and dereferences it: the caller must guarantee the variant
    /// genuinely holds a live `QObject` of a compatible type. The metaobject
    /// dispatch upholds this (Qt type-checks the property write).
    unsafe fn from_qvariant(value: &QVariant) -> Option<Self>;

    /// Returns `true` if `self` and `other` are equal.
    /// Used to decide whether the notify signal should be emitted and the
    /// stored value replaced on a property write.
    fn property_eq(&self, other: &Self) -> bool;
}

impl<T: PartialEq + QMetaTypeCompatible + QVariantConvertible> QPropertyMember for T {
    fn qmetatype() -> QMetaType {
        <Self as QMetaTypeCompatible>::compatible_qmetatype()
    }

    unsafe fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        QVariantConvertible::to_qvariant(self)
    }

    unsafe fn from_qvariant(value: &QVariant) -> Option<Self> {
        QVariantConvertible::try_from_qvariant(value)
    }

    fn property_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T: QObjectHolder> QPropertyMember for Rc<RefCell<T>> {
    fn qmetatype() -> QMetaType {
        <T as QObjectHolder>::get_qobject_ptr_qmetatype()
    }

    unsafe fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        let ptr = T::rc_ref_cell_to_qobject(self).cast_mut();
        let ptr_wrap = unsafe { QObjectMutPtr::from_raw(ptr.cast()) };
        (&ptr_wrap).into()
    }

    unsafe fn from_qvariant(value: &QVariant) -> Option<Self> {
        let ptr_wrap: QObjectMutPtr = value.value()?;
        let ptr: *mut cxx_qt::QObject = ptr_wrap.into_raw();
        Some(unsafe { T::qobject_to_rc_ref_cell(ptr.cast()) })
    }

    fn property_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T: QmlElement> QPropertyMember for Vec<Rc<RefCell<T>>> {
    fn qmetatype() -> QMetaType {
        T::get_list_qmetatype()
    }

    unsafe fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant {
        T::list_to_qvariant(owner, self, |_: &mut Owner| {})
    }

    unsafe fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        T::list_to_qvariant(owner, self, notify)
    }

    unsafe fn from_qvariant(_value: &QVariant) -> Option<Self> {
        // Vec<Rc<RefCell<T>>> is exposed as writeable view and no write operation will ever happen
        None
    }

    fn property_eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| Rc::ptr_eq(a, b))
    }
}

/// Returns the [`QMetaType`] of the return value of a `FnOnce`. Given a function
/// |this: &Self| { &this.member } this allows to infer the metatype of a member
/// field without requiring the type to be known at macro expansion time. The
/// closure is never called.
#[doc(hidden)]
pub fn get_meta_type_of_fn_return_value<F, This, R>(_f: F) -> QMetaType
where
    F: FnOnce(&This) -> &R,
    R: QPropertyMember,
{
    R::qmetatype()
}
