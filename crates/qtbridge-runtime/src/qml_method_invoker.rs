// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use qtbridge_type_lib::{QObject, QVariantList};

use crate::QObjectHolder;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qvariant.h");
        type QList_QVariant = qtbridge_type_lib::QList_QVariant;

        include!("cpp/qml_method_invoker.h");

        unsafe fn connect_destroyed_callback(obj: *mut QObject, flag_ptr: usize);

        unsafe fn invoke_method(obj: *mut QObject, name: &str, args: &QList_QVariant) -> bool;
    }

    extern "Rust" {
        fn on_qobject_destroyed(flag_ptr: usize);
    }
}

fn on_qobject_destroyed(flag_ptr: usize) {
    let arc = unsafe { Arc::from_raw(flag_ptr as *const AtomicBool) };
    arc.store(false, Ordering::Release);
}

/// Invokes a slot or signal by name on a [`QmlMethodInvoker`].
///
/// Without extra arguments, delegates to [`QmlMethodInvoker::invoke_method`].
/// With extra arguments, constructs the argument list and delegates to
/// [`QmlMethodInvoker::invoke_method_with_args`].
///
/// ```
/// use qtbridge::invoke_method;
/// use qtbridge::{qobject, QObjectHolder};
///
/// #[derive(Default)]
/// pub struct MyClass { }
///
/// #[qobject]
/// impl MyClass {
///     #[qslot]
///     fn set_value(&mut self, value: i32) { }
///
///     #[qslot]
///     fn reset(&self) { }
/// }
///
/// let obj = MyClass::default_with_attached_qobject();
/// let invoker = obj.borrow().get_qml_method_invoker();
/// invoke_method!(invoker, "reset");
/// invoke_method!(invoker, "setValue", 42);
/// ```
///
#[macro_export]
macro_rules! invoke_method {
    ($invoker:expr, $name:expr $(,)?) => {{
        $invoker.invoke_method($name)
    }};

    ($invoker:expr, $name:expr, $($arg:expr),+ $(,)?) => {{
        let args: qtbridge::qtbridge_type_lib::QVariantList = [
            $(qtbridge::qtbridge_type_lib::QVariant::from(&$arg)),+
        ]
        .iter()
        .collect();
        $invoker.invoke_method_with_args($name, &args)
    }};
}

/// A thread-safe handle for invoking slots and signals on a [`QObjectHolder`].
///
/// Calls are scheduled on the Qt event loop and execute on the Qt thread.
/// If the target object has been dropped, calls are silently discarded.
///
/// Arguments follow the same coercion rules QML applies when it calls a
/// method: an argument matches a parameter as long as it can be converted to
/// the parameter's type. These rules apply uniformly, even when a Rust method
/// is invoked from Rust; this path does *not* enforce Rust's exact-type
/// semantics, so conversions QML permits (for example between numeric types,
/// or between a number and its textual form) are permitted here too.
///
/// When a call executes, Qt borrows the `Rc<RefCell<_>>` held by the QML
/// engine. If the object is already mutably borrowed on the Qt thread at
/// that moment, the call will panic.
///
/// Obtain an instance via [`QObjectHolder::get_qml_method_invoker`].
///
/// # Example
///
/// ```
/// # use qtbridge::{qobject, QObjectHolder};
/// # #[qobject]
/// # pub mod example {
/// #     #[derive(Default)]
/// #     pub struct Backend {}
/// #     impl Backend {
/// #         #[qsignal]
/// #         pub fn data_ready(&mut self);
/// #     }
/// # }
/// # use example::Backend;
/// let backend = Backend::default_with_attached_qobject();
/// let invoker = backend.borrow().get_qml_method_invoker();
/// std::thread::spawn(move || {
///     invoker.invoke_method("dataReady");
/// }).join().unwrap();
/// ```
pub struct QmlMethodInvoker {
    obj: *mut QObject,
    alive: Arc<AtomicBool>,
}

unsafe impl Send for QmlMethodInvoker {}

impl QmlMethodInvoker {

    /// Creates a `QmlMethodInvoker` for `target` and tracks its lifetime.
    ///
    /// Prefer [`QObjectHolder::get_qml_method_invoker`] over calling this directly.
    pub fn new<T: QObjectHolder>(target: &T) -> Self {
        let obj = target.get_qobject_ptr();
        let alive = Arc::new(AtomicBool::new(true));
        let flag_ptr = Arc::into_raw(alive.clone()) as usize;
        unsafe { ffi::connect_destroyed_callback(obj, flag_ptr) };
        Self { obj, alive }
    }

    fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Acquire)
    }

    /// Schedules `name` to run on the Qt thread via the Qt event loop.
    ///
    /// `name` is resolved over the object's meta-object most-derived-first
    /// (QML-added members, then Rust, then the C++ base). Returns `false` if
    /// the target has been dropped or no method named `name` takes zero
    /// arguments; returns `true` otherwise.
    pub fn invoke_method(&self, name: &str) -> bool {
        if !self.is_alive() {
            return false;
        }
        unsafe { ffi::invoke_method(self.obj, name, &QVariantList::default()) }
    }

    /// Schedules `name` to run on the Qt thread via the Qt event loop,
    /// passing `args` to the method.
    ///
    /// `name` is resolved over the object's meta-object most-derived-first
    /// (QML-added members, then Rust, then the C++ base), choosing the first
    /// candidate whose parameter count matches `args` and whose arguments are
    /// all convertible to the parameter types. Returns `false` if the target
    /// has been dropped, no candidate matches, or a conversion fails; returns
    /// `true` otherwise.
    pub fn invoke_method_with_args(&self, name: &str, args: &QVariantList) -> bool {
        if !self.is_alive() {
            return false;
        }
        unsafe { ffi::invoke_method(self.obj, name, args) }
    }
}
