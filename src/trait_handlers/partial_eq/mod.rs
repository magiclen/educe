mod models;

mod partial_eq_enum;
mod partial_eq_struct;

use partial_eq_enum::PartialEqEnumHandler;
use partial_eq_struct::PartialEqStructHandler;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{panic, Trait};

pub struct PartialEqHandler;

impl TraitHandler for PartialEqHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        tokens: &mut TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) {
        match ast.data {
            Data::Struct(_) => {
                PartialEqStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Enum(_) => {
                PartialEqEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            },
            Data::Union(_) => panic::trait_not_support_union(Trait::PartialEq),
        }
    }
}
