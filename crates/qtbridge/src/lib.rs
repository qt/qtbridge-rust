// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use qtbridge_runtime;
pub use qtbridge_runtime::QModelItem;
pub use qtbridge_runtime::invoke_method;
pub use qtbridge_runtime::QMetaTypeCompatible;
pub use qtbridge_runtime::QPropertyMember;
pub use qtbridge_runtime::registry::collect_garbage;
#[doc(hidden)]
pub use qtbridge_gen;
#[doc(hidden)]
pub use qtbridge_interfaces;
#[doc(hidden)]
pub use qtbridge_type_lib;
#[doc(hidden)]
pub use qtbridge_build_utils;

pub mod special_traits {
      //! Traits that enable Rust types to fulfill specific QML roles.
      //!
      //! Implement one of these on your struct and pass it as `Base = ...`
      //! to [`qobject`](crate::qobject).
      //!
      //! Note that only one of these traits can be implemented for the same
      //! type.
      //!
      //! - [`QListModel`](crate::QListModel) exposes a list to QML ListView / Repeater
      //! - [`QTableModel`](crate::QTableModel) exposes a table to QML TableView
      //! - [`QParserStatus`](crate::QParserStatus) receives notifications during component construction
}

/// Annotate an `impl` or `mod` block to make its struct accessible from QML.
///
/// The macro implements a range of traits that enable bridging from Rust
/// to QML. The mechanism is based on the implementation of various traits with
/// some code generated at macro expansion time. You should not implement these
/// traits yourself. As a user, you should only interact with:
///
/// * [`QObjectHolder`]
/// * [`QmlRegister`] (only non-generic types)
///
/// This macro makes it possible to declare the following items within the
/// `impl` block:
///
/// * signals with the [`qsignal`] attribute macro
/// * invokable functions with the [`qslot`] attribute macro
/// * struct properties with the [`qproperty`] macro
///
/// Further, it allows the struct to implement traits to fulfill specific QML purposes.
/// These are called `Base` traits. The available base traits are:
///
/// * [`QParserStatus`] to receive notifications during QML component construction.
/// * [`QListModel`] to make a `struct` accessible by QML ListView, QML Repeater, or similar.
/// * [`QTableModel`] to make a `struct` accessible by QML TableView.
///
/// Only one of those traits can be implemented at the same time.
///
/// # Usage
///
/// The [`qobject`] macro can be applied to a `impl` block of the
/// target `struct`. Only a single `impl` block can be annotated with this macro and
/// all applications of [`qsignal`], [`qslot`] and [`qproperty`] have to be limited to
/// this block.
///
/// Alternatively, it may be applied to the `mod` block that contains the `struct`
/// definition and its associated `impl` blocks.
///
/// In order to communicate with QML, the macro creates bridging objects that are attached
/// to the respective structs. Therefore, objects created with [`qobject`] should be
/// created with [`default_with_attached_qobject`](QObjectHolder::default_with_attached_qobject)
/// or expanded with [`attach_qobject`](QObjectHolder::attach_qobject). This is not necessary if
/// the struct is instantiated in QML.
///
/// When [`register`](QmlRegister::register) is called, the macro creates a QML
/// module whose name matches your Cargo package name. So for a `Cargo.toml` with
/// ```toml
/// [package]
/// name = "hello_world"
///
/// [dependencies]
/// qtbridge
/// ```
/// the QML file has to contain
///
/// ```qml
/// import hello_world
/// ```
///
/// ## Requirements
///
/// A `struct` using [`qobject`] must implement the [`Default`] trait.
/// The static function [`register`](QmlRegister::register) has to be called at the start of the
/// main function to make this `struct` instantiable from QML.
///
/// ## Parameters
///
/// Parameters to adjust the macro behavior are passed as comma-separated keywords or keyword-value pairs.
///
/// **Base = BaseTrait**
///
/// Set the base trait. Must be one of the [`special_traits`] and must be
/// implemented for the corresponding `struct`. Only one base trait can be set
/// per type.
///
/// **ConvertToCamelCase**
///
/// Rust uses snake_case for function names, while in QML camelCase is more common. Use this option
/// to convert function names to camelCase when exposed to QML.
///
/// **NoQmlElement**
///
/// Do not implement [`QmlRegister`]. [`QmlRegister`] registers the `struct` in the QML type system,
/// allowing you to instantiate this type in QML. The `NoQmlElement` option can be useful to turn off
/// instantiatability within QML or to provide a manual implementation of this trait with better control
/// over naming and versioning.
///
/// **Singleton**
///
/// Implement [`QmlRegister`] as a [singleton](https://doc.qt.io/qt-6/qml-singleton.html). A singleton
/// is accessed from QML as a single shared instance of the type, using the type name as identifier.
/// This is useful for application-wide data, global settings, or service objects.
///
/// ## Automatic registration
///
/// When the `linkme` cargo feature is enabled, [`register`](QmlRegister::register) is called at
/// application start for every annotated type without any additional code. The crate
/// [`Linkme`](https://crates.io/crates/linkme) is used for this purpose:
///
/// ```toml
/// qtbridge = { version = "0.2", features = ["linkme"] }
/// ```
///
/// The feature applies to every type annotated with [`qobject`] in the whole application,
/// including its dependencies. Types annotated with `NoQmlElement` are exempt, as they do
/// not implement [`QmlRegister`].
///
/// ## Example
///
/// ```rust
/// use qtbridge::{QApp, qobject};
///
/// #[derive(Default)]
/// pub struct Counter {
///    value: i32,
/// }
///
/// #[qobject(Singleton)]
/// impl Counter {
///     qproperty!("value", Member = value, Notify = value_changed);
///
///     #[qsignal]
///     fn value_changed(&mut self);
///
///     #[qslot]
///     fn change_value(&mut self, inc: bool) {
///         self.value = match inc {
///             true => self.value.saturating_add(1),
///             false => self.value.saturating_sub(1),
///         };
///         self.value_changed();
///     }
/// }
///
/// const QML_CODE: &str =
/// r#"
///     import QtQuick
///     import QtQuick.Controls
///     import QtQuick.Layouts
///     import qtbridge // must match your cargo package name
///
///     ApplicationWindow {
///         visible: true
///         title: qsTr("Counter QML app")
/// #       Component.onCompleted: closeTimer.start()
/// #       Timer {
/// #           id: closeTimer
/// #           interval: 1
/// #           onTriggered: Qt.quit()
/// #       }
///         RowLayout {
///             anchors.centerIn: parent
///             Button {
///                 text: "-"
///                 onClicked: Counter.changeValue(false)
///             }
///             Button {
///                 text: "+"
///                 onClicked: Counter.changeValue(true)
///             }
///         }
///     }
/// "#;
///
/// fn main() {
///     QApp::new()
///         .register::<Counter>()
///         .load_qml(QML_CODE.as_bytes())
///         .run();
/// }
/// ```
///
#[doc(inline)]
pub use qtbridge_gen::qobject;


