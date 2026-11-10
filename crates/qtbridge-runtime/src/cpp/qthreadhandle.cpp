// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qthreadhandle.h"

#include "qtbridge-runtime/src/qthreadhandle.rs.h"

#include <QCoreApplication>
#include <QMutex>
#include <QThread>

#include <utility>

Q_CONSTINIT static QMutex g_appMutex;
Q_CONSTINIT static QCoreApplication *g_app = nullptr;

static void onAppShutdown()
{
    QMutexLocker lock(&g_appMutex);
    // After this function is called, g_appMutex stays unlocked
    // but we cannot post any events to g_app anymore. Set
    // it to nullptr to allow no more events. Don't use
    // QCoreApplication::instance() because it is callable after
    // this function is executed.
    g_app = nullptr;
}

static void onAppStartup()
{
    QMutexLocker lock(&g_appMutex);
    g_app = QCoreApplication::instance();
    qAddPostRoutine(onAppShutdown);
}
Q_COREAPP_STARTUP_FUNCTION(onAppStartup)

std::unique_ptr<QThreadHandleCpp> make_qt_thread_handle(QObject *target)
{
    // Closures run on the application's thread, so only its objects may be targeted.
    // We assume that no QObject are moved to other threads.
    QMutexLocker lock(&g_appMutex);
    if (target && !g_app) {
        qWarning("QThreadHandle: no QApplication is running, "
                 "all calls will be discarded");
        target = nullptr;
    } else if (target && target->thread() != g_app->thread()) {
        qWarning("QThreadHandle: target does not live on the QApplication thread, "
                 "all calls will be discarded");
        target = nullptr;
    }
    return std::make_unique<QThreadHandleCpp>(target);
}

void post_closure(const QThreadHandleCpp &handle, rust::Box<QObjectClosureBox> closure)
{
    // The lock keeps the application alive until the event is posted.
    QMutexLocker lock(&g_appMutex);
    if (!g_app) {
        qWarning("QThreadHandle: no QApplication is running, the calls are discarded");
        return; // no event loop running. Box can be dropped
    }

    // Use the application as context object, the target could be destroyed
    // between a check and the invokeMethod call.
    QMetaObject::invokeMethod(g_app, [target = handle.target, closure = std::move(closure)]() mutable {
        // Runs on the Qt thread. We assume that QObjects can only
        // be destroyed in this thread.
        if (QObject *obj = target.data()) {
            Q_ASSERT(obj->thread()->isCurrentThread());
            closure->run(*obj);
        }
    }, Qt::QueuedConnection);
}

void post_void_closure(rust::Box<VoidClosureBox> closure)
{
    QMutexLocker lock(&g_appMutex);
    if (!g_app) {
        qWarning("QThreadHandle: no QApplication is running, the calls are discarded");
        return;
    }

    // The lambda only runs while its context object, the application, is alive.
    QMetaObject::invokeMethod(g_app, [closure = std::move(closure)]() mutable {
        closure->run();
    }, Qt::QueuedConnection);
}
