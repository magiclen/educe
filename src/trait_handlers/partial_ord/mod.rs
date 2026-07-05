use crate::trait_handlers::TraitHandlerContext;
mod models;
mod panic;
mod partial_ord_enum;
mod partial_ord_struct;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

/// Dispatches the `PartialOrd` derive to the specialized handler for the shape of the input.
pub(crate) struct PartialOrdHandler;

impl TraitHandler for PartialOrdHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => partial_ord_struct::PartialOrdStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => partial_ord_enum::PartialOrdEnumHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Union(_) => {
                Err(crate::panic::trait_not_support_union(meta.path().get_ident().unwrap()))
            },
        }
    }
}

/// Returns the traits whose recorded bounds `PartialOrd` inherits when its own bound is automatic.
pub(crate) fn prerequisites() -> Vec<Trait> {
    vec![
        #[cfg(feature = "PartialEq")]
        Trait::PartialEq,
    ]
}
