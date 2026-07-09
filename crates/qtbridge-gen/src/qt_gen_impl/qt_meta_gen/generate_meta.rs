// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_gen_common::naming;

use quote::{ToTokens, quote};
use proc_macro2::TokenStream;

use crate::qt_gen_impl::qt_meta_gen;
use qt_meta_gen::{QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

pub struct QMetaInfoContext<'a> {
    pub struct_ident: &'a syn::Ident,
    pub iface_ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub signals: &'a [QSignalInfo],
    pub slots: &'a [QSlotInfo],
    pub properties: &'a [QPropertyInfo],
    pub class_infos: &'a [QClassInfo],
}

pub fn generate_qmetainfo_trait_impl(ctx: &QMetaInfoContext) -> syn::Result<syn::ItemImpl> {
    let generics = &ctx.generics;
    let signals_meta_reg = generate_signals_meta_registration(ctx.signals)?;
    let slots_meta_reg = generate_slots_meta_registration(ctx.slots)?;
    let properties_meta_reg = generate_properties_meta_registration(ctx.properties, ctx.signals)?;
    let class_infos_reg = generate_class_infos_meta_registration(ctx.class_infos)?;

    let iface_module = naming::rust::module::from_struct_name(ctx.iface_ident);
    let proxy_cpp = naming::rust::structure::proxy_cpp(ctx.iface_ident);

    let struct_ident = &ctx.struct_ident;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let has_generics = !generics.params.is_empty();
    let get_dyn_meta_object_body = if has_generics {
        quote! {
            qtbridge::qtbridge_runtime::qmetainfo::dynamic_meta_object_data_for_generic::<Self>()
        }
    } else {
        quote! {
            use std::sync::OnceLock;
            thread_local! {
                static DYNAMIC_META_OBJECT: OnceLock<&'static qtbridge::qtbridge_runtime::DynamicMetaObjectData> = OnceLock::new();
            }

            DYNAMIC_META_OBJECT.with(|cell| {
                *cell.get_or_init(|| {
                    let ptr = Self::create_dynamic_meta_object_data_for_type();
                    unsafe { ptr.as_ref() }.unwrap()
                })
            })
        }
    };

    let get_qmetatype_body = if has_generics {
        quote! {
            let iface = qtbridge::qtbridge_runtime::qmetatypeforqobject::interface_for_generic::<Self>();
        }
    } else {
        quote! {
            use std::sync::OnceLock;
            static META_TYPE_INTERFACE: OnceLock<qtbridge::qtbridge_type_lib::QMetaTypeInterface> = OnceLock::new();
            let iface = META_TYPE_INTERFACE.get_or_init(qtbridge::qtbridge_runtime::qmetatypeforqobject::init_interface_for::<Self>);
        }
    };

    let code = quote! {
        impl #impl_generics qtbridge::qtbridge_runtime::QMetaInfo for #struct_ident #type_generics #where_clause {


            type CppProxy = qtbridge::qtbridge_interfaces::#iface_module::#proxy_cpp;

            fn build_dynamic_meta_type(mut meta_obj: std::pin::Pin<&mut qtbridge::qtbridge_runtime::DynamicMetaObjectBuilder>) {
                use qtbridge::qtbridge_runtime::{QMetaCallArg, QMetaTypeGet, QPropertyMember};
                use qtbridge::qtbridge_type_lib;
                use qtbridge_type_lib::QMetaType;
                #signals_meta_reg
                #slots_meta_reg
                #properties_meta_reg
                #class_infos_reg

                meta_obj.as_mut().end_meta_registration();
            }

            fn get_shared_dynamic_meta_object_data() -> &'static qtbridge::qtbridge_runtime::DynamicMetaObjectData {
                #get_dyn_meta_object_body
            }

            fn get_qmetatype() -> qtbridge::qtbridge_type_lib::QMetaType {
                #get_qmetatype_body
                qtbridge::qtbridge_type_lib::QMetaType::new_with_interface(iface as *const _)
            }
        }
    };
    syn::parse2(code)
}


//TODO: move to generic function and introduce trait for signal, slot, properties (e.g. 'RegisterMeta')
fn generate_signals_meta_registration(signals: &[QSignalInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for signal in signals {
        let register_signal = signal.get_meta_registration_code()?;
        register_signal.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_slots_meta_registration(slots: &[QSlotInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for slot in slots {
        let register_slot = slot.get_meta_registration_code()?;
        register_slot.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_properties_meta_registration(properties: &[QPropertyInfo], signals: &[QSignalInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for property in properties {
        let mut signal = None;
        if let Some(notify_signal) = property.get_notify_signal() {
            signal = signals.iter().find(|s| s.get_rust_name() == *notify_signal);
            if signal.is_none() {
                return Err(syn::Error::new(notify_signal.span(), format!("Failed to find signal with name '{notify_signal}'")));
            }
        }
        let register_property = property.get_meta_registration_code(signal)?;
        register_property.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_class_infos_meta_registration(class_infos: &[QClassInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for class_info in class_infos {
        let register_class_info = class_info.get_meta_registration_code()?;
        register_class_info.to_tokens(&mut result);
    }

    Ok(result)
}
