// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::HashMap;

use qtbridge_type_lib::QVariant;

use crate::{QVariantConvertible, ToQVariant, TryFromQVariant};

fn option_to_qvariant<T: ToQVariant>(value: Option<&T>) -> QVariant {
    value.map(<T>::to_qvariant)
            .unwrap_or_default()
}

/// Trait representing a single item in an item model.
///
/// This trait is implemented automatically by `#[derive(QModelItem)]`
/// for structs and tuple structs and an implementation for primitive types
/// and tuples is provided. It allows QtBridge to use the type in item models.
///
/// # Typical usage
/// Normally, you should **not implement this manually**. Use
/// `#[derive(QModelItem)]` on your struct:
///
/// ```rust, ignore
/// #[derive(QModelItem)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// ```
///
/// # Manual Implementation
/// If your struct contains fields that **cannot be converted to `QVariant`**,
/// you need to implement `QModelItem` manually. Only include fields that
/// are known to QtBridge in the `getN` and `setN` methods, and adjust
/// `role_names()` accordingly.
///
/// Example:
///
/// ```rust
///
/// use std::collections::HashMap;
/// use qtbridge::QModelItem;
///
/// struct CustomType {
///     data: Vec<u8>, // cannot be directly converted to QVariant
///     name: String,
/// }
///
/// impl QModelItem for CustomType {
///     type T0 = String;
///     type T1 = ();
///     type T2 = ();
///     type T3 = ();
///     type T4 = ();
///     type T5 = ();
///     type T6 = ();
///     type T7 = ();
///     type T8 = ();
///     type T9 = ();
///     type T10 = ();
///     type T11 = ();
///     type T12 = ();
///     type T13 = ();
///     type T14 = ();
///
///     fn get0(&self) -> Option<&String> {
///         Some(&self.name)
///     }
///
///     fn set0(&mut self, value: String) {
///         self.name = value;
///     }
///
///     fn role_names() -> HashMap<i32, String> {
///         let roles = [(0x0, "name".into())];
///         roles.into()
///     }
/// }
/// ```
pub trait QModelItem {
    // TODO: When associated type defaults become stable, set () as default type.
    // https://github.com/rust-lang/rust/issues/29661
    type T0 : QVariantConvertible;
    type T1 : QVariantConvertible;
    type T2 : QVariantConvertible;
    type T3 : QVariantConvertible;
    type T4 : QVariantConvertible;
    type T5 : QVariantConvertible;
    type T6 : QVariantConvertible;
    type T7 : QVariantConvertible;
    type T8 : QVariantConvertible;
    type T9 : QVariantConvertible;
    type T10 : QVariantConvertible;
    type T11 : QVariantConvertible;
    type T12 : QVariantConvertible;
    type T13 : QVariantConvertible;
    type T14 : QVariantConvertible;

    fn get0(&self) -> Option<&Self::T0> { None }
    fn get1(&self) -> Option<&Self::T1> { None }
    fn get2(&self) -> Option<&Self::T2> { None }
    fn get3(&self) -> Option<&Self::T3> { None }
    fn get4(&self) -> Option<&Self::T4> { None }
    fn get5(&self) -> Option<&Self::T5> { None }
    fn get6(&self) -> Option<&Self::T6> { None }
    fn get7(&self) -> Option<&Self::T7> { None }
    fn get8(&self) -> Option<&Self::T8> { None }
    fn get9(&self) -> Option<&Self::T9> { None }
    fn get10(&self) -> Option<&Self::T10> { None }
    fn get11(&self) -> Option<&Self::T11> { None }
    fn get12(&self) -> Option<&Self::T12> { None }
    fn get13(&self) -> Option<&Self::T13> { None }
    fn get14(&self) -> Option<&Self::T14> { None }

