// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qmlprivate.h"

namespace rust::bridge::qmlprivate {

void qml_register_element(QMetaType type_id, QMetaType list_id, uint32_t object_size,
                          int32_t parser_status_cast, size_t create_fn,
                          rust::Slice<uint8_t const> uri, uint8_t version_major,
                          uint8_t version_minor, rust::Slice<uint8_t const> elm_name,
                          QMetaObject const &meta_object)
{
    const QByteArray uriBa = RustByteSliceToQByteArray(uri);
    const QByteArray elmNameBa = RustByteSliceToQByteArray(elm_name);
    QQmlPrivate::RegisterType rt = {};
    rt.structVersion = QQmlPrivate::RegisterType::CurrentVersion;
    rt.typeId = type_id;
    rt.listId = list_id;
    rt.objectSize = object_size;
    rt.create = reinterpret_cast<void (*)(void *, void *)>(create_fn);
    rt.userdata = nullptr;
    rt.noCreationReason = QString();
    rt.createValueType = nullptr;
    rt.uri = uriBa.data();
    rt.version = QTypeRevision::fromVersion(version_major, version_minor);
    rt.elementName = elmNameBa;
    rt.metaObject = &meta_object;
    rt.attachedPropertiesFunction = nullptr;
    rt.attachedPropertiesMetaObject = nullptr;
    rt.parserStatusCast = parser_status_cast;
    rt.valueSourceCast = -1;
    rt.valueInterceptorCast = -1;
    rt.extensionObjectCreate = nullptr;
    rt.extensionMetaObject = nullptr;
    rt.customParser = nullptr;
    rt.revision = QTypeRevision::fromVersion(0, 0);
    rt.finalizerCast = -1;
    rt.creationMethod = QQmlPrivate::ValueTypeCreationMethod::None;
    QQmlPrivate::qmlregister(QQmlPrivate::TypeRegistration, &rt);
}

void qml_register_singleton(QMetaType type_id, size_t create_fn,
                            rust::Slice<uint8_t const> uri, uint8_t version_major,
                            uint8_t version_minor, rust::Slice<uint8_t const> elm_name,
                            QMetaObject const &meta_object)
{
    auto createQmlSingletonType = [create_fn](QQmlEngine *, QJSEngine *) -> QObject * {
        auto ctr = reinterpret_cast<QObject *(*)()>(create_fn);
        return ctr();
    };
    const QByteArray uriBa = RustByteSliceToQByteArray(uri);
    const QByteArray elmNameBa = RustByteSliceToQByteArray(elm_name);
    QQmlPrivate::RegisterSingletonType api = {};
    api.structVersion = 0;
    api.uri = uriBa.data();
    api.version = QTypeRevision::fromVersion(version_major, version_minor);
    api.typeName = elmNameBa;
    api.scriptApi = nullptr;
    api.qObjectApi = createQmlSingletonType;
    api.instanceMetaObject = &meta_object;
    api.typeId = type_id;
    api.extensionObjectCreate = nullptr;
    api.extensionMetaObject = nullptr;
    api.revision = QTypeRevision::fromVersion(0, 0);
    QQmlPrivate::qmlregister(QQmlPrivate::SingletonRegistration, &api);
}

} // namespace rust::bridge::qmlprivate
