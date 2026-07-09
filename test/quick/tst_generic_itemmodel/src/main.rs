// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::QApp;
use qtbridge::QObjectHolder;

use tst_generic_itemmodel::Backend;

//Manual test
fn main() {
    let data = vec![1, 2, 3, 4, 5, 10, 100, 1000];
    let backend = Rc::new(RefCell::new(Backend::<i32>::new(data)));
    Backend::attach_qobject(&backend);
    let data2 = Vec::from(["one", "two", "three", "ten", "hundred"].map(String::from));
    let backend2 = Rc::new(RefCell::new(Backend::<String>::new(data2)));
    Backend::attach_qobject(&backend2);

    use qtbridge::qtbridge_runtime::QMetaInfo;
    let a = <Backend<i32> as QMetaInfo>::get_qmetatype();
    let b = <Backend<String> as QMetaInfo>::get_qmetatype();
    assert_ne!(a, b, "QMetaTypes are not unique");

    let initial_properties = [
        ("rustmodel", backend.borrow().as_qvariant()),
        ("rustmodel2", backend2.borrow().as_qvariant()),
    ];

    QApp::new()
        .with_initial_properties(&initial_properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
