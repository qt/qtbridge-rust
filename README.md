This library enables building modern Qt Quick user interfaces with a Rust backend.
It allows you to run [QML](https://doc.qt.io/qt-6/qmlreference.html) code and use
Rust types in QML. This combines a declarative UI with Rust-based business logic.

Running QML code and starting any Qt-based application is generally done through the
[`QApp`] type.

Rust types that should be used in QML must be annotated with either a [`qobject`] or
[`qobject_impl`] attribute macro. Within those blocks you can use the [`qproperty`],
[`qslot`], and [`qsignal`] attribute macros to define how the Rust types appear in QML.

The library provides some special traits that enable Rust types to fulfill specific
roles. These are [`QListModel`] and [`QTableModel`].

## API Example:

Exposing a Rust type to QML and starting the declarative UI is as simple as:

Main.rs
```rust
use qtbridge::{qobject_impl, QApp};

#[derive(Default)]
pub struct Backend {
}

#[qobject_impl(Singleton)]
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

The UI can then be described in a declarative way in QML:

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
        onClicked: Backend.sayHello()
    }
}
```

## Getting started

### Prerequisites

To use this library, you need:

- One of the supported platforms:
   - Linux (`x86_64`)
   - Windows (`x64`)
   - macOS (`arm64`) - experimental
- **Rust** (stable, version >= 1.87)
   - Visit [rustup.rs](https://rustup.rs) to install
- **Cargo and rustfmt** (comes with Rust)
- **C++ toolchain** (compiler, linker, etc)
- [**Qt 6**](#qt-installation)

### Qt installation

A Qt installation must be present in the system and `qmake` must be in the
system PATH. There are no special requirements for the Qt installation. It
can be built from source or downloaded from <https://download.qt.io/>.
To ensure qmake is available, add the Qt bin directory to your PATH.

- On Windows:
```bat
set PATH=%PATH%;D:\dev\qt_build\qtbase\bin\
```

- On Linux:
```sh
export PATH=/home/john_doe/dev/qt_build/qtbase/bin:$PATH
export LD_LIBRARY_PATH="/home/john_doe/dev/qt_build/lib:$LD_LIBRARY_PATH"
```

- On macOS:
```sh
export PATH=/Users/john_doe/dev/qt_build/qtbase/bin:$PATH
export DYLD_FRAMEWORK_PATH=/Users/john_doe/dev/qt_build/qtbase/lib:$DYLD_FRAMEWORK_PATH
```

### Dependency

QtBridge has a single crate with all public APIs:
```toml
[dependencies]
qtbridge = "*"
```

### Provided Examples

Note: In the preliminary version of this repository, the package dependency is given
as a relative path. With the final release, the package dependency can be resolved
with cargo and crates.io. Until then you need to adapt the dependency or check out
the whole repository.

#### [Hello World!](https://github.com/qt/qtbridge-rust/tree/dev/apps/hello_world)

The classic "Hello World!" example showing the minimal building bricks for a
QtBridge application.

#### [Minimal App](https://github.com/qt/qtbridge-rust/tree/dev/apps/minimal_app)

This example shows the "minimum viable example" with working backend and data
visualisation in QML.

#### [Host Monitor](https://github.com/qt/qtbridge-rust/tree/dev/apps/host_monitor)

This example shows how to combine a Qt UI with [tokio](https://tokio.rs/) runtime
in a multithreaded environment using [`QmlMethodInvoker`].

## API Overview

### Borrow checker and rules

QtBridge aims to respect the borrowing rules of Rust. All QObjects in Rust (implemented
with [`qobject_impl`]) are designed to be held in `Rc<RefCell<_>>` structs. The QML
engine also holds Rust objects through such references. With this construct, many
references can be held and borrowed at runtime. Their QML representation can be copied
following the ordinary JavaScript rules. A Rust object is only borrowed when a slot or
property defined in Rust is invoked. If QML is unable to borrow the Rust object, it
will panic. We are currently investigating alternative ways to handle this situation
more gracefully.

## Further information:

Qt itself is written in C++, and QtBridge builds on [CXX](https://cxx.rs/) to access
the required Qt interfaces. As a user, you do not need to write any C++ code.
Instead, Rust structs and data can be exposed to QML using attribute macros provided
by the library.

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

## Future Plans

* Enable interoperability with [CXX-Qt](https://crates.io/crates/cxx-qt)
* Enable interoperability with other libraries such as [tokio](https://tokio.rs/)
* Enable QObjects as properties (complex properties).
* Provide Qt dependencies as a crate to enable a streamlined installation through cargo.
* Remove the dependency on Qt binaries like qmake and maybe a C++ tool chain.
* Extend IDE support in particular for VS Code. Enable the QML language server to understand
  types generated in Rust.


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

