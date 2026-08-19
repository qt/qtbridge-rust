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
    assert!(check_fn(&QVariant::from_cxx_qt(args.get(0).unwrap())));
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_bool_arg() {
    test_signal_with_arg_value("bool",
        |obj| obj.signal_bool(true),
        |var| {
            let value: bool = var.try_into().unwrap();
            value
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i8_arg() {
    test_signal_with_arg_value("i8",
        |obj| obj.signal_i8(42),
        |var| {
            let arg: i8 = var.try_into().unwrap();
            arg == 42
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u8_arg() {
    test_signal_with_arg_value("u8",
        |obj| obj.signal_u8(43),
        |var| {
            let arg: u8 = var.try_into().unwrap();
            arg == 43
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i16_arg() {
    test_signal_with_arg_value("i16",
        |obj| obj.signal_i16(-44),
        |var| {
            let arg: i16 = var.try_into().unwrap();
            arg == -44
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u16_arg() {
    test_signal_with_arg_value("u16",
        |obj| obj.signal_u16(45),
        |var| {
            let arg: u16 = var.try_into().unwrap();
            arg == 45
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i32_arg() {
    test_signal_with_arg_value("i32",
        |obj| obj.signal_i32(46),
        |var| {
            let arg: i32 = var.try_into().unwrap();
            arg == 46
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u32_arg() {
    test_signal_with_arg_value("u32",
        |obj| obj.signal_u32(47),
        |var| {
            let arg: u32 = var.try_into().unwrap();
            arg == 47
        });

}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i64_arg() {
    test_signal_with_arg_value("i64",
        |obj| obj.signal_i64(48),
        |var| {
            let arg: i64 = var.try_into().unwrap();
            arg == 48
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u64_arg() {
    test_signal_with_arg_value("u64",
        |obj| obj.signal_u64(49),
        |var| {
            let arg: u64 = var.try_into().unwrap();
            arg == 49
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_isize_arg() {
    test_signal_with_arg_value("isize",
        |obj| obj.signal_isize(-50),
        |var| {
            let arg: isize = var.try_into().unwrap();
            arg == -50
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_usize_arg() {
    test_signal_with_arg_value("usize",
        |obj| obj.signal_usize(51),
        |var| {
            let arg: usize = var.try_into().unwrap();
            arg == 51
        });
}


#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f32_arg() {
    test_signal_with_arg_value("f32",
        |obj| obj.signal_f32(0.5),
        |var| {
            let arg: f32 = var.try_into().unwrap();
            arg == 0.5
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f64_arg() {
    test_signal_with_arg_value("f64",
        |obj| obj.signal_f64(0.25),
        |var| {
            let arg: f64 = var.try_into().unwrap();
            arg == 0.25
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_string_arg() {
    test_signal_with_arg_value("string",
        |obj| obj.signal_string("ABC".to_owned()),
        |var| {
            let arg: String = var.try_into().unwrap();
            arg == "ABC"
        });
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
        |var| {
            let value: bool = var.try_into().unwrap();
            value
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i8_ref_arg() {
    test_signal_with_arg_value("i8_ref",
        |obj| obj.signal_i8_ref(&i8::MIN),
        |var| {
            let value: i8 = var.try_into().unwrap();
            value == i8::MIN
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u8_ref_arg() {
    test_signal_with_arg_value("u8_ref",
        |obj| obj.signal_u8_ref(&u8::MAX),
        |var| {
            let value: u8 = var.try_into().unwrap();
            value == u8::MAX
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i16_ref_arg() {
    test_signal_with_arg_value("i16_ref",
        |obj| obj.signal_i16_ref(&i16::MIN),
        |var| {
            let value: i16 = var.try_into().unwrap();
            value == i16::MIN
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u16_ref_arg() {
    test_signal_with_arg_value("u16_ref",
        |obj| obj.signal_u16_ref(&u16::MIN),
        |var| {
            let value: u16 = var.try_into().unwrap();
            value == u16::MIN
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i32_ref_arg() {
    test_signal_with_arg_value("i32_ref",
        |obj| obj.signal_i32_ref(&i32::MIN),
        |var| {
            let value: i32 = var.try_into().unwrap();
            value == i32::MIN
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u32_ref_arg() {
    test_signal_with_arg_value("u32_ref",
        |obj| obj.signal_u32_ref(&u32::MAX),
        |var| {
            let value: u32 = var.try_into().unwrap();
            value == u32::MAX
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_i64_ref_arg() {
    test_signal_with_arg_value("i64_ref",
        |obj| obj.signal_i64_ref(&MIN_SAFE_INTEGER),
        |var| {
            let value: i64 = var.try_into().unwrap();
            value == MIN_SAFE_INTEGER
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_u64_ref_arg() {
    test_signal_with_arg_value("u64_ref",
        |obj| obj.signal_u64_ref(&MAX_SAFE_INTEGER),
        |var| {
            let value: u64 = var.try_into().unwrap();
            value == MAX_SAFE_INTEGER
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_isize_ref_arg() {
    test_signal_with_arg_value("isize_ref",
        |obj| obj.signal_isize_ref(&(MIN_SAFE_INTEGER as isize)),
        |var| {
            let value: isize = var.try_into().unwrap();
            value == MIN_SAFE_INTEGER as isize
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_usize_ref_arg() {
    test_signal_with_arg_value("usize_ref",
        |obj| obj.signal_usize_ref(&(MAX_SAFE_INTEGER as usize)),
        |var| {
            let value: usize = var.try_into().unwrap();
            value == MAX_SAFE_INTEGER as usize
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f32_ref_arg() {
    test_signal_with_arg_value("f32_ref",
        |obj| obj.signal_f32_ref(&0.5),
        |var| {
            let value: f32 = var.try_into().unwrap();
            value == 0.5
        });
}

#[test]
#[cfg(not(miri))]
fn signal_is_emitted_when_called_with_f64_ref_arg() {
    test_signal_with_arg_value("f64_ref",
        |obj| obj.signal_f64_ref(&0.25),
        |var| {
            let value: f64 = var.try_into().unwrap();
            value == 0.25
        });
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
        |i| QVariant::from_cxx_qt(arg_list.get(i as isize).unwrap()));

    assert_eq!(Ok("123".into()), TryInto::<String>::try_into(&args[0]));
    assert_eq!(Ok(700), TryInto::<i32>::try_into(&args[1]));
    assert_eq!(Ok(0.75), TryInto::<f32>::try_into(&args[2]));
    assert_eq!(Ok(0.125), TryInto::<f64>::try_into(&args[3]));
    assert_eq!(Ok("Café".into()), TryInto::<String>::try_into(&args[4]));
    assert_eq!(Ok(65535), TryInto::<u16>::try_into(&args[5]));
    assert_eq!(Ok(false), TryInto::<bool>::try_into(&args[6]));
    assert_eq!(Ok(true), TryInto::<bool>::try_into(&args[7]));
    assert_eq!(Ok(-100000), TryInto::<i32>::try_into(&args[8]));
    assert_eq!(Ok(-10000000000), TryInto::<i64>::try_into(&args[9]));
}
