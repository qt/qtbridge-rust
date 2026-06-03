// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge::{QApp, QObjectHolder};
use qtbridge::{QmlRegister, qobject};

#[qobject]
pub mod cat {
    #[derive(Default)]
    pub struct Cat {
        pub legs: i32,
    }
    impl Cat {
        qproperty!("legs", Member = legs);
    }
}
pub use cat::Cat;

#[qobject]
pub mod dog {
    #[derive(Default)]
    pub struct Dog {
        pub legs: i32,
    }
    impl Dog {
        qproperty!("legs", Member = legs);
    }
}
pub use dog::Dog;

// Reporter is injected as an instance so QML can hand a result back to Rust for assertion.
#[qobject]
pub mod reporter {
    #[derive(Default)]
    pub struct Reporter {
        pub count: i32,
    }
    impl Reporter {
        #[qslot]
        fn report(&mut self, n: i32) {
            self.count = n;
        }
    }
}
pub use reporter::Reporter;

fn qml_list_of_cat_accepts_cats() {
    Cat::register();
    Dog::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            property list<Cat> cats: [ Cat {}, Cat {} ]
            property list<Dog> dogs: [ Dog {}, Dog {} ]

            Component.onCompleted: reporter.report(cats.length)
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 2, "list<Cat> should hold the two Cats");
}

fn main() {
    qml_list_of_cat_accepts_cats();
}
