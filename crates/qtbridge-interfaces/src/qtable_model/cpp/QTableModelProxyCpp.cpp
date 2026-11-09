// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QTableModelProxyCpp.h"

namespace rust::bridge {

QTableModelProxyCpp::QTableModelProxyCpp(QTableModelProxyRust* rustProxy)
    : QBaseProxy(rustProxy)
{}
QTableModelProxyCpp::~QTableModelProxyCpp() = default;

// Virtual methods
QModelIndex QTableModelProxyCpp::index(int32_t row, int32_t column, const QModelIndex&) const
{
    return Base::createIndex(row, column);
}
QModelIndex QTableModelProxyCpp::parent(const QModelIndex& child) const
{
    return m_rustProxy->parent(child);
}
int32_t QTableModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
int32_t QTableModelProxyCpp::columnCount(const QModelIndex& parent) const
{
    return m_rustProxy->columnCount(parent);
}
QVariant QTableModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QTableModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QTableModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QTableModelProxyCpp::removeColumns(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeColumns(first, count, parent);
}
bool QTableModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}

// Access to base implementation of virtual functions
QHash<int32_t,QByteArray> QTableModelProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QTableModelProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QTableModelProxyCpp::base_removeColumns(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeColumns(first, count, parent);
}
bool QTableModelProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}

// Access to base implementation of non virtual functions
void QTableModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QTableModelProxyCpp::beginInsertColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertColumns(parent, first, last);
}
void QTableModelProxyCpp::endInsertColumns()
{
    Base::endInsertColumns();
}
void QTableModelProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QTableModelProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QTableModelProxyCpp::beginMoveColumns(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveColumns(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QTableModelProxyCpp::endMoveColumns()
{
    Base::endMoveColumns();
}
void QTableModelProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QTableModelProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QTableModelProxyCpp::beginRemoveColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveColumns(parent, first, last);
}
void QTableModelProxyCpp::endRemoveColumns()
{
    Base::endRemoveColumns();
}
void QTableModelProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QTableModelProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QTableModelProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QTableModelProxyCpp::endResetModel()
{
    Base::endResetModel();
}
QModelIndex QTableModelProxyCpp::createIndex(int32_t row, int32_t column, size_t ptr) const
{
    return Base::createIndex(row, column, ptr);
}

} // namespace rust::bridge
