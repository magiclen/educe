mod models;

mod partial_eq_struct;
//mod debug_enum;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

use partial_eq_struct::PartialEqStructHandler;
//use debug_enum::DebugEnumHandler;

pub struct PartialEqHandler;

impl TraitHandler for PartialEqHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                PartialEqStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                unimplemented!();
            }
            Data::Union(_) => {
                unimplemented!();
            }
        }
    }
}