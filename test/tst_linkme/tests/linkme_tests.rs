// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

//! None of the types in this test are registered explicitly. They must reach
//! QML solely through the automatic registration of the `linkme` feature,
//! which runs the collected registration callbacks in `QApp::new()`.

use qtbridge::{QApp, QObjectHolder, qobject};

#[qobject]
pub mod probe {
    #[derive(Default)]
    pub struct Probe {
        pub element_answer: Option<i32>,
        pub singleton_answer: Option<i32>,
    }

    impl Probe {
        #[qslot]
        fn receive_element_answer(&mut self, value: i32) {
            self.element_answer = Some(value);
        }

        #[qslot]
        fn receive_singleton_answer(&mut self, value: i32) {
            self.singleton_answer = Some(value);
        }
    }
}
pub use probe::Probe;

#[qobject]
pub mod linkme_backend {
    #[derive(Default)]
    pub struct LinkMeBackend {}

    impl LinkMeBackend {
        #[qslot]
        fn answer_to_everything(&self) -> i32 {
            42
        }
    }
}

#[qobject(Singleton)]
pub mod linkme_singleton {
    #[derive(Default)]
    pub struct LinkMeSingleton {}

    impl LinkMeSingleton {
        #[qslot]
        fn answer_to_everything(&self) -> i32 {
            43
        }
    }
}

fn auto_registered_element_is_instantiable() {
    let probe = Probe::default_with_attached_qobject();
    let var = probe.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("probe", &var)
        .load_qml(
            r#"import QtQuick
               import tst_linkme
               Item {
                   required property var probe
                   LinkMeBackend { id: backend }
                   Component.onCompleted: probe.receive_element_answer(backend.answer_to_everything())
               }"#
            .as_bytes(),
        );

    assert_eq!(probe.borrow().element_answer, Some(42));
}

fn auto_registered_singleton_is_accessible() {
    let probe = Probe::default_with_attached_qobject();
    let var = probe.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("probe", &var)
        .load_qml(
            r#"import QtQuick
               import tst_linkme
               Item {
                   required property var probe
                   Component.onCompleted: probe.receive_singleton_answer(LinkMeSingleton.answer_to_everything())
               }"#
            .as_bytes(),
        );

    assert_eq!(probe.borrow().singleton_answer, Some(43));
}

fn main() {
    if cfg!(miri) {
        return;
    }
    auto_registered_element_is_instantiable();
    auto_registered_singleton_is_accessible();
}
