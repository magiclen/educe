mod models;

mod ord_struct;
mod ord_enum;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};
use crate::panic;

use ord_struct::OrdStructHandler;
use ord_enum::OrdEnumHandler;

pub struct OrdHandler;

impl TraitHandler for OrdHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                OrdStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                OrdEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Union(_) => panic::trait_not_support_union("Ord")
        }
    }
}