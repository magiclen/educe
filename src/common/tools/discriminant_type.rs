use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Data, DeriveInput, Expr, Lit, Meta, Token, UnOp, punctuated::Punctuated};

#[derive(Debug)]
/// The integer type that holds the discriminant values of an enum.
///
/// The enum comparison handlers read the discriminant by reinterpreting the leading bytes of an enum value as this type, so it has to match the actual layout.
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
        tokens.append(Ident::new(self.as_str(), Span::call_site()));
    }
}

impl DiscriminantType {
    /// Determines the discriminant type of an enum.
    ///
    /// An explicit `#[repr(intN)]` attribute wins; otherwise the explicit discriminant expressions are evaluated (implicit ones count up from the previous value) and the smallest integer type that covers their range is chosen, mirroring what the compiler does for the default representation.
    pub(crate) fn from_ast(ast: &DeriveInput) -> syn::Result<Self> {
        if let Data::Enum(data) = &ast.data {
            for attr in ast.attrs.iter() {
                if attr.path().is_ident("repr") {
                    // #[repr(u8)], #[repr(u16)], ..., etc.
                    if let Meta::List(list) = &attr.meta {
                        let result =
                            list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;

                        if let Some(value) = result.into_iter().next()
                            && let Some(t) = Self::parse_str(value.to_string())
                        {
                            return Ok(t);
                        }
                    }
                }
            }

            // Track the smallest and largest discriminant values while walking the variants; `counter` is the value the next variant gets when it has no explicit discriminant.
            let mut min = i128::MAX;
            let mut max = i128::MIN;
            let mut counter = 0i128;

            for variant in data.variants.iter() {
                if let Some((_, exp)) = variant.discriminant.as_ref() {
                    match exp {
                        Expr::Lit(lit) => {
                            if let Lit::Int(lit) = &lit.lit {
                                counter = lit
                                    .base10_parse()
                                    .map_err(|error| syn::Error::new_spanned(lit, error))?;
                            } else {
                                return Err(syn::Error::new_spanned(lit, "not an integer"));
                            }
                        },
                        Expr::Unary(unary) => {
                            if let UnOp::Neg(_) = unary.op {
                                if let Expr::Lit(lit) = unary.expr.as_ref() {
                                    if let Lit::Int(lit) = &lit.lit {
                                        match lit.base10_parse::<i128>() {
                                            Ok(i) => {
                                                counter = -i;
                                            },
                                            Err(error) => {
                                                // overflow
                                                if lit.base10_digits()
                                                    == "170141183460469231731687303715884105728"
                                                {
                                                    counter = i128::MIN;
                                                } else {
                                                    return Err(syn::Error::new_spanned(
                                                        lit, error,
                                                    ));
                                                }
                                            },
                                        }
                                    } else {
                                        return Err(syn::Error::new_spanned(lit, "not an integer"));
                                    }
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        &unary.expr,
                                        "not a literal",
                                    ));
                                }
                            } else {
                                return Err(syn::Error::new_spanned(
                                    unary.op,
                                    "this operation is not allow here",
                                ));
                            }
                        },
                        _ => return Err(syn::Error::new_spanned(exp, "not a literal")),
                    }
                }

                if min > counter {
                    min = counter;
                }

                if max < counter {
                    max = counter;
                }

                counter = counter.saturating_add(1);
            }

            Ok(if min >= i8::MIN as i128 && max <= i8::MAX as i128 {
                Self::I8
            } else if min >= i16::MIN as i128 && max <= i16::MAX as i128 {
                Self::I16
            } else if min >= i32::MIN as i128 && max <= i32::MAX as i128 {
                Self::I32
            } else if min >= i64::MIN as i128 && max <= i64::MAX as i128 {
                Self::I64
            } else {
                Self::I128
            })
        } else {
            Err(syn::Error::new_spanned(ast, "not an enum"))
        }
    }
}
