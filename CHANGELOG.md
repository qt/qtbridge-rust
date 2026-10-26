# Changelog

All notable changes to QtBridge for Rust are documented in this file.

## 0.4.0 - 2026-10-26

- `QThreadHandle<T>` runs closures on the Qt thread from any thread, with
  shared (`invoke`) or mutable (`invoke_mut`) access to a `#[qobject]`
  instance. The target's liveness is checked on delivery; closures for a
  deleted target are discarded. `QThreadHandle::default()` runs closures
  without a target.
- **Breaking:** `QmlMethodInvoker` was removed. It was unsound and not
  type-safe; use `QThreadHandle` instead. Calling methods declared in QML from
  Rust is no longer possible.
- `QApp::qml_engine()` exposes the `QQmlEngine`, e.g. for setting up
  translation contexts.
- Mutable base trait functions (e.g. of `QListModelBase`) no longer create
  aliasing `&mut` references to the proxy, completing the fix for mutable
  slots and property writes in 0.3.0. Thanks to Shenghao Chen for the report
  and analysis.
- Several internal traits and functions are now `unsafe` to reflect their
  contracts (`QObjectHolder`, `QCppProxy`, `QRustProxy`, `AdapterUpcast`,
  `QMetaTypeCompatible::from_compatible`, `QmlElement::list_to_qvariant`).
  They are implemented and called by generated code, so code using
  `#[qobject]` is not affected.
- `qtbridge-gen-common` was merged into `qtbridge-gen`.
- On Ubuntu/Debian, building requires `qt6-declarative-private-dev` (for
  `private/qqmldata_p.h`). This was already the case for 0.3.0 but not
  documented.

## 0.3.0 - 2026-09-21

- Registry-based ownership of QObjects created in Rust. Objects stay alive
  while in flight to QML, and ownership is handed to the QML engine once Rust
  holds no more references. Collection runs after each QML garbage collection
  and under allocation pressure; `qtbridge::collect_garbage()` triggers it
  deterministically. This fixes a use-after-free when returning a newly
  created `Rc<RefCell<T>>` from a slot.
- QObjects are attached lazily: a `#[qobject]` type is a plain Rust type until
  it is first exposed to QML. A value whose QObject was deleted by the engine
  gets a fresh one on its next exposure instead of aborting.
- **Breaking:** The Qt type library is now based on `cxx-qt` / `cxx-qt-lib`
  (`QString`, `QByteArray`, `QModelIndex`, `QList`, `QHash`, `QMap`, `QVariant`,
  the JSON types, `QQmlApplicationEngine`, `QGuiApplication`). APIs follow
  `cxx-qt-lib`, e.g. `QList::remove()` changed signature and
  `QList::capacity()` was removed.
- New `QmlObject` trait with the user-facing instance API, split out of
  `QObjectHolder`.
- **Breaking:** `QApp::with_initial_properties` was removed. Use
  `set_initial_property` and the new `set_initial_object`, which takes a
  `Rc<RefCell<T>>` and keeps it alive until the engine takes over. Both can
  be combined.
- **Breaking:** `as_qvariant` was removed. It returned an untracked
  `QObject*`; use `QApp::set_initial_object` instead.
- **Breaking:** `QmlRegister` was renamed to `QmlElement`, and `QMetaCallArg`
  to `QMetaTypeCompatible`.
- **Breaking:** The `#[qobject(LinkMe)]` option was replaced by the `linkme`
  crate feature. `linkme` is re-exported, so it is no longer a direct
  dependency of user crates.
- The `NoDrop` option of `#[qobject]` was removed; the macro no longer
  generates or modifies `Drop` implementations.
- `QVariantConvertible` marks the types that cross to and from QML by value. It
  replaces the `ToQVariant` and `TryFromQVariant` traits.
- `Default` is only required for types registered as QML elements, no longer
  for every `#[qobject]` type.
- QObject properties and arguments (`Rc<RefCell<T>>`) are declared with their
  concrete `T*` metatype, so QML rejects an object of the wrong type with a
  type error instead of a panic in Rust.
- QML-extended instances (types with properties, functions or signals added
  in QML) can be passed back to Rust.
- Emitting a signal, or mutating a list or table model, on an instance without
  an attached QObject is a no-op instead of a panic.
- Slots returning a `#[qobject]` type work (fixes "Unknown method return type"
  error).
- QmlMethodInvoker resolves the most-derived method with matching arguments,
  instead of an inherited same-named overload that dropped the arguments.
