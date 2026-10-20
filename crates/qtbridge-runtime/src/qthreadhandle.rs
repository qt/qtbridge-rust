// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use qtbridge_type_lib::QObject;

use crate::QObjectHolder;
use crate::qproxies::QRustProxy;
use crate::rustobjectgetter::get_rust_proxy;

#[cxx::bridge]
pub mod ffi {
    #[allow(clippy::missing_safety_doc)]
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;

        include!("cpp/qthreadhandle.h");
        type QThreadHandleCpp;

        /// Creates the C++ guard holding a `QPointer<QObject>` with `target`.
        ///
        /// The returned guard may be moved to, used from and destroyed on
        /// any thread.
        ///
        /// A null `target`, or one not living on the application thread,
        /// yields a guard whose calls are all discarded.
        ///
        /// # Safety
        ///
        /// Must be called on `target`'s thread, with `target` either alive
        /// or null.
        unsafe fn make_qt_thread_handle(target: *mut QObject) -> UniquePtr<QThreadHandleCpp>;

        /// Posts `closure` to the event loop running on the Qt thread.
        /// Thread-safe; never touches the target object on the calling thread.
        fn post_closure(handle: &QThreadHandleCpp, closure: Box<QObjectClosureBox>);

        /// Posts `closure` to the event loop running on the Qt thread, without
        /// a target. Thread-safe.
        fn post_void_closure(closure: Box<VoidClosureBox>);
    }

    extern "Rust" {
        type VoidClosureBox;
        fn run(self: &mut VoidClosureBox);

        type QObjectClosureBox;
        fn run(self: &mut QObjectClosureBox, target: &QObject);
    }
}


type VoidClosure = dyn FnOnce() + Send;
#[doc(hidden)]
pub struct VoidClosureBox {
    f: Option<Box<VoidClosure>>,
}

impl VoidClosureBox {
    fn run(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}

type QObjectClosure = dyn FnOnce(&QObject) + Send;
#[doc(hidden)]
pub struct QObjectClosureBox {
    f: Option<Box<QObjectClosure>>,
}

impl QObjectClosureBox {
    fn run(&mut self, target: &QObject) {
        if let Some(f) = self.f.take() {
            f(target);
        }
    }
}

/// A thread-safe handle that schedules Rust closures to run on the Qt thread.
///
/// A `QThreadHandle<T>` is bound to a `#[qobject]` instance of type `T` and
/// hands the closure access to it: shared via [`invoke`](Self::invoke),
/// mutable via [`invoke_mut`](Self::invoke_mut). Obtain one via
/// [`QmlObject::get_thread_handle`](crate::QmlObject::get_thread_handle).
/// The calls are type-checked at compile time and need no `#[qslot]` annotation.
/// A `QThreadHandle<T>` has to be created on the Qt thread and can only be bound
/// to `#[qobject]` instances that live on that thread.
///
/// A `QThreadHandle<()>`, created via [`QThreadHandle::default`], is not
/// bound to any `#[qobject]` instance and just runs closures on the Qt thread.
/// It can be created in any thread.
///
/// Closures are posted to the Qt event loop through the application instance;
/// On delivery the target's liveness is checked on the Qt thread, so a target
/// deleted at any point is safe: the closure is then silently discarded.
/// Without an application, closures are discarded as well, and so are the
/// closures of a handle whose target was created on another thread.
///
/// While a closure runs, it holds a borrow of the `Rc<RefCell<_>>` shared
/// with the QML engine. If the object is already mutably borrowed at that
/// point (for example by a slot running a nested event loop), the application
/// is aborted.
///
/// # Example
///
/// ```
/// # use qtbridge::{qobject, QmlObject};
/// # #[qobject]
/// # pub mod example {
/// #     #[derive(Default)]
/// #     pub struct Backend { pub value: i32 }
/// #     impl Backend {
/// #         #[qsignal]
/// #         pub fn data_ready(&mut self);
/// #     }
/// # }
/// # use example::Backend;
/// let backend = Backend::default_with_attached_qobject();
/// let handle = backend.borrow().get_thread_handle();
/// std::thread::spawn(move || {
///     handle.invoke_mut(|b| {
///         b.value = 42;
///         b.data_ready();
///     });
/// }).join().unwrap();
/// ```
pub struct QThreadHandle<T> {
    cpp: cxx::UniquePtr<ffi::QThreadHandleCpp>,
    _target_type: PhantomData<fn(T) -> T>,
}

// SAFETY: the C++ guard is immutable after construction. `post_closure`
// only copies the weak reference `QPointer` and posts to the application
// instance. Dropping the guard releases the weak reference. No
// dereferencing of the Qt thread.
unsafe impl<T> Send for QThreadHandle<T> {}
unsafe impl<T> Sync for QThreadHandle<T> {}

impl Default for QThreadHandle<()> {
    /// Creates a handle without a target. May be called on any thread.
    fn default() -> Self {
        Self { cpp: cxx::UniquePtr::null(), _target_type: PhantomData }
    }
}

impl QThreadHandle<()> {
    /// Schedules `callback` in the Qt event loop.
    ///
    /// If there is no application, the callback is silently discarded, along
    /// with all captured objects.
    ///
    /// # Example
    ///
    /// ```
    /// # use qtbridge::QThreadHandle;
    /// let handle = QThreadHandle::default();
    /// std::thread::spawn(move || {
    ///     handle.invoke(|| println!("on the Qt thread"));
    ///     QThreadHandle::default().invoke(|| println!("also on the Qt thread"));
    /// }).join().unwrap();
    /// ```
    pub fn invoke(&self, callback: impl FnOnce() + Send + 'static) {
        ffi::post_void_closure(Box::new(VoidClosureBox { f: Some(Box::new(callback)) }));
    }
}

impl<T: QObjectHolder> QThreadHandle<T> {
    /// Creates a `QThreadHandle` for `target`.
    ///
    /// Prefer
    /// [`QmlObject::get_thread_handle`](crate::QmlObject::get_thread_handle)
    /// over calling this directly.
    pub fn new(target: &T) -> Self {
        let obj = target.get_qobject_ptr();
        let cpp = unsafe { ffi::make_qt_thread_handle(obj) };
        Self { cpp, _target_type: PhantomData }
    }

