// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::{Path, PathBuf};
use qtbridge_build_utils::qt_build::{QtInstallation, get_cxx_qt_lib_include_path};

fn main() {

    let bridge_files = vec!(
        "quicktestmain",
        "messagecapture"
    );

    let other_rust_files: Vec<&str> = vec!(
    );

    let other_cpp_files: Vec<&str> = vec!(
    );

    let moc_files = [
        "src/cpp/qtestsetup.h",
    ];

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

    let qt = QtInstallation::default();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")
        .expect("Failed to get OUT_DIR"));
    for moc_file in &moc_files {
        let input = PathBuf::from(moc_file);
        let output = out_dir
            .join(input.file_stem().unwrap())
            .with_extension("moc");
        qt.run_moc(&input, &output);
    }

    let cxx_qt_include_dir = get_cxx_qt_lib_include_path()
        .expect("Failed to get cxx-qt-lib include dir");

    let mut builder = cxx_build::bridges(rust_bridge_files);

    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("../crates/")
        .include("src")
        .include("../crates/qtbridge-type-lib/src/")
        .include(cxx_qt_include_dir)
        .include(out_dir);
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

    builder.compile("quicktest");

    qt.link_modules(qt_modules);
}
