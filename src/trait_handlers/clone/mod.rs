use crate::trait_handlers::TraitHandlerContext;
mod clone_enum;
mod clone_struct;
mod clone_union;
mod models;

use quote::quote;
use syn::{Data, DeriveInput, Generics, Meta, Path, Type};

use super::TraitHandler;
use crate::Trait;

/// Builds a module-level marker that references a field's custom clone method so it stays counted as used.
///
/// The generated `Clone` impl is marked `#[automatically_derived]`, and `Clone` carries `#[rustc_trivial_field_reads]`, so the compiler skips its body during dead-code analysis. A custom clone method used only inside that body would therefore be wrongly reported as unused. This marker calls the method from an ordinary item that is still analyzed, and it takes the same generics and where clause as the impl so it compiles under exactly the same conditions.
pub(crate) fn create_mark_method_used(
    generics: &Generics,
    field_ty: &Type,
    method: &Path,
) -> proc_macro2::TokenStream {
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // This function is generated glue whose only purpose is to reference the custom method, so its signature can look problematic in isolation (e.g. `&Vec<T>` would normally suggest `clippy::ptr_arg`, or an unused generic would trigger `clippy::extra_unused_type_parameters`). Lints like these already do not fire on code coming from an external proc-macro, but the `clippy::all` allow is kept here as a low-cost safeguard in case that exemption ever narrows.
    quote!(
        const _: () = {
            #[allow(dead_code, clippy::all)]
            fn __educe_clone_method_used #impl_generics (
                educe__value: &#field_ty,
            ) -> #field_ty #where_clause {
                #method(educe__value)
            }
        };
    )
}

/// Dispatches the `Clone` derive to the specialized handler for the shape of the input (struct, enum, or union).
pub(crate) struct CloneHandler;

impl TraitHandler for CloneHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => clone_struct::CloneStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => clone_enum::CloneEnumHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Union(_) => clone_union::CloneUnionHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
        }
    }
}