    fn set0(&mut self, _value: Self::T0) {}
    fn set1(&mut self, _value: Self::T1) {}
    fn set2(&mut self, _value: Self::T2) {}
    fn set3(&mut self, _value: Self::T3) {}
    fn set4(&mut self, _value: Self::T4) {}
    fn set5(&mut self, _value: Self::T5) {}
    fn set6(&mut self, _value: Self::T6) {}
    fn set7(&mut self, _value: Self::T7) {}
    fn set8(&mut self, _value: Self::T8) {}
    fn set9(&mut self, _value: Self::T9) {}
    fn set10(&mut self, _value: Self::T10) {}
    fn set11(&mut self, _value: Self::T11) {}
    fn set12(&mut self, _value: Self::T12) {}
    fn set13(&mut self, _value: Self::T13) {}
    fn set14(&mut self, _value: Self::T14) {}

    #[doc(hidden)]
    fn get_role(&self, id: i32) -> QVariant
    {
        match id {
            0 => option_to_qvariant(self.get0()),
            1 => option_to_qvariant(self.get1()),
            2 => option_to_qvariant(self.get2()),
            3 => option_to_qvariant(self.get3()),
            4 => option_to_qvariant(self.get4()),
            5 => option_to_qvariant(self.get5()),
            6 => option_to_qvariant(self.get6()),
            7 => option_to_qvariant(self.get7()),
            8 => option_to_qvariant(self.get8()),
            9 => option_to_qvariant(self.get9()),
            10 => option_to_qvariant(self.get10()),
            11 => option_to_qvariant(self.get11()),
            12 => option_to_qvariant(self.get12()),
            13 => option_to_qvariant(self.get13()),
            14 => option_to_qvariant(self.get14()),
            _ => QVariant::default(),
        }
    }

    #[doc(hidden)]
    fn set_role(&mut self, id: i32, value: &QVariant) -> bool
    {
        match id {
            0 => <Self::T0>::try_from_qvariant(value).map(|val| self.set0(val)),
            1 => <Self::T1>::try_from_qvariant(value).map(|val| self.set1(val)),
            2 => <Self::T2>::try_from_qvariant(value).map(|val| self.set2(val)),
            3 => <Self::T3>::try_from_qvariant(value).map(|val| self.set3(val)),
            4 => <Self::T4>::try_from_qvariant(value).map(|val| self.set4(val)),
            5 => <Self::T5>::try_from_qvariant(value).map(|val| self.set5(val)),
            6 => <Self::T6>::try_from_qvariant(value).map(|val| self.set6(val)),
            7 => <Self::T7>::try_from_qvariant(value).map(|val| self.set7(val)),
            8 => <Self::T8>::try_from_qvariant(value).map(|val| self.set8(val)),
            9 => <Self::T9>::try_from_qvariant(value).map(|val| self.set9(val)),
            10 => <Self::T10>::try_from_qvariant(value).map(|val| self.set10(val)),
            11 => <Self::T11>::try_from_qvariant(value).map(|val| self.set11(val)),
            12 => <Self::T12>::try_from_qvariant(value).map(|val| self.set12(val)),
            13 => <Self::T13>::try_from_qvariant(value).map(|val| self.set13(val)),
            14 => <Self::T14>::try_from_qvariant(value).map(|val| self.set14(val)),
            _ =>  return false,
        }.is_ok()
    }

    fn role_names() -> HashMap<i32, String>;
}

macro_rules! impl_qmodel_item_for_primitive {
    ($t:ty) => {
        impl QModelItem for $t {
            type T0 = $t;
            type T1 = ();
            type T2 = ();
            type T3 = ();
            type T4 = ();
            type T5 = ();
            type T6 = ();
            type T7 = ();
            type T8 = ();
            type T9 = ();
            type T10 = ();
            type T11 = ();
            type T12 = ();
            type T13 = ();
            type T14 = ();

            fn get0(&self) -> Option<&Self::T0> {
                Some(self)
            }

            fn set0(&mut self, value: Self::T0) {
                *self = value;
            }
            fn role_names() -> HashMap<i32, String> {
                let roles = [(0x0, "value".into())];
                roles.into()
            }
        }
    };
}

