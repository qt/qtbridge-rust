// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::qobject;
use qtbridge::QModelItem;
use quicktest::run_quick_test;

#[derive(Clone, Debug, Default, QModelItem)]
pub struct MyClass {
    value: i32,
    decoration: String,
    display: String,
}

#[qobject(Base = QListModel, ConvertToCamelCase)]
mod backend {
    use qtbridge::{QListModel, QListModelBase};

    #[derive(Default)]
    pub struct Backend {
        internal_list: Vec<crate::MyClass>,
    }

    impl Backend {
        #[qslot]
        fn reset_items(&mut self) {
            self.reset();
        }
        #[qslot]
        fn push_four_items(&mut self) {
            self.push( crate::MyClass { value: 4, decoration: "orange".to_string(), display: "Four".to_string() } );
            self.push( crate::MyClass { value: 5, decoration: "orange".to_string(), display: "Five".to_string() } );
            self.push( crate::MyClass { value: 6, decoration: "orange".to_string(), display: "Six".to_string() } );
            self.push( crate::MyClass { value: 7, decoration: "orange".to_string(), display: "Seven".to_string() } );
        }
        #[qslot]
        fn remove_first_two_items(&mut self) {
            self.remove(1);
            self.remove(0);
        }
        #[qslot]
        fn pop_two_items(&mut self) {
            self.pop();
            self.pop();
        }
        #[qslot]
        fn set_two_to_null(&mut self) {
            self.set(1, crate::MyClass { value: 0, decoration: "".to_string(), display: "".to_string()});
            self.set(2, crate::MyClass { value: 0, decoration: "".to_string(), display: "".to_string()});
        }
        #[qslot]
        fn insert_two_after_first(&mut self) {
            self.insert(1, crate::MyClass { value: 0, decoration: "".to_string(), display: "".to_string()});
            self.insert(1, crate::MyClass { value: 0, decoration: "".to_string(), display: "".to_string()});
        }
    }

    impl QListModel for Backend {
        type Item = crate::MyClass;

        fn len(&self) -> usize {
            self.internal_list.len()
        }
        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.internal_list.get(index)
        }
        fn reset_unnotified(&mut self) {
            self.internal_list = vec![
                crate::MyClass { value: 1, decoration: "red".to_string(), display: "One".to_string() },
                crate::MyClass { value: 2, decoration: "green".to_string(), display: "Two".to_string() },
                crate::MyClass { value: 3, decoration: "blue".to_string(), display: "Three".to_string() },
            ];
        }
        fn set_unnotified(&mut self, index: usize, value: Self::Item) -> bool {
            self.internal_list[index] = value;
            true
        }
        fn insert_unnotified(&mut self, index:usize, value: Self::Item) {
            self.internal_list.insert(index, value);
        }
        fn push_unnotified(&mut self, value: Self::Item) {
            self.internal_list.push(value);
        }
        fn remove_unnotified(&mut self, index: usize) -> Self::Item {
            self.internal_list.remove(index)
        }
    }
}

pub use backend::Backend;

#[run_quick_test(Class = Backend, Name = "rustmodel", Input = "qml", Harness = false)]
fn test_qlistmodel() {}
