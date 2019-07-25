mod debug;

use std::str::FromStr;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{self, DeriveInput, Meta, LitStr, Path};
use crate::quote::ToTokens;

pub use debug::DebugHandler;

pub trait TraitHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta);
}

#[inline]
pub fn create_path_from_lit_str(s: &LitStr) -> Option<Path> {
    let s = s.value();

    let s = s.trim();

    if s.is_empty() {
        None
    } else {
        let tokens = TokenStream::from_str(s).unwrap();

        Some(syn::parse(tokens.into()).unwrap())
    }
}

#[inline]
pub fn create_path_string_from_lit_str(s: &LitStr) -> Option<String> {
    create_path_from_lit_str(s).map(|path| path.into_token_stream().to_string().replace(" ", ""))
}