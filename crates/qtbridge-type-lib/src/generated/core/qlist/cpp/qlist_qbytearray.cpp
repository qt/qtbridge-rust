// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_qbytearray.h"
#include <QMetaType>

namespace rust::bridge::qlist_qbytearray {

static_assert(qMetaTypeId<QList_QByteArray>() == 49);

void QList_Drop(QList_QByteArray &v)
{
    v.~QList_QByteArray();
}

QList_QByteArray QList_Default()
{
    return QList_QByteArray();
}

QList_QByteArray QList_Clone(const QList_QByteArray &src)
{
    return { src };
}

bool QList_Eq(const QList_QByteArray &lhs, const QList_QByteArray &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_QByteArray &self, QByteArray value)
{
    self.append(value);
}

size_t inlineCppFn_capacity(QList_QByteArray const &self)
{
    return self.capacity();
}

void inlineCppFn_clear(QList_QByteArray &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_QByteArray const &self, QByteArray const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_QByteArray &self, QByteArray value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_QByteArray &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_QByteArray &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_QByteArray const &self)
{
    return self.size();
}

QByteArray const &inlineCppFn_first(QList_QByteArray const &self)
{
    return self.constFirst();
}

QByteArray const &inlineCppFn_last(QList_QByteArray const &self)
{
    return self.constLast();
}

QByteArray const *
inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_QByteArray_index(QList_QByteArray const &self,
                                                                     size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_QByteArray_N_for_QList_QByteArray_eq(
        QList_QByteArray const &self, rust::Slice<QByteArray const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_qbytearray
