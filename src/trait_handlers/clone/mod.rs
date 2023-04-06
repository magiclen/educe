mod models;

mod clone_enum;
mod clone_struct;
mod clone_union;

use clone_enum::CloneEnumHandler;
use clone_struct::CloneStructHandler;
use clone_union::CloneUnionHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

pub struct CloneHandler;

impl TraitHandler for CloneHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                CloneStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                CloneEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => {
                CloneUnionHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
        }
    }
}
