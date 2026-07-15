// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_i64.h"

namespace rust::bridge::qlist_i64 {

void QList_Drop(QList_i64 &v)
{
    v.~QList_i64();
}

QList_i64 QList_Default()
{
    return QList_i64();
}

QList_i64 QList_Clone(const QList_i64 &src)
{
    return { src };
}

bool QList_Eq(const QList_i64 &lhs, const QList_i64 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_i64 &self, int64_t value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_i64 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_i64 const &self, int64_t const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_i64 &self, int64_t value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_i64 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_i64 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_i64 const &self)
{
    return self.size();
}

int64_t const &inlineCppFn_first(QList_i64 const &self)
{
    return self.constFirst();
}

int64_t const &inlineCppFn_last(QList_i64 const &self)
{
    return self.constLast();
}

rust::Vec<int64_t> inlineCppFn_TraitImpl_From_ref_QList_i64_for_Vec_i64_from(QList_i64 const &src)
{
    rust::Vec<int64_t> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (int64_t item : src)
        result.push_back(item);
    return result;
}

int64_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i64_index(QList_i64 const &self,
                                                                             size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_i64_N_for_QList_i64_eq(QList_i64 const &self,
                                                                     rust::Slice<int64_t const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_i64
