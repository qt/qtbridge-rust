// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::qobject;
use qtbridge::QModelItem;
use quicktest::run_quick_test;

#[derive(Clone, Debug, QModelItem)]
pub struct MyClass {
    value: i32,
    decoration: String,
    display: String,
}

impl Default for MyClass {
    fn default() -> Self {
        MyClass {
            value: 42,
            decoration: "red".into(),
            display: "42".into()
        }
    }
}
#[qobject(Base = QTableModel, ConvertToCamelCase)]
mod backend {
    use qtbridge::{QTableModel, QTableModelBase};

    use crate::MyClass;

    #[derive(Default)]
    pub struct Backend {
        data: Vec<Vec<crate::MyClass>>,
        column_count: usize
    }

    impl Backend {
        #[qslot]
        fn reset_items(&mut self) {
            self.reset();
        }
        #[qslot]
        fn add_a_column(&mut self) {
            let column = vec![MyClass::default(); self.row_count()];
            self.push_column(&column);
        }
        #[qslot]
        fn add_a_row(&mut self) {
            let row = vec![MyClass::default(); self.column_count()];
            self.push_row(&row);
        }
        #[qslot]
        fn pop_a_column(&mut self) {
            self.pop_column();
        }
        #[qslot]
        fn pop_a_row(&mut self) {
            self.pop_row();
        }
        #[qslot]
        fn insert_a_column(&mut self) {
            let column = vec![MyClass::default(); self.row_count()];
            self.insert_column(0, &column);
        }
        #[qslot]
        fn insert_a_row(&mut self) {
            let row = vec![MyClass::default(); self.column_count()];
            self.insert_row(0, &row);
        }
        #[qslot]
        fn remove_a_column(&mut self) {
            self.remove_column(0);
        }
        #[qslot]
        fn remove_a_row(&mut self) {
            self.remove_row(0);
        }
        #[qslot]
        fn change_item_at(&mut self, i: i32, j: i32) {
            self.set((i as usize, j as usize), crate::MyClass { value: 0, decoration: "changed".to_string(), display: "black".to_string()});
        }
    }

    impl QTableModel for Backend {
        type Item = crate::MyClass;

        fn row_count(&self) -> usize {
            self.data.len()
        }
        fn column_count(&self) -> usize {
            self.column_count
        }
        fn get(&self, index: (usize, usize)) -> Option<&Self::Item> {
            self.data.get(index.0)?.get(index.1)
        }
        fn reset_unnotified(&mut self) {
            self.data = Vec::default();
            self.column_count = 0;
        }
        fn set_unnotified(&mut self, index: (usize, usize), value: Self::Item) -> bool {
            self.data[index.0][index.1] = value;
            true
        }
        fn push_row_unnotified(&mut self, values: &[Self::Item]) {
            assert_eq!(self.column_count, values.len(),
                "Length of values must match number of columns"
            );
            self.data.push(values.to_vec())
        }
        fn push_column_unnotified(&mut self, values: &[Self::Item]) {
            assert_eq!( self.data.len(), values.len(),
                "Length of values must match number of rows"
            );

            for (row, value) in self.data.iter_mut().zip(values.iter()) {
                row.push(value.clone());
            }
            self.column_count += 1;
        }
        fn insert_row_unnotified(&mut self, index: usize, values: &[Self::Item]) {
            assert_eq!(self.column_count, values.len(),
                "Length of values must match number of columns"
            );
            self.data.insert(index, values.to_vec())
        }
        fn insert_column_unnotified(&mut self, index: usize, values: &[Self::Item]) {
            assert_eq!( self.data.len(), values.len(),
                "Length of values must match number of rows"
            );

            for (row, value) in self.data.iter_mut().zip(values.iter()) {
                row.insert(index, value.clone());
            }
            self.column_count += 1;
        }
        fn pop_row_unnotified(&mut self) -> Option<Vec<Self::Item>> {
            self.data.pop()
        }
        fn pop_column_unnotified(&mut self) -> Option<Vec<Self::Item>> {
            let mut poped_column = Vec::default();
            for row in self.data.iter_mut() {
                poped_column.push(row.pop()?);
            }
            self.column_count -= 1;
            Some(poped_column)
        }
        fn remove_row_unnotified(&mut self, index: usize) -> Vec<Self::Item> {
            self.data.remove(index)
        }
        fn remove_column_unnotified(&mut self, index: usize) -> Vec<Self::Item> {
            let mut removed_column = Vec::with_capacity(self.data.len());
            for row in self.data.iter_mut() {
                removed_column.push(row.remove(index));
            }
            self.column_count -= 1;
            removed_column
        }
    }
}

pub use backend::Backend;

#[run_quick_test(Class = Backend, Name = "rustmodel", Input = "qml", Harness = false)]
fn test_qtablemodel() {}
