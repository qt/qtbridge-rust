// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;

        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;
    }

    unsafe extern "C++" {
        include!("cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData;

        #[rust_name = "get_meta_object"]
        fn getMetaObject(&self) -> *mut QMetaObject;
    }
}

pub use ffi::DynamicMetaObjectData;
