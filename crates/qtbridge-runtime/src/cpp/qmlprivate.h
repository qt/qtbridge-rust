// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMLPRIVATE_RUST_BRIDGE_H_
#define _QMLPRIVATE_RUST_BRIDGE_H_

#include <cstdint>
#include <qqmlprivate.h>
#include <QMetaType>
#include "qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h"
#include "rust/cxx.h"
#include "rustconv.h"

namespace rust::bridge::qmlprivate {

void qml_register_element(QMetaType type_id, QMetaType list_id, uint32_t object_size,
                          int32_t parser_status_cast, size_t create_fn,
                          rust::Slice<uint8_t const> uri, uint8_t version_major,
                          uint8_t version_minor, rust::Slice<uint8_t const> elm_name,
                          QMetaObject const &meta_object);

void qml_register_singleton(QMetaType type_id, size_t create_fn,
                            rust::Slice<uint8_t const> uri, uint8_t version_major,
                            uint8_t version_minor, rust::Slice<uint8_t const> elm_name,
                            QMetaObject const &meta_object);

} // namespace rust::bridge::qmlprivate

#endif // _QMLPRIVATE_RUST_BRIDGE_H_
