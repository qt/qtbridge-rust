// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::QApp;

fn quit_on_load_failure() {
    // Fails and returns
    QApp::new()
        .load_qml(br#"
            Backend {}"#)
        .run();
}

fn main() {
    if cfg!(miri) {
        return;
    }
    quit_on_load_failure();
}
