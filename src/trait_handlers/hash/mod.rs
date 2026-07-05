use crate::trait_handlers::TraitHandlerContext;
mod hash_enum;
mod hash_struct;
mod hash_union;
mod models;
mod panic;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

/// Dispatches the `Hash` derive to the specialized handler for the shape of the input (struct, enum, or union).
pub(crate) struct HashHandler;

impl TraitHandler for HashHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => hash_struct::HashStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => {
                hash_enum::HashEnumHandler::trait_meta_handler(ast, ctx, token_stream, traits, meta)
            },
            Data::Union(_) => hash_union::HashUnionHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
        }
    }
}