/// Annotates a function as a signal that can be handled in QML.
///
/// Signals can be called from Rust and the signal handler can be defined in QML. This is the
/// recommended way to invoke QML code from Rust.
///
/// ### Requirements
///
/// - The signal must be defined within a `mod` or `impl` block, annotated with [`qobject`].
/// - The first argument of the annotated function must be `&mut self`.
/// - All other parameter types and the return type must implement [`QMetaTypeCompatible`].
/// - The function must not have a body (end with a semicolon or empty curly braces `{}`).
///
/// ```rust
/// # use qtbridge::qobject;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject]
/// impl Backend {
///     #[qsignal]
///     fn value_changed(&mut self, new_value: i32);
///     #[qsignal]
///     fn event_triggered(&mut self){}
/// }
/// ```
///
/// To receive a notification on the QML side, the object definition has to declare a signal handler named
/// `on<Signal>`, where `<Signal>` is the name of the signal, with the first letter capitalized. Note that
/// the rest of the function name is not affected and the signal handler for e.g. `value_changed` will be
/// `onValue_changed`.
///
/// ```qml,ignore
/// Backend {
///     onValue_changed: console.log("Value changed");
/// }
/// ```
/// Alternatively you can instantiate a `Connection` object with the respective signal handler.
/// ```qml,ignore
/// Connection {
///     target: backend
///     function onValue_changed() {
///         console.log("Value changed");
///     }
/// }
/// ```
///
/// For more details see <https://doc.qt.io/qt-6/qtqml-syntax-signals.html>
///
/// ### Parameters
///
/// ***qml_name***
///
/// The signal name as seen in QML. Defaults to the Rust function name.
///
#[doc(inline)]
pub use qtbridge_gen::qsignal;

