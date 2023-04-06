mod models;

mod deref_enum;
mod deref_struct;

use deref_enum::DerefEnumHandler;
use deref_struct::DerefStructHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{panic, Trait};

pub struct DerefHandler;

impl TraitHandler for DerefHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                DerefStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                DerefEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => panic::trait_not_support_union(Trait::Deref),
        }
    }
}
