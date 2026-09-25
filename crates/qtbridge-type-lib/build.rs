// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::Path;
use qtbridge_build_utils::qt_build::{QtInstallation, get_cxx_qt_lib_include_path};

fn main() {

    let bridge_files = [
        "src/core/qmetaobject/qmetaobject.rs",
        "src/core/qmetatype/qmetatype.rs",
        "src/core/qmetatypeinterface/qmetatypeinterface.rs",
        "src/core/qobject/qobject.rs",
        "src/testlib/qsignalspy/qsignalspy.rs",
    ];

    let cpp_files = [
        "src/core/qmetatype/cpp/qmetatype.cpp",
        "src/core/qmetatypeinterface/cpp/qmetatypeinterface.cpp",
        "src/core/qobject/cpp/qobject.cpp",
        "src/testlib/qsignalspy/cpp/qsignalspy.cpp",
    ];

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = std::path::Path::new(&manifest_dir).join("src");

    // This becomes DEP_QTBRIDGE_TYPE_LIB_INCLUDE in dependents
    println!("cargo:include={}", include_path.display());
    println!("cargo::metadata=include={}", include_path.display());

    let qt = QtInstallation::default();
    for file in bridge_files {
        println!("cargo::rerun-if-changed={file}");
    }

    let cxx_qt_include_dir = get_cxx_qt_lib_include_path()
        .expect("Failed to get cxx-qt-lib include dir");


    let mut builder = cxx_build::bridges(bridge_files);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include(cxx_qt_include_dir)
        .include("../");
    qt.configure_builder(&mut builder);

    cpp_files.iter()
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

    // Force-link cxx-qt init functions. See src/cxx_qt_init.cpp for details.
    cc::Build::new()
        .cpp(true)
        .file("src/cxx_qt_init.cpp")
        .cargo_metadata(false)
        .compile("qtbridge-cxx-qt-init");
    println!("cargo::rustc-link-lib=static:+whole-archive=qtbridge-cxx-qt-init");

    qt.link_modules(qt_modules);
}
