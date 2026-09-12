use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident};
use syn::{Ident, Index};

use super::quote_mixed;

/// A field accessor that is either a name (for named fields) or a position index (for tuple fields).
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
        if let Some(ident) = ident { Self::from(ident) } else { Self::from(index) }
    }

    /// Builds a local binding named after the field, e.g. `_s_value` or `_s_0`, with macro hygiene so it cannot capture anything the user wrote.
    #[inline]
    pub(crate) fn to_binding(&self, prefix: &str) -> Ident {
        match self {
            Self::Ident(ident) => format_ident!("{}{}", prefix, ident, span = Span::mixed_site()),
            Self::Index(index) => {
                format_ident!("{}{}", prefix, index.index, span = Span::mixed_site())
            },
        }
    }

    /// Builds one element of a struct pattern or literal; the field name is only written when the fields are named.
    #[inline]
    pub(crate) fn to_field(&self, value: &TokenStream) -> TokenStream {
        match self {
            Self::Ident(ident) => quote_mixed!(#ident: #value,),
            Self::Index(_) => quote_mixed!(#value,),
        }
    }
}
