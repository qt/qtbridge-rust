// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::braced;

pub enum BlockOrSemi {
    Block(syn::Block),
    Semi,
}

pub struct FunctionWithAttributes {
    pub attrs: Vec<syn::Attribute>, // Attributes other than qsignal
    pub vis: syn::Visibility,
    pub sig: syn::Signature,
    pub block: BlockOrSemi,
}

impl syn::parse::Parse for FunctionWithAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        // Inspired by
        // impl Parse for ForeignItemFn
        //   and
        // impl Parse for ImplItemFn

        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let _defaultness: Option<syn::Token![default]> = input.parse()?;
        let sig: syn::Signature = input.parse()?;
        if input.peek(syn::Token![;]) {
            let _semi: syn::Token![;] = input.parse()?;
            Ok(Self {
                attrs,
                vis,
                sig,
                block: BlockOrSemi::Semi,
            })
        }
        else if input.peek(syn::token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            attrs.extend(content.call(syn::Attribute::parse_inner)?);
            let block = syn::Block {
                brace_token,
                stmts: content.call(syn::Block::parse_within)?,
            };
            Ok(Self{
                attrs,
                vis,
                sig,
                block: BlockOrSemi::Block(block),
            })
        } else {
            Err(input.error("Unexpected function syntax"))
        }
    }
}
