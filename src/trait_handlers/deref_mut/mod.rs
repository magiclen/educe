mod models;

mod deref_mut_struct;
mod deref_mut_enum;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};
use crate::panic;

use deref_mut_struct::DerefMutStructHandler;
use deref_mut_enum::DerefMutEnumHandler;

pub struct DerefMutHandler;

impl TraitHandler for DerefMutHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                DerefMutStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                DerefMutEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Union(_) => panic::trait_not_support_union("DerefMut")
        }
    }
}