// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, qobject};
use qtbridge::qtbridge_type_lib::QVariant;

use crate::common::{capitalize_first_char, get_type_name};

trait TestObjHelper: QObjectHolder {
    fn property_type() -> &'static str;
    fn create_default() -> Rc<RefCell<Self>> {
        Self::create_with_values(TestValues::default())
    }
    fn create_with_values(values: TestValues) -> Rc<RefCell<Self>>;
}

#[derive(Clone, Default)]
struct TestValues {
    pub bool: bool,
    pub i8: i8,
    pub u8: u8,
    pub i16: i16,
    pub u16: u16,
    pub i32: i32,
    pub u32: u32,
    pub i64: i64,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub isize: isize,
    pub usize: usize,
    pub string: String,
    pub string_list: Vec<String>,
}


#[qobject]
pub mod member_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub bool: bool,
        pub i8: i8,
        pub u8: u8,
        pub i16: i16,
        pub u16: u16,
        pub i32: i32,
        pub u32: u32,
        pub i64: i64,
        pub u64: u64,
        pub f32: f32,
        pub f64: f64,
        pub isize: isize,
        pub usize: usize,
        pub string: String,
        pub string_list: Vec<String>,
    }

    impl TestObject {
        qproperty!("propertyBool", Member = bool);
        qproperty!("propertyI8", Member = i8);
        qproperty!("propertyU8", Member = u8);
        qproperty!("propertyI16", Member = i16);
        qproperty!("propertyU16", Member = u16);
        qproperty!("propertyI32", Member = i32);
        qproperty!("propertyU32", Member = u32);
        qproperty!("propertyI64", Member = i64);
        qproperty!("propertyU64", Member = u64);
        qproperty!("propertyIsize", Member = isize);
        qproperty!("propertyUsize", Member = usize);
        qproperty!("propertyF32", Member = f32);
        qproperty!("propertyF64", Member = f64);
        qproperty!("propertyString", Member = string);
        qproperty!("propertyVecString", Member = string_list);
    }

    impl From<TestValues> for TestObject {
        fn from(src: TestValues) -> Self {
            let TestValues {
                bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string, string_list
            } = src;
            Self {
                bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string, string_list
            }
        }
    }

    impl From<&TestObject> for TestValues {
        fn from(src: &TestObject) -> Self {
            Self {
                bool: src.bool,
                i8: src.i8,
                u8: src.u8,
                i16: src.i16,
                u16: src.u16,
                i32 :src.i32,
                u32: src.u32,
                i64: src.i64,
                u64: src.u64,
                isize: src.isize,
                usize: src.usize,
                f32: src.f32,
                f64: src.f64,
                string: src.string.clone(),
                string_list: src.string_list.clone(),
            }
        }
    }

    impl TestObjHelper for TestObject {
        fn property_type() -> &'static str {
            "member based properties"
        }
        fn create_with_values(values: TestValues) -> std::rc::Rc<std::cell::RefCell<Self>>{
            let obj = std::rc::Rc::new(std::cell::RefCell::new(Self::from(values)));
            <Self as super::QObjectHolder>::attach_qobject(&obj);
            obj
        }
    }
}

