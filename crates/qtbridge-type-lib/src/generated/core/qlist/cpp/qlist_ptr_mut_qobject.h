// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_PTR_MUT_QOBJECT_RUST_BRIDGE_H_
#define _QLIST_PTR_MUT_QOBJECT_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h"
#include "rust/cxx.h"

using QList_ptr_mut_QObject = ::QList<QObject *>;

namespace rust::bridge::qlist_ptr_mut_qobject {

void QList_Drop(QList_ptr_mut_QObject &v);
QList_ptr_mut_QObject QList_Default();
QList_ptr_mut_QObject QList_Clone(const QList_ptr_mut_QObject &src);
bool QList_Eq(const QList_ptr_mut_QObject &lhs, const QList_ptr_mut_QObject &rhs);

void inlineCppFn_append(QList_ptr_mut_QObject &self, QObject *value);

void inlineCppFn_reserve(QList_ptr_mut_QObject &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_ptr_mut_QObject const &self);

QObject *const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_ptr_mut_QObject_index(
        QList_ptr_mut_QObject const &self, size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_ptr_mut_QObject_N_for_QList_ptr_mut_QObject_eq(
        QList_ptr_mut_QObject const &self, rust::Slice<QObject *const> rhs);

} // namespace rust::bridge::qlist_ptr_mut_qobject

namespace rust {

template <>
struct IsRelocatable<::QList_ptr_mut_QObject> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_PTR_MUT_QOBJECT_RUST_BRIDGE_H_
