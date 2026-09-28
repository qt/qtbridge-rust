// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[macro_export]
macro_rules! call_rust_trait_impl {
    (unsafe mut $self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_call_rust_with_handle_mut(|vtable| {
                unsafe { vtable.$method($($arg),*) }
            })
            .expect(concat!(
                "Failed to borrow mutably for ",
                stringify!($method)
            ))
    };

    (unsafe $self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_call_rust_with_handle(|vtable| {
                unsafe { vtable.$method($($arg),*) }
            })
            .expect(concat!(
                "Failed to borrow for ",
                stringify!($method)
            ))
    };

    (mut $self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_call_rust_with_handle_mut(|vtable| {
                vtable.$method($($arg),*)
            })
            .expect(concat!(
                "Failed to borrow mutably for ",
                stringify!($method)
            ))
    };

    ($self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_call_rust_with_handle(|vtable| {
                vtable.$method($($arg),*)
            })
            .expect(concat!(
                "Failed to borrow for ",
                stringify!($method)
            ))
    };
}

#[macro_export]
macro_rules! call_cpp_impl {
    (mut $self:expr, $mut_reference:expr, $method:ident ( $($arg:expr),* )) => {{
        let proxy = unsafe {
            $self.cpp_proxy
                .as_mut()
                .expect("cpp_proxy was null")
        };
        let proxy_pinned = unsafe { std::pin::Pin::new_unchecked(proxy) };
        $self.rust_obj
            .try_store_handle_and_call_cpp_mut($mut_reference, || proxy_pinned.$method($($arg),*))
            .expect(concat!(
                "Failed to borrow mutably for ",
                stringify!($method)
            ))
    }};

    ($self:expr, $reference:expr, $method:ident ( $($arg:expr),* )) => {{
        let proxy = unsafe {
            $self.cpp_proxy
                .as_ref()
                .expect("cpp_proxy was null")
        };
         $self.rust_obj
            .try_store_handle_and_call_cpp($reference, || proxy.$method($($arg),*))
            .expect(concat!(
                "Failed to borrow for ",
                stringify!($method)
            ))
    }};
}

/// Manages shared access to a Rust object across the Rust/C++ boundary.
///
/// When Rust calls into C++ and C++ calls back into Rust, the inner callback
/// cannot re-borrow the `RefCell` without panicking. This struct solves
/// this by caching a raw pointer before crossing into C++ and restoring it as
/// a reference on the way back, avoiding a second borrow while preserving a
/// valid borrow stack per the "Stacked Borrows" aliasing model.
pub struct RustObjAccess<T: ?Sized> {
    /// The strong reference that keeps the Rust object alive: a value
    /// lives exactly as long as its proxy pair, plus any user handles.
    shared_reference: Rc<RefCell<T>>,
    borrow: Cell<BorrowState<T>>,
}

impl<T: ?Sized> RustObjAccess<T> {
    pub fn new(ptr: Rc<RefCell<T>>) -> Self {
        Self {
            shared_reference: ptr,
            borrow: Cell::new(BorrowState::None),
        }
    }

    pub fn try_call_rust_with_handle<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where
        F: FnOnce(&T) -> R,
    {
        let guard = BorrowState::consume(&self.borrow);

        match guard.content() {
            BorrowState::Immutable(ptr) => Ok(f(unsafe { &**ptr })),
            BorrowState::Mutable(ptr) => Ok(f(unsafe { &**ptr })),
            BorrowState::None => {
                // Protect from a garbage collection.
                let rc = self.shared_reference.clone();
                let ref_guarded = rc.try_borrow()
                    .map_err(RustObjAccessError::BorrowError)?;
                Ok(f(&*ref_guarded))
            }
        }
    }

    pub fn try_call_rust_with_handle_mut<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where
        F: FnOnce(&mut T) -> R,
    {
        let guard = BorrowState::consume(&self.borrow);

        match guard.content() {
            BorrowState::Mutable(ptr) => Ok(f(unsafe { &mut **ptr })),
            BorrowState::Immutable(_) => Err(RustObjAccessError::BorrowConflict),
            BorrowState::None => {
                // Protect from a garbage collection.
                let rc = self.shared_reference.clone();
                let mut ref_guarded = rc.try_borrow_mut()
                    .map_err(RustObjAccessError::BorrowMutError)?;
                Ok(f(&mut *ref_guarded))
            }
        }
    }

    pub fn try_store_handle_and_call_cpp<F, R>(&self, rust_obj: &T, f: F) -> Result<R, RustObjAccessError>
    where
        F: FnOnce() -> R,
    {
        assert!(
            self.contains(rust_obj),
            "The rust_obj you want to call a function on does not match the shared reference."
        );
        let guard = BorrowState::store(&self.borrow, rust_obj);
        // It is possible that multiple immutable references are alive, but no mutable
        if matches!(guard.content(), BorrowState::Mutable(_)) {
            return Err(RustObjAccessError::BorrowConflict);
        }
        Ok(f())
    }

    pub fn try_store_handle_and_call_cpp_mut<F, R>(&self, rust_obj: &mut T, f: F) -> Result<R, RustObjAccessError>
    where
        F: FnOnce() -> R,
    {
        assert!(
            self.contains(rust_obj),
            "The rust_obj you want to call a function on does not match the shared reference."
        );
        let guard = BorrowState::store_mut(&self.borrow, rust_obj);
        // If we get a mutable reference, there should be no other reference alive
        if  matches!(guard.content(), BorrowState::Mutable(_)) || matches!(guard.content(), BorrowState::Immutable(_)) {
            return Err(RustObjAccessError::BorrowConflict);
        }
        Ok(f())
    }

    pub fn get_rc(&self) -> Rc<RefCell<T>> {
        self.shared_reference.clone()
    }

    fn contains(&self, obj: &T) -> bool {
        let expected = self.shared_reference.as_ptr() as *const ();
        let actual = obj as *const T as *const ();
        expected == actual
    }
}


enum BorrowState<T: ?Sized> {
    None,
    Immutable(*const T),
    Mutable(*mut T),
}

impl<T: ?Sized> BorrowState<T> {
    fn consume(cell: &Cell<Self>) -> BorrowGuard<'_, T> {
        let consumed = cell.replace(BorrowState::None);
        BorrowGuard { cell, consumed }
    }

    fn store<'a>(cell: &'a Cell<Self>, rust_obj: &T) -> BorrowGuard<'a, T> {
        let old = cell.replace(BorrowState::Immutable(rust_obj as *const T));
        BorrowGuard { cell, consumed: old }
    }

    fn store_mut<'a>(cell: &'a Cell<Self>, rust_obj: &mut T) -> BorrowGuard<'a, T> {
        let old = cell.replace(BorrowState::Mutable(rust_obj as *mut T));
        BorrowGuard { cell, consumed: old }
    }
}

struct BorrowGuard<'a, T: ?Sized> {
    cell: &'a Cell<BorrowState<T>>,
    consumed: BorrowState<T>,
}

impl<T: ?Sized> BorrowGuard<'_, T> {
    fn content(&self) -> &BorrowState<T> {
        &self.consumed
    }
}

impl<T: ?Sized> Drop for BorrowGuard<'_, T> {
    fn drop(&mut self) {
        self.cell.set(mem::replace(&mut self.consumed, BorrowState::None));
    }
}

#[derive(Debug)]
pub enum RustObjAccessError {
    BorrowError(std::cell::BorrowError),
    BorrowMutError(std::cell::BorrowMutError),
    BorrowConflict,
}
