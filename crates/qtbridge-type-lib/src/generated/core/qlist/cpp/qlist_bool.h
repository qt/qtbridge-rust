// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_BOOL_RUST_BRIDGE_H_
#define _QLIST_BOOL_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_bool = ::QList<bool>;

namespace rust::bridge::qlist_bool {

void QList_Drop(QList_bool &v);
QList_bool QList_Default();
QList_bool QList_Clone(const QList_bool &src);
bool QList_Eq(const QList_bool &lhs, const QList_bool &rhs);

void inlineCppFn_append(QList_bool &self, bool value);

void inlineCppFn_clear(QList_bool &self);

bool inlineCppFn_contains(QList_bool const &self, bool const &value);

void inlineCppFn_push_back(QList_bool &self, bool value);

void inlineCppFn_remove(QList_bool &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_bool &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_bool const &self);

bool const &inlineCppFn_first(QList_bool const &self);

bool const &inlineCppFn_last(QList_bool const &self);

rust::Vec<bool> inlineCppFn_TraitImpl_From_ref_QList_bool_for_Vec_bool_from(QList_bool const &src);

bool const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_bool_index(QList_bool const &self,
                                                                           size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_bool_N_for_QList_bool_eq(QList_bool const &self,
                                                                       rust::Slice<bool const> rhs);

} // namespace rust::bridge::qlist_bool

namespace rust {

template <>
struct IsRelocatable<::QList_bool> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_BOOL_RUST_BRIDGE_H_
