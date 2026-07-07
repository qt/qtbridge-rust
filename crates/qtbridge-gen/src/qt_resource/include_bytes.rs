// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::quote;
use proc_macro::TokenStream;
use syn::spanned::Spanned;

struct Input {
    file: String,
    prefix: Option<String>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file: syn::LitStr = input.parse()?;
        let comma: Option<syn::Token![,]> = input.parse()?;
        let mut prefix = None;
        if comma.is_some() {
            prefix = Some(input.parse::<syn::LitStr>()?);
        }

        Ok(Self {
            file: file.value(),
            prefix: prefix.map(|p| p.value()),
        })
    }
}
pub fn include_bytes_qml(input: TokenStream) -> TokenStream {
    include_bytes_qml_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub fn include_bytes_qml_impl(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {

    let input2 = proc_macro2::TokenStream::from(input.clone());
    let input_span = input2.span();

    let Input{ file, prefix } = syn::parse2(input2)?;
    let prefix = prefix.unwrap_or_default();

    // `local_file()` gives the on-disk path of the invoking source file, just
    // like the built-in `include_bytes!`. It requires a toolchain that exposes
    // the source location to procedural macros; if it doesn't, emit a clear
    // diagnostic instead of panicking.
    let Some(span_file) = input_span.local_file() else {
        return Err(syn::Error::new(input_span,
            "include_bytes_qml! could not determine the invoking source file path. \
             This requires a compiler/toolchain that exposes the on-disk source \
             location to procedural macros."));
    };

    // `local_file()` returns the call-site path relative to rustc's working
    // directory. As a proc-macro we run inside rustc, so `current_dir()` is
    // exactly that directory and is the correct anchor to join against.
    let span_file_path = if span_file.is_absolute() {
        span_file
    } else {
        match std::env::current_dir() {
            Ok(dir) => dir.join(&span_file),
            Err(err) => return Err(syn::Error::new(input_span,
                format!("include_bytes_qml! failed to resolve the current directory.\nError: {err}"))),
        }
    };

    let Some(span_file_dir) = span_file_path.parent() else {
        return Err(syn::Error::new(input_span,
            format!("include_bytes_qml! failed to get the parent directory of '{}'.", span_file_path.display())));
    };

    let include_path = span_file_dir.join(&file);
    let raw_data = match std::fs::read(&include_path) {
        Ok(data) => data,
        Err(err) => return Err(syn::Error::new(input_span,
            format!("Failed to read file '{}'.\nError: {err}", include_path.display()))),
    };

    let mut folder_chain: Vec<&str> = prefix.split('/').filter(|s| !s.is_empty()).collect();
    folder_chain.extend(file.split('/'));

    let overall_flags: usize = 0;
    let mut data = Vec::<u8>::new();
    let mut names_offsets = Vec::<usize>::new();

    let qt_hash = |key: &str| -> u32 {
        let mut h: u32 = 0;
        for c in key.chars() {
            h = (h << 4).wrapping_add(c as u32);
            h ^= (h & 0xf0000000) >> 23;
        }
        h & 0x0fffffff
    };

    let write_u32_be = |data: &mut Vec<u8>, start: usize, value: u32| {
        data[start..start + 4].copy_from_slice(&value.to_be_bytes());
    };

    let append_u64_be = |data: &mut Vec<u8>, value: u64| {
        data.extend_from_slice(&value.to_be_bytes());
    };

    let append_u32_be = |data: &mut Vec<u8>, value: u32| {
        data.extend_from_slice(&value.to_be_bytes());
    };

    let append_u16_be = |data: &mut Vec<u8>, value: u16| {
        data.extend_from_slice(&value.to_be_bytes());
    };

    let append_utf16_be = |data: &mut Vec<u8>, s: &str| {
        for unit in s.encode_utf16() {
            data.extend_from_slice(&unit.to_be_bytes());
        }
    };

    let append_folder_entry = |data: &mut Vec<u8>, name_offset: u32, flags: u16,
        children_size: u32, children_offset: u32, date_modified: u64| {
        append_u32_be(data, name_offset);
        append_u16_be(data, flags);
        append_u32_be(data, children_size);
        append_u32_be(data, children_offset);
        append_u64_be(data, date_modified);
    };

    let append_file_entry = |data: &mut Vec<u8>, name_offset: u32, flags: u16,
        territory: u16, language: u16, data_offset: u32, date_modified: u64| {
        append_u32_be(data, name_offset);
        append_u16_be(data, flags);
        append_u16_be(data, territory);
        append_u16_be(data, language);
        append_u32_be(data, data_offset);
        append_u64_be(data, date_modified);
    };


    // see qtbase/src/tools/rcc
    // RCCResourceLibrary::output
    data.extend_from_slice(b"qres");
    append_u32_be(&mut data, 3); // version
    data.extend(std::iter::repeat_n(0, 16)); // reserve space

    // see qtbase/src/tools/rcc
    // RCCFileInfo::writeDataBlobs
    let data_offset = data.len();
    append_u32_be(&mut data, raw_data.len() as u32);
    data.extend_from_slice(&raw_data);

    // see qtbase/src/tools/rcc
    // RCCResourceLibrary::writeDataNames
    let names_offset = data.len();
    for name in folder_chain {
        names_offsets.push(data.len() - names_offset);
        append_u16_be(&mut data, name.len() as u16);
        append_u32_be(&mut data, qt_hash(name));
        append_utf16_be(&mut data, name);
    }

    // see qtbase/src/tools/rcc
    // RCCResourceLibrary::writeDataStructure
    let tree_offset = data.len();
    // root
    append_folder_entry(&mut data, 0, 2, 1, 1, 0);
    // folders
    for (i, n_off) in (2..).zip(names_offsets[..names_offsets.len().saturating_sub(1)].iter()) {
        append_folder_entry(&mut data, *n_off as u32, 2, 1, i, 0);
    }

    append_file_entry(&mut data, *names_offsets.last().unwrap() as u32, 0, 0, 1, 0, 0);

    // see qtbase/src/tools/rcc
    // RCCResourceLibrary::writeInitializer
    write_u32_be(&mut data, 8, tree_offset as u32);
    write_u32_be(&mut data, 12, data_offset as u32);
    write_u32_be(&mut data, 16, names_offset as u32);
    write_u32_be(&mut data, 20, overall_flags as u32);

    let output: proc_macro2::TokenStream = quote!{
        // tell the compiler that a rebuild is required when #file changes.
        // include_bytes! is a build-in macro that has this side effect.
        let _ = include_bytes!(#file);
        qtbridge::qresource::register_bytes(&[#(#data),*]);
    };
    Ok(output)
}
