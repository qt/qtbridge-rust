// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

use cxx_qt_lib::QObjectMutPtr;
use qtbridge_type_lib::{QObject, QVariant};
use crate::qproxies::{QRustProxy, ConstructionMode};
use crate::rustobjectgetter::get_rust_proxy;
use crate::{DispatchMetaCall, QMetaInfo, QmlMethodInvoker};
use std::collections::HashMap;


pub trait QObjectHolder : DispatchMetaCall + QMetaInfo + Default + 'static {
    /// Alias for the Rust proxy type corresponding to the user-defined type.
    /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
    /// forwarding calls in both directions and managing borrowing of the Rust object
    /// during C++ calls.
    #[doc(hidden)]
    type ProxyRust: QRustProxy<ProxyCppType = <Self as QMetaInfo>::CppProxy>;

    #[doc(hidden)]
    fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
    where
        F: FnOnce( &mut HashMap<*const u8, *const u8>) -> R
    {
        thread_local! { static INSTANCES: RefCell<HashMap<*const u8, *const u8>> =
                RefCell::new(HashMap::new());
        }
        INSTANCES.with_borrow_mut(f)
    }

    /// Return a pointer to the Rust proxy associated with the specified object,
    /// or `None` if no proxy is registered.
    #[doc(hidden)]
    fn try_get_rust_proxy_ptr_from_ptr(rust_obj_ptr: *const Self) -> Option<*mut Self::ProxyRust> {
        let proxy_ptr = Self::try_borrow_mut_proxies_map(|map| {
            map.get(&rust_obj_ptr.cast::<u8>()).copied().unwrap_or_default()
        });
        NonNull::new(proxy_ptr as *mut Self::ProxyRust).map(|nn| nn.as_ptr())
    }

    /// Return a pointer to the Rust proxy associated with the specified object,
    /// or `None` if no proxy is registered.
    #[doc(hidden)]
    fn try_get_rust_proxy_ptr(&self) -> Option<*mut Self::ProxyRust> {
        Self::try_get_rust_proxy_ptr_from_ptr(std::ptr::from_ref(self))
    }

    /// Return `QObject` attached to the specified Rust object.
    #[doc(hidden)]
    fn get_qobject_ptr(&self) -> *mut QObject {
        let Some(proxy_ptr) = Self::try_get_rust_proxy_ptr(self) else {
            return std::ptr::null_mut()
        };
        let rust_proxy = unsafe { &*proxy_ptr };
        let cpp_proxy = rust_proxy.get_cpp_proxy();
        cpp_proxy as *mut QObject
    }

    /// Return `QObject` attached to the specified Rust object.
    #[doc(hidden)]
    fn rc_ref_cell_to_qobject(self_obj: &Rc<RefCell<Self>>) -> *const QObject {
        let Some(proxy_ptr) = Self::try_get_rust_proxy_ptr_from_ptr(self_obj.as_ptr()) else {
            return std::ptr::null_mut()
        };
        let rust_proxy = unsafe { &*proxy_ptr };
        let cpp_proxy = rust_proxy.get_cpp_proxy();
        cpp_proxy as *mut QObject
    }

    /// Return the Rust object attached to the specified `QObject`.
    #[doc(hidden)]
    unsafe fn qobject_to_rc_ref_cell(qobj_ptr: *const QObject) -> Rc<RefCell<Self>>
    {
        let qobj_ref = unsafe { qobj_ptr.as_ref() }
            .expect("Input QObject is null");
        let proxy_ptr = get_rust_proxy(qobj_ref);
        debug_assert!(!proxy_ptr.is_null());

        // Verify the QObject really is a `Self` before reinterpreting its
        // proxy/object as `Self`'s - otherwise the casts below are UB. Use an
        // a inherits check rather than meta-object identity: a `Self`
        // instantiated and extended in QML carries a derived `QMetaObject`
        // that is not identical to `Self`'s dynamic meta-object, yet the
        // underlying proxy/object is still a `Self` (QML only layers a
        // meta-object on top, it does not change the Rust type).
        let qobj_meta_obj = unsafe { qobj_ref.get_qmeta_object().as_ref() };
        let self_meta_obj = unsafe {
            <Self as QMetaInfo>::get_shared_dynamic_meta_object_data().get_meta_object().as_ref()
        };
        let inherits = match (qobj_meta_obj, self_meta_obj) {
            (Some(d), Some(b)) => d.inherits(b),
            _ => false,
        };
        if !inherits {
            let qobj_name = qobj_meta_obj.map_or("<null>".into(), |m| m.meta_type().name());
            let self_name = self_meta_obj.map_or("<null>".into(), |m| m.meta_type().name());
            panic!("Value of wrong type: '{qobj_name}' is not a '{self_name}' (nor a subclass)")
        }

        let proxy = unsafe { &*(proxy_ptr as *const Self::ProxyRust) };
        let rc_adapter = proxy.get_rust_object_rc()
            .expect("Rust object associated with given QObject was already dropped");

        // Rust interest exists again: take ownership back if it was handed
        // to the engine.
        crate::registry::repin(rc_adapter.as_ptr() as *const u8);

        // SAFETY: the inherits check above proves the `QObject` is a `Self` (or
        // a QML-derived subclass of it) - and therefore the allocation behind
        // `rc_adapter` - was created as `RefCell<Self>`.
        // The adapter `Rc` only layers a vtable over that same allocation, so
        // its data pointer addresses a real `RefCell<Self>` with matching size
        // and alignment; reinterpreting it back is sound. `into_raw` parks the
        // `+1` produced by `get_rust_object_rc` and `from_raw` reclaims it, so
        // the reference count stays balanced.
        let raw_ref_cell = Rc::into_raw(rc_adapter).cast();
        unsafe { Rc::from_raw(raw_ref_cell) }
    }

    /// Returns a [`QmlMethodInvoker`] that can invoke methods on the underlying
    /// `QObject` from any thread.
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
    /// invoker.invoke_method("dataReady");
    /// ```
    fn get_qml_method_invoker(&self) -> QmlMethodInvoker
    {
        QmlMethodInvoker::new(self)
    }

    /// This function has to be implemented on the specific type and
    /// provides the conversion from the specific type to the dynamic
    /// trait type.
    ///
    /// This function ensures that the type indeed implements the trait
    /// specified by the [`QRustProxy`].
    #[doc(hidden)]
    fn as_adaptor_trait(rust_obj_rc: Rc<RefCell<Self>>) -> Rc<RefCell<<Self::ProxyRust as QRustProxy>::AdapterType>>;

    /// Register the given Rust object instance in the multiton.
    /// Create Rust and C++ proxies and links them to the Rust object.
    /// If `construction` is `AtAddress`, the C++ proxy is created using
    /// placement new operator at respective address
    #[doc(hidden)]
    fn register_instance_in_map(rust_obj_rc: Rc<RefCell<Self>>, construction: ConstructionMode) {
        let key = (*rust_obj_rc).as_ptr() as *const u8;
        // Rust-created objects are owned by `crate::registry`,
        // which holds a strong reference and deletion is
        // centralized in `crate::registry::collect_garbage`.
        // QML-created objects are owned by the engine,
        // the proxy holds a strong reference and deletion is
        // triggered by the QML-engine.
        let keep: Rc<RefCell<Self>> = rust_obj_rc.clone();
        let dyn_rc = Self::as_adaptor_trait(rust_obj_rc);
        let dynamic_meta = <Self as QMetaInfo>::get_shared_dynamic_meta_object_data();
        let proxy = Self::ProxyRust::new(&dyn_rc, dynamic_meta, &construction, Box::new(move || {
            Self::unregister_instance_in_map(key);
            crate::registry::unregister(key);
        }));
        Self::try_borrow_mut_proxies_map(|proxies| {
            proxies.insert(key, proxy as *const u8);
        });
        if matches!(construction, ConstructionMode::Weak) {
            // SAFETY: We constructed proxy just above.
            let qobject = unsafe { &*proxy }.get_cpp_proxy() as *mut QObject;
            crate::registry::register(keep, key, qobject);
        }
    }

    /// Removes the entry associated with the specified Rust object from the multiton map.
    #[doc(hidden)]
    fn unregister_instance_in_map(rust_obj_ptr: *const u8) {
        Self::try_borrow_mut_proxies_map(|proxies| proxies.remove(&rust_obj_ptr))
            .expect("Proxy object for rust object is not registered");
    }

    /// Creates a default-initialized instance and attaches the required
    /// [`QObject`], enabling its use in QML.
    ///
    /// The returned `Rc<RefCell<Self>>` is an ordinary handle, shared with
    /// QML. Dropping the handle does not drop the instance if it is in use by
    /// QML or until the garbage collection deletes the QML instance.
    fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
        let instance = Default::default();
        Self::attach_qobject(&instance);
        instance
    }

    /// Attaches a dedicated [`QObject`] to an existing `instance`,
    /// enabling its use in QML.
    fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
        Self::register_instance_in_map(
            instance.clone(),
            ConstructionMode::Weak
        );
    }

    /// Detaches and deletes the dedicated [`QObject`] of this instance.
    fn detach_qobject(&self) {
        let qobj_ptr = self.get_qobject_ptr();
        if !qobj_ptr.is_null() {
            QObject::delete(qobj_ptr);
        }
    }

    /// Returns a [`QVariant`] containing a pointer to this object.
    fn as_qvariant(&self) -> QVariant {
        let qobj_ptr = self.get_qobject_ptr() as *mut cxx_qt::QObject;
        assert!(!qobj_ptr.is_null(), "QObject is not attached");
        let qobj_ptr_wrap = unsafe { QObjectMutPtr::from_raw(qobj_ptr) };
        (&qobj_ptr_wrap).into()
    }
}
