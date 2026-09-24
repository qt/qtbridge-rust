// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::{QMap_QString_QVariant, QObject};

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmap/cpp/qmap_qstring_qvariant.h");
        type QMap_QString_QVariant = super::QMap_QString_QVariant;
        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = super::QObject;
    }


    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("cpp/quicktestmain.h");

        #[rust_name = "quick_test_main"]
        fn quickTestMain(args: &Vec<String>, name: &String) -> i32;

        #[rust_name = "quick_test_main_with_properties"]
        fn quickTestMainWithProperties(args: &Vec<String>, name: &String, properties: &QMap_QString_QVariant) -> i32;
    }
}

pub use ffi::quick_test_main;
pub use ffi::quick_test_main_with_properties;
