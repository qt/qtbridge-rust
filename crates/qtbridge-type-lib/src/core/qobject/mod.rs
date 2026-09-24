// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pub use cxx_qt::QObject;

pub mod qobject;
pub use qobject::{delete, destruct, meta_object, property, set_property};
