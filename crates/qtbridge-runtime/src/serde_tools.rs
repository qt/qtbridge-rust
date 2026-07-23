// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(feature = "serde_json")]

use cxx_qt_lib::{QMetaTypeType, QVariantValue};
use qtbridge_type_lib::{
    QJsonArray, QJsonObject, QJsonValue, QString, QVariant, QVariantMap,
};

pub(crate) fn qvariant_to_serde(v: &QVariant) -> Result<serde_json::Value, ()> {
    match v.type_id() {
        QMetaTypeType::Bool =>
            return Ok(serde_json::Value::from(v.value_or_default::<bool>())),
        QMetaTypeType::SChar |
        QMetaTypeType::Short |
        QMetaTypeType::Int |
        QMetaTypeType::Long |
        QMetaTypeType::LongLong =>
            return Ok(serde_json::Value::from(v.value_or_default::<i64>())),
        QMetaTypeType::UChar |
        QMetaTypeType::UShort |
        QMetaTypeType::UInt |
        QMetaTypeType::ULong |
        QMetaTypeType::ULongLong =>
            return Ok(serde_json::Value::from(v.value_or_default::<u64>())),
        QMetaTypeType::Float =>
            return Ok(serde_json::Value::from(v.value_or_default::<f32>())),
        QMetaTypeType::Double =>
            return Ok(serde_json::Value::from(v.value_or_default::<f64>())),
        QMetaTypeType::QString => {
            let qstr = v.value_or_default::<QString>();
            return Ok(serde_json::Value::String(qstr.into()))
        }
        QMetaTypeType::QJsonValue => {
            return try_from_qvariant(v)
                .map(|jv| qjsonvalue_to_serde(&jv));
        }
        QMetaTypeType::QJsonObject => {
            return try_from_qvariant(v)
                .map(|jo| qjsonobject_to_serde(&jo))
        }
        QMetaTypeType::QJsonArray => {
            return try_from_qvariant(v)
                .map(|jv| qjsonarray_to_serde(&jv))
        }
        _ => {}
    }

    if v.type_name() == "QJSValue" {
        if let Ok(map) = try_from_qvariant(v) { return qvariantmap_to_serde(&map); }
        if let Ok(list) = try_from_qvariant(v) { return qvariantlist_to_serde(&list) }
    }

    Err(())
}

fn try_from_qvariant<T: QVariantValue>(v: &QVariant) -> Result<T, ()> {
    v.value()
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
        let val = qvariant_to_serde(value)?;
        map.insert(String::from(key), val);
    }
    Ok(serde_json::Value::Object(map))
}

fn qvariantlist_to_serde(v: &cxx_qt_lib::QList<QVariant>) -> Result<serde_json::Value, ()> {
    let array = v.into_iter()
        .map(qvariant_to_serde)
        .collect::<Result<_, _>>()?;
    Ok(serde_json::Value::Array(array))
}
