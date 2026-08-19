use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::DeriveInput;

pub fn try_derive_qmodelitem(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    let name = &input.ident;

    let s = match &input.data {
        syn::Data::Struct(st) => st,
        _ => return Err(syn::Error::new_spanned(&input, "QModelItem only supports structs")),
    };

    let fields = s.fields.iter().collect::<Vec<_>>();

    let (members, field_names): (Vec<_>, Vec<_>) = fields.iter().enumerate()
    .map(|(i, f)|
        match f.ident.as_ref() {
            Some(ident) => (quote! { self.#ident }, ident.to_string()),
            None => {
                let index = syn::Index::from(i);
                (quote! { self.#index }, format!("_{}", i))
            },
        })
    .unzip();

    let len = fields.len();
    let max_elements = 15;

    if len > max_elements {
        return Err(syn::Error::new_spanned(&s.fields, "QModelItem supports only up to 15 fields"));
    }

    let field_types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    // see  Qt::ItemDataRole
    let default_roles: std::collections::HashMap<&str, i32> = std::collections::HashMap::from([
        ("display", 0x0),
        ("decoration", 0x1),
        ("edit", 0x2),
        ("toolTip", 0x3),
        ("statusTip", 0x4),
        ("whatsThis", 0x5),
    ]);

    let mut prop_mapping = Vec::new();

    // Do some mapping so the default types go to the respective index
    // if they are present. This is important due to the current implementation
    // of some default delegates.
    prop_mapping.resize(field_names.len(), max_elements);
    for (i, f) in field_names.iter().enumerate() {
        if let Some(&predef) = default_roles.get(f.as_str()) {
            prop_mapping[predef as usize] = i;
        }
    }
    let mut j = 0;
    for (i, f) in field_names.iter().enumerate() {
        if !default_roles.contains_key(f.as_str()){
            while prop_mapping[j] < max_elements {
                j += 1;
            }
            prop_mapping[j] = i;
            j += 1;
        }
    }

    let mut typdefs = Vec::new();
    let mut getters = Vec::new();
    let mut setters = Vec::new();

    for (i, member) in members.iter().enumerate() {
        let getter_fn_ident = format_ident!("get{}", prop_mapping[i]);
        let setter_fn_ident = format_ident!("set{}", prop_mapping[i]);
        let type_ident = format_ident!("T{}", prop_mapping[i]);
        let ty = &field_types[i];

        typdefs.push(quote! { type #type_ident = #ty; });
        getters.push(quote! { fn #getter_fn_ident(&self) -> Option<&#ty> { Some(&#member) } });
        setters.push(quote! { fn #setter_fn_ident(&mut self, value: #ty) { #member = value; } });
    }

    for i in len..15 {
        let type_ident = format_ident!("T{}", i);
        typdefs.push(quote! { type #type_ident = (); });
    }

    let mut role_tokens = Vec::new();

    for (i,f) in field_names.iter().enumerate() {
        let idx = prop_mapping[i] as i32;

        role_tokens.push(quote! {
            (#idx, #f.into()),
        });
    }

    Ok(quote! {
        impl QModelItem for #name {

            #(#typdefs)*

            #(#getters)*

            #(#setters)*

            fn role_names() -> std::collections::HashMap<i32, String> {
                let roles = [
                #(#role_tokens)*
                ];
                roles.iter().cloned().collect()
            }
        }
    }.into())
}
