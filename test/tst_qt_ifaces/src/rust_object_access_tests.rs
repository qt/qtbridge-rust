// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use std::ops::AddAssign;
use std::cell::RefCell;
use std::rc::Rc;
use qtbridge_interfaces::object_access::rust_object_access::{RustObjAccess, RustObjAccessError};

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn require_that_new_creates_an_instance() {
    let rc = Rc::new(RefCell::new(0i32));
    let _instance = RustObjAccess::new(rc);
}

// ---------------------------------------------------------------------------
// Basic borrow / borrow_mut
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_invokes_function_with_value_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess::new(rc.clone());

    let result = instance.try_call_rust_with_handle(|value| value * 2).unwrap();
    assert_eq!(result, 84);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_invokes_function_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess::new(rc.clone());

    assert!(instance.try_call_rust_with_handle_mut(|value| *value += 1).is_ok());
    assert_eq!(*rc.borrow(), 43);
}

// ---------------------------------------------------------------------------
// Dispatch counts as Rust interest
//
// A running call holds an extra strong reference on purpose: the registry's
// collector spares objects with a strong count above the proxy's own, so an
// executing dispatch must be visible in the count.
// ---------------------------------------------------------------------------

#[test]
fn require_that_a_running_call_holds_an_extra_strong_reference() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let instance = RustObjAccess::new(rc);
    assert_eq!(weak.strong_count(), 1);
    instance.try_call_rust_with_handle(|_| {
        assert_eq!(weak.strong_count(), 2);
    }).unwrap();
    assert_eq!(weak.strong_count(), 1);
}

#[test]
fn require_that_a_running_mutable_call_holds_an_extra_strong_reference() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let instance = RustObjAccess::new(rc);
    assert_eq!(weak.strong_count(), 1);
    instance.try_call_rust_with_handle_mut(|_| {
        assert_eq!(weak.strong_count(), 2);
    }).unwrap();
    assert_eq!(weak.strong_count(), 1);
}

// ---------------------------------------------------------------------------
// Direct recursion
// ---------------------------------------------------------------------------

// In RustObjAccess, direct recursion through try_call_rust_with_handle works because
// consume() sets state to None, and RefCell allows multiple immutable borrows.

struct RecursionChecker {
    value: RustObjAccess<i32>,
}
impl RecursionChecker {
    pub fn new(obj: Rc<RefCell<i32>>) -> Self {
        Self {
            value: RustObjAccess::new(obj)
        }
    }

    fn calc_product_recursively_borrow_immut(&self, times: u32, init_value: u32) -> u32 {
        self.value.try_call_rust_with_handle(|value| -> u32 {
            let new_value = init_value + *value as u32;
            if times > 1 {
                return self.calc_product_recursively_borrow_immut(times - 1, new_value);
            }
            new_value
        }).unwrap()
    }
}

#[test]
fn require_that_borrowing_immutably_recursively_succeeds() {
    let rc = Rc::new(RefCell::new(10));
    let checker = RecursionChecker::new(rc.clone());
    let result = checker.calc_product_recursively_borrow_immut(4, 0);
    assert_eq!(result, 40)
}

// In RustObjAccess, direct mutable recursion is blocked: consume() sets state
// to None, so re-entry tries RefCell::try_borrow_mut() which fails because the
// outer RefMut is still on the stack. This is intentional: it prevents aliasing UB.

#[test]
fn require_that_borrowing_mutably_recursively_fails() {
    let rc = Rc::new(RefCell::new(10));
    let instance = RustObjAccess::new(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|value| {
        *value += 1;
        instance.try_call_rust_with_handle_mut(|_| {})
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowMutError(_))));
}

// Similarly, try_call_rust_with_handle inside try_call_rust_with_handle_mut fails because
// RefCell::try_borrow() fails when a RefMut is active.

#[test]
fn require_that_borrowing_immutably_within_mutable_borrow_fails() {
    let rc = Rc::new(RefCell::new(10));
    let instance = RustObjAccess::new(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| {
        instance.try_call_rust_with_handle(|_| {})
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowError(_))));
}

// ---------------------------------------------------------------------------
// Borrow conflicts between try_call_rust_with_handle and try_call_rust_with_handle_mut
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_mut_within_try_call_rust_with_handle_fails() {
    let rc = Rc::new(RefCell::new(0));
    let instance = RustObjAccess::new(rc);
    let result = instance.try_call_rust_with_handle(|_| {
        instance.try_call_rust_with_handle_mut(|_| {})
            .expect_err("Expected to be borrowed")
    });
    let error = result.unwrap();
    assert!(matches!(error, RustObjAccessError::BorrowMutError(_)))
}

// ---------------------------------------------------------------------------
// Interaction with original RefCell
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_borrow_original_ref_cell_succeeds_when_called_within_try_call_rust_with_handle() {
    let rc = Rc::new(RefCell::new(36));
    let instance = RustObjAccess::new(rc.clone());
    instance.try_call_rust_with_handle(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_ok());
        assert_eq!(*nested_borrow_result.unwrap(), 36);
    }).unwrap();
}

