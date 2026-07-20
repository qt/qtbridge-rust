// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QVARIANT_RUST_BRIDGE_H_
#define _QVARIANT_RUST_BRIDGE_H_

#include <QJsonValue>
#include <QList>
#include <QVariant>
#include <cstdint>
#include "qtbridge-type-lib/src/generated/core/qbytearray/cpp/qbytearray.h"
#include "qtbridge-type-lib/src/generated/core/qjsonarray/cpp/qjsonarray.h"
#include "qtbridge-type-lib/src/generated/core/qjsonobject/cpp/qjsonobject.h"
#include "qtbridge-type-lib/src/generated/core/qjsonvalue/cpp/qjsonvalue.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_bool.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_f32.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_f64.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_i16.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_i32.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_i64.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_i8.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_ptr_mut_qobject.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qbytearray.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_u16.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_u32.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_u64.h"
#include "qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_u8.h"
#include "qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h"
#include "qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h"
#include "rust/cxx.h"
#include "rustconv.h"

namespace rust::bridge::qvariant {

void QVariant_Drop(QVariant &v);
QVariant QVariant_Default();
QVariant QVariant_Clone(const QVariant &src);

bool inlineCppFn_is_valid(QVariant const &self);

QMetaType inlineCppFn_meta_type(QVariant const &self);

rust::String inlineCppFn_TraitImpl_ToString_for_QVariant_to_string(QVariant const &self);

QVariant inlineCppFn_TraitImpl_From_string_slice_for_QVariant_from(rust::Str from);

QVariant
inlineCppFn_TraitImpl_From_ref_Vec_String_for_QVariant_from(rust::Vec<rust::String> const &from);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_String_try_from(QVariant const &from,
                                                                    rust::String &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_String_try_from(
        QVariant const &from, rust::Vec<rust::String> &result);

QVariant inlineCppFn_TraitImpl_From_ref_bool_for_QVariant_from(bool const &value);

QVariant inlineCppFn_TraitImpl_From_ref_i8_for_QVariant_from(int8_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_u8_for_QVariant_from(uint8_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_i16_for_QVariant_from(int16_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_u16_for_QVariant_from(uint16_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_i32_for_QVariant_from(int32_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_u32_for_QVariant_from(uint32_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_i64_for_QVariant_from(int64_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_u64_for_QVariant_from(uint64_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_isize_for_QVariant_from(ptrdiff_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_usize_for_QVariant_from(size_t const &value);

QVariant inlineCppFn_TraitImpl_From_ref_f32_for_QVariant_from(float const &value);

QVariant inlineCppFn_TraitImpl_From_ref_f64_for_QVariant_from(double const &value);

QVariant inlineCppFn_TraitImpl_From_ref_ptr_mut_QObject_for_QVariant_from(QObject *const &value);

QVariant inlineCppFn_TraitImpl_From_ref_QObjectList_for_QVariant_from(QObjectList const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_bool_for_QVariant_from(rust::Vec<bool> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_i8_for_QVariant_from(rust::Vec<int8_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_u8_for_QVariant_from(rust::Vec<uint8_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_i16_for_QVariant_from(rust::Vec<int16_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_u16_for_QVariant_from(rust::Vec<uint16_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_i32_for_QVariant_from(rust::Vec<int32_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_u32_for_QVariant_from(rust::Vec<uint32_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_i64_for_QVariant_from(rust::Vec<int64_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_u64_for_QVariant_from(rust::Vec<uint64_t> const &value);

QVariant
inlineCppFn_TraitImpl_From_ref_Vec_isize_for_QVariant_from(rust::Vec<ptrdiff_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_usize_for_QVariant_from(rust::Vec<size_t> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_f32_for_QVariant_from(rust::Vec<float> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_f64_for_QVariant_from(rust::Vec<double> const &value);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_QByteArray_for_QVariant_from(QList<QByteArray> list);

QVariant inlineCppFn_TraitImpl_From_ref_Vec_QString_for_QVariant_from(QList<QString> list);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_bool_try_from(QVariant const &from,
                                                                  bool &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i8_try_from(QVariant const &from,
                                                                int8_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u8_try_from(QVariant const &from,
                                                                uint8_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i16_try_from(QVariant const &from,
                                                                 int16_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u16_try_from(QVariant const &from,
                                                                 uint16_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i32_try_from(QVariant const &from,
                                                                 int32_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u32_try_from(QVariant const &from,
                                                                 uint32_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_i64_try_from(QVariant const &from,
                                                                 int64_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_u64_try_from(QVariant const &from,
                                                                 uint64_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_isize_try_from(QVariant const &from,
                                                                   ptrdiff_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_usize_try_from(QVariant const &from,
                                                                   size_t &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_f32_try_from(QVariant const &from,
                                                                 float &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_f64_try_from(QVariant const &from,
                                                                 double &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_ptr_mut_QObject_try_from(QVariant const &from,
                                                                             QObject *&result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QObjectList_try_from(QVariant const &from,
                                                                         QObjectList &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_bool_try_from(QVariant const &from,
                                                                      QList<bool> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i8_try_from(QVariant const &from,
                                                                    QList<int8_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u8_try_from(QVariant const &from,
                                                                    QList<uint8_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i16_try_from(QVariant const &from,
                                                                     QList<int16_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u16_try_from(QVariant const &from,
                                                                     QList<uint16_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i32_try_from(QVariant const &from,
                                                                     QList<int32_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u32_try_from(QVariant const &from,
                                                                     QList<uint32_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_i64_try_from(QVariant const &from,
                                                                     QList<int64_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_u64_try_from(QVariant const &from,
                                                                     QList<uint64_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_isize_try_from(QVariant const &from,
                                                                       QList<ptrdiff_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_usize_try_from(QVariant const &from,
                                                                       QList<size_t> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_f32_try_from(QVariant const &from,
                                                                     QList<float> &result);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_Vec_f64_try_from(QVariant const &from,
                                                                     QList<double> &result);

QVariant inlineCppFn_TraitImpl_From_ref_QJsonArray_for_QVariant_from(QJsonArray const &value);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonArray_try_from(QVariant const &from,
                                                                        QJsonArray &result);

QVariant inlineCppFn_TraitImpl_From_ref_QJsonObject_for_QVariant_from(QJsonObject const &value);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonObject_try_from(QVariant const &from,
                                                                         QJsonObject &result);

QVariant inlineCppFn_TraitImpl_From_ref_QJsonValue_for_QVariant_from(QJsonValue const &value);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QJsonValue_try_from(QVariant const &from,
                                                                        QJsonValue &result);

} // namespace rust::bridge::qvariant

namespace rust {

template <>
struct IsRelocatable<::QVariant> : ::std::true_type
{
};

} // namespace rust

#endif // _QVARIANT_RUST_BRIDGE_H_
