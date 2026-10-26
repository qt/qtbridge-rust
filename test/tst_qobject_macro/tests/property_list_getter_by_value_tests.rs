// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! A list property exposed through a getter that returns the list by value
//! is exposed to QML as a `QList<QObject*>` copy instead of a
//! `QQmlListProperty` view onto the object.

use qtbridge::{QApp, QmlObject, QmlElement, qobject};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
pub struct Cat {
    pub legs: i32,
}
#[qobject]
impl Cat {
    qproperty!("legs", Member = legs);
}

// Reporter is injected as an instance so QML can hand a result back to Rust for assertion.
#[derive(Default)]
pub struct Reporter {
    pub count: i32,
    pub legs: i32,
}
#[qobject]
impl Reporter {
    #[qslot]
    fn report(&mut self, count: i32, legs: i32) {
        self.count = count;
        self.legs = legs;
    }
}

#[derive(Default)]
pub struct Container {
    kittens: Vec<Rc<RefCell<Cat>>>,
}
#[qobject]
impl Container {
    qproperty!("kittens", Read = get_kittens, Write = set_kittens, Notify = kittens_changed);

    #[qsignal]
    fn kittens_changed(&mut self);

    pub fn get_kittens(&self) -> Vec<Rc<RefCell<Cat>>> {
        self.kittens.clone()
    }

    pub fn set_kittens(&mut self, kittens: Vec<Rc<RefCell<Cat>>>) {
        self.kittens = kittens;
        self.kittens_changed();
    }
}

// Without a setter, only the getter decides that the list is exposed as a copy.
#[derive(Default)]
pub struct ReadOnlyContainer;
#[qobject]
impl ReadOnlyContainer {
    qproperty!("kittens", Read = get_kittens, Notify = kittens_changed);

    #[qsignal]
    fn kittens_changed(&mut self);

    // Returns a temporary list, not a field of `self`.
    pub fn get_kittens(&self) -> Vec<Rc<RefCell<Cat>>> {
        [4, 3].into_iter()
            .map(|legs| {
                let cat = Cat::default_with_attached_qobject();
                cat.borrow_mut().legs = legs;
                cat
            })
            .collect()
    }
}

/// QML reads the list returned by value from the getter.
fn qml_reads_list_from_by_value_getter() {
    Cat::register();

    let reporter = Reporter::default_with_attached_qobject();
    let container = ReadOnlyContainer::default_with_attached_qobject();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            required property var container
            Component.onCompleted: {
                let list = container.kittens;
                let total = 0;
                for (let i = 0; i < list.length; ++i) {
                    total += list[i].legs;
                }
                reporter.report(list.length, total);
            }
        }
    "#;

    QApp::new()
        .set_initial_object("reporter", reporter.clone())
        .set_initial_object("container", container.clone())
        .load_qml(qml.as_bytes());

    let reporter = reporter.borrow();
    assert_eq!((reporter.count, reporter.legs), (2, 7), "QML should see 2 Cats with 4 + 3 = 7 legs");
}

/// QML assigns a whole list, which goes through the setter.
fn qml_writes_list_through_setter() {
    Cat::register();

    let container = Container::default_with_attached_qobject();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var container
            property list<Cat> source: [ Cat { legs: 2 }, Cat { legs: 6 } ]
            Component.onCompleted: {
                container.kittens = source;
            }
        }
    "#;

    QApp::new()
        .set_initial_object("container", container.clone())
        .load_qml(qml.as_bytes());

    let kittens = container.borrow().get_kittens();
    assert_eq!(kittens.len(), 2, "Rust Vec should hold the QML-assigned Cats");
    assert_eq!(kittens[0].borrow().legs, 2, "Cat 0 should have 2 legs after QML write");
    assert_eq!(kittens[1].borrow().legs, 6, "Cat 1 should have 6 legs after QML write");
}

fn main() {
    qml_reads_list_from_by_value_getter();
    qml_writes_list_through_setter();
}
