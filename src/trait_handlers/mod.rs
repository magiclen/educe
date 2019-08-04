#![cfg_attr(not(feature = "default"), allow(dead_code))]

#[cfg(feature = "Debug")]
mod debug;
#[cfg(feature = "PartialEq")]
mod partial_eq;
#[cfg(feature = "Eq")]
mod eq;
#[cfg(feature = "PartialOrd")]
mod partial_ord;
#[cfg(feature = "Ord")]
mod ord;
#[cfg(feature = "Hash")]
mod hash;
#[cfg(feature = "Default")]
mod default;
#[cfg(feature = "Clone")]
mod clone;
#[cfg(feature = "Copy")]
mod copy;
#[cfg(feature = "Deref")]
mod deref;
#[cfg(feature = "DerefMut")]
mod deref_mut;

use std::str::FromStr;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{self, DeriveInput, Meta, LitStr, Path, Expr, WhereClause, WherePredicate, GenericParam, punctuated::Punctuated, token::Comma};
use crate::quote::ToTokens;

#[cfg(feature = "Debug")]
pub use debug::DebugHandler;
#[cfg(feature = "PartialEq")]
pub use partial_eq::PartialEqHandler;
#[cfg(feature = "Eq")]
pub use eq::EqHandler;
#[cfg(feature = "PartialOrd")]
pub use partial_ord::PartialOrdHandler;
#[cfg(feature = "Ord")]
pub use ord::OrdHandler;
#[cfg(feature = "Hash")]
pub use hash::HashHandler;
#[cfg(feature = "Default")]
pub use default::DefaultHandler;
#[cfg(feature = "Clone")]
pub use clone::CloneHandler;
#[cfg(feature = "Copy")]
pub use copy::CopyHandler;
#[cfg(feature = "Deref")]
pub use deref::DerefHandler;
#[cfg(feature = "DerefMut")]
pub use deref_mut::DerefMutHandler;

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