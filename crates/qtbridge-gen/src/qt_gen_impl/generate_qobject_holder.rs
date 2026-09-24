// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::{format_ident, quote};

use crate::utils::case_conv;

/// Generate the implementation of the `QObjectHolder` trait.
pub fn generate_qobject_holder(
    struct_ident: &syn::Ident,     // Name of struct that implements the given trait.
    iface_ident: &syn::Ident,      // The name of the Qt-interface the struct is implementing
    impl_generics: &syn::Generics, // All the generics added to the implementation and their clauses
) -> syn::Result<syn::ItemImpl> {

    let iface_name = iface_ident;
    let iface_module = format_ident!("{}", case_conv::camel_to_snake(&iface_name.to_string()));
    let proxy_rust = format_ident!("{iface_name}ProxyRust");

    let has_generics = !impl_generics.params.is_empty();
    let (impl_generics, type_generics, where_clause) = impl_generics.split_for_impl();

    // Per-type OnceLock overrides of the map-based defaults; generic types
    // cannot have per-instantiation statics and keep the defaults.
    let realization_overrides = if has_generics {
        quote! {}
    } else {
        quote! {
            fn get_shared_dynamic_meta_object_data() -> &'static qtbridge::qtbridge_runtime::DynamicMetaObjectData {
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

            fn get_qobject_ptr_qmetatype() -> qtbridge::qtbridge_type_lib::QMetaType {
                use std::sync::OnceLock;
                static PTR_META_TYPE_INTERFACE: OnceLock<qtbridge::qtbridge_type_lib::QMetaTypeInterface> = OnceLock::new();
                let iface = PTR_META_TYPE_INTERFACE.get_or_init(qtbridge::qtbridge_runtime::qmetatypeforqobject::init_ptr_interface_for::<Self>);
                qtbridge::qtbridge_type_lib::QMetaType::new_with_interface(iface as *const _)
            }
        }
    };

    let code = quote! {
        impl #impl_generics qtbridge::qtbridge_runtime::QObjectHolder for #struct_ident #type_generics #where_clause {
            type ProxyRust = qtbridge::qtbridge_interfaces::#iface_module::#proxy_rust;

            #realization_overrides
        }
    };
    syn::parse2(code)
}

