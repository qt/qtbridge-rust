// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Capture of Qt log messages, used by tests to inspect QML diagnostics.

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("cpp/messagecapture.h");

        #[rust_name = "install_message_capture"]
        fn installMessageCapture();

        #[rust_name = "take_captured_messages"]
        fn takeCapturedMessages() -> Vec<String>;
    }
}

pub use ffi::{install_message_capture, take_captured_messages};
