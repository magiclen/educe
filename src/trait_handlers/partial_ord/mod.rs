mod models;

mod partial_ord_struct;
mod partial_ord_enum;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};
use crate::panic;

use partial_ord_struct::PartialOrdStructHandler;
use partial_ord_enum::PartialOrdEnumHandler;

pub struct PartialOrdHandler;

impl TraitHandler for PartialOrdHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                PartialOrdStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                PartialOrdEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Union(_) => panic::trait_not_support_union("PartialOrd")
        }
    }
}