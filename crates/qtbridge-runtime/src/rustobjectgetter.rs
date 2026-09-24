// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;
    }

    unsafe extern "C++" {
        include!("cpp/rustobjectgetter.h");

        #[rust_name = "get_rust_proxy"]
        fn getRustProxy(qobj: &QObject) -> *const u8;
    }
}

pub use ffi::get_rust_proxy;
