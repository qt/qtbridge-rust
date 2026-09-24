// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::Span;
use crate::parse_utils::parse_include_path;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum CppIncludeDelimiter {
    Brackets,
    Quotes,
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CppInclude {
    delim: CppIncludeDelimiter,
    file_path: String,
}

impl CppInclude {
    pub fn new_in_brackets(path: &str) -> Self {
        Self {
            delim: CppIncludeDelimiter::Brackets,
            file_path : path.to_owned(),
        }
    }

    pub fn new_in_quotes(path: &str) -> Self {
        Self {
            delim: CppIncludeDelimiter::Quotes,
            file_path: path.to_owned(),
        }
    }

    pub fn new_from_str(file_path_with_delim: &str) -> syn::Result<Self> {
        let mut chars = file_path_with_delim.chars();
        let first_ch = chars.next()
            .ok_or_else(|| syn::Error::new(Span::call_site(), "Empty path in include"))?;
        let last_ch = chars.next_back()
            .ok_or_else(|| syn::Error::new(Span::call_site(), "Invalid path include"))?;

        match (first_ch, last_ch) {
            ('"', '"') => Ok(Self::new_in_quotes(chars.as_str())),
            ('<', '>') => Ok(Self::new_in_brackets(chars.as_str())),
            _ => Err(syn::Error::new(Span::call_site(), format!("Invalid format of include path: '{file_path_with_delim}'"))),
        }
    }

    pub fn is_for_me(input: syn::parse::ParseStream) -> bool {
        input.peek(keywords::include_in_cpp)
    }

    pub fn path_with_delims(&self) -> String {
        let (left_delim, right_delim) = match &self.delim {
            CppIncludeDelimiter::Brackets => ('<', '>'),
            CppIncludeDelimiter::Quotes => ('"', '"'),
        };
        let file = &self.file_path;
        format!("{left_delim}{file}{right_delim}")
    }

    pub fn to_cpp_code(&self) -> String {
        format!("#include {}\n", self.path_with_delims())
    }
}

mod keywords {
    syn::custom_keyword!(include_in_cpp);
}

impl syn::parse::Parse for CppInclude {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _kw: keywords::include_in_cpp = input.parse()?;
        let _excl: syn::Token![!] = input.parse()?;

        let path_with_delims;
        {
            let content;
            syn::parenthesized!(content in input);
            path_with_delims = content.step(parse_include_path)?;

            if !content.is_empty() {
                return Err(content.error("Unexpected trailing tokens"));
            }
        }

        let _maybe_semi: Option<syn::Token![;]> = input.parse()?;

        Self::new_from_str(&path_with_delims)
    }
}
