mod models;

mod clone_struct;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

use clone_struct::CloneStructHandler;

pub struct CloneHandler;

impl TraitHandler for CloneHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                CloneStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                unimplemented!()
            }
            Data::Union(_) => {
                unimplemented!()
            }
        }
    }
}