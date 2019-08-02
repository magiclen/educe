mod models;

mod deref_struct;
mod deref_enum;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};
use crate::panic;

use deref_struct::DerefStructHandler;
use deref_enum::DerefEnumHandler;

pub struct DerefHandler;

impl TraitHandler for DerefHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                DerefStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                DerefEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Union(_) => panic::trait_not_support_union("PartialEq")
        }
    }
}