// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use cxx::UniquePtr;
use std::pin::Pin;
use crate::QVariantMap;

#[qt_gen::bridge]
mod qqmlapplicationengine {
    include_in_cpp!(<QQmlApplicationEngine>);
    include_in_cpp!("rustconv.h");

    /// QQmlApplicationEngine provides a convenient way to load an application from a single QML file.
    ///
    /// See also: [QQmlApplicationEngine documentation](https://doc.qt.io/qt-6/qqmlapplicationengine.html).
    struct QQmlApplicationEngine;

    /// Create a new QQmlApplicationEngine instance.
    pub fn new() -> UniquePtr<Self> {
        cpp_fn!(|| -> UniquePtr<Self> {
            return std::make_unique<QQmlApplicationEngine>();
        })()
    }

    /// Adds path as a directory where the engine searches for installed modules
    /// in a URL-based directory structure.
    pub fn add_import_path(mut self: Pin<&mut Self>, path: &str) {
        let cpp = cpp_fn!(|&mut self, path: &str| {
            self.addImportPath(RustStrToQString(path));
        });
        cpp(self.as_mut(), path)
    }

    /// Loads the root QML file located at filePath.
    /// filePath must be a path to a local file or a path to a file in the resource file system.
    /// If filePath is a relative path, it is taken as relative to the application's working directory.
    /// The object tree defined by the file is instantiated immediately.
    pub fn load(mut self: Pin<&mut Self>, file_path: &str) {
        let cpp = cpp_fn!(|&mut self, file_path: &str| {
            self.load(RustStrToQString(file_path));
        });
        cpp(self.as_mut(), file_path)
    }

    /// Loads the QML given in data. The object tree defined by data is instantiated immediately.
    pub fn load_data(mut self: Pin<&mut Self>, data: &[u8]) {
        let cpp = cpp_fn!(|&mut self, data: &[u8]| {
            self.loadData(RustSliceToQByteArray(data));
        });
        cpp(self.as_mut(), data)
    }

    /// Loads the QML type typeName from the module specified by uri.
    /// If the type originates from a QML file located at a remote url,
    /// the type will be loaded asynchronously.
    pub fn load_from_module(mut self: Pin<&mut Self>, uri: &str, type_name: &str) {
        let cpp = cpp_fn!(|&mut self, uri: &str, type_name: &str| {
            self.loadFromModule(RustStrToQString(uri), RustStrToQString(type_name));
        });
        cpp(self.as_mut(), uri, type_name)
    }

    /// Sets the initialProperties with which the QML component gets initialized after it gets loaded.
    /// # Examples
    /// ```ignore
    /// let properties = QVariantMap::from([
    ///     ("startValue", 0.into()),
    ///     ("endValue", 1.into()),
    ///     ("speed", (0.2).into()),
    ///     ("opacity", (0.7).into()),
    /// ]);
    /// qml_engine.pin_mut().set_initial_properties(&properties);
    /// ```
    pub fn set_initial_properties(mut self: Pin<&mut Self>, initial_properties: &QVariantMap) {
        let cpp = cpp_fn!(|&mut self, props: &QVariantMap| {
            self.setInitialProperties(props);
        });
        cpp(self.as_mut(), initial_properties)
    }
}