#[qobject]
mod accessor_value_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub values: super::TestValues
    }

    impl TestObject {
        qproperty!("propertyBool", Read = bool_get, Write = bool_set);
        qproperty!("propertyI8", Read = i8_get, Write = i8_set);
        qproperty!("propertyU8", Read = u8_get, Write = u8_set);
        qproperty!("propertyI16", Read = i16_get, Write = i16_set);
        qproperty!("propertyU16", Read = u16_get, Write = u16_set);
        qproperty!("propertyI32", Read = i32_get, Write = i32_set);
        qproperty!("propertyU32", Read = u32_get, Write = u32_set);
        qproperty!("propertyI64", Read = i64_get, Write = i64_set);
        qproperty!("propertyU64", Read = u64_get, Write = u64_set);
        qproperty!("propertyIsize", Read = isize_get, Write = isize_set);
        qproperty!("propertyUsize", Read = usize_get, Write = usize_set);
        qproperty!("propertyF32", Read = f32_get, Write = f32_set);
        qproperty!("propertyF64", Read = f64_get, Write = f64_set);
        qproperty!("propertyString", Read = string_get, Write = string_set);
        qproperty!("propertyVecString", Read = string_list_get, Write = string_list_set);

        fn bool_get(&self) -> bool {
            self.values.bool
        }
        fn bool_set(&mut self, new: bool) {
            self.values.bool = new;
        }

        fn i8_get(&self) -> i8 {
            self.values.i8
        }
        fn i8_set(&mut self, new: i8) {
            self.values.i8 = new;
        }

        fn u8_get(&self) -> u8 {
            self.values.u8
        }
        fn u8_set(&mut self, new: u8) {
            self.values.u8 = new;
        }

        fn i16_get(&self) -> i16 {
            self.values.i16
        }
        fn i16_set(&mut self, new: i16) {
            self.values.i16 = new;
        }

        fn u16_get(&self) -> u16 {
            self.values.u16
        }
        fn u16_set(&mut self, new: u16) {
            self.values.u16 = new;
        }

        fn i32_get(&self) -> i32 {
            self.values.i32
        }
        fn i32_set(&mut self, new: i32) {
            self.values.i32 = new;
        }

        fn u32_get(&self) -> u32 {
            self.values.u32
        }
        fn u32_set(&mut self, new: u32) {
            self.values.u32 = new;
        }

        fn i64_get(&self) -> i64 {
            self.values.i64
        }
        fn i64_set(&mut self, new: i64) {
            self.values.i64 = new;
        }

        fn u64_get(&self) -> u64 {
            self.values.u64
        }
        fn u64_set(&mut self, new: u64) {
            self.values.u64 = new;
        }

        fn isize_get(&self) -> isize {
            self.values.isize
        }
        fn isize_set(&mut self, new: isize) {
            self.values.isize = new;
        }

        fn usize_get(&self) -> usize {
            self.values.usize
        }
        fn usize_set(&mut self, new: usize) {
            self.values.usize = new;
        }

        fn f32_get(&self) -> f32 {
            self.values.f32
        }
        fn f32_set(&mut self, new: f32) {
            self.values.f32 = new;
        }

        fn f64_get(&self) -> f64 {
            self.values.f64
        }
        fn f64_set(&mut self, new: f64) {
            self.values.f64 = new;
        }

        fn string_get(&self) -> String {
            self.values.string.clone()
        }
        fn string_set(&mut self, new: String){
            self.values.string = new;
        }

        fn string_list_get(&self) -> Vec<String> {
            self.values.string_list.clone()
        }
        fn string_list_set(&mut self, new: Vec<String>) {
            self.values.string_list = new;
        }
    }

    impl TestObjHelper for TestObject {
        fn property_type() -> &'static str {
            "accessor value based properties"
        }

        fn create_with_values(values: TestValues) -> std::rc::Rc<std::cell::RefCell<Self>>{
            let obj = std::rc::Rc::new(std::cell::RefCell::new(Self::from(values)));
            <Self as super::QObjectHolder>::attach_qobject(&obj);
            obj
        }
    }

    impl From<TestValues> for TestObject {
        fn from(from: TestValues) -> Self {
            Self {
                values: from
            }
        }
    }

    impl From<&TestObject> for TestValues {
        fn from(src: &TestObject) -> Self {
            src.values.clone()
        }
    }
}

