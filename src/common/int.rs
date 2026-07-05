use syn::{Expr, Lit, Meta, MetaNameValue, UnOp};

use super::path::path_to_string;

// These helpers parse integer parameters such as `rank = -5`, accepting both bare integers and integers in strings.
#[inline]
pub(crate) fn meta_name_value_2_isize(name_value: &MetaNameValue) -> syn::Result<isize> {
    match &name_value.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(lit) => {
                return lit
                    .value()
                    .parse::<isize>()
                    .map_err(|error| syn::Error::new_spanned(lit, error));
            },
            Lit::Int(lit) => return lit.base10_parse(),
            _ => (),
        },
        Expr::Unary(unary) => {
            if let UnOp::Neg(_) = unary.op
                && let Expr::Lit(lit) = unary.expr.as_ref()
                && let Lit::Int(lit) = &lit.lit
            {
                let s = format!("-{}", lit.base10_digits());

                return s.parse::<isize>().map_err(|error| syn::Error::new_spanned(lit, error));
            }
        },
        _ => (),
    }

    Err(syn::Error::new_spanned(
        &name_value.value,
        format!("expected `{path} = integer`", path = path_to_string(&name_value.path)),
    ))
}

#[inline]
pub(crate) fn meta_2_isize(meta: &Meta) -> syn::Result<isize> {
    match &meta {
        Meta::NameValue(name_value) => meta_name_value_2_isize(name_value),
        Meta::List(list) => {
            let lit = list.parse_args::<Lit>()?;

            match &lit {
                Lit::Str(lit) => lit
                    .value()
                    .parse::<isize>()
                    .map_err(|error| syn::Error::new_spanned(lit, error)),
                Lit::Int(lit) => lit.base10_parse(),
                _ => Err(syn::Error::new_spanned(lit, "not an integer")),
            }
        },
        Meta::Path(path) => Err(syn::Error::new_spanned(
            path,
            format!(
                "expected `{path} = integer` or `{path}(integer)`",
                path = path_to_string(path)
            ),
        )),
    }
}
