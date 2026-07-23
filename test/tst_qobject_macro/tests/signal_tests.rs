// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use qtbridge::qtbridge_type_lib::{QSignalSpy, QString, QVariant};
use qtbridge::{qobject, QObjectHolder};
use common::{MAX_SAFE_INTEGER, MIN_SAFE_INTEGER};

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
    }

    impl TestObject {
        // Pass the signal argument by value.
        #[qsignal]
        pub fn signal_no_args(&mut self);
        #[qsignal]
        pub fn signal_bool(&mut self, arg: bool);
        #[qsignal]
        pub fn signal_i8(&mut self, arg: i8);
        #[qsignal]
        pub fn signal_u8(&mut self, arg: u8);
        #[qsignal]
        pub fn signal_i16(&mut self, arg: i16);
        #[qsignal]
        pub fn signal_u16(&mut self, arg: u16);
        #[qsignal]
        pub fn signal_i32(&mut self, arg: i32);
        #[qsignal]
        pub fn signal_u32(&mut self, arg: u32);
        #[qsignal]
        pub fn signal_i64(&mut self, arg: i64);
        #[qsignal]
        pub fn signal_u64(&mut self, arg: u64);
        #[qsignal]
        pub fn signal_isize(&mut self, arg: isize);
        #[qsignal]
        pub fn signal_usize(&mut self, arg: usize);
        #[qsignal]
        pub fn signal_f32(&mut self, arg: f32);
        #[qsignal]
        pub fn signal_f64(&mut self, arg: f64);
        #[qsignal]
        pub fn signal_string(&mut self, arg: String);

        // Pass the signal argument by reference.
        #[qsignal]
        pub fn signal_bool_ref(&mut self, arg: &bool);
        #[qsignal]
        pub fn signal_i8_ref(&mut self, arg: &i8);
        #[qsignal]
        pub fn signal_u8_ref(&mut self, arg: &u8);
        #[qsignal]
        pub fn signal_i16_ref(&mut self, arg: &i16);
        #[qsignal]
        pub fn signal_u16_ref(&mut self, arg: &u16);
        #[qsignal]
        pub fn signal_i32_ref(&mut self, arg: &i32);
        #[qsignal]
        pub fn signal_u32_ref(&mut self, arg: &u32);
        #[qsignal]
        pub fn signal_i64_ref(&mut self, arg: &i64);
        #[qsignal]
        pub fn signal_u64_ref(&mut self, arg: &u64);
        #[qsignal]
        pub fn signal_isize_ref(&mut self, arg: &isize);
        #[qsignal]
        pub fn signal_usize_ref(&mut self, arg: &usize);
        #[qsignal]
        pub fn signal_f32_ref(&mut self, arg: &f32);
        #[qsignal]
        pub fn signal_f64_ref(&mut self, arg: &f64);
        #[qsignal]
        pub fn signal_string_ref(&mut self, arg: &String);

        // Test with multiple arguments
        #[qsignal]
        pub fn signal_many_args(&mut self,
            arg1: &String,
            arg2: i32,
            arg3: f32,
            arg4: &f64,
            arg5: String,
            arg6: u16,
            arg7: bool,
            arg8: &bool,
            arg9: i32,
            arg10: i64,
        );

    }
}

pub use test_object::TestObject;

// Tests that verify that signals are emitted and detected by QSignalSpy when invoked from the Rust side.

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_without_arguments() {
    let obj = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(unsafe { &*obj.borrow().get_qobject_ptr() }, "signal_no_args");
    obj.borrow_mut().signal_no_args();
    assert_eq!(spy.count(), 1);
}

