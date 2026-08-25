// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use qtbridge::{QObjectHolder, qobject};
use qtbridge_type_lib::{QList, QListElement, QString, QVariant, QVariantValue};

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
    pub bool: Vec<bool>,
    pub i8: Vec<i8>,
    pub u8: Vec<u8>,
    pub i16: Vec<i16>,
    pub u16: Vec<u16>,
    pub i32: Vec<i32>,
    pub u32: Vec<u32>,
    pub i64: Vec<i64>,
    pub u64: Vec<u64>,
    pub isize: Vec<isize>,
    pub usize: Vec<usize>,
    pub f32: Vec<f32>,
    pub f64: Vec<f64>,
    pub string: Vec<String>,
}

#[qobject(ConvertToCamelCase)]
pub mod member_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub bool: Vec<bool>,
        pub i8: Vec<i8>,
        pub u8: Vec<u8>,
        pub i16: Vec<i16>,
        pub u16: Vec<u16>,
        pub i32: Vec<i32>,
        pub u32: Vec<u32>,
        pub i64: Vec<i64>,
        pub u64: Vec<u64>,
        pub isize: Vec<isize>,
        pub usize: Vec<usize>,
        pub f32: Vec<f32>,
        pub f64: Vec<f64>,
        pub string: Vec<String>,
    }

    impl TestObject {
        qproperty!("propertyVecBool", Member = bool);
        qproperty!("propertyVecI8", Member = i8);
        qproperty!("propertyVecU8", Member = u8);
        qproperty!("propertyVecI16", Member = i16);
        qproperty!("propertyVecU16", Member = u16);
        qproperty!("propertyVecI32", Member = i32);
        qproperty!("propertyVecU32", Member = u32);
        qproperty!("propertyVecI64", Member = i64);
        qproperty!("propertyVecU64", Member = u64);
        qproperty!("propertyVecIsize", Member = isize);
        qproperty!("propertyVecUsize", Member = usize);
        qproperty!("propertyVecF32", Member = f32);
        qproperty!("propertyVecF64", Member = f64);
        qproperty!("propertyVecString", Member = string);
    }

    impl From<TestValues> for TestObject {
        fn from(src: TestValues) -> Self {
            let TestValues {
                bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string
            } = src;
            Self {
                bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string
            }
        }
    }

    impl From<&TestObject> for TestValues {
        fn from(src: &TestObject) -> Self {
            Self {
                bool: src.bool.clone(),
                i8: src.i8.clone(),
                u8: src.u8.clone(),
                i16: src.i16.clone(),
                u16: src.u16.clone(),
                i32 :src.i32.clone(),
                u32: src.u32.clone(),
                i64: src.i64.clone(),
                u64: src.u64.clone(),
                isize: src.isize.clone(),
                usize: src.usize.clone(),
                f32: src.f32.clone(),
                f64: src.f64.clone(),
                string: src.string.clone(),
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

#[qobject(ConvertToCamelCase)]
mod accessor_value_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub values: super::TestValues
    }

    impl TestObject {
        qproperty!("propertyVecBool", Read = bool_get, Write = bool_set);
        qproperty!("propertyVecI8", Read = i8_get, Write = i8_set);
        qproperty!("propertyVecU8", Read = u8_get, Write = u8_set);
        qproperty!("propertyVecI16", Read = i16_get, Write = i16_set);
        qproperty!("propertyVecU16", Read = u16_get, Write = u16_set);
        qproperty!("propertyVecI32", Read = i32_get, Write = i32_set);
        qproperty!("propertyVecU32", Read = u32_get, Write = u32_set);
        qproperty!("propertyVecI64", Read = i64_get, Write = i64_set);
        qproperty!("propertyVecU64", Read = u64_get, Write = u64_set);
        qproperty!("propertyVecIsize", Read = isize_get, Write = isize_set);
        qproperty!("propertyVecUsize", Read = usize_get, Write = usize_set);
        qproperty!("propertyVecF32", Read = f32_get, Write = f32_set);
        qproperty!("propertyVecF64", Read = f64_get, Write = f64_set);
        qproperty!("propertyVecString", Read = string_get, Write = string_set);

        fn bool_get(&self) -> Vec<bool> {
            self.values.bool.clone()
        }
        fn bool_set(&mut self, new: Vec<bool>) {
            self.values.bool = new;
        }

        fn i8_get(&self) -> Vec<i8> {
            self.values.i8.clone()
        }
        fn i8_set(&mut self, new: Vec<i8>) {
            self.values.i8 = new;
        }

        fn u8_get(&self) -> Vec<u8> {
            self.values.u8.clone()
        }
        fn u8_set(&mut self, new: Vec<u8>) {
            self.values.u8 = new.clone();
        }

        fn i16_get(&self) -> Vec<i16> {
            self.values.i16.clone()
        }
        fn i16_set(&mut self, new: Vec<i16>) {
            self.values.i16 = new;
        }

        fn u16_get(&self) -> Vec<u16> {
            self.values.u16.clone()
        }
        fn u16_set(&mut self, new: Vec<u16>) {
            self.values.u16 = new;
        }

        fn i32_get(&self) -> Vec<i32> {
            self.values.i32.clone()
        }
        fn i32_set(&mut self, new: Vec<i32>) {
            self.values.i32 = new;
        }

        fn u32_get(&self) -> Vec<u32> {
            self.values.u32.clone()
        }
        fn u32_set(&mut self, new: Vec<u32>) {
            self.values.u32 = new;
        }

        fn i64_get(&self) -> Vec<i64> {
            self.values.i64.clone()
        }
        fn i64_set(&mut self, new: Vec<i64>) {
            self.values.i64 = new;
        }

        fn u64_get(&self) -> Vec<u64> {
            self.values.u64.clone()
        }
        fn u64_set(&mut self, new: Vec<u64>) {
            self.values.u64 = new;
        }

        fn isize_get(&self) -> Vec<isize> {
            self.values.isize.clone()
        }
        fn isize_set(&mut self, new: Vec<isize>) {
            self.values.isize = new;
        }

        fn usize_get(&self) -> Vec<usize> {
            self.values.usize.clone()
        }
        fn usize_set(&mut self, new: Vec<usize>) {
            self.values.usize = new;
        }

        fn f32_get(&self) -> Vec<f32> {
            self.values.f32.clone()
        }
        fn f32_set(&mut self, new: Vec<f32>) {
            self.values.f32 = new;
        }

        fn f64_get(&self) -> Vec<f64> {
            self.values.f64.clone()
        }
        fn f64_set(&mut self, new: Vec<f64>) {
            self.values.f64 = new;
        }

        fn string_get(&self) -> Vec<String> {
            self.values.string.clone()
        }
        fn string_set(&mut self, new: Vec<String>){
            self.values.string = new;
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

#[qobject(ConvertToCamelCase)]
mod accessor_reference_properties {
    use super::{TestObjHelper, TestValues};

    #[derive(Default)]
    pub struct TestObject {
        pub values: super::TestValues
    }

    impl TestObject {
        qproperty!("propertyVecBool", Read = bool_get, Write = bool_set);
        qproperty!("propertyVecI8", Read = i8_get, Write = i8_set);
        qproperty!("propertyVecU8", Read = u8_get, Write = u8_set);
        qproperty!("propertyVecI16", Read = i16_get, Write = i16_set);
        qproperty!("propertyVecU16", Read = u16_get, Write = u16_set);
        qproperty!("propertyVecI32", Read = i32_get, Write = i32_set);
        qproperty!("propertyVecU32", Read = u32_get, Write = u32_set);
        qproperty!("propertyVecI64", Read = i64_get, Write = i64_set);
        qproperty!("propertyVecU64", Read = u64_get, Write = u64_set);

        qproperty!("propertyVecIsize", Read = isize_get, Write = isize_set);
        qproperty!("propertyVecUsize", Read = usize_get, Write = usize_set);

        qproperty!("propertyVecF32", Read = f32_get, Write = f32_set);
        qproperty!("propertyVecF64", Read = f64_get, Write = f64_set);
        qproperty!("propertyVecString", Read = string_get, Write = string_set);

        fn bool_get(&self) -> &Vec<bool> {
            &self.values.bool
        }
        fn bool_set(&mut self, new: &Vec<bool>) {
            self.values.bool = new.clone();
        }

        fn i8_get(&self) -> &Vec<i8> {
            &self.values.i8
        }
        fn i8_set(&mut self, new: &Vec<i8>) {
            self.values.i8 = new.clone();
        }

        fn u8_get(&self) -> &Vec<u8> {
            &self.values.u8
        }
        fn u8_set(&mut self, new: &Vec<u8>) {
            self.values.u8 = new.clone();
        }

        fn i16_get(&self) -> &Vec<i16> {
            &self.values.i16
        }
        fn i16_set(&mut self, new: &Vec<i16>) {
            self.values.i16 = new.clone();
        }

        fn u16_get(&self) -> &Vec<u16> {
            &self.values.u16
        }
        fn u16_set(&mut self, new: &Vec<u16>) {
            self.values.u16 = new.clone();
        }

        fn i32_get(&self) -> &Vec<i32> {
            &self.values.i32
        }
        fn i32_set(&mut self, new: &Vec<i32>) {
            self.values.i32 = new.clone();
        }

        fn u32_get(&self) -> &Vec<u32> {
            &self.values.u32
        }
        fn u32_set(&mut self, new: &Vec<u32>) {
            self.values.u32 = new.clone();
        }

        fn i64_get(&self) -> &Vec<i64> {
            &self.values.i64
        }
        fn i64_set(&mut self, new: &Vec<i64>) {
            self.values.i64 = new.clone();
        }

        fn u64_get(&self) -> &Vec<u64> {
            &self.values.u64
        }
        fn u64_set(&mut self, new: &Vec<u64>) {
            self.values.u64 = new.clone();
        }


        fn isize_get(&self) -> &Vec<isize> {
            &self.values.isize
        }
        fn isize_set(&mut self, new: &Vec<isize>) {
            self.values.isize = new.clone();
        }

        fn usize_get(&self) -> &Vec<usize> {
            &self.values.usize
        }
        fn usize_set(&mut self, new: &Vec<usize>) {
            self.values.usize = new.clone();
        }

        fn f32_get(&self) -> &Vec<f32> {
            &self.values.f32
        }
        fn f32_set(&mut self, new: &Vec<f32>) {
            self.values.f32 = new.clone();
        }

        fn f64_get(&self) -> &Vec<f64> {
            &self.values.f64
        }
        fn f64_set(&mut self, new: &Vec<f64>) {
            self.values.f64 = new.clone();
        }

        fn string_get(&self) -> &Vec<String> {
            &self.values.string
        }
        fn string_set(&mut self, new: &Vec<String>){
            self.values.string = new.clone();
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


// #[qobject] impl variant — struct defined outside the macro so the macro cannot see its fields.
// Member = <field> must resolve via QPropertyMember trait dispatch.
#[derive(Default)]
pub struct TestObjectImpl {
    pub bool: Vec<bool>,
    pub i8: Vec<i8>,
    pub u8: Vec<u8>,
    pub i16: Vec<i16>,
    pub u16: Vec<u16>,
    pub i32: Vec<i32>,
    pub u32: Vec<u32>,
    pub i64: Vec<i64>,
    pub u64: Vec<u64>,
    pub isize: Vec<isize>,
    pub usize: Vec<usize>,
    pub f32: Vec<f32>,
    pub f64: Vec<f64>,
    pub string: Vec<String>,
}

#[qobject(ConvertToCamelCase)]
impl TestObjectImpl {
    qproperty!("propertyVecBool", Member = bool);
    qproperty!("propertyVecI8", Member = i8);
    qproperty!("propertyVecU8", Member = u8);
    qproperty!("propertyVecI16", Member = i16);
    qproperty!("propertyVecU16", Member = u16);
    qproperty!("propertyVecI32", Member = i32);
    qproperty!("propertyVecU32", Member = u32);
    qproperty!("propertyVecI64", Member = i64);
    qproperty!("propertyVecU64", Member = u64);
    qproperty!("propertyVecIsize", Member = isize);
    qproperty!("propertyVecUsize", Member = usize);
    qproperty!("propertyVecF32", Member = f32);
    qproperty!("propertyVecF64", Member = f64);
    qproperty!("propertyVecString", Member = string);
}

impl From<TestValues> for TestObjectImpl {
    fn from(src: TestValues) -> Self {
        let TestValues {
            bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string
        } = src;
        Self { bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, string }
    }
}

impl From<&TestObjectImpl> for TestValues {
    fn from(src: &TestObjectImpl) -> Self {
        Self {
            bool: src.bool.clone(),
            i8: src.i8.clone(),
            u8: src.u8.clone(),
            i16: src.i16.clone(),
            u16: src.u16.clone(),
            i32: src.i32.clone(),
            u32: src.u32.clone(),
            i64: src.i64.clone(),
            u64: src.u64.clone(),
            isize: src.isize.clone(),
            usize: src.usize.clone(),
            f32: src.f32.clone(),
            f64: src.f64.clone(),
            string: src.string.clone(),
        }
    }
}

impl TestObjHelper for TestObjectImpl {
    fn property_type() -> &'static str {
        "member based properties (#[qobject] applied to impl block)"
    }
    fn create_with_values(values: TestValues) -> Rc<RefCell<Self>> {
        let obj = Rc::new(RefCell::new(Self::from(values)));
        <Self as QObjectHolder>::attach_qobject(&obj);
        obj
    }
}

fn test_property_can_be_read<TestObj, T, VarT>(values: TestValues, expected: &[T])
where
    TestObj: TestObjHelper,
    T: Debug + PartialEq + TryFrom<VarT>,
    VarT: Clone + QListElement,
    QList<VarT>: QVariantValue,
    <T as TryFrom<VarT>>::Error: Debug,
{
    let obj = TestObj::create_with_values(values);

    // Read the value of the tested property and compare it to the expected one.
    let type_name = get_type_name::<Vec<T>>();
    let property_name = format!("property{}", capitalize_first_char(&type_name));
    let var = unsafe { &*obj.borrow().get_qobject_ptr() }.property(&property_name);
    let actual_var_t: QList<VarT> = var.value().unwrap();
    let actual: Vec<T> = actual_var_t.into_iter()
        .map(|arg| T::try_from(arg.clone()).unwrap())
        .collect();
    let property_type = TestObj::property_type();
    assert_eq!(actual, expected, "check failed for type {type_name} of {property_type}");
}

fn test_property_can_be_written<TestObj, T, VarT, GetValueFieldFn>(test_value: &[T], get_value: GetValueFieldFn)
where
    TestObj: TestObjHelper,
    TestValues: for<'a> From<&'a TestObj>,
    T: Clone + Debug + PartialEq + TryInto<VarT>,
    <T as TryInto<VarT>>::Error: Debug,
    VarT: QListElement,
    QList<VarT>: FromIterator<VarT> + QVariantValue,
    GetValueFieldFn: FnOnce(&TestValues) -> &[T],
{
    let obj = TestObj::create_default();

    // Write the value to the tested property and compare it to value returned from getter functor.
    let type_name = get_type_name::<Vec<T>>();
    let property_name = format!("property{}", capitalize_first_char(&type_name));
    let qobj_ptr = obj.borrow().get_qobject_ptr();
    let qobj = unsafe { qobj_ptr.as_mut() }.unwrap();
    let test_values_qlist: QList<VarT> = test_value.iter()
        .map(|a| a.clone().try_into().unwrap())
        .collect();
    let test_values_var = QVariant::from(&test_values_qlist);
    qobj.set_property(&property_name, test_values_var);
    let values = TestValues::from(&obj.borrow());
    let property_type = TestObj::property_type();
    assert_eq!(test_value, get_value(&values), "check failed for type {type_name} of {property_type}");
}

fn test_cases_property_can_be_read<TestObj>() -> Vec<fn()>
where
    TestObj: TestObjHelper,
{
    vec![
        || test_property_can_be_read::<TestObj, bool, bool>(
                TestValues { bool: vec![true, false, true], ..Default::default() },
                &[true, false, true]),
        || test_property_can_be_read::<TestObj, i8, i8>(
                TestValues { i8: vec![101, 102, 103], ..Default::default() },
                &[101, 102, 103]),
        || test_property_can_be_read::<TestObj, u8, u8>(
                TestValues { u8: vec![104, 105, 106], ..Default::default() },
                &[104, 105, 106]),
        || test_property_can_be_read::<TestObj, i16, i16>(
                TestValues { i16: vec![-107, 108, -109], ..Default::default() },
                &[-107, 108, -109]),
        || test_property_can_be_read::<TestObj, u16, u16>(
                TestValues { u16: vec![110, 111, 112], ..Default::default() },
                &[110, 111, 112]),
        || test_property_can_be_read::<TestObj, i32, i32>(
                TestValues { i32: vec![113, 114, -115], ..Default::default() },
                &[113, 114, -115]),
        || test_property_can_be_read::<TestObj, u32, u32>(
                TestValues { u32: vec![116, 117, 118], ..Default::default() },
                &[116, 117, 118]),
        || test_property_can_be_read::<TestObj, i64, i64>(
                TestValues { i64: vec![-119, 120, 121], ..Default::default() },
                &[-119, 120, 121]),
        || test_property_can_be_read::<TestObj, u64, u64>(
                TestValues { u64: vec![122, 123, 124], ..Default::default() },
                &[122, 123, 124]),
        || test_property_can_be_read::<TestObj, isize, i64>(
                TestValues { isize: vec![-125, 126, 127], ..Default::default() },
                &[-125, 126, 127]),
        || test_property_can_be_read::<TestObj, usize, u64>(
                TestValues { usize: vec![128, 129, 130], ..Default::default() },
                &[128, 129, 130]),
        || test_property_can_be_read::<TestObj, f32, f32>(
                TestValues { f32: vec![0.5, -0.125, 0.25], ..Default::default() },
                &[0.5, -0.125, 0.25]),
        || test_property_can_be_read::<TestObj, f64, f64>(
                TestValues { f64: vec![0.125, 0.0625, 0.03125], ..Default::default() },
                &[0.125, 0.0625, 0.03125]),
        || test_property_can_be_read::<TestObj, String, QString>(
                TestValues { string: vec!["Crème".into(), "brûlée".into()], ..Default::default() },
                &["Crème".into(), "brûlée".into()]),
    ]
}

fn test_cases_property_can_be_written<TestObj>() -> Vec<fn()>
where
    TestObj: TestObjHelper,
    TestValues: for<'a> From<&'a TestObj>
{
    vec![
        || test_property_can_be_written::<TestObj, _, bool, _>(&[false, false, true], |values| &values.bool),
        || test_property_can_be_written::<TestObj, _, i8,  _>(&[i8::MIN, 0, i8::MAX], |values| &values.i8),
        || test_property_can_be_written::<TestObj, _, u8,  _>(&[u8::MIN, 128, u8::MAX], |values| &values.u8),
        || test_property_can_be_written::<TestObj, _, i16, _>(&[i16::MIN, 1000, i16::MAX], |values| &values.i16),
        || test_property_can_be_written::<TestObj, _, u16, _>(&[u16::MAX, u16::MIN], |values| &values.u16),
        || test_property_can_be_written::<TestObj, _, i32, _>(&[i32::MIN, 0, 100, 240, i32::MAX], |values| &values.i32),
        || test_property_can_be_written::<TestObj, _, u32, _>(&[u32::MIN, 42, u32::MAX], |values| &values.u32),
        || test_property_can_be_written::<TestObj, _, i64, _>(&[i64::MIN, i64::MAX], |values| &values.i64),
        || test_property_can_be_written::<TestObj, _, u64, _>(&[u64::MIN, u64::MAX], |values| &values.u64),
        || test_property_can_be_written::<TestObj, _, i64, _>(&[isize::MIN, isize::MAX], |values| &values.isize),
        || test_property_can_be_written::<TestObj, _, u64, _>(&[usize::MIN, usize::MAX], |values| &values.usize),
        || test_property_can_be_written::<TestObj, _, f32, _>(&[0.25, 0.0, 0.5], |values| &values.f32),
        || test_property_can_be_written::<TestObj, _, f64, _>(&[0.125, -0.25, 0.0625], |values| &values.f64),
        || test_property_can_be_written::<TestObj, _, QString, _>(&["Xin".into(), "chào".into(), "thế giới".into()], |values| &values.string),
    ]
}

#[test]
#[cfg(not(miri))]
fn qproperty_member_based_vec_can_be_read() {
    test_cases_property_can_be_read::<member_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_value_based_vec_can_be_read() {
    test_cases_property_can_be_read::<accessor_value_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_reference_based_vec_can_be_read() {
    test_cases_property_can_be_read::<accessor_reference_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}


#[test]
#[cfg(not(miri))]
fn qproperty_member_based_vec_can_be_written() {
    test_cases_property_can_be_written::<member_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_value_based_vec_can_be_written() {
    test_cases_property_can_be_written::<accessor_value_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_accessor_reference_based_vec_can_be_written() {
    test_cases_property_can_be_written::<accessor_reference_properties::TestObject>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_member_impl_based_vec_can_be_read() {
    test_cases_property_can_be_read::<TestObjectImpl>()
        .iter()
        .for_each(|test| test());
}

#[test]
#[cfg(not(miri))]
fn qproperty_member_impl_based_vec_can_be_written() {
    test_cases_property_can_be_written::<TestObjectImpl>()
        .iter()
        .for_each(|test| test());
}
