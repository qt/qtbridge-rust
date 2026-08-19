// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use cxx::UniquePtr;
use qtbridge_type_lib::{QGuiApplication, QQmlApplicationEngine, QString, QVariant, QVariantMap};
use crate::qml_register::QmlRegister;

/// Entry point for a QML application.
///
/// Wraps the Qt application and QML engine. Configure it with the builder
/// methods and call [`run`](QApp::run) to start the event loop.
///
/// # Example
///
/// A minimal “Hello World” application without a Rust backend:
///
/// ```rust
///# use qtbridge_runtime::QApp;
/// QApp::new()
///     .load_qml(br#"
///         import QtQuick
///         import QtQuick.Controls
///         Text {
///             text: "Hello Rust!"
///#            Component.onCompleted: closeTimer.start()
///#            Timer {
///#                id: closeTimer
///#                interval: 1
///#                onTriggered: Qt.quit()
///#            }
///         }"#)
///     .run();
/// ```
pub struct QApp {
    engine: UniquePtr<QQmlApplicationEngine>, // engine must be first field so its dropped before app
    #[allow(dead_code)]
    app: UniquePtr<QGuiApplication>,
    initial_properties: QVariantMap,
}

impl QApp {
    /// Creates the Qt application and QML engine.
    ///
    /// Must be called before any QML or GUI functionality is used.
    pub fn new() -> Self {
        let app = QGuiApplication::new();
        crate::qmlprivate::call_qml_register_callbacks();
        let engine = QQmlApplicationEngine::new();
        Self {
            engine: engine,
            app: app,
            initial_properties: QVariantMap::default(),
        }
    }

    /// Enters the Qt main event loop.
    ///
    /// Blocks until the application exits and returns the exit code.
    /// Usually the last call in `main`.
    pub fn run(&mut self) -> i32 {
        QGuiApplication::exec()
    }

    /// Queues an initial property to be set on the root QML object.
    ///
    /// Properties are applied when [`load_qml`](QApp::load_qml) or
    /// [`load_qml_from_file`](QApp::load_qml_from_file) is called.
    /// Call multiple times to set several properties.
    ///
    /// # Example
    ///
    /// ```rust
    ///# use qtbridge_runtime::QApp;
    /// let prop = 42;
    ///
    /// QApp::new()
    /// .add_initial_property("answer", &(&prop).into())
    /// .load_qml(br#"
    ///     import QtQuick
    ///     import QtQuick.Controls
    ///     ApplicationWindow {
    ///         required property var answer
    ///#        Component.onCompleted: closeTimer.start()
    ///#        Timer {
    ///#            id: closeTimer
    ///#            interval: 1
    ///#            onTriggered: Qt.quit()
    ///#        }
    ///     }"#)
    /// .run();
    /// ```
    pub fn add_initial_property(&mut self, id: &str, value: &QVariant) -> &mut Self {
        self.initial_properties.insert(QString::from(id), value.clone());
        self
    }

    /// Sets multiple initial properties on the root QML object at once.
    ///
    /// Must be called before [`load_qml`](QApp::load_qml) or
    /// [`load_qml_from_file`](QApp::load_qml_from_file).
    ///
    /// # Example
    ///
    /// ```rust
    ///# use qtbridge_runtime::QApp;
    /// let prop = 42;
    ///
    /// QApp::new()
    ///     .with_initial_properties(&[
    ///         ("answer", (&prop).into()),
    ///     ])
    ///     .load_qml(br#"
    ///         import QtQuick
    ///         import QtQuick.Controls
    ///         ApplicationWindow {
    ///             required property var answer
    ///#            Component.onCompleted: closeTimer.start()
    ///#            Timer {
    ///#                id: closeTimer
    ///#                interval: 1
    ///#                onTriggered: Qt.quit()
    ///#            }
    ///         }"#)
    ///     .run();
    /// ```
    pub fn with_initial_properties(&mut self, properties: &[(&str, QVariant)]) -> &mut Self {
        let map = properties.iter()
            .map(|(k, v)| (QString::from(*k), v))
            .collect();
        self.engine.pin_mut().set_initial_properties(&map);
        self
    }

    /// Loads QML source from an in-memory byte slice.
    ///
    /// Applies any properties queued with [`add_initial_property`](QApp::add_initial_property)
    /// before loading.
    pub fn load_qml(&mut self, code: &[u8]) -> &mut Self {
        if !self.initial_properties.is_empty() {
            self.engine.pin_mut().set_initial_properties(&self.initial_properties);
        }
        self.engine.pin_mut().load_data(&code.into(), &Default::default());
        self
    }

    /// Loads the entry-point QML file by URL.
    ///
    /// Use this instead of [`load_qml`](QApp::load_qml) when the QML is
    /// embedded in a Qt resource (`qrc:`) or accessible as a file path.
    /// Accepts URLs such as `"qrc:/qt/qml/MyApp/Main.qml"` or
    /// `"file:///path/to/main.qml"`.
    ///
    /// Import paths for any modules the file uses must be registered with
    /// [`add_import_path`](QApp::add_import_path) before this call.
    pub fn load_qml_from_file(&mut self, url: &str) -> &mut Self {
        if !self.initial_properties.is_empty() {
            self.engine.pin_mut().set_initial_properties(&self.initial_properties);
        }
        self.engine.pin_mut().load(&url.into());
        self
    }

    /// Adds a directory to the QML engine's module import search path.
    ///
    /// Call before [`load_qml_from_file`](QApp::load_qml_from_file) when the
    /// loaded QML imports modules from a directory the engine would not
    /// otherwise find. Accepts both URLs and file-system paths.
    pub fn add_import_path(&mut self, path: &str) -> &mut Self {
        self.engine.pin_mut().add_import_path(&path.into());
        self
    }

    /// Registers `T` with the QML type system, making it instantiable from QML.
    ///
    /// ```rust
    ///# use qtbridge::{QApp, qobject};
    /// #[derive(Default)]
    /// pub struct Backend {
    /// }
    /// #[qobject]
    /// impl Backend {
    /// }
    ///
    /// QApp::new()
    ///     .register::<Backend>()
    ///     .load_qml(br#"
    ///         import QtQuick
    ///         import QtQuick.Controls
    ///#        import qtbridge_runtime
    ///         ApplicationWindow {
    ///             Backend {}
    ///#            Component.onCompleted: closeTimer.start()
    ///#            Timer {
    ///#                id: closeTimer
    ///#                interval: 1
    ///#                onTriggered: Qt.quit()
    ///#            }
    ///      }"#)
    ///     .run();
    /// ```
    pub fn register<T: QmlRegister>(&mut self) -> &mut Self {
        T::register();
        self
    }

    /// Sets the application name reported to the OS.
    pub fn application_name(&mut self, name: &str) -> &mut Self {
        QGuiApplication::set_application_name(name);
        self
    }
}
