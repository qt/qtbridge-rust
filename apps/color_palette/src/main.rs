// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

use qtbridge::QApp;
use qtbridge::include_bytes_qml;
use qtbridge::qtbridge_type_lib::{QString, QVariant};

mod basic_login;
mod paginated_source;
mod rest_service;
mod utils;

use basic_login::BasicLogin;
use paginated_source::PaginatedResource;
use rest_service::RestService;

const DEFAULT_URL: &str = "http://127.0.0.1:49425/api";

/// Parses the `--url <url>` command line option, falling back to `DEFAULT_URL`.
fn parse_url_arg() -> String {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--url" => {
                return args.next().unwrap_or_else(|| {
                    eprintln!("--url requires a value");
                    std::process::exit(1);
                });
            }
            "-h" | "--help" => {
                println!("Usage: {} [--url <url>]", env!("CARGO_BIN_NAME"));
                println!("  --url <url>  URL of the REST API server (default: {DEFAULT_URL})");
                println!("  -h, --help   Print this help message");
                std::process::exit(0);
            }
            _ => {
                if let Some(value) = arg.strip_prefix("--url=") {
                    return value.to_string();
                }
                eprintln!("Unknown argument: {arg}");
                std::process::exit(1);
            }
        }
    }
    DEFAULT_URL.to_string()
}

fn main() {
    let url = parse_url_arg();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        eprintln!("Invalid url \"{url}\": must start with http:// or https://");
        std::process::exit(1);
    }

    // Qt QML modules live under the `:/qt/qml/<Module>/` resource prefix, and the
    // QML hard-codes icon paths like `qrc:/qt/qml/color_palette/icons/qt.png`, so
    // everything is registered under `qt/qml/...` to match.
    include_bytes_qml!("icons", "qt/qml/color_palette");
    include_bytes_qml!("color_palette", "qt/qml");

    QApp::new()
        // Register the Rust back-end types into the "color_palette" QML module so
        // the .qml files can use RestService / PaginatedResource / BasicLogin
        // without an explicit import (they are part of the same module).
        .register::<RestService>()
        .register::<PaginatedResource>()
        .register::<BasicLogin>()
        .add_initial_property("serverUrl", &QVariant::from(&QString::from(url)))
        .add_import_path("qrc:/qt/qml")
        .load_qml_from_file("qrc:/qt/qml/color_palette/Main.qml")
        .run();
}
