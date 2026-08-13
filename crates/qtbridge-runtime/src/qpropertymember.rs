// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{
    QMetaType, QObject, QString, QVariant,
    QList_bool, QList_i8, QList_u8, QList_i16, QList_u16,
    QList_i32, QList_u32, QList_i64, QList_u64,
    QList_f32, QList_f64, QList_QString,
};

#[cfg(feature = "serde_json")]
use qtbridge_type_lib::{QJsonArray, QJsonValue};

use crate::{QMetaTypeGet, QObjectHolder, QMetaInfo, QmlRegister};

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

macro_rules! impl_primitive {
    ($($t:ty),*) => {
        $(impl QPropertyMember for $t {
            fn qmetatype() -> QMetaType { <$t as QMetaTypeGet>::get_qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

impl_primitive!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

macro_rules! impl_primitive_as {
    ($($t:ty as $underlying:ty),*) => {
        $(impl QPropertyMember for $t {
            fn qmetatype() -> QMetaType { <$underlying>::qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { <$underlying>::to_qvariant(&(*self as $underlying), _owner) }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> { <$underlying>::from_qvariant(value).map(|v| v as $t) }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

#[cfg(target_pointer_width = "64")]
impl_primitive_as!(
    usize as u64,
    isize as i64
);
#[cfg(target_pointer_width = "32")]
impl_primitive_as!(
    usize as u32,
    isize as i32
);

macro_rules! impl_vec {
    ($($t:ty => $qlist:ty),*) => {
        $(impl QPropertyMember for Vec<$t> {
            fn qmetatype() -> QMetaType { <$qlist as QMetaTypeGet>::get_qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

impl_vec!(
    bool   => QList_bool,
    i8     => QList_i8,
    u8     => QList_u8,
    i16    => QList_i16,
    u16    => QList_u16,
    i32    => QList_i32,
    u32    => QList_u32,
    i64    => QList_i64,
    u64    => QList_u64,
    f32    => QList_f32,
    f64    => QList_f64,
    String => QList_QString
);

macro_rules! impl_vec_as {
    ($($t:ty as $underlying:ty),*) => {
        $(impl QPropertyMember for Vec<$t> {
            fn qmetatype() -> QMetaType { <Vec<$underlying> as QPropertyMember>::qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
                let vec: Vec<$underlying> = self.iter()
                    .map(|a| *a as $underlying)
                    .collect();
                QVariant::from(vec)
            }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
                let vec = <Vec<$underlying>>::try_from(value)?;
                Ok(vec.into_iter()
                    .map(|a| a as $t)
                    .collect())
            }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

#[cfg(target_pointer_width = "64")]
impl_vec_as! (
    usize as u64,
    isize as i64
);

#[cfg(target_pointer_width = "32")]
impl_vec_as! (
    usize as u32,
    isize as i32
);

impl QPropertyMember for String {
    fn qmetatype() -> QMetaType { <QString as QMetaTypeGet>::get_qmetatype() }
    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
    fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
    fn property_eq(&self, other: &Self) -> bool { self == other }
}

impl<T: QObjectHolder> QPropertyMember for Rc<RefCell<T>> {
    fn qmetatype() -> QMetaType {
        <T as QMetaInfo>::get_qobject_ptr_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        T::rc_ref_cell_to_qobject(self).cast_mut().into()
    }

    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        let ptr = <*mut QObject>::try_from(value)?;
        Ok(unsafe { T::qobject_to_rc_ref_cell(ptr) })
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

#[cfg(feature = "serde_json")]
impl QPropertyMember for serde_json::Value {
    fn qmetatype() -> QMetaType { <QJsonValue as QMetaTypeGet>::get_qmetatype() }
    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        let jv = crate::serde_tools::serde_to_qjsonvalue(self);
        let qvar_cxx = <QJsonValue as cxx_qt_lib::QVariantValue>::construct(&jv);
        QVariant::from_cxx_qt(&qvar_cxx)
    }
    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        crate::serde_tools::qvariant_to_serde(value)
    }
    fn property_eq(&self, other: &Self) -> bool { self == other }
}

#[cfg(feature = "serde_json")]
impl QPropertyMember for Vec<serde_json::Value> {
    fn qmetatype() -> QMetaType { <QJsonArray as QMetaTypeGet>::get_qmetatype() }
    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        let ja = crate::serde_tools::serde_to_qjsonarray(self);
        let qvar_cxx = <QJsonArray as cxx_qt_lib::QVariantValue>::construct(&ja);
        QVariant::from_cxx_qt(&qvar_cxx)
    }
    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        match crate::serde_tools::qvariant_to_serde(value)? {
            serde_json::Value::Array(arr) => Ok(arr),
            _ => Err(()),
        }
    }
    fn property_eq(&self, other: &Self) -> bool { self == other }
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
