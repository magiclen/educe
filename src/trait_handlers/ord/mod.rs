use crate::trait_handlers::TraitHandlerContext;
mod models;
mod ord_enum;
mod ord_struct;
mod panic;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

/// Dispatches the `Ord` derive to the specialized handler for the shape of the input.
pub(crate) struct OrdHandler;

impl TraitHandler for OrdHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) => ord_struct::OrdStructHandler::trait_meta_handler(
                ast,
                ctx,
                token_stream,
                traits,
                meta,
            ),
            Data::Enum(_) => {
                ord_enum::OrdEnumHandler::trait_meta_handler(ast, ctx, token_stream, traits, meta)
            },
            Data::Union(_) => {
                Err(crate::panic::trait_not_support_union(meta.path().get_ident().unwrap()))
            },
        }
    }
}

/// Returns the traits whose recorded bounds `Ord` inherits when its own bound is automatic.
pub(crate) fn prerequisites() -> Vec<Trait> {
    vec![
        #[cfg(feature = "Eq")]
        Trait::Eq,
        #[cfg(feature = "PartialOrd")]
        Trait::PartialOrd,
    ]
}
