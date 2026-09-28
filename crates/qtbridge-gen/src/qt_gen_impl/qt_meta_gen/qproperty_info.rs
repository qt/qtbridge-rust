// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, spanned::Spanned};

use crate::utils::parse_utils::parse_name_value;
use crate::utils::type_to_string::type_to_string_fallback;
use crate::utils::type_utils::{ValuePass, get_take_value_code, get_type_pass, is_ref, remove_ref, remove_ref_to_string, remove_refs};
use crate::qt_gen_impl::qt_meta_gen;
use qt_meta_gen::qproperty_type_deduction::{deduce_type_from_getter, deduce_type_from_member, deduce_type_from_setter};
use qt_meta_gen::QSignalInfo;
use qt_meta_gen::traits::QmlName;

pub struct QPropertyInfo{
    name: syn::LitStr,
    span: Span,

    /// Id that will be used to identify a property in the handler function.
    id: u32,
    read_method: Option<syn::Ident>,
    write_method: Option<syn::Ident>,
    notify_signal: Option<syn::Ident>,
    member: Option<syn::Ident>,
    constant: Option<syn::Ident>,
    default: Option<syn::Ident>,

    getter_type: Option<syn::Type>,
    setter_type: Option<syn::Type>,
    member_type: Option<syn::Type>,
}

impl QPropertyInfo {
    pub fn new(item: &syn::ImplItemMacro, id: u32) -> syn::Result<Option<Self>> {
        if !item.mac.path.is_ident("qproperty") {
            return Ok(None); // Not a 'qproperty!' macro
        }

        if let Some(first_attr) = item.attrs.first() {
            return Err(syn::Error::new(first_attr.span(), "Attributes for qproperty! macro are not supported"));
        }

        let mut prop = item.mac.parse_body::<QPropertyInfo>()?;
        prop.span = item.mac.span(); // Change span to include whole macro
        prop.id = id;
        Ok(Some(prop))
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_const(&self) -> bool {
        self.constant.is_some()
    }

    pub fn get_notify_signal(&self) -> Option<&syn::Ident> {
        self.notify_signal.as_ref()
    }

    fn get_deduced_type(&self) -> Option<&syn::Type> {
        self.getter_type.as_ref()
            .or(self.setter_type.as_ref())
            .or(self.member_type.as_ref())
    }

    pub fn validate(&self, signals: &[QSignalInfo]) -> syn::Result<()> {
        self.validate_name()?;

        if self.member.is_some() && self.read_method.is_some() && self.write_method.is_some() {
            return Err(syn::Error::new(self.member.span(), "qproperty must not have 'member' flag if both Read and Write functions are set"));
        }

        if self.constant.is_some() {
            if self.write_method.is_some() {
                return Err(syn::Error::new(self.write_method.span(), "Constant qproperty must not have Write method"));
            }
            if self.notify_signal.is_some() {
                return Err(syn::Error::new(self.notify_signal.span(), "Constant qproperty must not have Notify signal"));
            }
        }

        if let Some(notify_signal) = self.notify_signal.as_ref() {
            let signal = signals.iter().find(|s| s.get_rust_name() == *notify_signal)
                .ok_or_else(|| syn::Error::new(notify_signal.span(), format!("Signal '{}' not found", notify_signal)))?;
            match signal.get_typed_arg_count() {
                0 => {}
                1 => {
                    if let Some(prop_type) = self.get_deduced_type() {
                        let prop_type_str = remove_ref_to_string(prop_type)?;
                        let signal_type_str = remove_ref_to_string(signal.get_arg_type(0)?)?;
                        if prop_type_str != signal_type_str {
                            return Err(syn::Error::new(notify_signal.span(), format!("Property/signal types mismatch: '{prop_type_str}' and '{signal_type_str}'")));
                        }
                    }
                }
                _ => return Err(syn::Error::new(notify_signal.span(), "Notify signal of a property must have either 2 arguments (&self and value) or only 1 (&self)"))
            }
        };

        Ok(())
    }

    fn validate_name(&self) -> syn::Result<()> {
        // https://doc.qt.io/qt-6/qtqml-syntax-objectattributes.html#defining-property-attributes

        let name = self.name.value();
        let first_ch = name.chars().next()
            .ok_or_else(|| syn::Error::new(name.span(), "Property name is empty"))?;

        if !first_ch.is_lowercase() {
            return Err(syn::Error::new(name.span(), format!("Property name must begin with a lower case letter. Found: '{first_ch}'")))
        }

        for ch in name.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                return Err(syn::Error::new(name.span(), format!("Illegal char in property name: '{ch}'")))
            }
        }