fn test_signal_with_arg_value<EmitFn, CheckFn>(type_suffix: &str, emit_fn: EmitFn, check_fn: CheckFn)
where
    EmitFn: FnOnce(&mut TestObject),
    CheckFn: FnOnce(&QVariant) -> bool
{
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(unsafe { &*obj.borrow().get_qobject_ptr() }, &format!("signal_{type_suffix}"));
    emit_fn(&mut obj.borrow_mut());
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    assert!(check_fn(args.get(0).unwrap()));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_bool_arg() {
    test_signal_with_arg_value("bool",
        |obj| obj.signal_bool(true),
        |var| var.value::<bool>().unwrap());
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i8_arg() {
    test_signal_with_arg_value("i8",
        |obj| obj.signal_i8(42),
        |var| var.value::<i8>() == Some(42));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u8_arg() {
    test_signal_with_arg_value("u8",
        |obj| obj.signal_u8(43),
        |var| var.value::<u8>() == Some(43));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i16_arg() {
    test_signal_with_arg_value("i16",
        |obj| obj.signal_i16(-44),
        |var| var.value::<i16>() == Some(-44));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u16_arg() {
    test_signal_with_arg_value("u16",
        |obj| obj.signal_u16(45),
        |var| var.value::<u16>() == Some(45));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i32_arg() {
    test_signal_with_arg_value("i32",
        |obj| obj.signal_i32(46),
        |var| var.value::<i32>() == Some(46));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u32_arg() {
    test_signal_with_arg_value("u32",
        |obj| obj.signal_u32(47),
        |var| var.value::<u32>() == Some(47));

}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i64_arg() {
    test_signal_with_arg_value("i64",
        |obj| obj.signal_i64(48),
        |var| var.value::<i64>() == Some(48));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u64_arg() {
    test_signal_with_arg_value("u64",
        |obj| obj.signal_u64(49),
        |var| var.value::<u64>() == Some(49));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_isize_arg() {
    test_signal_with_arg_value("isize",
        |obj| obj.signal_isize(-50),
        |var| var.value::<i64>() == Some(-50));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_usize_arg() {
    test_signal_with_arg_value("usize",
        |obj| obj.signal_usize(51),
        |var| var.value::<u64>() == Some(51));
}


#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f32_arg() {
    test_signal_with_arg_value("f32",
        |obj| obj.signal_f32(0.5),
        |var| var.value::<f32>() == Some(0.5));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f64_arg() {
    test_signal_with_arg_value("f64",
        |obj| obj.signal_f64(0.25),
        |var| var.value::<f64>() == Some(0.25));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_string_arg() {
    test_signal_with_arg_value("string",
        |obj| obj.signal_string("ABC".to_owned()),
        |var| var.value::<QString>() == Some("ABC".into()));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_string_ref_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(unsafe { &*obj.borrow().get_qobject_ptr() }, "signal_string_ref");
    obj.borrow_mut().signal_string_ref(&String::from("DEF"));
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: QString = args.get(0)
        .map(cxx_qt_lib::QVariant::value)
        .flatten()
        .unwrap_or_default();
    assert_eq!(arg, "DEF".into());
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_bool_ref_arg() {
    test_signal_with_arg_value("bool_ref",
        |obj| obj.signal_bool_ref(&true),
        |var| var.value::<bool>().unwrap());
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i8_ref_arg() {
    test_signal_with_arg_value("i8_ref",
        |obj| obj.signal_i8_ref(&i8::MIN),
        |var| var.value::<i8>() == Some(i8::MIN));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u8_ref_arg() {
    test_signal_with_arg_value("u8_ref",
        |obj| obj.signal_u8_ref(&u8::MAX),
        |var| var.value::<u8>() == Some(u8::MAX));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i16_ref_arg() {
    test_signal_with_arg_value("i16_ref",
        |obj| obj.signal_i16_ref(&i16::MIN),
        |var| var.value::<i16>() == Some(i16::MIN));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u16_ref_arg() {
    test_signal_with_arg_value("u16_ref",
        |obj| obj.signal_u16_ref(&u16::MIN),
        |var| var.value::<u16>() == Some(u16::MIN));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i32_ref_arg() {
    test_signal_with_arg_value("i32_ref",
        |obj| obj.signal_i32_ref(&i32::MIN),
        |var| var.value::<i32>() == Some(i32::MIN));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u32_ref_arg() {
    test_signal_with_arg_value("u32_ref",
        |obj| obj.signal_u32_ref(&u32::MAX),
        |var| var.value::<u32>() == Some(u32::MAX));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i64_ref_arg() {
    test_signal_with_arg_value("i64_ref",
        |obj| obj.signal_i64_ref(&MIN_SAFE_INTEGER),
        |var| var.value::<i64>() == Some(MIN_SAFE_INTEGER));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u64_ref_arg() {
    test_signal_with_arg_value("u64_ref",
        |obj| obj.signal_u64_ref(&MAX_SAFE_INTEGER),
        |var| var.value::<u64>() == Some(MAX_SAFE_INTEGER));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_isize_ref_arg() {
    test_signal_with_arg_value("isize_ref",
        |obj| obj.signal_isize_ref(&(MIN_SAFE_INTEGER as isize)),
        |var| var.value::<i64>() == Some(MIN_SAFE_INTEGER));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_usize_ref_arg() {
    test_signal_with_arg_value("usize_ref",
        |obj| obj.signal_usize_ref(&(MAX_SAFE_INTEGER as usize)),
        |var| var.value::<u64>() == Some(MAX_SAFE_INTEGER));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f32_ref_arg() {
    test_signal_with_arg_value("f32_ref",
        |obj| obj.signal_f32_ref(&0.5),
        |var| var.value::<f32>() == Some(0.5));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f64_ref_arg() {
    test_signal_with_arg_value("f64_ref",
        |obj| obj.signal_f64_ref(&0.25),
        |var| var.value::<f64>() == Some(0.25));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_a_few_arguments_ref_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(unsafe { &*obj.borrow().get_qobject_ptr() }, "signal_many_args");
    obj.borrow_mut().signal_many_args(&"123".to_owned(), 700, 0.75, &0.125, "Café".to_owned(), 65535, false, &true, -100000, -10000000000);
    assert_eq!(spy.count(), 1);
    let arg_list = spy.pin_mut().take_first();
    assert_eq!(arg_list.len(), 10);
    let args: [QVariant; 10] = std::array::from_fn(
        |i| arg_list.get(i as isize).unwrap().clone());

    assert_eq!(Some("123".into()), args[0].value::<QString>());
    assert_eq!(Some(700), args[1].value::<i32>());
    assert_eq!(Some(0.75), args[2].value::<f32>());
    assert_eq!(Some(0.125), args[3].value::<f64>());
    assert_eq!(Some("Café".into()), args[4].value::<QString>());
    assert_eq!(Some(65535), args[5].value::<u16>());
    assert_eq!(Some(false), args[6].value::<bool>());
    assert_eq!(Some(true), args[7].value::<bool>());
    assert_eq!(Some(-100000), args[8].value::<i32>());
    assert_eq!(Some(-10000000000), args[9].value::<i64>());
}