impl_qmodel_item_for_primitive!(i8);
impl_qmodel_item_for_primitive!(i16);
impl_qmodel_item_for_primitive!(i32);
impl_qmodel_item_for_primitive!(i64);
impl_qmodel_item_for_primitive!(u8);
impl_qmodel_item_for_primitive!(u16);
impl_qmodel_item_for_primitive!(u32);
impl_qmodel_item_for_primitive!(u64);
impl_qmodel_item_for_primitive!(f32);
impl_qmodel_item_for_primitive!(f64);
impl_qmodel_item_for_primitive!(bool);
impl_qmodel_item_for_primitive!(String);

impl<T0, T1> QModelItem for (T0, T1)
where
    T0: QVariantConvertible,
    T1: QVariantConvertible
{
    type T0 = T0;
    type T1 = T1;
    type T2 = ();
    type T3 = ();
    type T4 = ();
    type T5 = ();
    type T6 = ();
    type T7 = ();
    type T8 = ();
    type T9 = ();
    type T10 = ();
    type T11 = ();
    type T12 = ();
    type T13 = ();
    type T14 = ();

    fn get0(&self) -> Option<&Self::T0> { Some(&self.0) }
    fn get1(&self) -> Option<&Self::T1> { Some(&self.1) }
    fn set0(&mut self, value: Self::T0) { self.0 = value; }
    fn set1(&mut self, value: Self::T1) { self.1 = value; }

    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x0, "_0".into()),
            (0x1, "_1".into()),
        ];
        roles.into()
    }
}

impl<T0, T1, T2> QModelItem for (T0, T1, T2)
where
    T0: QVariantConvertible,
    T1: QVariantConvertible,
    T2: QVariantConvertible
{
    type T0 = T0;
    type T1 = T1;
    type T2 = T2;
    type T3 = ();
    type T4 = ();
    type T5 = ();
    type T6 = ();
    type T7 = ();
    type T8 = ();
    type T9 = ();
    type T10 = ();
    type T11 = ();
    type T12 = ();
    type T13 = ();
    type T14 = ();

    fn get0(&self) -> Option<&Self::T0> { Some(&self.0) }
    fn get1(&self) -> Option<&Self::T1> { Some(&self.1) }
    fn get2(&self) -> Option<&Self::T2> { Some(&self.2) }
    fn set0(&mut self, value: Self::T0) { self.0 = value; }
    fn set1(&mut self, value: Self::T1) { self.1 = value; }
    fn set2(&mut self, value: Self::T2) { self.2 = value; }

    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x0, "_0".into()),
            (0x1, "_1".into()),
            (0x2, "_2".into()),
        ];
        roles.into()
    }
}

impl<T0, T1, T2, T3> QModelItem for (T0, T1, T2, T3)
where
    T0: QVariantConvertible,
    T1: QVariantConvertible,
    T2: QVariantConvertible,
    T3: QVariantConvertible
{
    type T0 = T0;
    type T1 = T1;
    type T2 = T2;
    type T3 = T3;
    type T4 = ();
    type T5 = ();
    type T6 = ();
    type T7 = ();
    type T8 = ();
    type T9 = ();
    type T10 = ();
    type T11 = ();
    type T12 = ();
    type T13 = ();
    type T14 = ();

    fn get0(&self) -> Option<&Self::T0> { Some(&self.0) }
    fn get1(&self) -> Option<&Self::T1> { Some(&self.1) }
    fn get2(&self) -> Option<&Self::T2> { Some(&self.2) }
    fn get3(&self) -> Option<&Self::T3> { Some(&self.3) }
    fn set0(&mut self, value: Self::T0) { self.0 = value; }
    fn set1(&mut self, value: Self::T1) { self.1 = value; }
    fn set2(&mut self, value: Self::T2) { self.2 = value; }
    fn set3(&mut self, value: Self::T3) { self.3 = value; }

    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x0, "_0".into()),
            (0x1, "_1".into()),
            (0x2, "_2".into()),
            (0x3, "_3".into()),
        ];
        roles.into()
    }
}

