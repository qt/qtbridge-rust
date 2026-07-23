// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::qobject;
use qtbridge::QObjectHolder;

#[qobject(Base = QAbstractItemModel, ConvertToCamelCase)]
mod backend {
    use qtbridge::qtbridge_type_lib::{QVariant, QModelIndex};
    use qtbridge::{QAbstractItemModel, QAbstractItemModelBase};

    #[derive(Default)]
    pub struct Backend<T>
    where T: 'static + Default,
        for<'a> qtbridge::qtbridge_type_lib::QVariant: From<&'a T>, // TODO: make it work without fully qualified path for QVariant
    {
        data: Vec<T>,
    }
    impl<T> Backend<T>
    where T: 'static + Default,
        for<'a> QVariant: From<&'a T>,
    {
        pub fn new(data: Vec<T>) -> Self {
            Self {
                data: data
            }
        }
    }

    impl<T> QAbstractItemModel for Backend<T>
        where
        T: 'static + Default,
        for<'a> qtbridge::qtbridge_type_lib::QVariant: From<&'a T>,
    {

        fn index(&self, row: i32, column: i32, _parent: &QModelIndex) -> QModelIndex {
            self.create_index(row, column, 0)
        }

        fn parent(&self, _child: &QModelIndex) -> QModelIndex {
            QModelIndex::default()
        }

        fn row_count(&self, parent: &QModelIndex) -> i32 {
            if !parent.is_valid() {
                self.data.len() as i32
            } else {
                0
            }
        }

        fn column_count(&self, _parent: &QModelIndex) -> i32 {
            1
        }

        fn data(&self, index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::from(&self.data[index.row() as usize])
        }

        fn set_data(&mut self, _index: &QModelIndex, _value: &QVariant, _role: i32) -> bool {
            false
        }
    }

}

pub use backend::Backend;

pub fn test_qabstractitemmodel() {
    use std::env;
    use std::path::PathBuf;

    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    input_path.push("qml");
    let input_folder = input_path.to_str().unwrap().to_string();

    println!("Running quick test with qml files in \"{}\"", &input_folder);

    let args: Vec<String> = vec![
        file!().to_string(),
        "-input".to_string(),
        input_folder,
    ];

    use qtbridge::qtbridge_type_lib::QVariantMap;
    use quicktest::quick_test_main_with_properties;
    let data = vec![1, 2, 3, 10, 100];
    let test_object = Rc::new(RefCell::new(Backend::<i32>::new(data)));
    Backend::attach_qobject(&test_object);

    let mut properties = QVariantMap::default();
    properties.insert("listmodel".into(), test_object.borrow().as_qvariant().to_cxx_qt());
    let result = quick_test_main_with_properties(&args, &"test_qabstractitemmodel".into(), &properties);

    assert_eq!(result, 0, "quick_test failed with code {}", result);
}
