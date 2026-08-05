// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "registry.h"

#include <QJSEngine>
#include <QQmlApplicationEngine>
#include <private/qqmldata_p.h>

#include "qtbridge-runtime/src/registry.rs.h"

namespace rust::bridge::registry {

namespace {

// Replacement for a "garbage collection finished" signal, which the engine
// does not offer.
//
// Adds a dummy QObject with JavaScriptOwnership to the engine, whose
// JS wrapper is referenced by nothing. The next garbage collection frees the
// wrapper and thereby deletes the sentinel; its destroyed() signal runs the
// registry's collect_garbage() and re-arms a new sentinel one event-loop turn
// later, so the trigger is exactly-once-per-collection and costs nothing
// while no collection happens.
void armGcSentinel(QJSEngine* engine)
{
    auto* sentinel = new QObject();
    sentinel->setObjectName("GC_Sentinel");
    QJSEngine::setObjectOwnership(sentinel, QJSEngine::JavaScriptOwnership);
    // Create the wrapper and drop the only reference to it.
    (void)engine->newQObject(sentinel);
    QObject::connect(sentinel, &QObject::destroyed, engine, [engine] {
        collect_garbage();
        // Re-arm from the event loop: during engine teardown the queued
        // call is dropped (dead context), ending the cycle; arming
        // directly from inside the collection would wrap mid-sweep.
        QMetaObject::invokeMethod(engine, [engine] {
            armGcSentinel(engine);
        }, Qt::QueuedConnection);
    });
}

} // namespace

void setCppOwnership(QObject* obj)
{
    QJSEngine::setObjectOwnership(obj, QJSEngine::CppOwnership);
}

void setJavaScriptOwnership(QObject* obj)
{
    QJSEngine::setObjectOwnership(obj, QJSEngine::JavaScriptOwnership);
}

bool isJavaScriptOwnership(QObject* obj)
{
    return QJSEngine::objectOwnership(obj) == QJSEngine::JavaScriptOwnership;
}

bool hasLiveJsWrapper(const QObject* obj)
{
    const QQmlData* ddata = QQmlData::get(obj);
    if (!ddata)
        return false;
    if (!ddata->jsWrapper.isNullOrUndefined())
        return true;
    // Wrapped by more than one engine: the per-engine wrappers live in the
    // multiply-wrapped map, not in jsWrapper. Conservatively keep alive.
    return ddata->hasTaintedV4Object;
}

void installGcSentinel(QQmlApplicationEngine& engine)
{
    armGcSentinel(&engine);
}

} // namespace rust::bridge::registry
