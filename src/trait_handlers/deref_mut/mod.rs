mod models;

mod deref_mut_enum;
mod deref_mut_struct;

use deref_mut_enum::DerefMutEnumHandler;
use deref_mut_struct::DerefMutStructHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{panic, Trait};

pub struct DerefMutHandler;

impl TraitHandler for DerefMutHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                DerefMutStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                DerefMutEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => panic::trait_not_support_union(Trait::DerefMut),
        }
    }
}
