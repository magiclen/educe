use crate::trait_handlers::TraitHandlerContext;
mod clone_enum;
mod clone_struct;
mod clone_union;
mod models;

use syn::{Data, DeriveInput, ExprPath, Generics, Meta, Type, visit_mut::VisitMut};

use super::TraitHandler;
use crate::{Trait, common::quote_mixed};

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
///
/// The generated `Clone` impl is marked `#[automatically_derived]`, and `Clone` carries `#[rustc_trivial_field_reads]`, so the compiler skips its body during dead-code analysis. A custom clone method used only inside that body would therefore be wrongly reported as unused. This marker calls the method from an ordinary item that is still analyzed, and it takes the same generics and where clause as the impl so it compiles under exactly the same conditions.
pub(crate) fn create_mark_method_used(
    ast: &DeriveInput,
    generics: &Generics,
    field_ty: &Type,
    method: &ExprPath,
) -> proc_macro2::TokenStream {
    let lint_attributes = crate::common::attributes::generated_lint_attributes(&ast.attrs);
    let mut replace_self = crate::common::generics::ReplaceSelf::new(ast);
    let mut generics = generics.clone();
    let mut field_ty = field_ty.clone();
    let mut method = method.clone();
    replace_self.visit_generics_mut(&mut generics);
    replace_self.visit_type_mut(&mut field_ty);
    replace_self.visit_expr_path_mut(&mut method);
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // This function is generated glue whose only purpose is to reference the custom method, so its signature can look problematic in isolation (e.g. `&Vec<T>` would normally suggest `clippy::ptr_arg`, or an unused generic would trigger `clippy::extra_unused_type_parameters`). Lints like these already do not fire on code coming from an external proc-macro, but the `clippy::all` allow is kept here as a low-cost safeguard in case that exemption ever narrows.
    quote_mixed!(
        #lint_attributes
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
