mod debug_struct;
mod debug_union;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

use debug_struct::DebugStructHandler;
use debug_union::DebugUnionHandler;

pub struct DebugHandler;

impl TraitHandler for DebugHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                DebugStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => unimplemented!(),
            Data::Union(_) => {
                DebugUnionHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
        }
    }
}