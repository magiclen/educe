mod models;

mod default_struct;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

use default_struct::DefaultStructHandler;

pub struct DefaultHandler;

impl TraitHandler for DefaultHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                DefaultStructHandler::trait_meta_handler(ast, tokens, traits, meta);
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