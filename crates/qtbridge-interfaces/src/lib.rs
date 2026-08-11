// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! # Qt Interfaces (Internal Documentation)
//!
//! This crate provides proxies for various Qt (C++) interfaces using Cxx.
//! Proxies serve as the bridge between Rust and C++ implementations of Qt
//! interfaces, allowing Rust types to implement behavior expected by C++
//! code and to access C++ functionality from Rust.
//!
//! ## The proxy
//!
//! Each Qt interface has a corresponding **proxy** in Rust. This proxy has
//! to implement the [`qtbridge_runtime::qproxies::QRustProxy`] trait and interacts mostly with
//! the [`qtbridge_runtime::QObjectHolder`] trait, that is automatically implemented for all
//! user types by the `qobject` macro.
//!
//! The demands on proxies are baked into the [`qtbridge_runtime::qproxies::QRustProxy`] trait.
//! For example, there
//! is the demand to provide a Qt static meta object, usually from the "base" this proxy
//! is derived from. Further, the proxy has to provide a marker trait called `AdapterType`
//! that is at a minimum used to store a reference to the user object as
//! `Rc<RefCell<dyn AdapterType>>`. Otherwise the proxy is mostly free in its design.
//!
//! All proxies provided in this module follow a similar architecture. The proxy works
//! together with a set of traits. The QObject proxy is very different since it is the
//! proxy that is used if only QMetaObject functionality needs to be provided but no
//! interface implementation.
//!
//! ## Traits
//!
//! 1. **User-implemented functionality**: Rust implementations of Qt virtual functions.
//! 2. **Rust access to C++ functionality**: Rust wrappers around C++ methods of the interface.
//! 3. **Internal type erasure and proxy plumbing**: Allows dynamic behavior when types
//!    are not fully known at compile time.
//!
//! These proxies rely on the following Rust traits to define functionality and structure,
//! shown on the example of `QListModel`.
//!
//! ### 1. `QListModel`
//! * **Responsibility**: Must be implemented by the user.
//! * **Purpose**: Functions in this trait are called from C++ via the proxy, providing Rust
//!   implementations of Qt virtual functions. Some modifications can be done in the functions
//!   to shape the API and to erase Qt types (e.g. QVariant and QModelIndex).
//! * **Associated Types**: Defines types (e.g., `Item`) that are part of the interface contract.
//! * **Notes**: Users implement function as expected from the Qt API. Default implementation
//!   can exist for non-pure virtual functions.
//!
//! ### 2. `QListModelBase`
//! * **Responsibility**: Default / blank implementation for all `QListModel`.
//! * **Purpose**: Provides access to C++ functions from Rust and other convenience functions
//!   that should not be overridden. Serves as a bridge between user logic and C++ functionality.
//!   Functions that should not be overridden by the user are implemented here.
//! * **Notes**: This trait forms the core Rust interface to call C++ functions for types implementing
//!   `QListModel`.
//!
//! ### 3. `QListModelAdapter` (Internal Only)
//! * **Responsibility**: Default / blank implementation for all `QListModel`.
//! * **Purpose**: Provides a type-erased interface for internal machinery. Traits with associated
//!   types (like `QListModel::Item`) cannot be used as `dyn` traits in Rust. `QListModelAdapter`
//!   erases unknown types while exposing the necessary interface internally.
//! * **Notes**: Never meant for user implementation or usage. Facilitates dynamic behavior and
//!   internal bridging without exposing associated types to user code.
//!
//! ## Memory Ownership and Lifetimes
//!
//! Every instance of a user-defined Rust struct annotated with `#[qobject]`
//! has two auxiliary parts invisible to the user:
//!
//! - **`CppProxy`** - a C++ class derived from`QObject` or `QAbstractItemModel` (or another base
//!   interface). This is the object the QML engine sees and interacts with. It is allocated with
//!   a regular `new` for Rust-created objects, or with placement `new` at a QML-engine-supplied
//!   address for QML-created elements.
//! - **[`RustProxy`](crate::genericrustproxy::GenericRustProxy)** - the Rust-side bridge,
//!   heap-allocated via [`Box::into_raw`] in
//!   [`QRustProxy::new`](qtbridge_runtime::qproxies::QRustProxy::new). It holds a pointer to
//!   `CppProxy` and a reference the user struct as `Rc<RefCell<UserStruct>>` or
//!   `Weak<RefCell<UserStruct>>` in [`RustObjAccess`].
//!
//! The `CppProxy` C++ destructor is the common teardown point for the proxy pair: it always
//! calls `GenericRustProxy::drop_self`, which fires the `on_drop` callback and then drops
//! `RustProxy` along with its `Rc`/`Weak` reference to the user struct.
//!
//! ### Who destroys `CppProxy`?
//!
//! That depends on who owns the object, determined at construction by `ConstructionMode`.
//!
//! #### QML-created objects
//!
//! `RustProxy` holds a strong `Rc<RefCell<UserStruct>>` (`SharedReferenceWithQml::OwnedByQml`);
//! the engine (or a parent) owns the `QObject`.
//!
//! ```text
//! JS GC deletes QObject
//!   → virtual ~CppProxy()
//!   → GenericRustProxy::drop_self
//!     → on_drop() fires
//!     → Rc<RefCell<UserStruct>> strong count −1
//!       → UserStruct::drop()                     (only if this was the last strong Rc)
//! ```
//!
//! #### Rust-created objects
//!
//! `RustProxy` holds only a `Weak<RefCell<UserStruct>>`
//! (`SharedReferenceWithQml::OwnedByRust`); the owner of record is the per-thread
//! registry (`qtbridge_runtime::registry`), which holds one strong `Rc` for each
//! object. `QObject`s are explicitly set `CppOwnership` and only set to `JavaScriptOwnership`
//! when there is no interest from Rust in the object. The QML engine's garbage collector then
//! reclaims their JS wrappers which in turn is the sign for the registry to delete the `Rc`
//! that keeps the user struct alive.
//!
//! ```text
//! registry::collect_garbage() runs & object has no user Rc
//!
//! -- never wrapped by any engine: deleted immediately
//!   → QObject::delete
//!     → virtual ~CppProxy()
//!       → GenericRustProxy::drop_self
//!         → on_drop() (removes the registry entry)
//!   → registry drops its Rc
//!     → UserStruct::drop()
//!
//! -- live JS wrapper: ownership handed to the engine --
//!   → setJavaScriptOwnership (entry and Rc stay in the registry)
//!   if a later engine gc() finds the wrapper unreachable
//!     → engine deletes the QObject
//!       → same teardown chain as above; removing the registry entry
//!         drops its Rc
//!           → UserStruct::drop()
//! ```

pub mod genericrustproxy;
pub use qtbridge_runtime::live_proxy_count;

pub mod qobject;

pub mod qabstract_item_model;
pub use qabstract_item_model::{QAbstractItemModel, QAbstractItemModelBase};

pub mod qlist_model;
pub use qlist_model::{QListModel, QListModelBase};

pub mod qparser_status;
pub use qparser_status::QParserStatus;

pub mod qtable_model;
pub use qtable_model::{QTableModel, QTableModelBase};

pub mod object_access;
pub use object_access::rust_object_access::RustObjAccess;
