// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro::TokenStream;

mod function_with_attributes;
mod meta_call_check;
mod qt_derive;
mod qt_gen_impl;
mod qt_resource;

#[proc_macro_attribute]
pub fn qobject(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut builder = qt_gen_impl::QObjectModuleBuilder::new();
    builder.build_token_stream(input.into(), args.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn qsignal(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qsignal] proc macro called outside #[qobject].")
}

#[proc_macro_attribute]
pub fn qslot(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qslot] proc macro called outside #[qobject].")
}

#[proc_macro]
pub fn qproperty(_: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("qproperty! macro called outside #[qobject].");
}

/// Derive macro that generates a `QModelItem` implementation.
///
/// Applying this macro to a struct allows it to be used inside `QVec<T>`
/// and exposed to QML, where it can be visualized with various views. The
/// delegates within the view are able to read and write to the struct
/// through the roles.
///
/// ## Roles
/// - **Named Field Struct:** The generated roles will match the names of the
///   fileds (e.g., `name`, `age`, …). Fields named `display`, `decoration`,
///   `edit`, `toolTip`, `statusTip`, or `whatsThis` are recognized as default
///   roles as used in Qt's default delegates.
/// - **Tuple Structs:** The generated roles are `"_0"`, `"_1"`, `"_2"`, ...
///
/// ## Type requirements
/// All fields must be convertible to and from `QVariant`.
///
/// ## Example
/// ```rust,ignore
/// #[derive(QModelItem)]
/// struct Person {
///     name: String,   // role "name"
///     age: u32,       // role "age"
/// }
///
/// #[derive(QModelItem)]
/// struct Pair(i32, String); // roles "_0", "_1"
/// ```
#[proc_macro_derive(QModelItem)]
pub fn derive_qmodelitem(input: TokenStream) -> TokenStream {
        match qt_derive::try_derive_qmodelitem(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

/// Includes a file and makes it accessible under the Qt resource system.
///
/// The macro uses the include_bytes! macro internally so the file path has to be
/// given relative to the current file.
///
/// An optional prefix can be added as a second macro parameter.
///
/// # Examples
///
/// ```ignore
/// fn main() {
///     include_bytes_qml!("icon.png", "images");
/// }
/// ```
///
/// This makes the file 'icon.png' that has to be in the same folder as
/// the file with the macro accessible in QML as 'qrc:/images/icon.png'
/// or ':/images.icon.png'
///
/// ```qml
/// Image {
///     source: "qrc:/images/icon.png"
/// }
/// ```
#[proc_macro]
pub fn include_bytes_qml(input: TokenStream) -> TokenStream {
    qt_resource::include_bytes_qml(input)
}
