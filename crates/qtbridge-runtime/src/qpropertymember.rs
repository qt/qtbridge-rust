// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use cxx_qt_lib::QObjectMutPtr;
use qtbridge_type_lib::{QMetaType, QVariant};

use crate::{QMetaInfo, QMetaTypeCompatible, QObjectHolder, QVariantConvertible, QmlRegister, ToQVariant, TryFromQVariant};

/// Enables a type to be used as a property.
///
/// Implemented for:
/// - Primitive numeric types and `bool`
/// - [`String`]
/// - [`Vec<T>`] where `T` is one of the above
/// - [`Rc<RefCell<T>>`] where `T` implements [`QObjectHolder`]
/// - [`Vec<Rc<RefCell<T>>>`] where `T` implements [`QmlRegister`]
///
/// You will not need to implement this trait yourself; adding support for custom types requires CXX/C++ bindings.
pub trait QPropertyMember: Sized {
    fn qmetatype() -> QMetaType;

    /// Returns a `QVariant` representation of `self` for read operations.
    /// `Owner` is the [`QObjectHolder`] that holds this property; passing it
    /// allows returning views onto its members and borrowing correctly on
    /// access. If the member is passed by value, `owner` can be ignored.
    fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant;

    /// Returns a `QVariant` view of `self` for read operations, with access to
    /// the property's notify signal. Unlike [`to_qvariant`](QPropertyMember::to_qvariant),
    /// this variant can return a live view that emits `notify` when the
    /// underlying data changes.
    ///
    /// The default implementation ignores `notify` and falls back to
    /// [`to_qvariant`](QPropertyMember::to_qvariant).
    fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        let _ = notify;
        self.to_qvariant(owner)
    }

    /// Converts `value` into the concrete type, used for write operations.
    fn from_qvariant(value: &QVariant) -> Result<Self, ()>;

    /// Returns `true` if `self` and `other` are equal.
    /// Used to decide whether the notify signal should be emitted and the
    /// stored value replaced on a property write.
    fn property_eq(&self, other: &Self) -> bool;
}

impl<T: PartialEq + QMetaTypeCompatible + QVariantConvertible> QPropertyMember for T {
    fn qmetatype() -> QMetaType {
        <Self as QMetaTypeCompatible>::compatible_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        ToQVariant::to_qvariant(self)
    }

    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        TryFromQVariant::try_from_qvariant(value)
    }

    fn property_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T: QObjectHolder> QPropertyMember for Rc<RefCell<T>> {
    fn qmetatype() -> QMetaType {
        <T as QMetaInfo>::get_qobject_ptr_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        let ptr = T::rc_ref_cell_to_qobject(self).cast_mut();
        let ptr_wrap = unsafe { QObjectMutPtr::from_raw(ptr.cast()) };
        (&ptr_wrap).into()
    }

    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        let ptr_wrap: QObjectMutPtr = value.value()
            .ok_or(())?;
        let ptr: *mut cxx_qt::QObject = ptr_wrap.into_raw();
        Ok(unsafe { T::qobject_to_rc_ref_cell(ptr.cast()) })
    }

    fn property_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T: QmlRegister> QPropertyMember for Vec<Rc<RefCell<T>>> {
    fn qmetatype() -> QMetaType {
        T::get_list_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant {
        T::list_to_qvariant(owner, self, |_: &mut Owner| {})
    }

    fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        T::list_to_qvariant(owner, self, notify)
    }

    fn from_qvariant(_value: &QVariant) -> Result<Self, ()> {
        // Vec<Rc<RefCell<T>>> is exposed as writeable view and no write operation will ever happen
        Err(())
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
