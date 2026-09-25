// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qmetatypeinterface.h"

#include <QByteArray>

namespace rust::bridge::qmetatypeinterface {

using ::QtPrivate::QMetaTypeInterface;

QMetaTypeInterface fillFields(size_t align, size_t size, uint32_t flags, rust::Str name,
                              size_t meta_obj_fn, size_t default_ctr_fn, size_t copy_ctr_fn,
                              size_t dtor_fn)
{
    auto metaObjFn = reinterpret_cast<QMetaTypeInterface::MetaObjectFn>(meta_obj_fn);
    auto defaultCtr = reinterpret_cast<QMetaTypeInterface::DefaultCtrFn>(default_ctr_fn);
    auto copyCtr = reinterpret_cast<QMetaTypeInterface::CopyCtrFn>(copy_ctr_fn);
    auto dtor = reinterpret_cast<QMetaTypeInterface::DtorFn>(dtor_fn);
    // Leaked: Qt keeps the name for the lifetime of the program.
    auto *typeName = new QByteArray(name.data(), qsizetype(name.size()));
    return QMetaTypeInterface{ QMetaTypeInterface::CurrentRevision,
                               static_cast<ushort>(align),
                               static_cast<uint>(size),
                               flags,
                               { 0 },
                               metaObjFn,
                               typeName->constData(),
                               defaultCtr,
                               copyCtr,
                               nullptr,
                               dtor,
                               nullptr,
                               nullptr,
                               nullptr,
                               nullptr,
                               nullptr,
                               nullptr };
}

} // namespace rust::bridge::qmetatypeinterface
