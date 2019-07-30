mod debug;
mod partial_eq;
mod eq;
mod hash;
mod default;

use std::str::FromStr;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{self, DeriveInput, Meta, LitStr, Path, Expr, WhereClause, WherePredicate, GenericParam, punctuated::Punctuated, token::Comma};
use crate::quote::ToTokens;

pub use debug::DebugHandler;
pub use partial_eq::PartialEqHandler;
pub use eq::EqHandler;
pub use hash::HashHandler;
pub use default::DefaultHandler;

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

#[inline]
pub fn create_expr_from_lit_str(s: &LitStr) -> Option<Expr> {
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
pub fn create_expr_string_from_lit_str(s: &LitStr) -> Option<String> {
    create_expr_from_lit_str(s).map(|expr| expr.into_token_stream().to_string().replace(" ", ""))
}

#[inline]
pub fn create_where_predicates_from_lit_str(s: &LitStr) -> Option<Punctuated<WherePredicate, Comma>> {
    let s = s.value();

    let s = s.trim();

    if s.is_empty() {
        None
    } else {
        let s = format!("where {}", s);

        let tokens = TokenStream::from_str(&s).unwrap();

        let where_clause: WhereClause = syn::parse(tokens.into()).unwrap();

        Some(where_clause.predicates)
    }
}

#[inline]
pub fn create_where_predicates_from_generic_parameters(p: &Punctuated<GenericParam, Comma>, bound_trait: &Path) -> Punctuated<WherePredicate, Comma> {
    let mut where_predicates = Punctuated::new();

    for param in p.iter() {
        if let GenericParam::Type(typ) = param {
            let ident = &typ.ident;

            where_predicates.push(syn::parse(quote! { #ident: #bound_trait }.into()).unwrap());
        }
    }

    where_predicates
}