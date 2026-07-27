// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls.Fusion
import QtQuick.Layouts
import QtQml.Models
import minimal_app

ApplicationWindow {
    id: root

    Backend {
        id: backend
        onDuplicateFound: (duplicate) => {
            statusDisplay.text = `List already contains entry ${duplicate}!`
            clearStatusTimer.restart();
        }
    }

    Timer {
        id: clearStatusTimer
        interval: 1500
        onTriggered: statusDisplay.text = ""
    }

    width: 640
    height: 480
    visible: true
    title: qsTr("Minimal QML app")
    // ### NOTE : editing should ideally use a Validator; but that's out of scope
    ColumnLayout {
        id: mainLayout
        anchors.fill: parent
        ListView {
            id: lv
            model: backend
            delegate: Control {
                id: ld
                required property var model
                required property int index
                width: lv.width
                implicitHeight: Math.max(textLabel.implicitHeight, deleteButton.implicitHeight)
                RowLayout {
                    anchors.fill: parent
                    Label {
                        property bool inEditMode: false
                        id: textLabel
                        text: inEditMode ? "" : `Item ${ld.index+1}: ${ld.model.value}`
                        Layout.fillWidth: true
                        TextInput {
                            id: editField
                            visible: textLabel.inEditMode
                            anchors.fill: parent
                            onEditingFinished: {
                                textLabel.inEditMode = false
                            }
                            onAccepted: {
                                ld.model.value = text
                            }
                        }
                    }
                    RoundButton {
                        text: "✏️"
                        down: textLabel.inEditMode || pressed
                        onReleased: () => {
                            if (textLabel.inEditMode) {
                                textLabel.inEditMode = false
                            } else {
                                editField.text = `${ld.model.value}`
                                textLabel.inEditMode = true
                                editField.forceActiveFocus()
                                editField.selectAll()
                            }
                        }
                    }
                    RoundButton {
                        id: deleteButton
                        text: "🗑️" // could use icons if we could ensure we have them
                        onReleased: () => { lv.model.removeRow(ld.index) }
                    }
                }
            }
            Layout.fillHeight: true
            Layout.preferredWidth: mainLayout.width
        }

        RowLayout {
            implicitHeight: Math.max(input.implicitHeight, submitButton.implicitHeight)
            TextField {
                id: input
                placeholderText: "Enter string to add"

                function submit() {
                    backend.addString(input.text)
                    input.clear()
                }

                onAccepted: submit()
                Layout.preferredWidth: mainLayout.width * 0.8
            }
            Button {
                id: submitButton
                text: "Add text"
                enabled: input.text !== ""
                onReleased: input.submit()
            }
        }
    }

    footer: Text {
        id: statusDisplay
    }
}
