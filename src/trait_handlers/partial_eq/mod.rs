use crate::trait_handlers::TraitHandlerContext;
mod models;
mod panic;
mod partial_eq_enum;
mod partial_eq_struct;
mod partial_eq_union;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

/// Dispatches the `PartialEq` derive to the specialized handler for the shape of the input (struct, enum, or union).
pub(crate) struct PartialEqHandler;

impl TraitHandler for PartialEqHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => partial_eq_struct::PartialEqStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => partial_eq_enum::PartialEqEnumHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Union(_) => partial_eq_union::PartialEqUnionHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
        }
    }
}
