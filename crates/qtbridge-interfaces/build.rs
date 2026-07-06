// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::PathBuf;

use qtbridge_build_utils::file_system_utils::find_files;
use qtbridge_build_utils::qt_build::QtInstallation;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const FILES_BRIDGE: [&'static str; 10] = [
    "src/qabstract_item_model/proxy_cpp_bridge.rs",
    "src/qabstract_item_model/proxy_rust_bridge.rs",
    "src/qlist_model/proxy_cpp_bridge.rs",
    "src/qlist_model/proxy_rust_bridge.rs",
    "src/qobject/proxy_cpp_bridge.rs",
    "src/qobject/proxy_rust_bridge.rs",
    "src/qparser_status/proxy_cpp_bridge.rs",
    "src/qparser_status/proxy_rust_bridge.rs",
    "src/qtable_model/proxy_cpp_bridge.rs",
    "src/qtable_model/proxy_rust_bridge.rs",
];

const FILES_CPP: [&'static str; 5] = [
    "src/qabstract_item_model/cpp/QAbstractItemModelProxyCpp.cpp",
    "src/qlist_model/cpp/QListModelProxyCpp.cpp",
    "src/qobject/cpp/QObjectProxyCpp.cpp",
    "src/qparser_status/cpp/QParserStatusProxyCpp.cpp",
    "src/qtable_model/cpp/QTableModelProxyCpp.cpp",
];

fn main() {

    let type_lib_include = std::env::var("DEP_QTBRIDGE_TYPE_LIB_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set. This variable should have been set by qtbridge-type-lib");

    let runtime_include = std::env::var("DEP_QTBRIDGE_RUNTIME_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set - This variable should have been set by qtbridge-runtime");

    let qt = QtInstallation::default();
    let mut builder = cxx_build::bridges(FILES_BRIDGE);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include("../")
        .include(type_lib_include)
        .include(runtime_include);
    qt.configure_builder(&mut builder);

    FILES_CPP.iter()
        .for_each(|file| {
            builder.file(file);
        });

    let qt_modules = ["Core", "Gui", "Qml"];
    for include_dir in qt.include_dirs(qt_modules, true) {
        builder.include(include_dir);
    }

    builder.compile("qtbridge-interfaces");

    qt.link_modules(qt_modules);
    // Trigger a rebuild when C++ files have changed.
    let src_path = PathBuf::from(MANIFEST_DIR).join("src");
    find_files(&src_path, true,
        |path| path.extension().is_some_and(|ext| ext == "cpp" || ext == "h"))
        .expect("Failed to find C++ files")
        .iter()
        .for_each(|path| println!("cargo::rerun-if-changed={}", path.display()));
}
