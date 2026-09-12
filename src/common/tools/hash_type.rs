use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Path, Type, spanned::Spanned};

#[derive(Debug, Clone)]
/// A type made comparable and hashable by its canonical token string, so it can serve as an ordered map key.
///
/// The original tokens are kept as well, so that writing the type back into generated code preserves its spans instead of parsing the string again.
pub(crate) struct HashType {
    key:    String,
    tokens: TokenStream,
    span:   Span,
}

impl HashType {
    #[inline]
    fn new(tokens: TokenStream, span: Span) -> Self {
        Self {
            key: tokens.to_string(),
            tokens,
            span,
        }
    }
}

impl PartialEq for HashType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl Eq for HashType {}

impl PartialOrd for HashType {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashType {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl Hash for HashType {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.key, state);
    }
}

impl Display for HashType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.key.replace("& '", "&'"), f)
    }
}

impl From<Type> for HashType {
    #[inline]
    fn from(value: Type) -> Self {
        Self::from(&value)
    }
}

impl From<&Type> for HashType {
    #[inline]
    fn from(value: &Type) -> Self {
        Self::new(value.to_token_stream(), value.span())
    }
}

impl From<Path> for HashType {
    #[inline]
    fn from(value: Path) -> Self {
        Self::from(&value)
    }
}

impl From<&Path> for HashType {
    #[inline]
    fn from(value: &Path) -> Self {
        Self::new(value.to_token_stream(), value.span())
    }
}

impl HashType {
    #[inline]
    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for HashType {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        token_stream.extend(self.tokens.clone());
    }
}
