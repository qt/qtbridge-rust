// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QABSTRACTITEMMODELPROXYCPP_RUST_BRIDGE__H_
#define _QABSTRACTITEMMODELPROXYCPP_RUST_BRIDGE__H_
#include <QAbstractItemModel>
#include <QByteArray>
#include <QHash>
#include <QMetaObject>
#include <QModelIndex>
#include <QVariant>
#include <QQmlListProperty>
#include <cstdint>
#include "qtbridge-interfaces/src/cpp/qbaseproxy.h"
#include "qtbridge-interfaces/src/qabstract_item_model/proxy_rust_bridge.rs.h"


namespace rust::bridge {

class QAbstractItemModelProxyCpp : public QAbstractItemModel, public QBaseProxy<QAbstractItemModelProxyCpp, QAbstractItemModelProxyRust>
{
    using Base = QAbstractItemModel;

public:
    QAbstractItemModelProxyCpp(QAbstractItemModelProxyRust* rustProxy);
    ~QAbstractItemModelProxyCpp();

    // Virtual methods
    QModelIndex index(int32_t row, int32_t column, const QModelIndex& parent) const override;
    QModelIndex parent(const QModelIndex& child) const override;
    int32_t rowCount(const QModelIndex& parent) const override;
    int32_t columnCount(const QModelIndex& parent) const override;
    QVariant data(const QModelIndex& index, int32_t role) const override;
    QHash<int32_t,QByteArray> roleNames() const override;
    bool setData(const QModelIndex& index, const QVariant& value, int32_t role) override;
    bool removeRows(int32_t first, int32_t count, const QModelIndex& parent) override;
    QModelIndex sibling(int32_t row, int32_t column, const QModelIndex& idx) const override;

    // Access to base implementation of virtual functions
    QHash<int32_t,QByteArray> base_roleNames() const;
    bool base_setData(const QModelIndex& index, const QVariant& value, int32_t role);
    bool base_removeRows(int32_t first, int32_t count, const QModelIndex& parent);
    QModelIndex base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const;

    // Access to base implementation of non virtual functions
    void dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight);
    void beginInsertColumns(const QModelIndex& parent, int32_t first, int32_t last);
    void endInsertColumns();
    void beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endInsertRows();
    bool beginMoveColumns(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild);
    void endMoveColumns();
    bool beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild);
    void endMoveRows();
    void beginRemoveColumns(const QModelIndex& parent, int32_t first, int32_t last);
    void endRemoveColumns();
    void beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endRemoveRows();
    void beginResetModel();
    void endResetModel();
    QModelIndex createIndex(int32_t row, int32_t column, size_t ptr) const;
};

} // namespace rust::bridge

#endif // _QABSTRACTITEMMODELPROXYCPP_RUST_BRIDGE__H_
