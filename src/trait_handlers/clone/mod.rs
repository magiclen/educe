mod clone_data;
mod clone_union;
mod models;

use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

pub(crate) struct CloneHandler;

impl TraitHandler for CloneHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        match ast.data {
            Data::Struct(_) | Data::Enum(_) => {
                clone_data::CloneDataHandler::trait_meta_handler(ast, token_stream, traits, meta)
            },
            Data::Union(_) => {
                clone_union::CloneUnionHandler::trait_meta_handler(ast, token_stream, traits, meta)
            },
        }
    }
}