        Ok(())
    }

    /// Returns an error if multiple properties have 'default' attribute.
    pub fn check_single_default_property(properties: &[Self]) -> syn::Result<()> {
        let mut first_default: Option<&Self> = None;

        for prop in properties {
            let Some(default_ident) = &prop.default else {
                continue
            };
            if let Some(first) = first_default {
                return Err(syn::Error::new(
                    default_ident.span(),
                    format!("Multiple default properties within the same object are not allowed. The first one was: '{}'", first.name.value())))
            }
            first_default = Some(prop);
        }

        Ok(())
    }

    // Sets the property type based on the type deduced from its accessors or member variable.
    pub fn set_type(&mut self, methods: &[syn::Signature], fields: Option<&syn::FieldsNamed>) -> syn::Result<()> {
        let mut deduced = Vec::new(); // Array of tuples (Type, Span, "deduced from")
        let mut getter_type = None;
        let mut setter_type = None;
        let mut member_type = None;

        if let Some(getter) = self.read_method.as_ref() {
            let ty = deduce_type_from_getter(getter, methods)?;
            deduced.push((ty, getter.span(), "getter"));
            getter_type = Some(ty.clone());
        }

        if let Some(setter) = self.write_method.as_ref() {
            let ty = deduce_type_from_setter(setter, methods)?;
            deduced.push((ty, setter.span(), "setter"));
            setter_type = Some(ty.clone());
        }

        if let Some(member) = self.member.as_ref() &&
           let Some(fields) = fields {
                let ty = deduce_type_from_member(member, fields.named.iter())?;
                deduced.push((ty, member.span(), "member"));
                member_type = Some(ty.clone());
            }

        let Some((first_type, _, first_src)) = deduced.first() else {
            return Ok(())
        };

        if let Some((second_type, second_span, second_src)) = deduced.get(1) {
            let first_type_no_ref = remove_refs(first_type);
            let second_type_no_ref = remove_refs(second_type);
            if first_type_no_ref != second_type_no_ref {
                return Err(syn::Error::new(*second_span,
                    format!("Property types deduced from '{first_src}' and '{second_src}' are inconsistent: '{}' vs '{}'",
                        type_to_string_fallback(first_type_no_ref),
                        type_to_string_fallback(second_type_no_ref)
                    )));
            }
        }

        self.getter_type = getter_type;
        self.setter_type = setter_type;
        self.member_type = member_type;

        Ok(())
    }

    pub fn get_meta_registration_code(&self, signal: Option<&QSignalInfo>) -> syn::Result<TokenStream> {

        let QPropertyInfo {
            name,
            id,
            span,
            notify_signal,
            member,
            ..
        } = self;

        let metatype_expr = match self.get_deduced_type() {
            // Type of property is deduced from getter, setter
            // or member (if struct fields are accessible to the macro).
            Some(ty) => {
                let t = remove_ref(ty);
                quote! { <#t as QPropertyMember>::qmetatype() }
            },
            None => {
                let member_var = member.as_ref()
                    .ok_or_else(||syn::Error::new(*span, "Can't deduce type of qproperty neither from accessor nor from member"))?;
                quote!{
                    qtbridge::qtbridge_runtime::get_meta_type_of_fn_return_value(|this: &Self| &this.#member_var)
                }
            }
        };

        let signal_name = signal.map_or(String::default(), |i| i.get_qml_name_span().0);

        if signal_name.is_empty() != notify_signal.is_none() {
            return Err(syn::Error::new(*span, "Error in signal handling logic. Signal name mismatch"));
        }

        let default_code = self.default.is_some()
            .then(|| quote! {
                meta_obj.as_mut().add_class_info("DefaultProperty", #name);
            });
        let is_const = self.is_const();
        let property_registration = quote! {
                meta_obj.as_mut().register_property(#name, #id, &(#metatype_expr), #is_const, #signal_name);
        };

        Ok(quote! {
            #property_registration
            #default_code
        })
    }

    pub fn get_read_code(&self) -> syn::Result<TokenStream> {
        let value_src_ref = self.get_value_src_ref()?;
        Ok(quote! {
            // SAFETY: the variant is consumed by the metaobject dispatch
            // within this call stack, never outliving the object it may point to.
            unsafe { (#value_src_ref).to_qvariant(self) }
        })
    }

    pub fn get_read_notifying_code(&self, signal: &QSignalInfo) -> syn::Result<TokenStream> {
        let value_src_ref = self.get_value_src_ref()?;
        let signal_name = signal.get_rust_name();
        Ok(quote! {
            // SAFETY: see get_read_code; the view variant is consumed by the
            // metaobject dispatch within this call stack.
            unsafe { (#value_src_ref).to_qvariant_view(self, Self::#signal_name) }
        })
    }

    fn get_value_src_ref(&self) -> syn::Result<TokenStream> {
        if let Some(getter_fn) = &self.read_method {
            let mut value_src_ref = quote! { self.#getter_fn() };
            let getter_type = self.getter_type.as_ref()
                .ok_or_else(|| syn::Error::new(getter_fn.span(), "Property type is not deduced for the getter"))?;
            if !is_ref(getter_type) {
                value_src_ref = quote! { (&#value_src_ref) };
            }
            Ok(value_src_ref)
        } else if let Some(member) = &self.member {
            Ok(quote! { &self.#member })
        } else {
            Err(syn::Error::new(self.span, "Neither 'Read' nor 'Member' is specified for property"))
        }
    }

    pub fn get_write_code(&self, signal: Option<&QSignalInfo>) -> syn::Result<Option<TokenStream>> {
        if self.is_const() {
            return Ok(None)
        }

        let name = &self.name;

        let input_conv_code = quote! {
            // SAFETY: value comes from the C++ metaobject dispatch, which
            // type-checks the property write before handing it to us.
            let Some(value) = (unsafe { QPropertyMember::from_qvariant(value) }) else {
                panic!("Failed to convert QVariant for qproperty '{}'", #name);
            };
        };

        if let Some(setter_fn) = &self.write_method {
            let write_value_pass = get_type_pass(self.setter_type.as_ref()
                .ok_or_else(|| syn::Error::new(setter_fn.span(), "Property setter type is not deduced"))?);
            let pass_arg = get_take_value_code(&format_ident!("value"), write_value_pass);
            return Ok(Some(quote! {
                #input_conv_code
                self.#setter_fn(#pass_arg);
            }));
        }

        let Some(member) = self.member.as_ref() else {
            return Ok(None) // A read-only property
        };

        let emit_signal_code = if let Some(notify_signal) = &self.notify_signal {
            let signal_info = signal
                .ok_or_else(|| syn::Error::new(self.notify_signal.span(), "Signal info is not provided"))?;
            if signal_info.get_rust_name() != *notify_signal {
                return Err(syn::Error::new(self.span, "Error in signal handling logic. Inconsistent signal name"));
            }
            let signal_name_ident = signal_info.get_rust_name();
            let signal_arg = signal_info.get_arg_type(0)
                .ok()
                .map(|arg_type| match get_type_pass(arg_type) {
                    ValuePass::ByValue => quote! { self.#member.clone() },
                    ValuePass::ByConstReference => quote! { &self.#member },
                    ValuePass::ByMutReference => quote! { &mut self.#member },
                });
            Some(quote! { self.#signal_name_ident(#signal_arg); })
        } else {
            None
        };

        Ok(Some(quote! {
            #input_conv_code
            if !QPropertyMember::property_eq(&self.#member, &value) {
                self.#member = value;
                #emit_signal_code
            }
        }))
    }
}

mod qproperty_keywords {
    syn::custom_keyword!(Read);
    syn::custom_keyword!(Write);
    syn::custom_keyword!(Notify);
    syn::custom_keyword!(Constant);
    syn::custom_keyword!(Default);
    syn::custom_keyword!(Member);
}

impl syn::parse::Parse for QPropertyInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        // TODO: charset check?
        let name = input.parse()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to get qproperty name.\nError: {err}")))?;

        let mut read_method = None;
        let mut write_method = None;
        let mut notify_signal = None;
        let mut member = None;
        let mut constant = None;
        let mut default = None;

        while !input.is_empty() {
            let token_begin = input.fork();
            input.parse::<syn::Token![,]>()
                .map_err(|_err| token_begin.error("Expected comma while parsing qproperty attributes"))?;

            if input.is_empty() {
                break; // Stop after a trailing coma
            }

            // Try to parse attribute name-value pair. Like
            //     Name = Value
            if input.peek(qproperty_keywords::Read) {
                read_attribute(input, &mut read_method, "Read")?;
            }
            else if input.peek(qproperty_keywords::Write) {
                read_attribute(input, &mut write_method, "Write")?;
            }
            else if input.peek(qproperty_keywords::Notify) {
                read_attribute(input, &mut notify_signal, "Notify")?;
            }
            else if input.peek(qproperty_keywords::Member) {
                read_attribute(input, &mut member, "Member")?;
            }
            // Try to parse bool flags
            else if input.peek(qproperty_keywords::Constant) {
                constant = Some(input.parse()?);
            }
            else if input.peek(qproperty_keywords::Default) {
                 default = Some(input.parse()?);
            }
            else {
                let attr: syn::Ident = input.parse()?;
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Unsupported qproperty attribute: {}", attr),
                ));
            }
        }

        Ok(QPropertyInfo {
            name,
            span,
            id: 0,
            read_method,
            write_method,
            notify_signal,
            member,
            constant,
            default,
            // Property type is not clear at the moment of parsing. Will be deduced later
            getter_type: None,
            setter_type: None,
            member_type: None,
        })
    }

}

fn read_attribute<T: Parse>(input: syn::parse::ParseStream, dst: &mut Option<T>, name: &'static str) -> syn::Result<()> {
    if dst.is_some() {
        return Err(syn::Error::new(input.span(), format!("'{name}' attribute is already defined for the property")));
    }

    *dst = Some(parse_name_value::<syn::Ident, T>(input)?.1);

    Ok(())
}

impl QmlName for QPropertyInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        (self.name.value(), self.name.span())
    }
}
