// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{
    QMetaType, QObject, QString, QObjectList,
    QList_bool, QList_i8, QList_u8, QList_i16, QList_u16,
    QList_i32, QList_u32, QList_i64, QList_u64,
    QList_f32, QList_f64, QList_QString,
};

#[cfg(feature = "serde_json")]
use qtbridge_type_lib::{QJsonArray, QJsonValue};

use crate::{QMetaTypeGet, QObjectHolder, QMetaInfo, QmlRegister};

/// Enables a type to be used as a signal or slot argument.
///
/// Implemented for:
/// - Primitive numeric types and `bool`
/// - [`String`]
/// - [`Vec<T>`] where `T` is one of the above
/// - [`Rc<RefCell<T>>`] where `T` implements [`QObjectHolder`]
/// - [`Vec<Rc<RefCell<T>>>`] where `T` implements [`QmlRegister`]
///
/// You will not need to implement this trait yourself; adding support for custom types requires CXX/C++ bindings.
pub trait QMetaCallArg: Sized {
    type WireType: Sized;

    fn to_wire(&self) -> Self::WireType;
    fn from_wire(wire: &Self::WireType) -> Self;
    fn wire_metatype() -> QMetaType;
}

macro_rules! impl_direct {
    ($($t:ty),*) => {
        $(impl QMetaCallArg for $t {
            type WireType = $t;
            fn to_wire(&self) -> $t { *self }
            fn from_wire(wire: &$t) -> $t { *wire }
            fn wire_metatype() -> QMetaType { <$t as QMetaTypeGet>::get_qmetatype() }
        })*
    }
}

impl_direct!(bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);

macro_rules! impl_vec_direct {
    ($($t:ty => $qlist:ty),*) => {
        $(impl QMetaCallArg for Vec<$t> {
            type WireType = $qlist;
            fn to_wire(&self) -> $qlist { self.into_iter().collect() }
            fn from_wire(wire: &$qlist) -> Vec<$t> { wire.into() }
            fn wire_metatype() -> QMetaType { <$qlist as QMetaTypeGet>::get_qmetatype() }
        })*
    }
}

impl_vec_direct!(
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
    f64    => QList_f64
);

impl QMetaCallArg for Vec<String> {
    type WireType = QList_QString;

    fn to_wire(&self) -> Self::WireType {
        self.iter()
            .map(QString::from)
            .collect()
    }

    fn from_wire(wire: &Self::WireType) -> Self {
        wire.iter()
            .map(String::from)
            .collect()
    }

    fn wire_metatype() -> QMetaType {
        QList_QString::get_qmetatype()
    }
}

impl QMetaCallArg for String {
    type WireType = QString;
    fn to_wire(&self) -> QString { self.into() }
    fn from_wire(wire: &QString) -> String { wire.into() }
    fn wire_metatype() -> QMetaType { <QString as QMetaTypeGet>::get_qmetatype() }
}

impl QMetaCallArg for Vec<usize> {
    #[cfg(target_pointer_width = "64")]
    type WireType = QList_u64;
    #[cfg(target_pointer_width = "32")]
    type WireType = QList_u32;

    fn to_wire(&self) -> Self::WireType {
        let mut result = Self::WireType::default();
        for elm in self {
            result.append(*elm as _);
        }
        result
    }

    fn from_wire(wire: &Self::WireType) -> Self {
        wire.into_iter()
            .map(|a| *a as usize)
            .collect()
    }

    fn wire_metatype() -> QMetaType {
        <Self::WireType as QMetaTypeGet>::get_qmetatype()
    }
}

impl QMetaCallArg for Vec<isize> {
    #[cfg(target_pointer_width = "64")]
    type WireType = QList_i64;
    #[cfg(target_pointer_width = "32")]
    type WireType = QList_i32;

    fn to_wire(&self) -> Self::WireType {
        let mut result = Self::WireType::default();
        for elm in self {
            result.append(*elm as _);
        }
        result
    }

    fn from_wire(wire: &Self::WireType) -> Self {
        wire.into_iter()
            .map(|a| *a as isize)
            .collect()
    }

    fn wire_metatype() -> QMetaType {
        <Self::WireType as QMetaTypeGet>::get_qmetatype()
    }
}

impl<T: QObjectHolder> QMetaCallArg for Rc<RefCell<T>> {
    type WireType = *mut QObject;

    fn to_wire(&self) -> *mut QObject {
        T::rc_ref_cell_to_qobject(self).cast_mut()
    }

    fn from_wire(wire: &*mut QObject) -> Self {
        unsafe { T::qobject_to_rc_ref_cell(*wire) }
    }

    fn wire_metatype() -> QMetaType {
        <T as QMetaInfo>::get_qobject_ptr_qmetatype()
    }
}

impl<T: QmlRegister> QMetaCallArg for Vec<Rc<RefCell<T>>> {
    type WireType = QObjectList;

    fn to_wire(&self) -> QObjectList {
        self.iter()
            .map(|rc| unsafe { QObject::to_cxx_qt(T::rc_ref_cell_to_qobject(rc).cast_mut()) })
            .collect()
    }

    fn from_wire(wire: &QObjectList) -> Self {
        wire.iter()
            .map(|ptr| unsafe { T::qobject_to_rc_ref_cell(QObject::ptr_from_cxx_qt(ptr)) })
            .collect()
    }

    fn wire_metatype() -> QMetaType {
        QObjectList::get_qmetatype()
    }
}

#[cfg(feature = "serde_json")]
impl QMetaCallArg for serde_json::Value {
    type WireType = QJsonValue;
    fn to_wire(&self) -> QJsonValue { crate::serde_tools::serde_to_qjsonvalue(self) }
    fn from_wire(wire: &QJsonValue) -> serde_json::Value { crate::serde_tools::qjsonvalue_to_serde(wire) }
    fn wire_metatype() -> QMetaType { <QJsonValue as QMetaTypeGet>::get_qmetatype() }
}

#[cfg(feature = "serde_json")]
impl QMetaCallArg for Vec<serde_json::Value> {
    type WireType = QJsonArray;
    fn to_wire(&self) -> QJsonArray { crate::serde_tools::serde_to_qjsonarray(self) }
    fn from_wire(wire: &QJsonArray) -> Vec<serde_json::Value> {
        (0..wire.size()).map(|i| crate::serde_tools::qjsonvalue_to_serde(&wire.at(i))).collect()
    }
    fn wire_metatype() -> QMetaType { <QJsonArray as QMetaTypeGet>::get_qmetatype() }
}
