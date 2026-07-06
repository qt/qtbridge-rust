// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::quote;

use qtbridge_gen_common::naming;

/// Generate the implementation of the `QObjectHolder` trait.
pub fn generate_qobject_holder(
    struct_ident: &syn::Ident,     // Name of struct that implements the given trait.
    iface_ident: &syn::Ident,      // The name of the Qt-interface the struct is implementing
    impl_generics: &syn::Generics, // All the generics added to the implementation and their clauses
) -> syn::Result<syn::ItemImpl> {

    let iface_name = iface_ident;
    let iface_module = naming::rust::module::from_struct_name(iface_name);
    let proxy_rust = naming::rust::structure::proxy_rust(iface_name);

    let (impl_generics, type_generics, where_clause) = impl_generics.split_for_impl();

    let code = quote! {
        impl #impl_generics qtbridge::qtbridge_runtime::QObjectHolder for #struct_ident #type_generics #where_clause {
            type ProxyRust = qtbridge::qtbridge_interfaces::#iface_module::#proxy_rust;

            fn as_adaptor_trait(
                rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>
            ) -> std::rc::Rc<std::cell::RefCell<
                <Self::ProxyRust as qtbridge::qtbridge_runtime::qproxies::QRustProxy>::AdapterType>>
            {
                rust_obj_rc
            }

        }
    };
    syn::parse2(code)
}