/// Annotates a function as invokable from QML.
///
/// Such a function is also registered as a Qt slot, so it can be the target of a
/// [signal-slot connection](https://doc.qt.io/qt-6/signalsandslots.html), or be
/// invoked by name from Rust through a [`QmlMethodInvoker`].
///
/// ### Requirements
///
/// - Has to be defined within a `mod` or `impl` block, annotated with [`qobject`].
/// - The annotated function must have a body.
/// - The first argument of the annotated function must be `&self` or `&mut self`.
/// - All other parameter types and the return type must implement [`QMetaTypeCompatible`].
///
/// ### Example
/// ```rust
/// # use qtbridge::qobject;
/// # #[derive(Default)]
/// # pub struct Backend {
/// #     value: i32,
/// # }
/// #
/// # #[qobject]
/// # impl Backend {
/// #[qslot]
/// fn set_value(&mut self, new_value: i32) {
///     self.value = new_value;
/// }
/// # }
/// ```
///
/// ### Parameters
///
/// **qml_name**
///
/// The function name as seen from QML. Defaults to the Rust function name.
#[doc(inline)]
pub use qtbridge_gen::qslot;

// TODO: Remove name mangling from doc snippets.
/// Registers a property to be accessible from QML.
///
/// ### Requirements
///
/// - The property must be defined within a `mod` or `impl` block, annotated with [`qobject`].
/// - The first parameter is the property name. It must begin with a lower case letter and
/// can only contain letters, numbers and underscores.
/// - The property type must implement [`QPropertyMember`].
/// - The return value of the getter (specified via `Read` parameter) must match the property type.
/// - The value parameter of the setter (specified via `Write` parameter) must match the property type.
/// - The member of the `struct` (specified via `Member` parameter) must match the property type.
/// - A signal indicating any property changes (specified via `Notify` parameter) must be
/// emitted explicitly by any code that changes the property. The framework does not emit
/// it automatically.
/// - Getter and setter methods must be defined within the same `impl` block in which the property
/// is declared.
///
/// A property may be **accessor-based** or **member-based** or a mix of both (see the
/// [syntax](#qproperty-syntax) section for details).
///
/// ### Accessor based property
///
/// A pure accessor-based property can be declared together with a range of functions:
/// ```rust
/// # use qtbridge::qobject;
/// # #[derive(Default)]
/// # pub struct Backend {
/// #     value: i32,
/// # }
/// #
/// # #[qobject]
/// # impl Backend {
/// qproperty!("myProperty", Read = get_value, Write = set_value, Notify = my_property_changed);
///
/// pub fn get_value(&self) -> i32 { self.value }
/// pub fn set_value(&mut self, value: i32) {
///     self.value = value;
///     self.my_property_changed();
/// }
/// #[qsignal]
/// pub fn my_property_changed(&mut self);
/// # }
/// ```
/// The getter method that returns the current value of the property, the setter (if provided) must
/// take the input value of the property as its first argument (after `&mut self`).
///
/// ### Member based property
///
/// Member based properties do not require a setter or getter and QML will directly read and write
/// to the member. A `Notify` signal must be provided. Qt emits it automatically when QML writes
/// the property, but when Rust code changes the member directly the notify signal must be emitted
/// explicitly.
///
/// A `struct` containing a member-based property may look like:
/// ```rust
/// # use qtbridge::qobject;
/// #[derive(Default)]
/// struct Text {
///     msg: String
/// }
///
/// #[qobject]
/// impl Text {
///     qproperty!("message", Member = msg, Notify = message_changed);
///
///     #[qsignal]
///     fn message_changed(&mut self);
/// }
/// ```
///
/// More information about Qt properties: <https://doc.qt.io/qt-6/properties.html>.
///
/// ### Parameters of `qproperty!`
///
/// **Name**
///
/// The first argument is a string literal specifying the name of the Qt property.
/// This is the name under which the property is exposed to QML and should follow the
/// naming rules from [requirements](#requirements-2).
///
/// **Read**
///
/// Specifies the getter method for the property in the format `Read = getter_name`.
///
/// **Write**
///
/// Specifies the setter method for the property in the format `Write = setter_name`.
///
/// **Member**
///
/// Specifies the struct member variable that will be accessed if no getter or setter
/// are provided. Expected format: `Member = var_name`.
///
/// **Notify**
///
/// Specifies the name of the signal that has to be emitted when the property changes.
/// Expected format: `Notify = signal_name`.
///
/// **Constant**
///
/// A constant property is not allowed to have `Write` or `Notify` parameters. If no
/// `Notify` is provided in combination with member, `Constant` is required.
/// Expected as a single keyword without an assignment expression.
///
/// **Default**
///
/// Marks this as the QML default property. Content placed inside an object literal
/// without an explicit property assignment is written to it.
/// Expected as a single keyword without an assignment expression.
///
#[doc(inline)]
pub use qtbridge_gen::qproperty;

