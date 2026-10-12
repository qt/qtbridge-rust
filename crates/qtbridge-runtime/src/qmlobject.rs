// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::core::qobject;
use crate::QThreadHandle;
use crate::qobjectholder::QObjectHolder;
use crate::registry::Owner;

pub trait QmlObject: QObjectHolder {
    /// Creates a default-initialized instance and attaches its
    /// [`QObject`](qtbridge_type_lib::QObject) eagerly.
    ///
    /// The returned `Rc<RefCell<Self>>` is an ordinary handle, shared with
    /// QML. Droping the handle does not drop the instance if it is in use by
    /// QML or until the garbage collection delete the QML instance.
    fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>>
    where
        Self: Default,
    {
        let instance = Default::default();
        Self::attach_qobject(&instance);
        instance
    }

    /// Attaches a dedicated [`QObject`](qtbridge_type_lib::QObject) to an existing `instance`,
    /// enabling its use in QML.
    fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
        Self::register_instance(instance.clone(), Owner::RustRegistry);
    }

    /// Detaches and deletes the dedicated [`QObject`](qtbridge_type_lib::QObject) of this instance.
    ///
    /// The instance continues as a plain Rust value and heals with a
    /// fresh `QObject` on its next exposure to QML.
    fn detach_qobject(&self) {
        let qobj_ptr = self.get_qobject_ptr();
        if !qobj_ptr.is_null() {
            unsafe { qobject::delete(qobj_ptr) };
        }
    }

    /// Returns a [`QThreadHandle`] that can schedule Rust closures to run
    /// on this object on the Qt thread, from any thread.
    ///
    /// This function can only be called on the Qt thread for QmlObject that
    /// live in this thread.
    ///
    /// # Example
    ///
    /// ```
    /// # use qtbridge::{qobject, QmlObject};
    /// # #[qobject]
    /// # pub mod example {
    /// #     #[derive(Default)]
    /// #     pub struct Backend { pub value: i32 }
    /// # }
    /// # use example::Backend;
    /// let backend = Backend::default_with_attached_qobject();
    /// let handle = backend.borrow().get_thread_handle();
    /// handle.invoke_mut(|b| b.value = 42);
    /// ```
    fn get_thread_handle(&self) -> QThreadHandle<Self>
    {
        QThreadHandle::new(self)
    }
}

impl<T: QObjectHolder> QmlObject for T {}
