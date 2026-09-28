// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::QVariant;

use crate::QMetaTypeCompatible;

/// A type that cross to and from QML by value.
///
/// This is the bound for values stored in a model role (see
/// [`QModelItem`](crate::QModelItem)) and for value-typed properties and
/// method arguments. Implemented for:
/// - Primitive numeric types and `bool`
/// - [`String`]
/// - [`Vec<T>`] where `T` is one of the above
///
/// `QObject` handles (`Rc<RefCell<T>>`) are deliberately not
/// `QVariantConvertible` as they cross as a pointer.
pub trait QVariantConvertible: Sized {
    /// Convert to QVariant. Intended for use with references only.
    fn to_qvariant(&self) -> QVariant;
    /// Fallible conversion from QVariant to a value type.
    fn try_from_qvariant(value: &QVariant) -> Option<Self>;

}

/// Implement `QVariantConvertible` traits using `QMetaTypeCompatible`.
macro_rules! impl_to_qvariant_and_try_from_qvariant {
    ($($t:ty),*) => {
        $(
            impl QVariantConvertible for $t {
                // Conversion from referenced value to QVariant.
                fn to_qvariant(&self) -> QVariant {
                    let compat = QMetaTypeCompatible::to_compatible(self);
                    (&compat).into()
                }
                // Conversion from a QVariant to value.
                fn try_from_qvariant(value: &QVariant) -> Option<Self> {
                    let compat: <$t as QMetaTypeCompatible>::CompatibleType = value.value()?;
                    Some(<Self as QMetaTypeCompatible>::from_compatible(&compat))
                }
            }
        )*
    }
}

impl_to_qvariant_and_try_from_qvariant!(
    bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, String,
    Vec<bool>, Vec<i8>, Vec<u8>, Vec<i16>, Vec<u16>, Vec<i32>, Vec<u32>, Vec<i64>, Vec<u64>,
    Vec<isize>, Vec<usize>, Vec<f32>, Vec<f64>, Vec<String>
);

impl QVariantConvertible for () {
    fn to_qvariant(&self) -> QVariant {
        QVariant::default()
    }
    fn try_from_qvariant(value: &QVariant) -> Option<Self> {
        match value.is_valid() {
            true => None,
            false => Some(()),
        }
    }
}

#[cfg(feature = "serde_json")]
impl QVariantConvertible for serde_json::Value {
    fn to_qvariant(&self) -> QVariant {
        let jv = crate::serde_tools::serde_to_qjsonvalue(self);
        (&jv).into()
    }
    fn try_from_qvariant(value: &QVariant) -> Option<Self> {
        crate::serde_tools::qvariant_to_serde(value)
    }
}

#[cfg(feature = "serde_json")]
impl QVariantConvertible for Vec<serde_json::Value> {
    fn to_qvariant(&self) -> QVariant {
        let ja = crate::serde_tools::serde_to_qjsonarray(self);
        (&ja).into()
    }
    fn try_from_qvariant(value: &QVariant) -> Option<Self> {
        match crate::serde_tools::qvariant_to_serde(value)? {
            serde_json::Value::Array(arr) => Some(arr),
            _ => None,
        }
    }
}
