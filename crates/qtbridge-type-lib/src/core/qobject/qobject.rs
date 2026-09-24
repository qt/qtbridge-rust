// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = crate::QObject;
        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = crate::QMetaObject;
        include!("qtbridge-type-lib/src/core/qvariant/cpp/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::bridge::qobject"]
    unsafe extern "C++" {
        /// Returns a pointer to the meta-object of `obj`.
        #[rust_name = "meta_object"]
        fn metaObject(obj: &QObject) -> *const QMetaObject;

        /// Calls C++ `delete` on `obj`.
        #[rust_name = "delete"]
        unsafe fn deleteQObject(obj: *mut QObject);

        /// Calls the (virtual) destructor of `obj` without freeing its memory.
        #[rust_name = "destruct"]
        unsafe fn destructQObject(obj: *mut QObject);

        /// Returns the value of the property `name`, or an invalid `QVariant` if there is none.
        fn property(obj: &QObject, name: &str) -> QVariant;

        /// Sets the property `name`. Returns `false` if it is not declared in the meta-object.
        #[rust_name = "set_property"]
        unsafe fn setProperty(obj: *mut QObject, name: &str, value: &QVariant) -> bool;
    }
}

pub use ffi::{delete, destruct, meta_object, property, set_property};
