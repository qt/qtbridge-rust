// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

mod function_with_attributes;
mod meta_call_check;
mod qt_derive;
mod qt_gen_impl;
mod qt_resource;

use proc_macro::TokenStream;
use crate::qt_gen_impl::qobject_module_builder;
use qobject_module_builder::{LinkmeSupport, QObjectModuleBuilder};


#[proc_macro_attribute]
pub fn qobject(args: TokenStream, input: TokenStream) -> TokenStream {
    // Automatic registration is enabled by the `linkme` cargo feature of qtbridge.
    // The re-exported linkme crate is used so that user crates do not need their
    // own dependency on it. This relies on the #[linkme(crate = ...)] attribute,
    // which is not documented but covered by linkme's own test suite
    // (tests/custom_linkme_path.rs). Kept separate from [`generate_qml_register`]
    // for feature-independent testing with insta.
    let linkme_support = match cfg!(feature = "linkme") {
        true => LinkmeSupport::Enabled,
        false => LinkmeSupport::Disabled,
    };
    let mut builder = QObjectModuleBuilder::new(linkme_support);
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

/// Includes a file or a directory and makes it accessible under the Qt resource system.
///
/// The path is resolved relative to the source file in which the macro is invoked,
/// similarly to Rust's [include_bytes!] macro.
///
/// If the path refers to a directory, all files in that directory and its
/// subdirectories are included recursively while preserving their relative paths.
///
/// An optional prefix can be added as a second macro parameter.
///
/// # Examples
///
/// Including a single file:
///
/// ```ignore
/// fn main() {
///     include_bytes_qml!("images/icon.png", "resources");
/// }
/// ```
///
/// Including a directory:
///
/// ```ignore
/// fn main() {
///     include_bytes_qml!("images", "resources");
/// }
/// ```
///
/// This makes the file `images/icon.png`, relative to the source file containing
/// the macro invocation, accessible in QML as `qrc:/resources/images/icon.png`
/// or `:/resources/images/icon.png`.
///
/// ```qml
/// Image {
///     source: "qrc:/resources/images/icon.png"
/// }
/// ```
#[proc_macro]
pub fn include_bytes_qml(input: TokenStream) -> TokenStream {
    qt_resource::include_bytes_qml(input.into()).into()
}
