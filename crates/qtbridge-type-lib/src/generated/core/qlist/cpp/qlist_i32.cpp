// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_i32.h"

namespace rust::bridge::qlist_i32 {

void QList_Drop(QList_i32 &v)
{
    v.~QList_i32();
}

QList_i32 QList_Default()
{
    return QList_i32();
}

QList_i32 QList_Clone(const QList_i32 &src)
{
    return { src };
}

bool QList_Eq(const QList_i32 &lhs, const QList_i32 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_i32 &self, int32_t value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_i32 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_i32 const &self, int32_t const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_i32 &self, int32_t value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_i32 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_i32 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_i32 const &self)
{
    return self.size();
}

int32_t const &inlineCppFn_first(QList_i32 const &self)
{
    return self.constFirst();
}

int32_t const &inlineCppFn_last(QList_i32 const &self)
{
    return self.constLast();
}

rust::Vec<int32_t> inlineCppFn_TraitImpl_From_ref_QList_i32_for_Vec_i32_from(QList_i32 const &src)
{
    rust::Vec<int32_t> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (int32_t item : src)
        result.push_back(item);
    return result;
}

int32_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i32_index(QList_i32 const &self,
                                                                             size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_i32_N_for_QList_i32_eq(QList_i32 const &self,
                                                                     rust::Slice<int32_t const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_i32
