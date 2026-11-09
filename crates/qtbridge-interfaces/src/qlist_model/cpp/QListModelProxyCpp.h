// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#define _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#include <QAbstractListModel>
#include <QByteArray>
#include <QHash>
#include <QMetaObject>
#include <QModelIndex>
#include <QVariant>
#include <QQmlListProperty>
#include <cstdint>
#include "qtbridge-interfaces/src/cpp/qbaseproxy.h"
#include "qtbridge-interfaces/src/qlist_model/proxy_rust_bridge.rs.h"

namespace rust::bridge {

class QListModelProxyCpp : public QAbstractListModel, public QBaseProxy<QListModelProxyCpp, QListModelProxyRust>
{
    using Base = QAbstractListModel;
public:
    QListModelProxyCpp(QListModelProxyRust* rustProxy);
    ~QListModelProxyCpp();

    // Virtual methods
    int32_t rowCount(const QModelIndex& parent) const override;
    QVariant data(const QModelIndex& index, int32_t role) const override;
    QHash<int32_t,QByteArray> roleNames() const override;
    bool setData(const QModelIndex& index, const QVariant& value, int32_t role) override;
    bool removeRows(int32_t first, int32_t count, const QModelIndex& parent) override;

    // Access to base implementation of virtual functions
    QHash<int32_t,QByteArray> base_roleNames() const;
    bool base_setData(const QModelIndex& index, const QVariant& value, int32_t role);
    bool base_removeRows(int32_t first, int32_t count, const QModelIndex& parent);

    // Access to base implementation of non virtual functions
    void dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight);
    void beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endInsertRows();
    void beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild);
    void endMoveRows();
    void beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endRemoveRows();
    void beginResetModel();
    void endResetModel();
};

} // namespace rust::bridge

#endif // _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
