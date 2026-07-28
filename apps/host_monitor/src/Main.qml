// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

import QtQuick 2.15
import QtQuick.Controls.Fusion
import QtQuick.Layouts 1.15
import host_monitor

ApplicationWindow {
    id: root
    width: 600
    height: 200
    visible: true
    title: qsTr("Host Monitor")

    Backend {
        id: backend
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 20
        width: parent.width

        RowLayout {
            spacing: 10
            width: parent.width

            TextField {
                id: hostnameField
                text: backend.hostname
                font.pixelSize: 16
                color: palette.windowText
                Layout.fillWidth: true
                onEditingFinished: {
                    backend.hostname = hostnameField.text;
                }
            }

            Rectangle {
                id: statusIndicator
                width: 24
                height: 24
                radius: 12
                color: backend.status ? "green" : "red"
            }

            Text {
                id: latencyLabel
                text: backend.latency > 0 ? `${backend.latency} ms` : ""
                font.pixelSize: 20
                color: palette.windowText
            }
        }
        RowLayout {
            spacing: 10
            width: parent.width

            Item {
                Layout.fillWidth: true
            }
            Button {
                text: "Check Ping"
                enabled: !autoCheck.checked
                onClicked: {
                    backend.make_request();
                }
            }
            CheckBox {
                id: autoCheck
                text: "Check automatically"
            }
            Item {
                Layout.fillWidth: true
            }
        }
    }
    Timer {
        interval: 1000
        running: autoCheck.checked
        repeat: true
        onTriggered: {
            backend.make_request();
        }
    }
}
