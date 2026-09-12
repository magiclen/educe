use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Meta, Token, punctuated::Punctuated};

use crate::common::quote_mixed;

#[derive(Debug)]
/// The integer type that holds the discriminant values of an enum.
///
/// Comparisons use the logical discriminant type without reading the enum memory layout.
pub(crate) enum DiscriminantType {
    ISize,
    I8,
    I16,
    I32,
    I64,
    I128,
    USize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl DiscriminantType {
    #[inline]
    pub(crate) fn parse_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match s.as_ref() {
            "i8" => Some(Self::I8),
            "i16" => Some(Self::I16),
            "i32" => Some(Self::I32),
            "i64" => Some(Self::I64),
            "i128" => Some(Self::I128),
            "isize" => Some(Self::ISize),
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "u32" => Some(Self::U32),
            "u64" => Some(Self::U64),
            "u128" => Some(Self::U128),
            "usize" => Some(Self::USize),
            _ => None,
        }
    }

    #[inline]
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::ISize => "isize",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::USize => "usize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
        }
    }
}

impl ToTokens for DiscriminantType {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(self.as_str(), Span::call_site());
        tokens.extend(quote!(::core::primitive::#ident));
    }
}

impl DiscriminantType {
    /// Leaves discriminant expressions to the compiler, using `isize` unless an integer repr is set.
    pub(crate) fn from_ast(ast: &DeriveInput) -> syn::Result<(Self, Vec<TokenStream>)> {
        let Data::Enum(data) = &ast.data else {
            return Err(syn::Error::new_spanned(ast, "not an enum"));
        };
        let mut repr = Self::ISize;
        for attr in &ast.attrs {
            if attr.path().is_ident("repr")
                && let Meta::List(list) = &attr.meta
                && let Ok(items) =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            {
                // A malformed `repr` is reported by the compiler on the user's own item, so a parse failure is simply skipped here.
                for item in items {
                    if let Meta::Path(path) = item
                        && let Some(ident) = path.get_ident()
                        && let Some(ty) = Self::parse_str(ident.to_string())
                    {
                        repr = ty;
                    }
                }
            }
        }
        let mut values = Vec::with_capacity(data.variants.len());
        let mut base = quote_mixed!(0);
        let mut offset = 0usize;
        for variant in &data.variants {
            if let Some((_, expression)) = &variant.discriminant {
                base = quote_mixed!(#expression);
                offset = 0;
            }
            let value = if offset == 0 {
                quote_mixed!(const { #base })
            } else {
                let offset = proc_macro2::Literal::usize_unsuffixed(offset);
                // A signed discriminant range can contain more variants than its positive maximum.
                quote_mixed!(const {
                    let base: #repr = #base;
                    base.wrapping_add(#offset as #repr)
                })
            };
            values.push(value);
            offset += 1;
        }
        Ok((repr, values))
    }
}