#[qobject]
mod accessor_reference_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub values: super::TestValues
    }

    impl TestObject {
        qproperty!("propertyBool", Read = bool_get, Write = bool_set);
        qproperty!("propertyI8", Read = i8_get, Write = i8_set);
        qproperty!("propertyU8", Read = u8_get, Write = u8_set);
        qproperty!("propertyI16", Read = i16_get, Write = i16_set);
        qproperty!("propertyU16", Read = u16_get, Write = u16_set);
        qproperty!("propertyI32", Read = i32_get, Write = i32_set);
        qproperty!("propertyU32", Read = u32_get, Write = u32_set);
        qproperty!("propertyI64", Read = i64_get, Write = i64_set);
        qproperty!("propertyU64", Read = u64_get, Write = u64_set);
        qproperty!("propertyIsize", Read = isize_get, Write = isize_set);
        qproperty!("propertyUsize", Read = usize_get, Write = usize_set);
        qproperty!("propertyF32", Read = f32_get, Write = f32_set);
        qproperty!("propertyF64", Read = f64_get, Write = f64_set);
        qproperty!("propertyString", Read = string_get, Write = string_set);
        qproperty!("propertyVecString", Read = string_list_get, Write = string_list_set);

        fn bool_get(&self) -> &bool {
            &self.values.bool
        }
        fn bool_set(&mut self, new: &bool) {
            self.values.bool = *new;
        }

        fn i8_get(&self) -> &i8 {
            &self.values.i8
        }
        fn i8_set(&mut self, new: &i8) {
            self.values.i8 = *new;
        }

        fn u8_get(&self) -> &u8 {
            &self.values.u8
        }
        fn u8_set(&mut self, new: &u8) {
            self.values.u8 = *new;
        }

        fn i16_get(&self) -> &i16 {
            &self.values.i16
        }
        fn i16_set(&mut self, new: &i16) {
            self.values.i16 = *new;
        }

        fn u16_get(&self) -> &u16 {
            &self.values.u16
        }
        fn u16_set(&mut self, new: &u16) {
            self.values.u16 = *new;
        }

        fn i32_get(&self) -> &i32 {
            &self.values.i32
        }
        fn i32_set(&mut self, new: &i32) {
            self.values.i32 = *new;
        }

        fn u32_get(&self) -> &u32 {
            &self.values.u32
        }
        fn u32_set(&mut self, new: &u32) {
            self.values.u32 = *new;
        }

        fn i64_get(&self) -> &i64 {
            &self.values.i64
        }
        fn i64_set(&mut self, new: &i64) {
            self.values.i64 = *new;
        }

        fn u64_get(&self) -> &u64 {
            &self.values.u64
        }
        fn u64_set(&mut self, new: &u64) {
            self.values.u64 = *new;
        }

        fn isize_get(&self) -> &isize {
            &self.values.isize
        }
        fn isize_set(&mut self, new: &isize) {
            self.values.isize = *new;
        }

        fn usize_get(&self) -> &usize {
            &self.values.usize
        }
        fn usize_set(&mut self, new: &usize) {
            self.values.usize = *new;
        }

        fn f32_get(&self) -> &f32 {
            &self.values.f32
        }
        fn f32_set(&mut self, new: &f32) {
            self.values.f32 = *new;
        }

        fn f64_get(&self) -> &f64 {
            &self.values.f64
        }
        fn f64_set(&mut self, new: &f64) {
            self.values.f64 = *new;
        }

        fn string_get(&self) -> &String {
            &self.values.string
        }
        fn string_set(&mut self, new: &String){
            self.values.string = new.clone();
        }

        fn string_list_get(&self) -> &Vec<String> {
            &self.values.string_list
        }
        fn string_list_set(&mut self, new: &Vec<String>) {
            self.values.string_list = new.clone();
        }
    }

    impl TestObjHelper for TestObject {
        fn property_type() -> &'static str {
            "accessor reference based properties"
        }

        fn create_with_values(values: TestValues) -> std::rc::Rc<std::cell::RefCell<Self>>{
            let obj = std::rc::Rc::new(std::cell::RefCell::new(Self::from(values)));
            <Self as super::QObjectHolder>::attach_qobject(&obj);
            obj
        }
    }

    impl From<TestValues> for TestObject {
        fn from(from: TestValues) -> Self {
            Self {
                values: from
            }
        }
    }

    impl From<&TestObject> for TestValues {
        fn from(src: &TestObject) -> Self {
            src.values.clone()
        }
    }
}

fn test_property_can_be_read<TestObj, T>(values: TestValues, expected: T)
where
    TestObj: TestObjHelper,
    T: std::fmt::Debug + PartialEq + TryFrom<QVariant>,
    <T as TryFrom<QVariant>>::Error: std::fmt::Debug,
{
    let obj = TestObj::create_with_values(values);

    // Read the value of the tested property and compare it to the expected one.
    let type_name = get_type_name::<T>();
    let property_name = format!("property{}", capitalize_first_char(&type_name));
    let var = unsafe { &*obj.borrow().get_qobject_ptr() }.property(&property_name);
    let actual: T = var.try_into().unwrap();
    let property_type = TestObj::property_type();
    assert_eq!(actual, expected, "check failed for type {type_name} of {property_type}");
}

fn test_property_can_be_written<TestObj, T, GetValueFieldFn>(test_value: T, get_value: GetValueFieldFn)
where
    TestObj: TestObjHelper,
    TestValues: for<'a> From<&'a TestObj>,
    T: std::fmt::Debug + PartialEq,
    QVariant: for<'a> From<&'a T>,
    GetValueFieldFn: FnOnce(&TestValues) -> T,
{
    let obj = TestObj::create_default();

    // Write the value to the tested property and compare it to value returned from getter functor.
    let type_name = get_type_name::<T>();
    let property_name = format!("property{}", capitalize_first_char(&type_name));
    let qobj_ptr = obj.borrow().get_qobject_ptr();
    let qobj = unsafe { qobj_ptr.as_mut() }.unwrap();
    qobj.set_property(&property_name, (&test_value).into());
    let values = TestValues::from(&obj.borrow());
    let property_type = TestObj::property_type();
    assert_eq!(test_value, get_value(&values), "check failed for type {type_name} of {property_type}");
}

