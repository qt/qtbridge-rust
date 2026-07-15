// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_i16.h"

namespace rust::bridge::qlist_i16 {

void QList_Drop(QList_i16 &v)
{
    v.~QList_i16();
}

QList_i16 QList_Default()
{
    return QList_i16();
}

QList_i16 QList_Clone(const QList_i16 &src)
{
    return { src };
}

bool QList_Eq(const QList_i16 &lhs, const QList_i16 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_i16 &self, int16_t value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_i16 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_i16 const &self, int16_t const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_i16 &self, int16_t value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_i16 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_i16 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_i16 const &self)
{
    return self.size();
}

int16_t const &inlineCppFn_first(QList_i16 const &self)
{
    return self.constFirst();
}

int16_t const &inlineCppFn_last(QList_i16 const &self)
{
    return self.constLast();
}

rust::Vec<int16_t> inlineCppFn_TraitImpl_From_ref_QList_i16_for_Vec_i16_from(QList_i16 const &src)
{
    rust::Vec<int16_t> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (int16_t item : src)
        result.push_back(item);
    return result;
}

int16_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i16_index(QList_i16 const &self,
                                                                             size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_i16_N_for_QList_i16_eq(QList_i16 const &self,
                                                                     rust::Slice<int16_t const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_i16
