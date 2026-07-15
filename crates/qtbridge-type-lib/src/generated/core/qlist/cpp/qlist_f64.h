// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_F64_RUST_BRIDGE_H_
#define _QLIST_F64_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_f64 = ::QList<double>;

namespace rust::bridge::qlist_f64 {

void QList_Drop(QList_f64 &v);
QList_f64 QList_Default();
QList_f64 QList_Clone(const QList_f64 &src);
bool QList_Eq(const QList_f64 &lhs, const QList_f64 &rhs);

void inlineCppFn_append(QList_f64 &self, double value);

void inlineCppFn_clear(QList_f64 &self);

bool inlineCppFn_contains(QList_f64 const &self, double const &value);

void inlineCppFn_push_back(QList_f64 &self, double value);

void inlineCppFn_remove(QList_f64 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_f64 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_f64 const &self);

double const &inlineCppFn_first(QList_f64 const &self);

double const &inlineCppFn_last(QList_f64 const &self);

rust::Vec<double> inlineCppFn_TraitImpl_From_ref_QList_f64_for_Vec_f64_from(QList_f64 const &src);

double const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_f64_index(QList_f64 const &self,
                                                                            size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_f64_N_for_QList_f64_eq(QList_f64 const &self,
                                                                     rust::Slice<double const> rhs);

} // namespace rust::bridge::qlist_f64

namespace rust {

template <>
struct IsRelocatable<::QList_f64> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_F64_RUST_BRIDGE_H_
