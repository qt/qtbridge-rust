// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMETATYPEINTERFACE_RUST_BRIDGE_H_
#define _QMETATYPEINTERFACE_RUST_BRIDGE_H_

#include <QMetaType>
#include <cstdint>
#include "rust/cxx.h"

namespace rust::bridge::qmetatypeinterface {

::QtPrivate::QMetaTypeInterface fillFields(size_t align, size_t size, uint32_t flags,
                                           rust::Str name, size_t meta_obj_fn,
                                           size_t default_ctr_fn, size_t copy_ctr_fn,
                                           size_t dtor_fn);

} // namespace rust::bridge::qmetatypeinterface

namespace rust {

template <>
struct IsRelocatable<::QtPrivate::QMetaTypeInterface> : ::std::true_type
{
};

} // namespace rust

#endif // _QMETATYPEINTERFACE_RUST_BRIDGE_H_