- Mutable slots and property writes no longer create aliasing `&mut`
  references to the proxy.
- `include_bytes_qml!` accepts directories (processed recursively), resolves
  paths correctly with a nested `Cargo.lock`, and reports errors as compile
  errors instead of panics.
- `usize` and `isize` are converted to their fixed-width counterparts in
  metacalls.
- `DispatchMetaCall`, `QMetaInfo` and `QPropertyMember` are `unsafe` traits;
  `DispatchMetaCall` and `QMetaInfo` are hidden from the documentation.
- `qtbridge-gen` and `qtbridge-build-utils` are no longer re-exported from
  `qtbridge`.
- Fixed linker errors for `cxx_qt_init_crate_*` symbols in user applications,
  and a wrong function call in `QTableModel`.
- Examples use the Fusion style and are licensed BSD-3-Clause.
- Minimum supported Rust version is 1.88 (previously stated as 1.87).

## 0.2.0 - 2026-07-01

- **Breaking:** The new `#[qobject]` macro replaces `#[qobject_impl]` and
  `#[qml_element]`. It works on `mod` and `impl` blocks and makes a type a QML
  element by default, with options:
  - `Base = ...` to implement a QML-specific base trait,
  - `Singleton` for a QML singleton,
  - `NoQmlElement` to skip QML element registration,
  - `ConvertToCamelCase` to convert slot and signal names to camelCase.
- **Breaking:** Slot and signal names are no longer converted to camelCase
  automatically; they are kept as written unless `ConvertToCamelCase` is set.
- **Breaking:** `QListModel` and `QTableModel`, Qt-free interfaces for
  exposing list and table data to QML views, replace `QVec`. Roles are declared
  with `QModelItem` (and a derive macro).
- `QParserStatus` interface for component construction notifications.
- **Breaking:** `#[overridden]` was replaced by traits, so the compiler checks
  user implementations.
- Nested `#[qobject]` types in properties, slots and signals:
  `Rc<RefCell<T>>` and `Vec<Rc<RefCell<T>>>` (exposed as a
  `QQmlListProperty`).
- `serde_json::Value` as a property and argument type (`serde_json` feature).
- `QmlMethodInvoker` to invoke signals and slots from other threads.
- New `QApp` methods: `register`, `add_initial_property`, `load_qml_from_file`
  and `add_import_path`.
- `Default` attribute for `qproperty!` to mark the default property (at most
  one per object).
- **Breaking:** The `Notify` parameter of `qproperty!` takes an identifier
  instead of a string literal.
- **Breaking:** `const` signals and the empty `QObject` trait were removed.
- `Vec<isize>` / `Vec<usize>` and accessor-based `str` properties.
- Metacalls are dispatched to Rust through the `DispatchMetaCall` trait, and
  types for properties and metacalls are deduced at compile time.
- `QSignalSpy` binding, including inherited signals, for tests.
  `#[run_quick_test]` can opt out of the test harness.
- macOS support, including Qt installed as frameworks.
- Qt is located through `qt-build-utils`, sharing the Qt installation with
  `cxx-qt`.
- **Breaking:** Crates were renamed and restructured: `qtbridge-runtime`,
  `qtbridge-gen`, `qtbridge-type-lib`, `qtbridge-interfaces`,
  `qtbridge-build-common`, `qtbridge-gen-common`.
- Fixed `QML_SINGLETON` registration, a use-after-free in `QMetaTypeInterface`,
  `isize` / `usize` in metacalls, notification signal setup for properties and
  `Rc<RefCell<T>>` member properties.
- New `host_monitor` and `color_palette` example applications.

## 0.1.0 - 2026-01-15

Initial alpha release of QtBridge for Rust: Qt Quick user interfaces with a
Rust backend, without writing C++. Built on [CXX](https://cxx.rs/).

- `QApp` runs a QML application and injects Rust objects into it as initial
  properties.
- `#[qobject_impl]` exposes a Rust struct to QML with signals (`#[qsignal]`),
  slots (`#[qslot]`) and properties (`qproperty!`).
- `#[qml_element]` makes a struct instantiable from QML.
- `QVec`, a `Vec`-like container that QML views such as `ListView` and
  `Repeater` can use as a model.
- Qt resources via `include_bytes_qml!` and `qresource::register_bytes`.
- Rust objects are held in `Rc<RefCell<_>>` and borrowed only while a slot or
  property is invoked from QML.
- Examples `hello_world` and `minimal_app`.
- Requires Rust 1.87 or newer and a Qt 6 installation with `qmake` in `PATH`.
