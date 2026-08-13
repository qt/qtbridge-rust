// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(feature = "serde_json")]

use cxx_qt_lib::QVariantValue;
use qtbridge_type_lib::{
    QJsonArray, QJsonObject, QJsonValue,
    QVariant, QVariantList, QVariantMap, QString,
};
use crate::QMetaTypeGet;

pub(crate) fn qvariant_to_serde(v: &QVariant) -> Result<serde_json::Value, ()> {
    if is_qvariant_type::<bool>(v) {
        return bool::try_from(v).map(serde_json::Value::Bool);
    }
    if is_qvariant_type::<i8>(v) || is_qvariant_type::<i16>(v) || is_qvariant_type::<i32>(v) || is_qvariant_type::<i64>(v) || is_qvariant_type::<isize>(v) {
        return i64::try_from(v).map(|n| serde_json::Value::Number(n.into()));
    }
    if is_qvariant_type::<u8>(v) || is_qvariant_type::<u16>(v) || is_qvariant_type::<u32>(v) || is_qvariant_type::<u64>(v) || is_qvariant_type::<usize>(v) {
        return u64::try_from(v).map(|n| serde_json::Value::Number(n.into()));
    }
    if is_qvariant_type::<f32>(v) || is_qvariant_type::<f64>(v) {
        return f64::try_from(v).map(|n| {
            serde_json::Number::from_f64(n)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        });
    }
    if is_qvariant_type::<QString>(v) {
        return String::try_from(v).map(serde_json::Value::String);
    }
    if is_qvariant_type::<QJsonValue>(v) {
        return try_from_qvariant(v)
            .map(|jv| qjsonvalue_to_serde(&jv))
    }
    if is_qvariant_type::<QJsonObject>(v) {
        return try_from_qvariant(v)
            .map(|jo| qjsonobject_to_serde(&jo))
    }
    if is_qvariant_type::<QJsonArray>(v) {
        return try_from_qvariant(v)
            .map(|ja| qjsonarray_to_serde(&ja))
    }
    // QML passes JS objects/arrays wrapped as QJSValue; QVariantMap/List use canConvert<T>()
    // which handles this case, unlike the QJson types which require exact type matching.
    if v.meta_type().name() == "QJSValue" {
        if let Ok(map) = try_from_qvariant(v) { return qvariantmap_to_serde(&map) }
        if let Ok(list) = try_from_qvariant(v) { return qvariantlist_to_serde(&list) }
    }
    Err(())
}

fn try_from_qvariant<T: QVariantValue>(v: &QVariant) -> Result<T, ()> {
    v.to_cxx_qt()
        .value()
        .ok_or(())
}

pub(crate) fn serde_to_qjsonvalue(v: &serde_json::Value) -> QJsonValue {
    match v {
        serde_json::Value::Null => QJsonValue::default(),
        serde_json::Value::Bool(b) => QJsonValue::from(*b),
        serde_json::Value::Number(n) => {
            n.as_i64().map(From::from)
                .or_else(|| n.as_f64().map(From::from))
                .unwrap_or_default()
        }
        serde_json::Value::String(s) => QJsonValue::from(&QString::from(s)),
        serde_json::Value::Array(arr) => QJsonValue::from(&serde_to_qjsonarray(arr)),
        serde_json::Value::Object(obj) => {
            let mut map = QJsonObject::default();
            for (key, value) in obj {
                map.insert(&QString::from(key), &serde_to_qjsonvalue(value));
            }
            QJsonValue::from(&map)
        }
    }
}

pub(crate) fn serde_to_qjsonarray(v: &[serde_json::Value]) -> QJsonArray {
    let mut array = QJsonArray::default();
    for item in v { array.append(&serde_to_qjsonvalue(item)); }
    array
}

pub(crate) fn qjsonvalue_to_serde(v: &QJsonValue) -> serde_json::Value {
    if v.is_null() || v.is_undefined() { return serde_json::Value::Null; }
    if v.is_bool() { return serde_json::Value::Bool(v.to_bool()); }
    if v.is_double() {
        return serde_json::Number::from_f64(v.to_double())
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null)
    }
    if v.is_string() { return serde_json::Value::String(String::from(&v.to_string())); }
    if v.is_array() { return qjsonarray_to_serde(&v.to_array()); }
    if v.is_object() { return qjsonobject_to_serde(&v.to_object()); }
    serde_json::Value::Null
}

fn is_qvariant_type<T: QMetaTypeGet>(var: &QVariant) -> bool {
    var.meta_type() == T::get_qmetatype()
}

fn qjsonarray_to_serde(v: &QJsonArray) -> serde_json::Value {
    let array = v.iter()
        .map(|item| qjsonvalue_to_serde(&item))
        .collect();
    serde_json::Value::Array(array)
}

fn qjsonobject_to_serde(v: &QJsonObject) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for key in v.keys().iter() {
        let val = qjsonvalue_to_serde(&v.value(key));
        map.insert(key.to_string(), val);
    }
    serde_json::Value::Object(map)
}

fn qvariantmap_to_serde(v: &QVariantMap) -> Result<serde_json::Value, ()> {
    let mut map = serde_json::Map::new();
    for (key, value) in v.iter() {
        let val = qvariant_to_serde(&QVariant::from_cxx_qt(value))?;
        map.insert(String::from(key), val);
    }
    Ok(serde_json::Value::Object(map))
}

fn qvariantlist_to_serde(v: &QVariantList) -> Result<serde_json::Value, ()> {
    let array = v.iter()
        .map(|var| qvariant_to_serde(&QVariant::from_cxx_qt(var)))
        .collect::<Result<_, _>>()?;
    Ok(serde_json::Value::Array(array))
}
