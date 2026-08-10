// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::QApp;
use qtbridge::QObjectHolder;

use tst_qabstractitemmodel::Backend;

fn main() {
    let backend = Rc::new(RefCell::new(Backend::default()));

    let properties = [("rustmodel", backend.borrow().as_qvariant())];
    QApp::new()
        .with_initial_properties(&properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
