// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::QVariant;

use crate::QMetaTypeCompatible;

#[doc(hidden)]
/// Convert to QVariant. Intended for use with references only.
pub trait ToQVariant {
    fn to_qvariant(&self) -> QVariant;
}

#[doc(hidden)]
/// Fallible conversion from QVariant to a value type.
pub trait TryFromQVariant: Sized {
    fn try_from_qvariant(value: &QVariant) -> Result<Self, ()>;
}

#[doc(hidden)]
/// Convenience trait that combines conversion to and from QVariant.
pub trait QVariantConvertible: ToQVariant + TryFromQVariant {}
impl<T: ToQVariant + TryFromQVariant> QVariantConvertible for T {}

/// Implement `ToQVariant` and `TryFromQVariant` traits using `QMetaTypeCompatible`.
macro_rules! impl_to_qvariant_and_try_from_qvariant {
    ($($t:ty),*) => {
        $(
            // Conversion from referenced value to QVariant.
            impl ToQVariant for $t {
                fn to_qvariant(&self) -> QVariant {
                    let compat = QMetaTypeCompatible::to_compatible(self);
                    (&compat).into()
                }
            }

            // Conversion from a QVariant to value.
            impl TryFromQVariant for $t {
                fn try_from_qvariant(value: &QVariant) -> Result<Self, ()> {
                    let compat: <$t as QMetaTypeCompatible>::CompatibleType = value.value()
                        .ok_or(())?;
                    Ok(<Self as QMetaTypeCompatible>::from_compatible(&compat))
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

impl ToQVariant for () {
    fn to_qvariant(&self) -> QVariant {
        QVariant::default()
    }
}

impl TryFromQVariant for () {
    fn try_from_qvariant(value: &QVariant) -> Result<Self, ()> {
        match value.is_valid() {
            true => Err(()),
            false => Ok(()),
        }
    }
}

#[cfg(feature = "serde_json")]
impl ToQVariant for serde_json::Value {
    fn to_qvariant(&self) -> QVariant {
        let jv = crate::serde_tools::serde_to_qjsonvalue(self);
        (&jv).into()
    }
}

#[cfg(feature = "serde_json")]
impl TryFromQVariant for serde_json::Value {
    fn try_from_qvariant(value: &QVariant) -> Result<Self, ()> {
        crate::serde_tools::qvariant_to_serde(value)
    }
}

#[cfg(feature = "serde_json")]
impl ToQVariant for Vec<serde_json::Value> {
    fn to_qvariant(&self) -> QVariant {
        let ja = crate::serde_tools::serde_to_qjsonarray(self);
        (&ja).into()
    }
}

#[cfg(feature = "serde_json")]
impl TryFromQVariant for Vec<serde_json::Value> {
    fn try_from_qvariant(value: &QVariant) -> Result<Self, ()> {
        match crate::serde_tools::qvariant_to_serde(value)? {
            serde_json::Value::Array(arr) => Ok(arr),
            _ => Err(()),
        }
    }
}
