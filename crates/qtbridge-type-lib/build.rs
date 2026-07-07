// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::Path;
use qtbridge_build_utils::qt_build::{QtInstallation, get_cxx_qt_lib_include_path};

mod generated_files_bridge;
use generated_files_bridge::GENERATED_FILES_BRIDGE;
mod generated_files_cpp;
use generated_files_cpp::GENERATED_FILES_CPP;

fn main() {

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = std::path::Path::new(&manifest_dir).join("src");

    // This becomes DEP_QTBRIDGE_TYPE_LIB_INCLUDE in dependents
    println!("cargo:include={}", include_path.display());
    println!("cargo::metadata=include={}", include_path.display());

    let qt = QtInstallation::default();
    for file in GENERATED_FILES_BRIDGE {
        println!("cargo::rerun-if-changed={file}");
    }

    let cxx_qt_include_dir = get_cxx_qt_lib_include_path()
        .expect("Failed to get cxx-qt-lib include dir");


    let mut builder = cxx_build::bridges(GENERATED_FILES_BRIDGE);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include(cxx_qt_include_dir)
        .include("../");
    qt.configure_builder(&mut builder);

    GENERATED_FILES_CPP.iter()
        .for_each(|file| {
            builder.file(file);
            println!("cargo::rerun-if-changed={file}");
            let h_path = Path::new(file).with_extension("").with_extension("h");
            if h_path.is_file() {
                println!("cargo::rerun-if-changed={}", h_path.to_str().unwrap());
            }
        });

    let qt_modules = ["Core", "Gui", "Qml", "Test"];
    for include_dir in qt.include_dirs(qt_modules, true) {
        builder.include(include_dir);
    }

    builder.compile("qtbridge-type-lib");

    qt.link_modules(qt_modules);
}
