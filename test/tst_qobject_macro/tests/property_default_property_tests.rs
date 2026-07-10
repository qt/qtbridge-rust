// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QApp, QObjectHolder, QmlRegister, qobject};

#[derive(Default)]
pub struct Cat {
    pub legs: i32,
}

#[qobject]
impl Cat {
    qproperty!("legs", Member = legs);
}

#[derive(Default)]
pub struct Dog {
    pub legs: i32,
}

#[qobject]
impl Dog {
    qproperty!("legs", Member = legs);
}

#[derive(Default)]
pub struct Reporter {
    pub count: i32,
}

#[qobject]
impl Reporter {
    #[qslot]
    fn report(&mut self, n: i32) {
        self.count = n;
    }
}

pub struct SingleHolder {
    kitten: Rc<RefCell<Cat>>,
}

#[qobject]
impl SingleHolder {
    qproperty!("kitten", Member = kitten, Default);
}

impl Default for SingleHolder {
    fn default() -> Self {
        // Placeholder Cat; QML overwrites it with the declared default child.
        Self { kitten: Cat::default_with_attached_qobject() }
    }
}

#[derive(Default)]
pub struct ListHolder {
    kittens: Vec<Rc<RefCell<Cat>>>,
}

#[qobject]
impl ListHolder {
    qproperty!("kittens", Member = kittens, Default);
}

fn single_object_default_property_receives_child() {
    Cat::register();
    SingleHolder::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            SingleHolder {
                id: holder
                Cat { legs: 5 }
            }
            Component.onCompleted: {
                reporter.report(holder.kitten.legs);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 5);
}

fn list_default_property_receives_children() {
    Cat::register();
    ListHolder::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            ListHolder {
                id: holder
                Cat { legs: 4 }
                Cat { legs: 3 }
            }
            Component.onCompleted: {
                let total = 0;
                for (let i = 0; i < holder.kittens.length; ++i) {
                    total += holder.kittens[i].legs;
                }
                reporter.report(total);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 7);
}

fn wrong_type_is_rejected_by_qml() {
    Cat::register();
    Dog::register();
    SingleHolder::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    // Assigning a `Dog` to `kitten` (declared type `Cat*`) must be refused by
    // QML with a type error - not accepted and then panicked on in Rust. The
    // property keeps its default `Cat` (legs == 0), so `report` still runs.
    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            SingleHolder {
                id: holder
                Dog { legs: 4 }
            }
            Component.onCompleted: {
                reporter.report(holder.kitten.legs);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .load_qml(qml.as_bytes());

    // Default Cat is untouched; crucially we reached here without a panic.
    assert_eq!(reporter.borrow().count, 0);
}

fn main() {
    single_object_default_property_receives_child();
    list_default_property_receives_children();
    wrong_type_is_rejected_by_qml();
}
