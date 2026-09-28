// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only


use qtbridge_type_lib::QMetaObject;
use crate::DispatchMetaCall;
use crate::DynamicMetaObjectData;
use std::rc::Rc;
use std::cell::RefCell;
use std::pin::Pin;


/// Address of engine-allocated memory in which a C++ proxy is
/// placement-constructed. QML-created elements pass `Some`; Rust-created
/// objects pass `None` and allocate on the C++ heap.
pub type PlacementAddress = *mut u8;

/// `QCppProxy` defines what a C++ proxy to a QObject (C++) must implement.
///
/// This includes access to the C++ static meta-object, which is then extended
/// on the Rust side to create a dynamic meta-object.
pub trait QCppProxy {
    type ProxyRustType: QRustProxy;
    fn get_static_meta_object() -> &'static QMetaObject;
    fn get_size() -> usize;
    fn get_align() -> usize;
    fn parser_status_cast() -> i32;
    /// # Safety
    ///
    /// rust_proxy must be a valid and live pointer to Self::ProxyRustType.
    /// The QCppProxy must be droped first and will drop rust_proxy in its
    /// destructor. Ensure that rust_proxy is not droped by anything but QCppProxy.
    unsafe fn create(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData) -> *mut Self;
    /// # Safety
    ///
    /// Same contract as [`QCppProxy::create`].
    unsafe fn create_at(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData, addr: PlacementAddress) -> *mut Self;
    fn emit_signal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
}

/// `QRustProxy` defines the Rust-side bridge object that binds:
///
/// - A Rust object stored in `Rc<RefCell<dyn _>>`
/// - A corresponding C++ QObject-based proxy
///
/// Implementations of this trait are the concrete glue layer between
/// Rust and Qt, usually using Cxx.
///
/// # Purpose
///
/// A `QRustProxy` implementation:
///
/// - Stores a raw pointer to the C++ proxy (`cpp_proxy`)
/// - Stores access to the Rust object through `RustObjAccess<dyn _>` (`rust_obj`)
/// - Coordinates destruction, layout and Qt meta-object information
/// - Forwards all foreign function calls to the C++ proxy
///
/// Typical structure:
///
/// ```rust, ignore
/// pub struct QObjectProxyRust {
///     cpp_proxy: *mut QObjectProxyCpp,
///     rust_obj: RustObjAccess<dyn QObjectProxyGet>,
///     on_drop: fn(rust_obj: *const u8),
/// }
/// ```
///
/// Where:
///
/// - `cpp_proxy` points to the actual C++ QObject subclass.
/// - `rust_obj` wraps access to the users rust object.
/// - `on_drop` cleaning up memory.
///
/// # Lifetime and Ownership
///
/// A `QRustProxy` instance is heap-allocated and owned through a raw pointer, because its
/// lifetime is jointly managed with C++ code and the C++ side requires a stable, non-movable
/// address.
///
/// Destruction is always initiated from the C++ side: the paired `CppProxy` destructor calls
/// `GenericRustProxy::drop_self`, which converts the raw pointer back to a `Box`, invokes
/// the stored `on_drop` callback, and then drops this instance along with its reference to the
/// Rust object.
///
/// The proxy always holds a strong `Rc` to the user's Rust object: the
/// object lives exactly as long as its proxy pair, plus any user handles.
///
/// # Associated Types
///
/// ## `ProxyCppType`
///
/// The concrete C++ proxy type. Has to implement [`QCppProxy`].
///
/// ## `AdapterType`
///
/// A wrapper trait for the interface trait that QtBridge users implement. This wrapper
/// is required because not all traits can be used with dyn and are thus incompatible with
/// `RustObjAccess` (See "object safety" or "dyn compatibility").
pub trait QRustProxy {
    type ProxyCppType: QCppProxy<ProxyRustType = Self>;
    type AdapterType: DispatchMetaCall + ?Sized;

    /// Creates a new instance of this struct on the heap and returns a raw pointer to it.
    ///
    /// Initializes the proxy pair by:
    /// - Creating a `RustObjAccess` wrapper holding a strong reference to `rust_obj`.
    /// - Constructing the paired C++ proxy: with placement new at `at_address`
    ///   if given (QML-created elements), on the heap otherwise.
    /// - Storing `on_drop` for invocation when the C++ proxy is eventually destroyed.
    fn new(rust_obj: &Rc<RefCell<Self::AdapterType>>, metaobject: &'static DynamicMetaObjectData, at_address: Option<PlacementAddress>, on_drop: Box<dyn FnOnce() + 'static>) -> *mut Self;
    fn get_cpp_proxy(&self) -> *const Self::ProxyCppType;
    fn get_cpp_proxy_mut(&self) -> *mut Self::ProxyCppType;
    fn emit_signal(&self, mut_ref: &mut Self::AdapterType, signal_name: &str, argv: &[*const u8]);
    fn with_rust_ref<R, F: FnOnce(&Self::AdapterType) -> R>(&self, f: F) -> R;
    fn with_rust_ref_mut<R, F: FnOnce(&mut Self::AdapterType) -> R>(&self, f: F) -> R;

    /// Returns an owned handle to the Rust object behind this proxy, or `None`
    /// if it has already been dropped (the proxy may outlive a weakly-held
    /// object). The handle is typed as the adapter trait object; recovering the
    /// concrete type requires a checked reinterpret (see
    /// `QObjectHolder::qobject_to_rc_ref_cell`).
    fn get_rust_object_rc(&self) -> Rc<RefCell<Self::AdapterType>>;
}

/// Coerces a concrete user type into the proxy's type-erased adapter handle.
///
/// Implemented once per interface.
pub trait AdapterUpcast<T>: QRustProxy {
    fn upcast(rc: std::rc::Rc<std::cell::RefCell<T>>) -> std::rc::Rc<std::cell::RefCell<Self::AdapterType>>;
}