    /// Schedules `callback` in the Qt event loop with shared access to the
    /// bound target.
    ///
    /// If the target has been dropped by the time the event loop delivers
    /// the call the callback is silently discarded, along with all captured
    /// objects.
    pub fn invoke(&self, callback: impl FnOnce(&T) + Send + 'static) {
        self.post(move |rc| callback(&rc.borrow()));
    }

    /// Schedules `callback` in the Qt event loop with mutable access to the
    /// target.
    ///
    /// If the target has been dropped by the time the event loop delivers
    /// the call the callback is silently discarded, along with all captured
    /// objects.
    pub fn invoke_mut(&self, callback: impl FnOnce(&mut T) + Send + 'static) {
        self.post(move |rc| callback(&mut rc.borrow_mut()));
    }

    #[doc(hidden)]
    fn post(&self, callback: impl FnOnce(Rc<RefCell<T>>) + Send + 'static) {
        let erased: Box::<dyn FnOnce(&QObject) + Send> = Box::new(move |qobj| callback(recover_rc::<T>(qobj)));
        ffi::post_closure(&self.cpp, Box::new(QObjectClosureBox { f: Some(erased) }));
    }
}

// This is `QObjectHolder::qobject_to_rc_ref_cell` minus two things it does
// not need: the inherits check (the handle is typed at construction, so the
// `QObject` guarded by the `QPointer` is statically a `T`), and the
// ownership `repin`.
fn recover_rc<T: QObjectHolder>(qobj: &QObject) -> Rc<RefCell<T>> {
    let proxy_ptr = get_rust_proxy(qobj) as *const T::ProxyRust;
    debug_assert!(!proxy_ptr.is_null());
    let proxy = unsafe { &*proxy_ptr };
    let rc_adapter = proxy.get_rust_object_rc();

    // SAFETY: see `qobject_to_rc_ref_cell`
    let raw_ref_cell = Rc::into_raw(rc_adapter).cast::<RefCell<T>>();
    unsafe { Rc::from_raw(raw_ref_cell) }
}
