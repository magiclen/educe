use crate::trait_handlers::TraitHandlerContext;
mod common;
mod into_enum;
mod into_struct;
mod models;
mod panic;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandlerMultiple;
use crate::Trait;

/// Dispatches the `Into` derive to the specialized handler for the shape of the input (struct, enum, or union).
pub(crate) struct IntoHandler;

impl TraitHandlerMultiple for IntoHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &[Meta],
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => into_struct::IntoStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => {
                into_enum::IntoEnumHandler::trait_meta_handler(ast, ctx, token_stream, traits, meta)
            },
            Data::Union(_) => {
                Err(crate::panic::trait_not_support_union(meta[0].path().get_ident().unwrap()))
            },
        }
    }
}
