mod models;

mod hash_enum;
mod hash_struct;

use hash_enum::HashEnumHandler;
use hash_struct::HashStructHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{panic, Trait};

pub struct HashHandler;

impl TraitHandler for HashHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                HashStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                HashEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => panic::trait_not_support_union(Trait::Hash),
        }
    }
}
