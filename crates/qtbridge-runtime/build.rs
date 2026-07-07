// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::Path;
use qtbridge_build_utils::qt_build::{QtInstallation, get_cxx_qt_lib_include_path};

fn main() {

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = std::path::Path::new(&manifest_dir).join("src");

    // This becomes DEP_QTBRIDGE_RUNTIME_INCLUDE in dependents
    println!("cargo:include={}", include_path.display());
    println!("cargo::metadata=include={}", include_path.display());

    let bridge_files = [
        "dynamicmetaobjectbuilder",
        "dynamicmetaobjectdata",
        "qmetatypeget",
        "qmlprivate",
        "qresource",
        "rustobjectgetter",
        "qml_method_invoker",
    ];

    let other_rust_files: Vec<&str> = vec!(
    );

    let other_cpp_files: Vec<&str> = vec!(
    );

    let qt_modules = [
        "Core",
        "Gui",
        "Qml",
        "QuickTest"
    ];

    let mut rust_bridge_files: Vec<String> = Vec::new();
    let mut cpp_files: Vec<String> = Vec::new();

    // Handle bridge files (rust+[cpp+h] file with the same stem)
    for bridge_file in &bridge_files {
        let rust_file = format!("src/{bridge_file}.rs");
        let cpp_file = format!("src/cpp/{bridge_file}.cpp");

        println!("cargo::rerun-if-changed={rust_file}");
        rust_bridge_files.push(rust_file);
        if Path::new(&cpp_file).is_file() {
            cpp_files.push(cpp_file);
        }
    }

    for rust_file in other_rust_files {
        let rust_file = format!("src/{rust_file}");
        println!("cargo::rerun-if-changed={rust_file}");
    }

    for cpp_file in other_cpp_files {
        let cpp_file = format!("src/cpp/{cpp_file}");
        cpp_files.push(cpp_file);
    }


    let type_lib_include = std::env::var("DEP_QTBRIDGE_TYPE_LIB_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set. This variable should have been set by qtbridge-type-lib");
    let cxx_qt_include_dir = get_cxx_qt_lib_include_path()
        .expect("Failed to get cxx-qt-lib include dir");

    let qt = QtInstallation::default();
    let mut builder = cxx_build::bridges(rust_bridge_files);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("../")
        .include(type_lib_include)
        .include(cxx_qt_include_dir)
        .include("src");
    qt.configure_builder(&mut builder);

    let qt_include_dirs = qt.include_dirs(qt_modules, true);
    for include_dir in qt_include_dirs {
        builder.include(include_dir);
    }

    for cpp_file in &cpp_files {
        builder.file(cpp_file);

        println!("cargo::rerun-if-changed={cpp_file}");
        let cpp_path = Path::new(cpp_file);
        let h_path = cpp_path.with_extension("").with_extension("h");
        if h_path.is_file() {
            println!("cargo::rerun-if-changed={}", h_path.to_str().unwrap());
        }
    }

    println!("cargo::rerun-if-changed=src/lib.rs");

    builder.compile("bridge");

    qt.link_modules(qt_modules);
}
