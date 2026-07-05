use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Data, DeriveInput, Expr, Lit, Meta, Token, UnOp, punctuated::Punctuated};

#[derive(Debug)]
/// The integer type that holds the discriminant values of an enum.
///
/// The enum comparison handlers order variants by their discriminant values, and this type is the one those values are compared as, so it has to match the type the compiler picks for the enum.
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
    /// Determines the discriminant type of an enum together with the discriminant value of every variant, in declaration order.
    ///
    /// An explicit `#[repr(intN)]` attribute decides the type; otherwise the smallest integer type that covers the range of the discriminant values is chosen, mirroring what the compiler does for the default representation. The values themselves are always computed by evaluating the explicit discriminant expressions (implicit ones count up from the previous value), which is exactly the order `core::mem::discriminant` would follow.
    pub(crate) fn from_ast(ast: &DeriveInput) -> syn::Result<(Self, Vec<i128>)> {
        if let Data::Enum(data) = &ast.data {
            let mut repr = None;

            for attr in ast.attrs.iter() {
                if attr.path().is_ident("repr") {
                    // #[repr(u8)], #[repr(u16)], ..., etc.
                    if let Meta::List(list) = &attr.meta {
                        let result =
                            list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;

                        for value in result {
                            if let Some(t) = Self::parse_str(value.to_string()) {
                                repr = Some(t);
                                break;
                            }
                        }
                    }

                    if repr.is_some() {
                        break;
                    }
                }
            }

            // Track the smallest and largest discriminant values while walking the variants; `counter` is the value the next variant gets when it has no explicit discriminant.
            let mut values = Vec::with_capacity(data.variants.len());
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

                values.push(counter);

                if min > counter {
                    min = counter;
                }

                if max < counter {
                    max = counter;
                }

                counter = counter.saturating_add(1);
            }

            let discriminant_type = if let Some(t) = repr {
                t
            } else if min >= i8::MIN as i128 && max <= i8::MAX as i128 {
                Self::I8
            } else if min >= i16::MIN as i128 && max <= i16::MAX as i128 {
                Self::I16
            } else if min >= i32::MIN as i128 && max <= i32::MAX as i128 {
                Self::I32
            } else if min >= i64::MIN as i128 && max <= i64::MAX as i128 {
                Self::I64
            } else {
                Self::I128
            };

            Ok((discriminant_type, values))
        } else {
            Err(syn::Error::new_spanned(ast, "not an enum"))
        }
    }
}
