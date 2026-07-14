// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::{
    QJsonArray, QJsonObject, QJsonValue,
    QList, QList_bool, QList_QString, QList_f32, QList_f64, QList_i8, QList_i16, QList_i32, QList_i64,
    QList_ptr_mut_QObject, QList_u8, QList_u16, QList_u32, QList_u64,
    QMetaType, QObject, QString
};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-runtime/src/cpp/qmetatypeget.h");
        include!("qtbridge-type-lib/src/generated/core/qjsonarray/cpp/qjsonarray.h");
        include!("qtbridge-type-lib/src/generated/core/qjsonobject/cpp/qjsonobject.h");
        include!("qtbridge-type-lib/src/generated/core/qjsonvalue/cpp/qjsonvalue.h");
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist.h");
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_ptr_mut_qobject.h");

        type QJsonArray = super::QJsonArray;
        type QJsonObject = super::QJsonObject;
        type QJsonValue = super::QJsonValue;

        type QMetaType = super::QMetaType;
        type QObject = super::QObject;
        type QString = super::QString;

        type QList_bool = super::QList_bool;
        type QList_i8 = super::QList_i8;
        type QList_u8 = super::QList_u8;
        type QList_i16 = super::QList_i16;
        type QList_u16 = super::QList_u16;
        type QList_i32 = super::QList_i32;
        type QList_u32 = super::QList_u32;
        type QList_i64 = super::QList_i64;
        type QList_u64 = super::QList_u64;
        type QList_f32 = super::QList_f32;
        type QList_f64 = super::QList_f64;
        type QList_QString = super::QList_QString;
        type QList_ptr_mut_QObject = super::QList_ptr_mut_QObject;
    }

    #[namespace = "rust::bridge::qmetatypeget"]
    unsafe extern "C++" {
        // Simple types

        #[rust_name = "get_qmeta_type_bool"]
        unsafe fn QMetaTypeForType(_: *const bool) -> QMetaType;

        #[rust_name = "get_qmeta_type_i8"]
        unsafe fn QMetaTypeForType(_: *const i8) -> QMetaType;
        #[rust_name = "get_qmeta_type_u8"]
        unsafe fn QMetaTypeForType(_: *const u8) -> QMetaType;

        #[rust_name = "get_qmeta_type_i16"]
        unsafe fn QMetaTypeForType(_: *const i16) -> QMetaType;
        #[rust_name = "get_qmeta_type_u16"]
        unsafe fn QMetaTypeForType(_: *const u16) -> QMetaType;

        #[rust_name = "get_qmeta_type_i32"]
        unsafe fn QMetaTypeForType(_: *const i32) -> QMetaType;
        #[rust_name = "get_qmeta_type_u32"]
        unsafe fn QMetaTypeForType(_: *const u32) -> QMetaType;

        #[rust_name = "get_qmeta_type_i64"]
        unsafe fn QMetaTypeForType(_: *const i64) -> QMetaType;
        #[rust_name = "get_qmeta_type_u64"]
        unsafe fn QMetaTypeForType(_: *const u64) -> QMetaType;

        #[rust_name = "get_qmeta_type_f32"]
        unsafe fn QMetaTypeForType(_: *const f32) -> QMetaType;
        #[rust_name = "get_qmeta_type_f64"]
        unsafe fn QMetaTypeForType(_: *const f64) -> QMetaType;

        #[rust_name = "get_qmeta_type_qstring"]
        unsafe fn QMetaTypeForType(_: *const QString) -> QMetaType;

        #[rust_name = "get_qmeta_type_qobject"]
        unsafe fn QMetaTypeForType(_: *const *mut QObject) -> QMetaType;

        #[rust_name = "get_qmeta_type_qjson_array"]
        unsafe fn QMetaTypeForType(_: *const QJsonArray) -> QMetaType;
        #[rust_name = "get_qmeta_type_qjson_object"]
        unsafe fn QMetaTypeForType(_: *const QJsonObject) -> QMetaType;
        #[rust_name = "get_qmeta_type_qjson_value"]
        unsafe fn QMetaTypeForType(_: *const QJsonValue) -> QMetaType;

        // QList types
        #[rust_name = "get_qmeta_type_qlist_bool"]
        unsafe fn QMetaTypeForType(_: *const QList_bool) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_i8"]
        unsafe fn QMetaTypeForType(_: *const QList_i8) -> QMetaType;
        #[rust_name = "get_qmeta_type_qlist_u8"]
        unsafe fn QMetaTypeForType(_: *const QList_u8) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_i16"]
        unsafe fn QMetaTypeForType(_: *const QList_i16) -> QMetaType;
        #[rust_name = "get_qmeta_type_qlist_u16"]
        unsafe fn QMetaTypeForType(_: *const QList_u16) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_i32"]
        unsafe fn QMetaTypeForType(_: *const QList_i32) -> QMetaType;
        #[rust_name = "get_qmeta_type_qlist_u32"]
        unsafe fn QMetaTypeForType(_: *const QList_u32) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_i64"]
        unsafe fn QMetaTypeForType(_: *const QList_i64) -> QMetaType;
        #[rust_name = "get_qmeta_type_qlist_u64"]
        unsafe fn QMetaTypeForType(_: *const QList_u64) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_f32"]
        unsafe fn QMetaTypeForType(_: *const QList_f32) -> QMetaType;
        #[rust_name = "get_qmeta_type_qlist_f64"]
        unsafe fn QMetaTypeForType(_: *const QList_f64) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_qstring"]
        unsafe fn QMetaTypeForType(_: *const QList_QString) -> QMetaType;

        #[rust_name = "get_qmeta_type_qlist_qobject"]
        unsafe fn QMetaTypeForType(_: *const QList_ptr_mut_QObject) -> QMetaType;
    }
}