impl<T0, T1, T2, T3, T4> QModelItem for (T0, T1, T2, T3, T4)
where
    T0: QVariantConvertible,
    T1: QVariantConvertible,
    T2: QVariantConvertible,
    T3: QVariantConvertible,
    T4: QVariantConvertible
{
    type T0 = T0;
    type T1 = T1;
    type T2 = T2;
    type T3 = T3;
    type T4 = T4;
    type T5 = ();
    type T6 = ();
    type T7 = ();
    type T8 = ();
    type T9 = ();
    type T10 = ();
    type T11 = ();
    type T12 = ();
    type T13 = ();
    type T14 = ();

    fn get0(&self) -> Option<&Self::T0> { Some(&self.0) }
    fn get1(&self) -> Option<&Self::T1> { Some(&self.1) }
    fn get2(&self) -> Option<&Self::T2> { Some(&self.2) }
    fn get3(&self) -> Option<&Self::T3> { Some(&self.3) }
    fn get4(&self) -> Option<&Self::T4> { Some(&self.4) }
    fn set0(&mut self, value: Self::T0) { self.0 = value; }
    fn set1(&mut self, value: Self::T1) { self.1 = value; }
    fn set2(&mut self, value: Self::T2) { self.2 = value; }
    fn set3(&mut self, value: Self::T3) { self.3 = value; }
    fn set4(&mut self, value: Self::T4) { self.4 = value; }

    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x0, "_0".into()),
            (0x1, "_1".into()),
            (0x2, "_2".into()),
            (0x3, "_3".into()),
            (0x4, "_4".into()),
        ];
        roles.into()
    }
}

impl<T0, T1, T2, T3, T4, T5> QModelItem for (T0, T1, T2, T3, T4, T5)
where
    T0: QVariantConvertible,
    T1: QVariantConvertible,
    T2: QVariantConvertible,
    T3: QVariantConvertible,
    T4: QVariantConvertible,
    T5: QVariantConvertible
{
    type T0 = T0;
    type T1 = T1;
    type T2 = T2;
    type T3 = T3;
    type T4 = T4;
    type T5 = T5;
    type T6 = ();
    type T7 = ();
    type T8 = ();
    type T9 = ();
    type T10 = ();
    type T11 = ();
    type T12 = ();
    type T13 = ();
    type T14 = ();

    fn get0(&self) -> Option<&Self::T0> { Some(&self.0) }
    fn get1(&self) -> Option<&Self::T1> { Some(&self.1) }
    fn get2(&self) -> Option<&Self::T2> { Some(&self.2) }
    fn get3(&self) -> Option<&Self::T3> { Some(&self.3) }
    fn get4(&self) -> Option<&Self::T4> { Some(&self.4) }
    fn get5(&self) -> Option<&Self::T5> { Some(&self.5) }
    fn set0(&mut self, value: Self::T0) { self.0 = value; }
    fn set1(&mut self, value: Self::T1) { self.1 = value; }
    fn set2(&mut self, value: Self::T2) { self.2 = value; }
    fn set3(&mut self, value: Self::T3) { self.3 = value; }
    fn set4(&mut self, value: Self::T4) { self.4 = value; }
    fn set5(&mut self, value: Self::T5) { self.5 = value; }

    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x0, "_0".into()),
            (0x1, "_1".into()),
            (0x2, "_2".into()),
            (0x3, "_3".into()),
            (0x4, "_4".into()),
            (0x5, "_5".into()),
        ];
        roles.into()
    }
}

#[cfg(test)]
macro_rules! test_tuple_index {
    ($tuple:expr, $set:ident, $get:ident, $field:tt, $value:expr) => {{
        $tuple.$set($value);
        assert_eq!(*$tuple.$get().unwrap(), $value);
        assert_eq!(*$tuple.$get().unwrap(), $tuple.$field);
    }};
}

#[cfg(test)]
macro_rules! assert_role_name {
    ($type:ty, $idx:expr, $expected:expr) => {
        assert_eq!(
            <$type>::role_names().get(&$idx).unwrap(),
            $expected
        );
    };
}

