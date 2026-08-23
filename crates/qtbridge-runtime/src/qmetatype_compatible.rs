// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{QList, QList_QString, QMetaType, QObject, QObjectList, QString};

#[cfg(feature = "serde_json")]
use qtbridge_type_lib::{QJsonArray, QJsonValue};

use crate::{QMetaInfo, QMetaTypeGet, QObjectHolder, QmlRegister};

/// Enables a type to be used as a meta call argument and to be convertible from/to QVariant (later).
///
/// Implemented for:
/// - Primitive numeric types and `bool`
/// - [`String`]
/// - [`Vec<T>`] where `T` is one of the above
/// - [`Rc<RefCell<T>>`] where `T` implements [`QObjectHolder`]
/// - [`Vec<Rc<RefCell<T>>>`] where `T` implements [`QmlRegister`]
///
pub trait QMetaTypeCompatible {
    type CompatibleType: QMetaTypeGet;

    fn to_compatible(&self) -> Self::CompatibleType;
    fn from_compatible(from: &Self::CompatibleType) -> Self;
    fn compatible_qmetatype() -> QMetaType {
        Self::CompatibleType::get_qmetatype()
    }
}

macro_rules! impl_primitive_direct {
    ($($t:ty),*) => {
        $(impl QMetaTypeCompatible for $t {
            type CompatibleType = $t;
            fn to_compatible(&self) -> Self::CompatibleType { *self }
            fn from_compatible(from: &Self::CompatibleType) -> Self { *from }
        })*
    }
}
impl_primitive_direct!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

macro_rules! impl_primitive_convert {
    ($($t:ty => $compat:ty),*) => {
        $(impl QMetaTypeCompatible for $t {
            type CompatibleType = $compat;
            fn to_compatible(&self) -> Self::CompatibleType { *self as $compat }
            fn from_compatible(from: &Self::CompatibleType) -> Self { *from as Self }
        })*
    }
}
#[cfg(target_pointer_width = "64")]
impl_primitive_convert!(
    isize => i64,
    usize => u64
);
#[cfg(target_pointer_width = "32")]
impl_primitive_convert!(
    isize => i32,
    usize => u32
);

// The wire pointer is safe single-threaded: arguments outlive delivery on
// the emitting stack frame, and no collection point runs between a return
// value leaving Rust and the engine taking ownership. Revisit when signals
// can cross threads.
macro_rules! impl_vec_direct {
    ($($t:ty),*) => {
        $(impl QMetaTypeCompatible for Vec<$t> {
            type CompatibleType = QList<$t>;
            fn to_compatible(&self) -> Self::CompatibleType { self.into_iter().collect() }
            fn from_compatible(from: &Self::CompatibleType) -> Self { from.into() }
        })*
    }
}
impl_vec_direct!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

macro_rules! impl_vec_convert {
    ($($t:ty => $compat:ty),*) => {
        $(impl QMetaTypeCompatible for Vec<$t> {
            type CompatibleType = QList<$compat>;
            fn to_compatible(&self) -> Self::CompatibleType {
                self.iter()
                    .map(|arg| *arg as $compat)
                    .collect()
            }
            fn from_compatible(from: &Self::CompatibleType) -> Self {
                from.iter()
                    .map(|arg| *arg as $t)
                    .collect()
            }
        })*
    }
}
#[cfg(target_pointer_width = "64")]
impl_vec_convert!(
    isize => i64,
    usize => u64
);
#[cfg(target_pointer_width = "32")]
impl_vec_convert!(
    isize => i32,
    usize => u32
);

impl QMetaTypeCompatible for String {
    type CompatibleType = QString;
    fn to_compatible(&self) -> QString { self.into() }
    fn from_compatible(from: &QString) -> String { from.into() }
}

impl QMetaTypeCompatible for Vec<String> {
    type CompatibleType = QList_QString;

    fn to_compatible(&self) -> Self::CompatibleType {
        self.iter()
            .map(QString::from)
            .collect()
    }

    fn from_compatible(from: &Self::CompatibleType) -> Self {
        from.iter()
            .map(String::from)
            .collect()
    }
}

impl<T: QObjectHolder> QMetaTypeCompatible for Rc<RefCell<T>> {
    type CompatibleType = *mut QObject;

    fn to_compatible(&self) -> *mut QObject {
        T::rc_ref_cell_to_qobject(self).cast_mut()
    }

    fn from_compatible(from: &*mut QObject) -> Self {
        unsafe { T::qobject_to_rc_ref_cell(*from) }
    }

    fn compatible_qmetatype() -> QMetaType {
        <T as QMetaInfo>::get_qobject_ptr_qmetatype()
    }
}

// The wire pointers share the in-flight guarantee of Rc<RefCell<T>> above.
impl<T: QmlRegister> QMetaTypeCompatible for Vec<Rc<RefCell<T>>> {
    type CompatibleType = QObjectList;

    fn to_compatible(&self) -> QObjectList {
        self.iter()
            .map(|rc| unsafe { QObject::to_cxx_qt(T::rc_ref_cell_to_qobject(rc).cast_mut()) })
            .collect()
    }

    fn from_compatible(from: &QObjectList) -> Self {
        from.iter()
            .map(|ptr| unsafe { T::qobject_to_rc_ref_cell(QObject::ptr_from_cxx_qt(ptr)) })
            .collect()
    }
}

#[cfg(feature = "serde_json")]
impl QMetaTypeCompatible for serde_json::Value {
    type CompatibleType = QJsonValue;
    fn to_compatible(&self) -> QJsonValue { crate::serde_tools::serde_to_qjsonvalue(self) }
    fn from_compatible(from: &QJsonValue) -> serde_json::Value { crate::serde_tools::qjsonvalue_to_serde(from) }
}

#[cfg(feature = "serde_json")]
impl QMetaTypeCompatible for Vec<serde_json::Value> {
    type CompatibleType = QJsonArray;
    fn to_compatible(&self) -> QJsonArray { crate::serde_tools::serde_to_qjsonarray(self) }
    fn from_compatible(from: &QJsonArray) -> Vec<serde_json::Value> {
        from.iter()
            .map(|jv| crate::serde_tools::qjsonvalue_to_serde(&jv))
            .collect()
    }
}
