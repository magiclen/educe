use crate::trait_handlers::TraitHandlerContext;
mod clone_enum;
mod clone_struct;
mod clone_union;
mod models;

use syn::{Data, DeriveInput, ExprPath, Generics, Meta, Type};

use super::TraitHandler;
use crate::{
    Trait,
    common::{marker::MethodMarker, quote_mixed},
};

/// Uses a whole-value copy only when it does not add requirements to `Clone`.
fn can_use_bitwise_copy(
    ast: &DeriveInput,
    ctx: &TraitHandlerContext,
    traits: &[Trait],
    has_custom_method: bool,
) -> syn::Result<bool> {
    #[cfg(feature = "Copy")]
    {
        if !traits.contains(&Trait::Copy) || has_custom_method {
            return Ok(false);
        }

        // A custom `Copy` bound can limit lifetimes or unused const parameters even when the fields do not mention a type parameter.
        if !ast.generics.params.is_empty() && super::copy::has_custom_bound(ctx)? {
            return Ok(false);
        }

        let uses_generics = |field: &syn::Field| {
            crate::common::r#type::type_uses_generic_params(&field.ty, &ast.generics.params)
        };

        Ok(match &ast.data {
            Data::Struct(data) => !data.fields.iter().any(uses_generics),
            Data::Enum(data) => {
                !data.variants.iter().any(|variant| variant.fields.iter().any(uses_generics))
            },
            Data::Union(_) => false,
        })
    }
    #[cfg(not(feature = "Copy"))]
    {
        let _ = (ast, ctx, traits, has_custom_method);
        Ok(false)
    }
}

/// Builds a module-level marker that references a field's custom clone method so it stays counted as used.
pub(crate) fn create_mark_method_used(
    ast: &DeriveInput,
    generics: &Generics,
    field_ty: &Type,
    method: &ExprPath,
) -> proc_macro2::TokenStream {
    let marker = MethodMarker::new(ast, generics, field_ty, method);
    let (impl_generics, _ty_generics, where_clause) = marker.generics.split_for_impl();
    let field_ty = &marker.field_ty;
    let method = &marker.method;

    marker.wrap(quote_mixed!(
        fn __educe_clone_method_used #impl_generics (
            educe__value: &#field_ty,
        ) -> #field_ty #where_clause {
            #method(educe__value)
        }
    ))
}

/// Dispatches the `Clone` derive to the specialized handler for the shape of the input (struct, enum, or union).
pub(crate) struct CloneHandler;

impl TraitHandler for CloneHandler {
    #[inline]
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        ctx: &mut TraitHandlerContext<'a>,
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
