use std::fmt;

use quote::{IdentFragment, ToTokens};
use syn::{Ident, Index};

#[derive(Clone)]
pub(crate) enum IdentOrIndex {
    Ident(Ident),
    Index(Index),
}

impl From<Ident> for IdentOrIndex {
    #[inline]
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<Index> for IdentOrIndex {
    #[inline]
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl From<&Ident> for IdentOrIndex {
    #[inline]
    fn from(value: &Ident) -> Self {
        Self::Ident(value.clone())
    }
}

impl From<usize> for IdentOrIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self::Index(Index::from(value))
    }
}

impl ToTokens for IdentOrIndex {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match self {
            Self::Ident(ident) => ToTokens::to_tokens(ident, token_stream),
            Self::Index(index) => ToTokens::to_tokens(index, token_stream),
        }
    }
}

impl IdentOrIndex {
    #[inline]
    pub(crate) fn from_ident_with_index(ident: Option<&Ident>, index: usize) -> IdentOrIndex {
        if let Some(ident) = ident {
            Self::from(ident)
        } else {
            Self::from(index)
        }
    }
}

impl IdentFragment for IdentOrIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ident(ident) => IdentFragment::fmt(ident, f),
            Self::Index(index) => IdentFragment::fmt(index, f),
        }
    }
}
