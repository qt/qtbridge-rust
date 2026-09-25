// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qqmllistproperty.h"

namespace rust::bridge::qqmllistproperty {

QVariant listPropertyToQVariant(QMetaType const &meta_type, QObject *object, uint8_t *data,
                                size_t append_fn, size_t count_fn, size_t at_fn, size_t clear_fn)
{
    auto appendFn = reinterpret_cast<QQmlListProperty<QObject>::AppendFunction>(append_fn);
    auto countFn = reinterpret_cast<QQmlListProperty<QObject>::CountFunction>(count_fn);
    auto atFn = reinterpret_cast<QQmlListProperty<QObject>::AtFunction>(at_fn);
    auto clearFn = reinterpret_cast<QQmlListProperty<QObject>::ClearFunction>(clear_fn);
    QQmlListProperty<QObject> prop(object, static_cast<void *>(data), appendFn, countFn, atFn,
                                   clearFn);
    return QVariant(meta_type, &prop);
}

::QtPrivate::QMetaTypeInterface listPropertyInterfaceFor(QMetaType const &element)
{
    const auto *base = QMetaType::fromType<QQmlListProperty<QObject>>().iface();
    auto *name = new QByteArray(QByteArrayLiteral("QQmlListProperty<") + element.name() + '>');
    return ::QtPrivate::QMetaTypeInterface{ base->revision,
                                            base->alignment,
                                            base->size,
                                            base->flags,
                                            { 0 },
                                            base->metaObjectFn,
                                            name->constData(),
                                            base->defaultCtr,
                                            base->copyCtr,
                                            base->moveCtr,
                                            base->dtor,
                                            base->equals,
                                            base->lessThan,
                                            base->debugStream,
                                            base->dataStreamOut,
                                            base->dataStreamIn,
                                            base->legacyRegisterOp };
}

} // namespace rust::bridge::qqmllistproperty