#[doc(hidden)]
pub trait QMetaTypeGet {
    fn get_qmetatype() -> QMetaType;
}

macro_rules! impl_qmetatype_get {
    ($($t:ty => $ffi_fn:ident),*) => {
        $(
            impl QMetaTypeGet for $t {
                fn get_qmetatype() -> QMetaType {
                    unsafe { ffi::$ffi_fn(std::ptr::null::<$t>()) }
                }
            }
        )*
    }
}

macro_rules! impl_qmetatype_get_qlist {
        ($($t:ty => $ffi_fn_qlist:ident),*) => {
        $(
            impl QMetaTypeGet for QList<$t> {
                fn get_qmetatype() -> QMetaType {
                    unsafe { ffi::$ffi_fn_qlist(std::ptr::null::<QList<$t>>()) }
                }
            }
        )*
    }
}

impl_qmetatype_get! (
    bool         => get_qmeta_type_bool,
    i8           => get_qmeta_type_i8,
    u8           => get_qmeta_type_u8,
    i16          => get_qmeta_type_i16,
    u16          => get_qmeta_type_u16,
    i32          => get_qmeta_type_i32,
    u32          => get_qmeta_type_u32,
    i64          => get_qmeta_type_i64,
    u64          => get_qmeta_type_u64,
    f32          => get_qmeta_type_f32,
    f64          => get_qmeta_type_f64,
    QString      => get_qmeta_type_qstring,
    *mut QObject => get_qmeta_type_qobject,
    QJsonArray   => get_qmeta_type_qjson_array,
    QJsonObject  => get_qmeta_type_qjson_object,
    QJsonValue   => get_qmeta_type_qjson_value
);

impl_qmetatype_get_qlist! (
    bool         => get_qmeta_type_qlist_bool,
    i8           => get_qmeta_type_qlist_i8,
    u8           => get_qmeta_type_qlist_u8,
    i16          => get_qmeta_type_qlist_i16,
    u16          => get_qmeta_type_qlist_u16,
    i32          => get_qmeta_type_qlist_i32,
    u32          => get_qmeta_type_qlist_u32,
    i64          => get_qmeta_type_qlist_i64,
    u64          => get_qmeta_type_qlist_u64,
    f32          => get_qmeta_type_qlist_f32,
    f64          => get_qmeta_type_qlist_f64,
    QString      => get_qmeta_type_qlist_qstring
);

impl QMetaTypeGet for usize {
    #[cfg(target_pointer_width = "64")]
    fn get_qmetatype() -> QMetaType {
        <u64 as QMetaTypeGet>::get_qmetatype()
    }

    #[cfg(target_pointer_width = "32")]
    fn get_qmetatype() -> QMetaType {
        <u32 as QMetaTypeGet>::get_qmetatype()
    }
}

impl QMetaTypeGet for isize {
    #[cfg(target_pointer_width = "64")]
    fn get_qmetatype() -> QMetaType {
        <i64 as QMetaTypeGet>::get_qmetatype()
    }

    #[cfg(target_pointer_width = "32")]
    fn get_qmetatype() -> QMetaType {
        <i32 as QMetaTypeGet>::get_qmetatype()
    }
}

 impl QMetaTypeGet for QList_ptr_mut_QObject {
    fn get_qmetatype() -> QMetaType {
        unsafe { ffi::get_qmeta_type_qlist_qobject(std::ptr::null()) }
    }
}
