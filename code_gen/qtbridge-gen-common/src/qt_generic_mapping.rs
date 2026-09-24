use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use crate::type_registry::qt::generic::QtGenericTypeWithoutArgs;
use crate::type_registry::type_traits::{FindType, TypeName};

/// Structure that maps generic Qt types given as syn::path to monomorphed version
/// E.g., QHash<i32, QString> -> QHash_i32_QString.
/// Methods perform mapping 'inplace' on argument passed by mutable reference.
pub struct QtGenericMapping {
    last_error: syn::Result<()>,
}

impl Default for QtGenericMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl QtGenericMapping {
    pub fn new() -> Self {
        Self { last_error: Ok(()) }
    }

    pub fn result(&self) -> syn::Result<()> {
        self.last_error.clone()
    }

    pub fn map_impl_item(src: &mut syn::ImplItem) -> syn::Result<()> {
        let mut map = Self::new();
        map.visit_impl_item_mut(src);
        map.result()
    }

    pub fn map_item_fn(src: &mut syn::ItemFn) -> syn::Result<()> {
        let mut map = Self::new();
        map.visit_item_fn_mut(src);
        map.result()
    }

    pub fn map_signature(src: &mut syn::Signature) -> syn::Result<()> {
        let mut map = Self::new();
        map.visit_signature_mut(src);
        map.result()
    }

    pub fn map_path(src: &mut syn::Path) -> syn::Result<()> {
        let mut map = Self::new();
        map.do_visit_path_mut(src)
    }

    fn do_visit_path_mut(&mut self, src: &mut syn::Path) -> syn::Result<()> {
        for seg in src.segments.iter_mut() {
            if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments
                && let Some(qt_generic_wo_args) = QtGenericTypeWithoutArgs::find_by_name(&seg.ident.to_string())
            {
                let qt_generic_w_args = qt_generic_wo_args.set_args_from_syn_generic_args(ab)?;
                if let Some(qt_monomorphed) = qt_generic_w_args.get_monomorphed_type() {
                    seg.arguments = syn::PathArguments::None;
                    seg.ident = syn::Ident::new(qt_monomorphed.name(), seg.span());
                };
            }
        }

        Ok(())
    }
}

impl VisitMut for QtGenericMapping {
    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        if let Err(err) = self.do_visit_path_mut(src) {
            self.last_error = Err(err);
        }
    }
}
