This library enables building modern Qt Quick user interfaces with a Rust backend,
without writing any C++ code. It allows you to run [QML](https://doc.qt.io/qt-6/qmlreference.html)
code and expose Rust types to QML using simple attribute macros.

## At a glance

Main.rs
```rust
use qtbridge::{qobject, QApp};

#[derive(Default)]
pub struct Backend {
}

#[qobject(Singleton)]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
```

Main.qml
```qml, ignore
import QtQuick
import QtQuick.Controls
import hello_world

ApplicationWindow {

    visible: true
    title: qsTr("Minimal QML app")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.say_hello()
    }
}
```

## Getting started

To use this library, you need:

- One of the supported platforms:
   - Linux (`x86_64`)
   - Windows (`x64`)
   - macOS (`arm64`) - experimental
- **Rust and Cargo** (stable, version >= 1.88) from [rustup.rs](https://rustup.rs)
- **C++ toolchain** (see [Qt platform requirements](https://doc.qt.io/qt-6/supported-platforms.html))
- [**Qt 6**](#qt-installation)

### Qt installation

A C++ toolchain and a Qt installation must be present in the system,
and `qmake` must be in the system `PATH`. QtBridge currently requires Qt 6.10
or higher.

Qt can be built from source, downloaded from <https://download.qt.io/> or installed
via your system's package manager.

#### Using the system package manager

Most Linux distributions provide a compatible Qt installation through their
system package manager. You need `qtbase`, `qtbase-private`, and `qtdeclarative`
as a baseline for the provided examples, plus any additional
QML modules you intend to use.

On Fedora 43 and 44 the required packages are:
```sh
sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel
sudo dnf install qt6-qtbase-private-devel
```

On Ubuntu 26.04 and Debian Testing (Forky) the packages are:
```sh
sudo apt install -y qt6-base-dev qt6-declarative-dev
sudo apt install -y qt6-base-private-dev
```

On Ubuntu and Debian, the Qt 6 qmake binary is named `qmake6` and the `QMAKE`
environment variable must be set correspondingly:
```sh
export QMAKE=qmake6
```
To make this permanent, add it to your ~/.bashrc:
```sh
echo 'export QMAKE=qmake6' >> ~/.bashrc
source ~/.bashrc
```

#### Qt build from source or Qt installer

##### Windows

On Windows, you need to add the binary path of the targeted Qt installation to
`PATH`. For the default installation of 6.10.1 by the Qt installer you have to
execute the following command. The exact path depends on the targeted installation.

Command Prompt:
```sh
set PATH=%PATH%;C:\Qt\6.10.1\msvc2022_64\bin\
```

PowerShell:
```sh
$env:PATH += ";C:\Qt\6.10.1\msvc2022_64\bin\"
```

##### Linux

On Linux, you need to add the binary path of the targeted Qt installation to
`PATH` and the library path to `LD_LIBRARY_PATH`:
```sh
export PATH=/home/john_doe/dev/qt_build/qtbase/bin:$PATH
export LD_LIBRARY_PATH="/home/john_doe/dev/qt_build/qtbase/lib:$LD_LIBRARY_PATH"
```

##### macOS

On macOS, you need to add the binary path of the targeted Qt installation to
`PATH` and the library path  `DYLD_FRAMEWORK_PATH`:
```sh
export PATH=/Users/john_doe/dev/qt_build/qtbase/bin:$PATH
export DYLD_FRAMEWORK_PATH=/Users/john_doe/dev/qt_build/qtbase/lib:$DYLD_FRAMEWORK_PATH
```

## Dependency

QtBridge has a single crate with all public APIs:
```TOML
[dependencies]
qtbridge = "*"
```

## Building the documentation

You can rely on cargo to generate the documentation of the project,
but make sure to include the features the project currently have. You can
achieve that by running:

```bash
cargo doc --features serde_json --no-deps
```

## API overview

Running QML code and starting any Qt-based application is generally done through the
[`QApp`] type.

Rust types that should be used in QML must either be defined within a `mod`,
or have an `impl` block, annotated with a [`qobject`] attribute macro. Within
those you can use the [`qproperty`], [`qslot`], and [`qsignal`] attribute macros
to define how the Rust types appear in QML.

The library provides some special traits that enable Rust types to fulfill specific
roles; see [special_traits].

All QObjects in Rust (implemented with [`qobject`]) are held in
`Rc<RefCell<_>>`. This allows multiple owners to coexist: your Rust code,
the QML engine, and any number of QML components can all hold a reference
to the same object. QML references follow ordinary JavaScript copy semantics.

A Rust object is only borrowed for the duration of a signal, slot or property
invocation from QML. Outside of those calls the object is not borrowed, so
Rust code can freely take mutable borrows between QML interactions.

`RefCell` enforces at runtime that a value is either shared by any number of
readers or held by a single writer, never both. Writing a property or invoking
a `&mut self` slot takes an exclusive borrow; reading a property or invoking a
`&self` slot takes only a shared borrow. When QML enters Rust through an object,
the borrow is cached and reborrowed for further calls on the same object,
keeping a single borrow on the stack instead of taking a new one. A conflict,
which surfaces as a panic, occurs when a call takes a fresh, independent borrow
rather than reborrowing the active one, which happens when the call chain is
interrupted.

## Provided examples

Note: In this repository the examples depend on `qtbridge` through a relative
path, since they build against the in-repo crate. In your own project, resolve
the dependency from crates.io instead, as shown in the [Dependency](#dependency)
section (`qtbridge = "*"`).

### [Hello World!](https://github.com/qt/qtbridge-rust/tree/dev/apps/hello_world)

The classic "Hello World!" example showing the minimal building bricks for a
QtBridge application.

### [Minimal App](https://github.com/qt/qtbridge-rust/tree/dev/apps/minimal_app)

A working backend with data displayed in QML.

### [Host Monitor](https://github.com/qt/qtbridge-rust/tree/dev/apps/host_monitor)

This example shows how to combine a Qt UI with [tokio](https://tokio.rs/) runtime
in a multithreaded environment using [`QmlMethodInvoker`].

### [Color Palette](https://github.com/qt/qtbridge-rust/tree/dev/apps/color_palette)

Port of the C++ Color Palette example to Rust. Shows a moderately complex application
using tokio, reqwest, and serde_json, as well as many QML constructs, such as complex
and default properties, including nested property structures.

## Further information

QtBridge builds on [CXX](https://cxx.rs/) to access the required Qt interfaces.

If your project requires mixing Rust and C++ code, using Qt Widgets, or accessing Qt
modules that only provide a C++ API, consider using
[CXX-Qt](https://github.com/KDAB/cxx-qt) instead.

Internally, the library relies on Qt concepts such as
[QObjects](https://doc.qt.io/qt-6/qobject.html),
[properties](https://doc.qt.io/qt-6/properties.html),
[signals and slots](https://doc.qt.io/qt-6/signalsandslots.html),
and the [Model/View architecture](https://doc.qt.io/qt-6/model-view-programming.html).
While these are exposed through a Rust-friendly API, familiarity with these Qt concepts
will help you get the most out of building UIs with Qt Quick.

## Future plans

* Streamline API.
* Enable interoperability with [CXX-Qt](https://crates.io/crates/cxx-qt).
* Extend IDE support in particular for VS Code. Enable the QML language server to understand
  types generated in Rust.
* Reduce the manual steps for installing and linking to Qt.

## Terms and Conditions

If you, your employer, or the legal entity you act on behalf of hold commercial license(s) with a Qt
Group entity, Qt Bridges constitutes Pre-Release Code under the Qt License/Frame Agreement governing
those licenses, and that agreement's terms and conditions relating to Pre-Release Code apply to your
use of Qt Bridges as found in this repo.
This Qt Bridges repo may provide links or access to third-party libraries or code (collectively
"Third-Party Software") to implement various functions. Use or distribution of Third-Party Software
is discretionary and in all respects subject to applicable license terms of applicable third-party
right holders.

### Additional Terms and Conditions

Qt Bridge for Rust is built using the Rust language and SDK, which is maintained by the Rust
Foundation.

Qt Bridge for Rust resides on top of Rust and does not modify it in any form. Rust is a trademark of
the Rust Foundation. This project is not affiliated with or endorsed by the Rust Foundation.

An application built with Qt Bridge for Rust will include code from other crates.
The main dependency is CXX, "Safe interop between Rust and C++", licensed under the Apache Version
2.0 License or MIT license.
