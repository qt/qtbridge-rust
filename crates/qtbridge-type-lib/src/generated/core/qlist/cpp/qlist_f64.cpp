// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_f64.h"

namespace rust::bridge::qlist_f64 {

void QList_Drop(QList_f64 &v)
{
    v.~QList_f64();
}

QList_f64 QList_Default()
{
    return QList_f64();
}

QList_f64 QList_Clone(const QList_f64 &src)
{
    return { src };
}

bool QList_Eq(const QList_f64 &lhs, const QList_f64 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_f64 &self, double value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_f64 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_f64 const &self, double const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_f64 &self, double value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_f64 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_f64 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_f64 const &self)
{
    return self.size();
}

double const &inlineCppFn_first(QList_f64 const &self)
{
    return self.constFirst();
}

double const &inlineCppFn_last(QList_f64 const &self)
{
    return self.constLast();
}

rust::Vec<double> inlineCppFn_TraitImpl_From_ref_QList_f64_for_Vec_f64_from(QList_f64 const &src)
{
    rust::Vec<double> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (double item : src)
        result.push_back(item);
    return result;
}

double const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_f64_index(QList_f64 const &self,
                                                                            size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_f64_N_for_QList_f64_eq(QList_f64 const &self,
                                                                     rust::Slice<double const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_f64
