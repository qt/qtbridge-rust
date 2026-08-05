// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _REGISTRY_RUST_BRIDGE_H_
#define _REGISTRY_RUST_BRIDGE_H_

class QObject;
class QQmlApplicationEngine;

namespace rust::bridge::registry {

bool hasLiveJsWrapper(const QObject* obj);
void setCppOwnership(QObject* obj);
void setJavaScriptOwnership(QObject* obj);
bool isJavaScriptOwnership(QObject* obj);
void installGcSentinel(QQmlApplicationEngine& engine);

} // namespace rust::bridge::registry

#endif // _REGISTRY_RUST_BRIDGE_H_
