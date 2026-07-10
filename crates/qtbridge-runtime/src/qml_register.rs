// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;
use std::cell::RefCell;

use crate::QObjectHolder;
use crate::QMetaInfo;
use crate::qqmllistproperty::{list_append, list_count, list_at, list_clear};
use crate::qproxies::QCppProxy;
use crate::qproxies::ConstructionMode;
use qtbridge_type_lib::QObject;
use qtbridge_type_lib::QMetaType;
use qtbridge_type_lib::QMetaTypeInterface;
use qtbridge_type_lib::{QVariant, list_property_to_qvariant};

pub trait QmlRegister : QObjectHolder + Default
{
    const URI: &str;
    const ELEMENT_NAME: &str;
    const MINOR_VERSION: u8;
    const MAJOR_VERSION: u8;
    const IS_SINGLETON: bool;

    fn get_list_qmetatype() -> QMetaType {
        // TODO: The HashMap can be replaced with OnceCell in a per-type generated implementation
        // of this function in qtbridge-gen. Might improve performance.
        use std::collections::HashMap;
        thread_local!(static LIST_IFACE_MAP: RefCell<HashMap<i32, *const QMetaTypeInterface>>
            = RefCell::new(HashMap::new()));

        let element = <Self as QMetaInfo>::get_qmetatype();
        let key = element.id();

        let existing = LIST_IFACE_MAP.with_borrow(|m| m.get(&key).copied().unwrap_or_default());
        let iface = if existing.is_null() {
            let leaked = std::ptr::from_ref(Box::leak(Box::new(
                QMetaTypeInterface::qqml_list_property_for(&element)
            )));
            LIST_IFACE_MAP.with_borrow_mut(|m| m.insert(key, leaked));
            leaked
        } else {
            existing
        };
        QMetaType::new_with_interface(iface)
    }

    fn list_to_qvariant<Owner, Notify>(owner: &Owner, store: &Vec<Rc<RefCell<Self>>>, _notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        debug_assert_eq!(std::mem::size_of::<Notify>(), 0, "Notify must be a zero-sized type");
        let qobject = owner.get_qobject_ptr();
        let base = owner as *const Owner as *const u8;
        let field = store as *const _ as *const u8;
        // SAFETY: `store` is a field within `owner`, so both pointers are in the same allocation.
        // The `owner` can be reconstructed safely, even when objects are moved since we store a
        // shared reference to it. With the known offset we can also reconstruct `store`. The callbacks
        // in crate::qqmllistproperty expect this exact format.
        let store_offset = unsafe { field.offset_from(base) } as usize;
        unsafe { list_property_to_qvariant(
            &Self::get_list_qmetatype(),
            qobject,
            store_offset as *mut u8,
            (list_append::<Owner, Self, Notify> as *const ()).addr(),
            (list_count::<Owner, Self> as *const ()).addr(),
            (list_at::<Owner, Self> as *const ()).addr(),
            (list_clear::<Owner, Self, Notify> as *const ()).addr(),
        ) }
    }

    fn register() {
        let meta_obj_data = <Self as QMetaInfo>::get_shared_dynamic_meta_object_data();
        let meta_obj = unsafe {
            meta_obj_data
                .get_meta_object()
                .as_ref()
                .expect("Failed to get QMetaObject")
        };

        if Self::IS_SINGLETON {
            crate::qmlprivate::qml_register_singleton(
                <Self as QMetaInfo>::get_qobject_ptr_qmetatype(),
                monomorphize_singleton_ctor::<Self>(),
                Self::URI.as_bytes(),
                Self::MAJOR_VERSION,
                Self::MINOR_VERSION,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            )
        } else {
            let list_metatype = Self::get_list_qmetatype();
            list_metatype.register_type();

            crate::qmlprivate::qml_register_element(
                <Self as QMetaInfo>::get_qobject_ptr_qmetatype(),
                list_metatype,
                <<Self as QMetaInfo>::CppProxy as QCppProxy>::get_size() as u32,
                <<Self as QMetaInfo>::CppProxy as QCppProxy>::parser_status_cast(),
                monomorphize_element_ctor::<Self>(),
                Self::URI.as_bytes(),
                Self::MAJOR_VERSION,
                Self::MINOR_VERSION,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            );
        }
    }
}

fn element_ctor<T: QmlRegister>(addr: *mut u8, _userdata: *mut u8) {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map(instance.clone(), ConstructionMode::AtAddress(addr));
}

fn singleton_ctor<T: QmlRegister>() -> *mut QObject {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map(instance.clone(), ConstructionMode::Strong);
    instance.borrow().get_qobject_ptr()
}

fn monomorphize_element_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>(addr: *mut u8, userdata: *mut u8) {
        element_ctor::<T>(addr, userdata)
    }
    default_ctor::<T> as *const () as usize
}

fn monomorphize_singleton_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>() -> *mut QObject {
        singleton_ctor::<T>()
    }
    default_ctor::<T> as *const () as usize
}