fn test_cases_property_can_be_read<TestObj>() -> Vec<fn()>
where
    TestObj: TestObjHelper,
{
    vec![
        || test_property_can_be_read::<TestObj, bool>(
                TestValues { bool: true, ..<_>::default() },
                true),
        || test_property_can_be_read::<TestObj, i8>(
                TestValues { i8: 101, ..<_>::default() },
                101),
        || test_property_can_be_read::<TestObj, u8>(
                TestValues { u8: 102, ..<_>::default() },
                102),
        || test_property_can_be_read::<TestObj, i16>(
                TestValues { i16: -103, ..<_>::default() },
                -103),
        || test_property_can_be_read::<TestObj, u16>(
                TestValues { u16: 104, ..<_>::default() },
                104),
        || test_property_can_be_read::<TestObj, i32>(
                TestValues { i32: -105, ..<_>::default() },
                -105),
        || test_property_can_be_read::<TestObj, u32>(
                TestValues { u32: 106, ..<_>::default() },
                106),
        || test_property_can_be_read::<TestObj, i64>(
                TestValues { i64: -107, ..<_>::default() },
                -107),
        || test_property_can_be_read::<TestObj, u64>(
                TestValues { u64: 108, ..<_>::default() },
                108),
        || test_property_can_be_read::<TestObj, isize>(
                TestValues { isize: -109, ..<_>::default() },
                -109),
        || test_property_can_be_read::<TestObj, usize>(
                TestValues { usize: 110, ..<_>::default() },
                110),
        || test_property_can_be_read::<TestObj, f32>(
                TestValues { f32: 0.5, ..<_>::default() },
                0.5),
        || test_property_can_be_read::<TestObj, f64>(
                TestValues { f64: 0.25, ..<_>::default() },
                0.25),
        || test_property_can_be_read::<TestObj, String>(
                TestValues { string: "こんにちは".into(), ..<_>::default() },
                "こんにちは".into()),
    ]
}

fn test_cases_property_can_be_written<TestObj>() -> Vec<fn()>
where
    TestObj: TestObjHelper,
    TestValues: for<'a> From<&'a TestObj>
{
    vec![
        || test_property_can_be_written::<TestObj, _, _>(true, |values| values.bool),
        || test_property_can_be_written::<TestObj, _, _>(i8::MIN, |values| values.i8),
        || test_property_can_be_written::<TestObj, _, _>(u8::MAX, |values| values.u8),
        || test_property_can_be_written::<TestObj, _, _>(i16::MIN, |values| values.i16),
        || test_property_can_be_written::<TestObj, _, _>(u16::MAX, |values| values.u16),
        || test_property_can_be_written::<TestObj, _, _>(i32::MIN, |values| values.i32),
        || test_property_can_be_written::<TestObj, _, _>(u32::MAX, |values| values.u32),
        || test_property_can_be_written::<TestObj, _, _>(i64::MIN, |values| values.i64),
        || test_property_can_be_written::<TestObj, _, _>(u64::MIN, |values| values.u64),
        || test_property_can_be_written::<TestObj, _, _>(isize::MIN, |values| values.isize),
        || test_property_can_be_written::<TestObj, _, _>(usize::MIN, |values| values.usize),
        || test_property_can_be_written::<TestObj, _, _>(0.25, |values| values.f32),
        || test_property_can_be_written::<TestObj, _, _>(0.125, |values| values.f64),
        || test_property_can_be_written::<TestObj, _, _>("Привіт, світе!".to_string(), |values| values.string.clone()),
    ]
}

#[test]
#[cfg(not(miri))]
fn qproperty_member_based_can_be_read() {
    test_cases_property_can_be_read::<member_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_value_based_can_be_read() {
    test_cases_property_can_be_read::<accessor_value_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_reference_based_can_be_read() {
    test_cases_property_can_be_read::<accessor_reference_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}


#[test]
#[cfg(not(miri))]
fn qproperty_member_based_can_be_written() {
    test_cases_property_can_be_written::<member_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_value_based_can_be_written() {
    test_cases_property_can_be_written::<accessor_value_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_reference_based_can_be_written() {
    test_cases_property_can_be_written::<accessor_reference_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}


