// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;
use std::cell::RefCell;

use crate::QObjectHolder;
use crate::qproxies::QRustProxy;
use crate::rustobjectgetter::get_rust_proxy;
use qtbridge_type_lib::QObject;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h");
        #[namespace = "QtPrivate"]
        type QMetaTypeInterface = qtbridge_type_lib::QMetaTypeInterface;
        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;
        include!("qtbridge-type-lib/src/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    }

    #[namespace = "rust::bridge::qqmllistproperty"]
    unsafe extern "C++" {
        include!("cpp/qqmllistproperty.h");

        /// Build a `QVariant` holding a `QQmlListProperty<QObject>` stamped with the per-type list metatype.
        #[rust_name = "list_property_to_qvariant"]
        unsafe fn listPropertyToQVariant(
            meta_type: &QMetaType,
            object: *mut QObject,
            data: *mut u8,
            append_fn: usize,
            count_fn: usize,
            at_fn: usize,
            clear_fn: usize,
        ) -> QVariant;

        /// The list metatype interface for an element type: a clone of `QQmlListProperty<QObject>`
        /// (layout-identical to any `QQmlListProperty<T>`), named `QQmlListProperty<{element.name}>`
        /// with the cached typeId reset to 0 so it registers as a fresh, distinct type.
        ///
        /// The name is derived from `element`'s own registered name (so the list and element types
        /// can't drift apart) and leaked in C++.
        #[rust_name = "list_property_interface_for"]
        fn listPropertyInterfaceFor(element: &QMetaType) -> QMetaTypeInterface;
    }
}

pub(crate) use ffi::{list_property_interface_for, list_property_to_qvariant};

/// Plain `#[repr(C)]` view of `QQmlListProperty<QObject>`, used only to read fields out of the
/// raw pointer QML passes to the callbacks. NOT a cxx type; it never crosses the FFI boundary.
#[repr(C)]
pub(crate) struct QQmlListPropertyCpp {
    pub object: *mut QObject,
    pub data: *mut u8,
    pub append: *const (),
    pub count: *const (),
    pub at: *const (),
    pub clear: *const (),
    pub replace: *const (),
    pub remove_last: *const (),
}

unsafe fn get_proxy_ptr<Owner>(
    qobj: *const QObject,
) -> *mut <Owner as QObjectHolder>::ProxyRust
where
    Owner: QObjectHolder,
{
    let qobj_ref = unsafe { qobj.as_ref() }.expect("QObject pointer is null");
    let ptr = get_rust_proxy(qobj_ref);
    assert!(!ptr.is_null(), "Rust proxy not registered for QObject");
    ptr as *mut <Owner as QObjectHolder>::ProxyRust
}

/// `CountFunction` callback for `QQmlListProperty`. Wraps the operation in
/// `with_rust_ref` to handle re-entrant borrows via `RustObjAccess`.
pub(crate) unsafe extern "C" fn list_count<Owner, Elem>(prop: *const u8) -> isize
where
    Owner: QObjectHolder,
{
    let list_prop = unsafe { &*(prop as *const QQmlListPropertyCpp) };
    let proxy_ptr = unsafe { get_proxy_ptr::<Owner>(list_prop.object) };
    let store_offset = list_prop.data as usize;
    unsafe { &*proxy_ptr }.with_rust_ref(|adapter| {
        unsafe {
            let owner = &*(adapter as *const _ as *const Owner);
            let store = &*((owner as *const Owner).byte_add(store_offset) as *const Vec<Rc<RefCell<Elem>>>);
            store.len() as isize
        }
    })
}

/// `AtFunction` callback for `QQmlListProperty`. Wraps the operation in
/// `with_rust_ref` to handle re-entrant borrows via `RustObjAccess`.
pub(crate) unsafe extern "C" fn list_at<Owner, Elem>(prop: *const u8, idx: isize) -> *mut QObject
where
    Owner: QObjectHolder,
    Elem: QObjectHolder,
{
    let list_prop = unsafe { &*(prop as *const QQmlListPropertyCpp) };
    let proxy_ptr = unsafe { get_proxy_ptr::<Owner>(list_prop.object) };
    let store_offset = list_prop.data as usize;
    unsafe { &*proxy_ptr }.with_rust_ref(|adapter| {
        unsafe {
            let owner = &*(adapter as *const _ as *const Owner);
            let store = &*((owner as *const Owner).byte_add(store_offset) as *const Vec<Rc<RefCell<Elem>>>);
            <Elem as QObjectHolder>::rc_ref_cell_to_qobject(&store[idx as usize]).cast_mut()
        }
    })
}

/// `AppendFunction` callback for `QQmlListProperty`. Wraps the operation in
/// `with_rust_ref_mut` to handle re-entrant borrows via `RustObjAccess`, then
/// pushes the new element and emits the `Notify` signal. `Notify` must be a
/// zero-sized function item (a method of `Owner` with no parameters beyond `&mut self`).
pub(crate) unsafe extern "C" fn list_append<Owner, Elem, Notify>(prop: *const u8, item: *mut QObject)
where
    Owner: QObjectHolder,
    Elem: QObjectHolder,
    Notify: Fn(&mut Owner) + 'static,
{
    debug_assert_eq!(std::mem::size_of::<Notify>(), 0, "Notify must be a zero-sized type");
    let list_prop = unsafe { &*(prop as *const QQmlListPropertyCpp) };
    let proxy_ptr = unsafe { get_proxy_ptr::<Owner>(list_prop.object) };
    let store_offset = list_prop.data as usize;
    unsafe { &*proxy_ptr }.with_rust_ref_mut(|adapter| {
        unsafe {
            let owner = &mut *(adapter as *mut _ as *mut Owner);
            let store = &mut *((owner as *mut Owner).byte_add(store_offset) as *mut Vec<Rc<RefCell<Elem>>>);
            store.push(<Elem as QObjectHolder>::qobject_to_rc_ref_cell(item));
            let notify = std::mem::zeroed::<Notify>();
            notify(owner);
        }
    });
}

/// `ClearFunction` callback for `QQmlListProperty`. Wraps the operation in
/// `with_rust_ref_mut` to handle re-entrant borrows via `RustObjAccess`, then
/// clears the list and emits the `Notify` signal. `Notify` must be a
/// zero-sized function item (a method of `Owner` with no parameters beyond `&mut self`).
pub(crate) unsafe extern "C" fn list_clear<Owner, Elem, Notify>(prop: *const u8)
where
    Owner: QObjectHolder,
    Notify: Fn(&mut Owner) + 'static,
{
    debug_assert_eq!(std::mem::size_of::<Notify>(), 0, "Notify must be a zero-sized type");
    let list_prop = unsafe { &*(prop as *const QQmlListPropertyCpp) };
    let proxy_ptr = unsafe { get_proxy_ptr::<Owner>(list_prop.object) };
    let store_offset = list_prop.data as usize;
    unsafe { &*proxy_ptr }.with_rust_ref_mut(|adapter| {
        unsafe {
            let owner = &mut *(adapter as *mut _ as *mut Owner);
            let store = &mut *((owner as *mut Owner).byte_add(store_offset) as *mut Vec<Rc<RefCell<Elem>>>);
            store.clear();
            let notify = std::mem::zeroed::<Notify>();
            notify(owner);
        }
    });
}
