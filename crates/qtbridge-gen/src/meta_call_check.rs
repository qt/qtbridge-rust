use syn::spanned::Spanned;

use qtbridge_gen_common::signature_utils::{get_return_type, get_typed_args, is_arg_self_ref};
use qtbridge_gen_common::type_to_string::type_to_string_fallback;
use qtbridge_gen_common::type_utils::{is_mut_ref, remove_ref};

/// Checks whether the given signature can participate in meta-calls
/// (as slot callbacks or property getters/setters).
///
/// Validates structural constraints only. Whether a concrete `QMetaCallArg`
/// impl exists for each type is deferred to the Rust compiler.
pub fn check_meta_call_signature(src: &syn::Signature) -> syn::Result<()> {
    if !src.inputs.first().is_some_and(|arg| is_arg_self_ref(arg, None)) {
        return Err(syn::Error::new(src.ident.span(), "First argument must be &self"));
    }

    get_typed_args(src)
        .try_for_each(|arg| {
            check_meta_call_type(arg.ty.as_ref())
                .map_err(|err| syn::Error::new(err.span(), format!("The function argument is not compatible with meta call.\n{err}")))
        })?;
    get_return_type(&src.output)
        .map(|ty| {
            check_meta_call_type(ty)
                .map_err(|err| syn::Error::new(err.span(), format!("The function return type is not compatible with meta call.\n{err}")))
        })
        .unwrap_or(Ok(()))
}

/// Checks whether the given type can appear as an argument or return type in a meta-call.
pub fn check_meta_call_type(ty: &syn::Type) -> syn::Result<()> {
    let ty_wo_ref = remove_ref(ty);
    match ty_wo_ref {
        syn::Type::Path(_) => Ok(()),
        syn::Type::Array(array) =>
            Err(syn::Error::new(array.span(), "Arrays are currently not supported")),
        syn::Type::Ptr(ptr) =>
            Err(syn::Error::new(ptr.span(), "Pointers are not supported")),
        syn::Type::Reference(type_ref) =>
            Err(syn::Error::new(type_ref.span(), "References to reference are not supported")),
        syn::Type::Slice(slice) =>
            Err(syn::Error::new(slice.span(), "Slices are currently not supported")),
        syn::Type::Tuple(tuple) =>
            Err(syn::Error::new(tuple.span(), "Tuples are not supported")),
        _ => Err(syn::Error::new(ty_wo_ref.span(), format!("Type category ('{:?}') of type '{}' is not supported", std::mem::discriminant(ty_wo_ref), type_to_string_fallback(ty_wo_ref))))
    }?;

    if is_mut_ref(ty) {
        return Err(syn::Error::new(ty.span(), format!("Mutable references are not supported. Found: '{}'", type_to_string_fallback(ty))))
    }

    Ok(())
}