#[test]
fn require_that_try_borrow_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle_mut() {
    let rc = Rc::new(RefCell::new(37));
    let instance = RustObjAccess::new(rc.clone());
    instance.try_call_rust_with_handle_mut(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle_mut() {
    let rc = Rc::new(RefCell::new(39));
    let instance = RustObjAccess::new(rc.clone());
    instance.try_call_rust_with_handle_mut(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle() {
    let rc = Rc::new(RefCell::new(38));
    let instance = RustObjAccess::new(rc.clone());
    instance.try_call_rust_with_handle(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

// ---------------------------------------------------------------------------
// External borrow then try_call_rust_with_handle
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_succeeds_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(40));
    let ref_ = rc.borrow();
    let instance: RustObjAccess<i32> = RustObjAccess::new(rc.clone());
    instance.try_call_rust_with_handle(|value| {
        assert_eq!(*value, 40);
    }).unwrap();
    assert_eq!(*ref_, 40);
}

#[test]
fn require_that_try_call_rust_with_handle_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(41));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess::new(rc.clone());
    let result = instance.try_call_rust_with_handle(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowError(_)));
    ref_mut.add_assign(1);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_fails_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let ref_ = rc.borrow();
    let instance = RustObjAccess::new(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    assert_eq!(*ref_, 42);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess::new(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    ref_mut.add_assign(1);
}

// ---------------------------------------------------------------------------
// store_handle: the C++ re-entry path
//
// In RustObjAccess, try_store_handle_and_call_cpp[_mut] takes a reference
// to the object and a closure that receives no arguments. The reference is
// stored so that deeper re-entry via try_call_rust_with_handle[_mut] can reconstruct
// the reference from the stored pointer.
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_store_handle_and_call_cpp_succeeds_when_called_with_immutable_reference() {
    let rc = Rc::new(RefCell::new(44));
    let instance = RustObjAccess::new(rc.clone());
    let b = rc.borrow();
    let result = instance.try_store_handle_and_call_cpp(&*b, || *b + 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn require_that_try_store_handle_and_call_cpp_mut_succeeds_when_called_with_mutable_reference() {
    let rc = Rc::new(RefCell::new(45));
    let instance = RustObjAccess::new(rc.clone());
    {
        let mut b = rc.borrow_mut();
        let result = instance.try_store_handle_and_call_cpp_mut(&mut *b, || {
            // Re-enter to modify through stored handle
            instance.try_call_rust_with_handle_mut(|value| *value += 6).unwrap()
        });
        assert!(result.is_ok());
    }
    assert_eq!(*rc.borrow(), 51);
}

#[test]
fn require_that_try_call_rust_with_handle_succeeds_when_called_after_try_store_handle_and_call_cpp() {
    let rc = Rc::new(RefCell::new(46));
    let instance = RustObjAccess::new(rc.clone());
    let b = rc.borrow();
    let result = instance.try_store_handle_and_call_cpp(&*b, || {
        instance.try_call_rust_with_handle(|value| value + 8)
    });

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), 54);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_succeeds_when_called_after_try_store_handle_and_call_cpp_mut() {
    let rc = Rc::new(RefCell::new(47));
    let instance = RustObjAccess::new(rc.clone());
    {
        let mut b = rc.borrow_mut();
        let result = instance.try_store_handle_and_call_cpp_mut(&mut b, || {
            instance.try_call_rust_with_handle_mut(|value| *value += 8)
        });
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
    assert_eq!(*rc.borrow(), 55);
}

// ---------------------------------------------------------------------------
// store_handle: possible conflicts when already stored
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_store_handle_and_call_cpp_succeeds_when_called_while_already_stored() {
    let rc = Rc::new(RefCell::new(48));
    let instance = RustObjAccess::new(rc.clone());
    let b = rc.borrow();
    let mark_hit = std::cell::Cell::new(false);
    let result = instance.try_store_handle_and_call_cpp(&*b, || {
        instance.try_store_handle_and_call_cpp(&*b, || {
            mark_hit.set(true);
        })
    });
    assert!(result.is_ok());
    assert!(mark_hit.get(), "Inner callback should have been called");
}

// We do a lot of bad stuff that would trigger miri, but just in the test!
#[test]
#[cfg(not(miri))]
fn require_that_try_store_handle_and_call_cpp_mut_fails_when_called_while_already_stored() {
    let rc = Rc::new(RefCell::new(48));
    let instance = RustObjAccess::new(rc.clone());
    let instance_ptr = &instance as *const _ as *mut RustObjAccess<i32>;
    let mut b = rc.borrow_mut();
    let b_ptr = &mut *b as *mut i32;
    let result = instance.try_store_handle_and_call_cpp_mut(&mut *b, || {
        let sneaky = unsafe { &*instance_ptr };
        sneaky.try_store_handle_and_call_cpp_mut(unsafe { &mut *b_ptr }, || {
            panic!("Not supposed to be called")
        })
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowConflict)));
}
