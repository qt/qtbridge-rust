// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_qstring.h"
#include <QMetaType>

namespace rust::bridge::qlist_qstring {

static_assert(qMetaTypeId<QList_QString>() == 11);

void QList_Drop(QList_QString &v)
{
    v.~QList_QString();
}

QList_QString QList_Default()
{
    return QList_QString();
}

QList_QString QList_Clone(const QList_QString &src)
{
    return { src };
}

bool QList_Eq(const QList_QString &lhs, const QList_QString &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_QString &self, QString value)
{
    self.append(value);
}

size_t inlineCppFn_capacity(QList_QString const &self)
{
    return self.capacity();
}

void inlineCppFn_clear(QList_QString &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_QString const &self, QString const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_QString &self, QString value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_QString &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_QString &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_QString const &self)
{
    return self.size();
}

QString const &inlineCppFn_first(QList_QString const &self)
{
    return self.constFirst();
}

QString const &inlineCppFn_last(QList_QString const &self)
{
    return self.constLast();
}

QString const *
inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_QString_index(QList_QString const &self,
                                                                  size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_QString_N_for_QList_QString_eq(
        QList_QString const &self, rust::Slice<QString const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_qstring
