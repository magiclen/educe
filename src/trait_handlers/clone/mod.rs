use crate::trait_handlers::TraitHandlerContext;
mod clone_enum;
mod clone_struct;
mod clone_union;
mod models;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

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
