mod models;

mod debug_enum;
mod debug_struct;
mod debug_union;

use debug_enum::DebugEnumHandler;
use debug_struct::DebugStructHandler;
use debug_union::DebugUnionHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

pub struct DebugHandler;

impl TraitHandler for DebugHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                DebugStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                DebugEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => {
                DebugUnionHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
        }
    }
}
