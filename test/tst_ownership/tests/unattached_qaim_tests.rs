// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Item model mutations on instances without an attached `QObject`:
//! the data changes are applied and the view notifications are skipped.

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QModelItem, qobject};

#[derive(Clone, Debug, Default, QModelItem)]
pub struct Entry {
    display: String,
}

#[qobject(Base = QListModel)]
pub mod model {
    use qtbridge::QListModel;

    #[derive(Default)]
    pub struct Model {
        items: Vec<crate::Entry>,
    }

    impl Model {}

    impl QListModel for Model {
        type Item = crate::Entry;

        fn len(&self) -> usize {
            self.items.len()
        }
        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
        fn set_unnotified(&mut self, index: usize, value: Self::Item) -> bool {
            self.items[index] = value;
            true
        }
        fn insert_unnotified(&mut self, index: usize, value: Self::Item) {
            self.items.insert(index, value);
        }
        fn push_unnotified(&mut self, value: Self::Item) {
            self.items.push(value);
        }
        fn remove_unnotified(&mut self, index: usize) -> Self::Item {
            self.items.remove(index)
        }
        fn reset_unnotified(&mut self) {
            self.items.clear();
        }
    }
}
pub use model::Model;

/// Model mutations without an attached QObject apply to the data and skip
/// the view notifications instead of panicking.
fn model_mutations_without_attached_qobject_are_applied() {
    use qtbridge::{QListModel, QListModelBase};

    let entry = |name: &str| Entry { display: name.to_string() };

    let model = Rc::new(RefCell::new(Model::default()));
    model.borrow_mut().push(entry("a"));
    model.borrow_mut().push(entry("b"));
    model.borrow_mut().insert(1, entry("c"));
    assert!(model.borrow_mut().set(0, entry("d")));
    assert_eq!(model.borrow().get(0).unwrap().display, "d");
    assert_eq!(model.borrow_mut().remove(1).display, "c");
    assert_eq!(model.borrow_mut().pop().unwrap().display, "b");
    assert_eq!(model.borrow().len(), 1);
    model.borrow_mut().reset();
    assert_eq!(model.borrow().len(), 0);
}

#[cfg(not(miri))]
fn main() {
    model_mutations_without_attached_qobject_are_applied();
}

#[cfg(miri)]
fn main() {}
