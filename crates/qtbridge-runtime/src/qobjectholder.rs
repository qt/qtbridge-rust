// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

use qtbridge_type_lib::{QMetaType, QObject};
use qtbridge_type_lib::core::qobject;
use crate::qproxies::{QCppProxy, QRustProxy, PlacementAddress, AdapterUpcast};
use crate::registry::Owner;
use crate::rustobjectgetter::get_rust_proxy;
use crate::{DispatchMetaCall, DynamicMetaObjectData, QMetaInfo};

/// The C++ half of the proxy pair of `T`.
pub type CppProxyOf<T> = <<T as QObjectHolder>::ProxyRust as QRustProxy>::ProxyCppType;

/// Bridge proxy selection and connector behind every `#[qobject]` type.
#[doc(hidden)]
pub trait QObjectHolder : DispatchMetaCall + QMetaInfo + Sized + 'static
where
    Self::ProxyRust: AdapterUpcast<Self>,
    Self::ProxyRust: QRustProxy,
{
    /// Alias for the Rust proxy type corresponding to the user-defined type.
    /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
    /// forwarding calls in both directions and managing borrowing of the Rust object
    /// during C++ calls.
    type ProxyRust;

    /// Creates a new `DynamicMetaObjectData` object and returns
    /// a raw pointer to the heap-allocated object.
    /// Ownership is not managed internally; the caller is responsible for it.
    fn create_dynamic_meta_object_data_for_type() -> *const DynamicMetaObjectData {
        let mut builder = crate::create_dynamic_meta_object_builder(
            Self::class_name(),
            <CppProxyOf<Self>>::get_static_meta_object());
        Self::build_dynamic_meta_type(builder.pin_mut());
        builder.pin_mut()
            .take_dynamic_metaobject_data()
    }

    /// Return DynamicMetaObjectData containing information
    /// about signals/slots/properties for given Rust object.
    ///
    /// The `#[qobject]` macro overrides this with a per-type `OnceLock` body; the
    /// default serves generic types and hand-written impls.
    fn get_shared_dynamic_meta_object_data() -> &'static DynamicMetaObjectData {
        dynamic_meta_object_data_for_generic::<Self>()
    }

    /// Returns the [`QMetaType`] for a pointer to this type (`Self *`).
    ///
    /// The `#[qobject]` macro overrides this with a per-type `OnceLock` body; the
    /// default serves generic types and hand-written impls.
    fn get_qobject_ptr_qmetatype() -> QMetaType {
        let iface = crate::qmetatypeforqobject::ptr_interface_for_generic::<Self>();
        QMetaType::new_with_interface(iface as *const _)
    }

    /// Return a pointer to the Rust proxy associated with the specified object,
    /// or `None` if no proxy is registered.
    fn try_get_rust_proxy_ptr_from_ptr(rust_obj_ptr: *const Self) -> Option<*mut Self::ProxyRust> {
        let proxy_ptr = crate::registry::proxy_ptr(rust_obj_ptr.cast::<u8>());
        NonNull::new(proxy_ptr as *mut Self::ProxyRust).map(|nn| nn.as_ptr())
    }

    /// Return a pointer to the Rust proxy associated with the specified object,
    /// or `None` if no proxy is registered.
    fn try_get_rust_proxy_ptr(&self) -> Option<*mut Self::ProxyRust> {
        Self::try_get_rust_proxy_ptr_from_ptr(std::ptr::from_ref(self))
    }

    /// Return `QObject` attached to the specified Rust object.
    fn get_qobject_ptr(&self) -> *mut QObject {
        let Some(proxy_ptr) = Self::try_get_rust_proxy_ptr(self) else {
            return std::ptr::null_mut()
        };
        let rust_proxy = unsafe { &*proxy_ptr };
        let cpp_proxy = rust_proxy.get_cpp_proxy();
        cpp_proxy as *mut QObject
    }

    /// Return the `QObject` attached to the given object, attaching one
    /// first if none exists.
    fn rc_ref_cell_to_qobject(self_obj: &Rc<RefCell<Self>>) -> *const QObject {
        let proxy_ptr = Self::try_get_rust_proxy_ptr_from_ptr(self_obj.as_ptr())
            .unwrap_or_else( || {
                Self::register_instance(self_obj.clone(), Owner::RustRegistry, None);
                Self::try_get_rust_proxy_ptr_from_ptr(self_obj.as_ptr())
                    .expect("Failed to attach and register a proxy")
            }
        );
        let rust_proxy = unsafe { &*proxy_ptr };
        rust_proxy.get_cpp_proxy() as *mut QObject
    }

    /// Return the Rust object attached to the specified `QObject`.
    unsafe fn qobject_to_rc_ref_cell(qobj_ptr: *const QObject) -> Rc<RefCell<Self>>
    {
        let qobj_ref = unsafe { qobj_ptr.as_ref() }
            .expect("Input QObject is null");
        let proxy_ptr = get_rust_proxy(qobj_ref);
        debug_assert!(!proxy_ptr.is_null());

        // Verify the QObject really is a `Self` before reinterpreting its
        // proxy/object as `Self`'s - otherwise the casts below are UB. Use
        // an inherits check rather than meta-object identity: a `Self`
        // instantiated and extended in QML carries a derived `QMetaObject`
        // that is not identical to `Self`'s dynamic meta-object, yet the
        // underlying proxy/object is still a `Self` (QML only layers a
        // meta-object on top, it does not change the Rust type).
        let qobj_meta_obj = unsafe { qobject::meta_object(qobj_ref).as_ref() };
        let self_meta_obj = unsafe {
            Self::get_shared_dynamic_meta_object_data().get_meta_object().as_ref()
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
        let rc_adapter = proxy.get_rust_object_rc();

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

    /// Creates the proxy pair for the given Rust object instance, links
    /// them together and registers the object in `crate::registry`.
    /// The C++ proxy is created with placement new at `at_address` if
    /// given (QML-created elements), on the heap otherwise.
    #[doc(hidden)]
    fn register_instance(
        rust_obj_rc: Rc<RefCell<Self>>, owner: Owner, at_address: Option<PlacementAddress>,
    ) {
        let key = (*rust_obj_rc).as_ptr() as *const u8;
        let keep: Rc<RefCell<Self>> = rust_obj_rc.clone();
        let dyn_rc = <Self::ProxyRust as AdapterUpcast<Self>>::upcast(rust_obj_rc);
        let dynamic_meta = Self::get_shared_dynamic_meta_object_data();
        let proxy = Self::ProxyRust::new(&dyn_rc, dynamic_meta, at_address, Box::new(move || {
            crate::registry::unregister(key);
        }));
        // SAFETY: We constructed proxy just above.
        let qobject = unsafe { &*proxy }.get_cpp_proxy() as *mut QObject;
        crate::registry::register(key, proxy as *const u8, qobject,
            Rc::<RefCell<Self>>::downgrade(&keep), owner);
    }
}

/// [`QObjectHolder::get_shared_dynamic_meta_object_data`] through a
/// TypeId-keyed cache, for types where a per-type static is not available.
pub fn dynamic_meta_object_data_for_generic<T: QObjectHolder>() -> &'static DynamicMetaObjectData
where
    T::ProxyRust: crate::qproxies::AdapterUpcast<T>,
{
    use std::any::TypeId;
    use std::collections::HashMap;
    thread_local!(static DYNAMIC_META_MAP: RefCell<HashMap<TypeId, *const DynamicMetaObjectData>> =
        RefCell::new(HashMap::new()));

    let type_id = TypeId::of::<T>();
    {
        let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_builder_map| {
            dynamic_meta_builder_map.get(&type_id)
                .copied()
                .unwrap_or_default()
        });
        if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
            return meta_data_ref;
        }
    }

    let meta_data_ptr = T::create_dynamic_meta_object_data_for_type();
    let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
    DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_builder_map| {
        dynamic_meta_builder_map.insert(type_id, meta_data_ptr);
    });

    meta_data_ref
}