pub use qtbridge_runtime::{QApp, qresource, QmlMethodInvoker};

/// Provides access to the underlying QObject for types exposed to QML.
///
/// Automatically implemented by [`qobject`]. Do not implement this trait manually.
///
#[doc(inline)]
pub use qtbridge_runtime::QObjectHolder;

/// QmlRegister enables QML to instantiate types of this trait.
///
/// The trait is usually implemented by [`qobject`]. If you
/// want to implement this trait manually, you have to add the `NoQmlElement`
/// option.
///
/// [`QmlRegister`] defines the [`ELEMENT_NAME`](QmlRegister::ELEMENT_NAME)
/// with which the `struct` can be instantiated in QML and the module name,
/// [`URI`](QmlRegister::URI), which has to be used as import in QML to
/// use this `struct`.
///
/// [`QmlRegister`] knows two ways of registering a type. The ordinary way
/// is to register as an element that can be instantiated in QML:
///
/// ```rust
/// # use qtbridge::qobject;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject(NoQmlElement)]
/// impl Backend {
///     #[qslot]
///     fn say_hello(&self) {
///         println!("Hello World!")
///     }
/// }
/// impl qtbridge::qtbridge_runtime::QmlRegister for Backend {
///     const URI: &str = "rust_backend";
///     const ELEMENT_NAME: &str = "Backend";
///     const MINOR_VERSION: u8 = 0u8;
///     const MAJOR_VERSION: u8 = 1u8;
///     const IS_SINGLETON: bool = false;
/// }
/// ```
///
/// ```qml
/// import rust_backend
/// Backend {
///     id: backend
/// }
/// Button {
///     anchors.centerIn: parent
///     text: "Hello World!"
///     onClicked: backend.sayHello()
/// }
/// ```
///
/// Alternatively, by setting [`IS_SINGLETON`](QmlRegister::IS_SINGLETON)
/// to true, the type is registered as a singleton. That means that only
/// one instance can be created. It can be accessed with the
/// [`ELEMENT_NAME`](QmlRegister::ELEMENT_NAME):
///
/// ```rust
/// # use qtbridge::qobject;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject(NoQmlElement)]
/// impl Backend {
///     #[qslot]
///     fn say_hello(&self) {
///         println!("Hello World!")
///     }
/// }
/// impl qtbridge::qtbridge_runtime::QmlRegister for Backend {
///     const URI: &str = "rust_backend";
///     const ELEMENT_NAME: &str = "Backend";
///     const MINOR_VERSION: u8 = 0u8;
///     const MAJOR_VERSION: u8 = 1u8;
///     const IS_SINGLETON: bool = true;
/// }
/// ```
///
/// ```qml
/// import rust_backend
/// Button {
///     anchors.centerIn: parent
///     text: "Hello World!"
///     onClicked: Backend.sayHello()
/// }
/// ```
///
/// Further, [`MAJOR_VERSION`](QmlRegister::MAJOR_VERSION) and
/// [`MINOR_VERSION`](QmlRegister::MINOR_VERSION) define the version of the
/// QML module. These fields are mandatory but QML can load a module without
/// specifying the version
///
#[doc(inline)]
pub use qtbridge_runtime::QmlRegister;

pub use qtbridge_gen::QModelItem;

pub use qtbridge_gen::include_bytes_qml;

pub use qtbridge_interfaces::QParserStatus;
pub use qtbridge_interfaces::{QListModel, QListModelBase};
pub use qtbridge_interfaces::{QTableModel, QTableModelBase};

#[doc(hidden)]
pub use qtbridge_interfaces::{QAbstractItemModel, QAbstractItemModelBase};
