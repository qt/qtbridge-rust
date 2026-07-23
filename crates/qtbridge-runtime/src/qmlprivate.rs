// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Low-level QML type registration and the registry of automatic
//! registration callbacks.

// Storage for callbacks performing QML registration of user defined types
#[doc(hidden)]
#[cfg(feature = "linkme")]
#[linkme::distributed_slice]
pub static QML_REGISTER_CALLBACKS: [fn()];

#[doc(hidden)]
pub fn call_qml_register_callbacks() {
    #[cfg(feature = "linkme")]
    {
        use std::sync::Once;
        static INIT_ONCE: Once = Once::new();
        INIT_ONCE.call_once(|| {
            for reg_fn in QML_REGISTER_CALLBACKS {
                reg_fn();
            }
        });
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;

        include!("cpp/qmlprivate.h");
    }
    #[namespace = "rust::bridge::qmlprivate"]
    unsafe extern "C++" {
        // Qt keeps the meta object pointer for the lifetime of the type
        // registration, hence &'static.
        fn qml_register_element(
            type_id: QMetaType,
            list_id: QMetaType,
            object_size: u32,
            parser_status_cast: i32,
            create_fn: usize,
            uri: &[u8],
            version_major: u8,
            version_minor: u8,
            elm_name: &[u8],
            meta_object: &'static QMetaObject,
        );
        fn qml_register_singleton(
            type_id: QMetaType,
            create_fn: usize,
            uri: &[u8],
            version_major: u8,
            version_minor: u8,
            elm_name: &[u8],
            meta_object: &'static QMetaObject,
        );
    }
}

#[doc(hidden)]
pub use ffi::{qml_register_element, qml_register_singleton};