#[test]
fn test_tuple_implementation() {
    let mut tuple2: (i32, i32) = (1, 1);
    assert_eq!(<(i32, i32)>::role_names().len(), 2);
    assert_role_name!((i32, i32), 0, "_0");
    assert_role_name!((i32, i32), 1, "_1");
    test_tuple_index!(tuple2, set0, get0, 0, 42);
    test_tuple_index!(tuple2, set1, get1, 1, 43);

    let mut tuple3: (i32, i32, i32) = (1, 1, 1);
    assert_eq!(<(i32, i32, i32)>::role_names().len(), 3);
    assert_role_name!((i32, i32, i32), 0, "_0");
    assert_role_name!((i32, i32, i32), 1, "_1");
    assert_role_name!((i32, i32, i32), 2, "_2");
    test_tuple_index!(tuple3, set0, get0, 0, 42);
    test_tuple_index!(tuple3, set1, get1, 1, 43);
    test_tuple_index!(tuple3, set2, get2, 2, 44);

    let mut tuple4: (i32, i32, i32, i32) = (1, 1, 1, 1);
    assert_eq!(<(i32, i32, i32, i32)>::role_names().len(), 4);
    assert_role_name!((i32, i32, i32, i32), 0, "_0");
    assert_role_name!((i32, i32, i32, i32), 1, "_1");
    assert_role_name!((i32, i32, i32, i32), 2, "_2");
    assert_role_name!((i32, i32, i32, i32), 3, "_3");
    test_tuple_index!(tuple4, set0, get0, 0, 42);
    test_tuple_index!(tuple4, set1, get1, 1, 43);
    test_tuple_index!(tuple4, set2, get2, 2, 44);
    test_tuple_index!(tuple4, set3, get3, 3, 45);

    let mut tuple5: (i32, i32, i32, i32, i32) = (1, 1, 1, 1, 1);
    assert_eq!(<(i32, i32, i32, i32, i32)>::role_names().len(), 5);
    assert_role_name!((i32, i32, i32, i32, i32), 0, "_0");
    assert_role_name!((i32, i32, i32, i32, i32), 1, "_1");
    assert_role_name!((i32, i32, i32, i32, i32), 2, "_2");
    assert_role_name!((i32, i32, i32, i32, i32), 3, "_3");
    assert_role_name!((i32, i32, i32, i32, i32), 4, "_4");
    test_tuple_index!(tuple5, set0, get0, 0, 42);
    test_tuple_index!(tuple5, set1, get1, 1, 43);
    test_tuple_index!(tuple5, set2, get2, 2, 44);
    test_tuple_index!(tuple5, set3, get3, 3, 45);
    test_tuple_index!(tuple5, set4, get4, 4, 46);

    let mut tuple6: (i32, i32, i32, i32, i32, i32) = (1, 1, 1, 1, 1, 1);
    assert_eq!(<(i32, i32, i32, i32, i32, i32)>::role_names().len(), 6);
    assert_role_name!((i32, i32, i32, i32, i32, i32), 0, "_0");
    assert_role_name!((i32, i32, i32, i32, i32, i32), 1, "_1");
    assert_role_name!((i32, i32, i32, i32, i32, i32), 2, "_2");
    assert_role_name!((i32, i32, i32, i32, i32, i32), 3, "_3");
    assert_role_name!((i32, i32, i32, i32, i32, i32), 4, "_4");
    assert_role_name!((i32, i32, i32, i32, i32, i32), 5, "_5");
    test_tuple_index!(tuple6, set0, get0, 0, 42);
    test_tuple_index!(tuple6, set1, get1, 1, 43);
    test_tuple_index!(tuple6, set2, get2, 2, 44);
    test_tuple_index!(tuple6, set3, get3, 3, 45);
    test_tuple_index!(tuple6, set4, get4, 4, 46);
    test_tuple_index!(tuple6, set5, get5, 5, 47);
}
