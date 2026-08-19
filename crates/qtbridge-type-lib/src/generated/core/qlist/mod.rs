// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![allow(non_camel_case_types)]

pub type QList<T> = cxx_qt_lib::QList<T>;

pub type QList_bool = QList<bool>;
pub type QList_i8 = QList<i8>;
pub type QList_u8 = QList<u8>;
pub type QList_i16 = QList<i16>;
pub type QList_u16 = QList<u16>;
pub type QList_i32 = QList<i32>;
pub type QList_u32 = QList<u32>;
pub type QList_i64 = QList<i64>;
pub type QList_u64 = QList<u64>;
pub type QList_f32 = QList<f32>;
pub type QList_f64 = QList<f64>;
pub type QList_QVariant = QList<cxx_qt_lib::QVariant>;
pub type QList_QString = QList<crate::QString>;
pub type QList_QByteArray = QList<crate::QByteArray>;
pub type QList_QObjectMutPtr = QList<cxx_qt_lib::QObjectMutPtr>;

pub type QStringList = QList_QString;
pub type QByteArrayList = QList_QByteArray;
pub type QObjectList = QList_QObjectMutPtr;
pub type QVariantList = QList_QVariant;
