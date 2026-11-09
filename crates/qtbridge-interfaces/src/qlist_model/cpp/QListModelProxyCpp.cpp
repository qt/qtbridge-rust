// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QListModelProxyCpp.h"

namespace rust::bridge {

QListModelProxyCpp::QListModelProxyCpp(QListModelProxyRust* rustProxy)
    : QBaseProxy(rustProxy)
{}
QListModelProxyCpp::~QListModelProxyCpp() = default;

// Virtual methods
int32_t QListModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
QVariant QListModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QListModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QListModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QListModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}

// Access to base implementation of virtual functions
QHash<int32_t,QByteArray> QListModelProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QListModelProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QListModelProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}

// Access to base implementation of non virtual functions
void QListModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QListModelProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QListModelProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QListModelProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QListModelProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QListModelProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QListModelProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QListModelProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QListModelProxyCpp::endResetModel()
{
    Base::endResetModel();
}

} // namespace rust::bridge
