// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::qobject;
use qtbridge::qtbridge_type_lib::QVariant;
use quicktest::run_quick_test;

#[derive(Clone)]
pub struct Cell {
    data: Vec<QVariant>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            data: vec![QVariant::default(); 6], // 6 default roles in Qt
        }
    }

    pub fn set(&mut self, role: usize, value: QVariant) {
        if role < self.data.len() {
            self.data[role] = value;
        }
    }

    pub fn get(&self, role: usize) -> QVariant {
        self.data.get(role).unwrap_or(&QVariant::default()).clone()
    }

    pub fn reset(&mut self, role: usize) {
        if role < self.data.len() {
            self.data[role] = QVariant::default();
        }
    }
}

pub struct Row {
    columns: Vec<Cell>,
    children: Vec<Row>,
}

impl Row {
    pub fn new(column_count: usize) -> Self {
        Self {
            columns: vec![Cell::new(); column_count],
            children: Vec::new(),
        }
    }

    pub fn from_children(row_vec: Vec<Row>) -> Self {
        Self {
            columns: vec![Cell::new(); 5],
            children: row_vec,
        }
    }

    pub fn row_count(&self) -> usize {
        self.children.len()
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn get_cell(&self, index: usize) -> Option<&Cell> {
        self.columns.get(index)
    }

    pub fn get_cell_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.columns.get_mut(index)
    }

    pub fn get_child(&self, index: usize) -> Option<&Row> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.children.get_mut(index)
    }

    pub fn add_child(&mut self, row: Row) {
        self.children.push(row);
    }

    pub fn insert_child(&mut self, index: usize, row: Row) {
        self.children.insert(index, row);
    }

    pub fn parent_of(&self, row: &Row) -> Option<&Row> {
        // Not the most efficient way of finding a parent but good enough for a test
        for child in &self.children {
            if child as *const Row == row as *const Row {
                return Some(self);
            }
            if let Some(found) = child.parent_of(row) {
                return Some(found);
            }
        }
        None
    }

    pub fn index_of(&self, row: &Row) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child as *const Row == row as *const Row {
                return Some(i);
            }
        }
        None
    }
}

#[qobject(Base = QAbstractItemModel, ConvertToCamelCase)]
mod backend {

    use super::Row;
    use qtbridge::qtbridge_type_lib::{QVariant, QModelIndex};
    use qtbridge::{QAbstractItemModel, QAbstractItemModelBase};

    pub struct Backend {
        root: Row,
    }

    impl Default for Backend {
        fn default() -> Self {

            let mut row0 = Row::new(5);
            let mut row1 = Row::new(5);
            let mut row10 = Row::new(5);
            let mut row11 = Row::new(5);
            let mut row110 = Row::new(5);
            let mut row111 = Row::new(5);
            let mut row2 = Row::new(5);

            for col in 0..=4 {
                if let Some(cell) = row0.columns.get_mut(col) {
                    let value = format!("Row0, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row1.columns.get_mut(col) {
                    let value = format!("Row1, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row10.columns.get_mut(col) {
                    let value = format!("Row1-0, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row11.columns.get_mut(col) {
                    let value = format!("Row1-1, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row110.columns.get_mut(col) {
                    let value = format!("Row1-1-0, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row111.columns.get_mut(col) {
                    let value = format!("Row1-1-1, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
                if let Some(cell) = row2.columns.get_mut(col) {
                    let value = format!("Row2, Column {}", col);
                    cell.set(0, QVariant::from(value));
                }
            }

            row1.add_child(row10);
            row11.add_child(row110);
            row11.add_child(row111);
            row1.add_child(row11);
            Self {
                root: Row::from_children(vec![row0, row1, row2]),
            }
        }
    }

    impl QAbstractItemModel for Backend {
        fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
            if parent.is_valid() {
                let parent_ptr = parent.internal_pointer_mut() as *const Row;
                if !parent_ptr.is_null() {
                    let parent_row_ref = unsafe { &*parent_ptr };
                    let maybe_row_ref = parent_row_ref.get_child(row as usize);
                    if let Some(row_ref) = maybe_row_ref {
                        let ptr = row_ref as *const Row as usize;
                        return self.create_index(row, column, ptr);
                    }
                }
            } else {
                if (row as usize) < self.root.row_count() {
                    let row_ref = &self.root.children[row as usize];
                    let ptr = row_ref as *const Row as usize;
                    return self.create_index(row, column, ptr)
                }
            }
            return QModelIndex::default();
        }

        fn parent(&self, child: &QModelIndex) -> QModelIndex {
            if child.is_valid() {
                let child_ptr: *const Row = child.internal_pointer_mut() as *const Row;
                if let Some(child_row_ref) = unsafe { child_ptr.as_ref() } {
                    if let Some(parent_ref) = self.root.parent_of(child_row_ref) {
                        if let Some(grandparent_ref) = self.root.parent_of(parent_ref) {
                            if let Some(row_index) = grandparent_ref.index_of(parent_ref) {
                                let ptr = parent_ref as *const Row as usize;
                                return self.create_index(row_index as i32, 0, ptr);
                            }
                        }
                    }
                }
            }
            return QModelIndex::default();
        }

        fn row_count(&self, parent: &QModelIndex) -> i32 {
            let parent_ptr: *const Row = parent.internal_pointer_mut() as *const Row;
            if !parent_ptr.is_null() {
                let parent_ref: &Row = unsafe { &*parent_ptr };
                return parent_ref.row_count() as i32;
            }
            return self.root.row_count() as i32;
        }

        fn column_count(&self, parent: &QModelIndex) -> i32 {
            // columns are usually the same for all rows. At least that is what the views expect
            let parent_ptr: *const Row = parent.internal_pointer_mut() as *const Row;
            if !parent_ptr.is_null() {
                let parent_ref: &Row = unsafe { &*parent_ptr };
                return parent_ref.column_count() as i32;
            }
            return self.root.column_count() as i32;
        }

        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
            if index.is_valid() {
                let ptr = index.internal_pointer_mut() as *const Row;
                if !ptr.is_null() {
                    let row_ref: &Row = unsafe { &*ptr };
                    if let Some(cell) = row_ref.get_cell(index.column() as usize) {
                        return cell.get(role as usize);
                    }
                }
            }
            return QVariant::default();
        }

        fn set_data(&mut self, _index: &QModelIndex, _value: &QVariant, _role: i32) -> bool {
            false
        }
    }
}

pub use backend::Backend;

#[run_quick_test(Class = Backend, Name = "model", Input = "qml", Harness = false)]
fn test_qabstractitemmodel() {}
