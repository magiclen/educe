mod models;

mod ord_enum;
mod ord_struct;

use ord_enum::OrdEnumHandler;
use ord_struct::OrdStructHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{panic, Trait};

pub struct OrdHandler;

impl TraitHandler for OrdHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                OrdStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                OrdEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => panic::trait_not_support_union(Trait::Ord),
        }
    }
}
