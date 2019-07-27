mod models;

mod hash_struct;
//mod debug_enum;
//mod debug_union;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};
use crate::panic;

use hash_struct::HashStructHandler;
//use debug_enum::DebugEnumHandler;
//use debug_union::DebugUnionHandler;

pub struct HashHandler;

impl TraitHandler for HashHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                HashStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                unimplemented!();
            }
            Data::Union(_) => panic::trait_not_support_union("Hash")
        }
    }
}