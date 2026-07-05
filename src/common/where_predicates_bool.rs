use std::collections::HashSet;

use quote::{ToTokens, quote};
use syn::{
    Expr, GenericParam, Ident, Lit, Meta, MetaNameValue, Path, Token, Type, WherePredicate,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

use super::{
    path::path_to_string,
    r#type::{
        BoundExceptions, find_bare_ident_in_type, find_idents_in_type, type_mentions_ident,
        type_uses_type_params,
    },
};

pub(crate) type WherePredicates = Punctuated<WherePredicate, Token![,]>;

pub(crate) enum WherePredicatesOrBool {
    WherePredicates(WherePredicates),
    Bool(bool),
    /// The `bound(*)` form, which asks for a bound on every generic type parameter like the built-in derives do.
    All,
}

impl WherePredicatesOrBool {
    /// Converts a literal into where predicates or a boolean flag: a bool literal toggles the automatic bound, a string literal is parsed as where predicates, and an empty string means `false`.
    fn from_lit(lit: &Lit) -> syn::Result<Self> {
        match lit {
            Lit::Bool(lit) => Ok(Self::Bool(lit.value)),
            Lit::Str(lit) => match lit.parse_with(WherePredicates::parse_terminated) {
                Ok(where_predicates) => Ok(Self::WherePredicates(where_predicates)),
                Err(_) if lit.value().is_empty() => Ok(Self::Bool(false)),
                Err(error) => Err(error),
            },
            _ => Err(syn::Error::new_spanned(
                lit,
                "expected a boolean literal or a string of where predicates",
            )),
        }
    }
}

impl Parse for WherePredicatesOrBool {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // `bound(*)` requests the built-in-derive behavior: bound every generic type parameter.
        if input.parse::<Token![*]>().is_ok() {
            return Ok(Self::All);
        }

        if let Ok(lit) = input.parse::<Lit>() {
            return Self::from_lit(&lit);
        }

        Ok(Self::WherePredicates(input.parse_terminated(WherePredicate::parse, Token![,])?))
    }
}

#[inline]
pub(crate) fn meta_name_value_2_where_predicates_bool(
    name_value: &MetaNameValue,
) -> syn::Result<WherePredicatesOrBool> {
    if let Expr::Lit(lit) = &name_value.value
        && matches!(&lit.lit, Lit::Bool(_) | Lit::Str(_))
    {
        return WherePredicatesOrBool::from_lit(&lit.lit);
    }

    Err(syn::Error::new_spanned(
        &name_value.value,
        format!(
            "expected `{path} = \"where_predicates\"` or `{path} = false`",
            path = path_to_string(&name_value.path)
        ),
    ))
}

#[inline]
pub(crate) fn meta_2_where_predicates(meta: &Meta) -> syn::Result<WherePredicatesOrBool> {
    match &meta {
        Meta::NameValue(name_value) => meta_name_value_2_where_predicates_bool(name_value),
        Meta::List(list) => list.parse_args::<WherePredicatesOrBool>(),
        Meta::Path(path) => Err(syn::Error::new_spanned(
            path,
            format!(
                "expected `{path} = \"where_predicates\"`, `{path}(where_predicates)`, \
                 `{path}(*)`, `{path} = false`, or `{path}(false)`",
                path = path.clone().into_token_stream()
            ),
        )),
    }
}

/// Creates a `Param: Trait` predicate for every generic type parameter, matching the behavior of the built-in derives.
#[inline]
pub(crate) fn create_where_predicates_from_all_generic_parameters(
    params: &Punctuated<GenericParam, Comma>,
    bound_trait: &Path,
) -> WherePredicates {
    let mut where_predicates = Punctuated::new();

    for param in params {
        if let GenericParam::Type(ty) = param {
            let ident = &ty.ident;

            where_predicates.push(syn::parse2(quote! { #ident: #bound_trait }).unwrap());
        }
    }

    where_predicates
}

/// The automatic bound engine: turns the field types into where predicates for one trait.
///
/// Each field type is processed with the following rules, in order:
/// 1. A type that the exception table marks as unconditional (e.g. `Arc<T>` for `Clone`) produces nothing, because its predicate would always hold.
/// 2. A type that uses no generic type parameter produces nothing, because its predicate would be constant and could send the trait solver into an infinite loop on indirectly recursive types.
/// 3. A type that forwards the trait to its type arguments (e.g. `Option<T>` for `Copy`) produces the predicates of its arguments instead, applying these rules recursively, so `Vec<Box<T>>` for `Clone` produces just `T: Clone`.
/// 4. A type that mentions the type currently being derived (e.g. `Box<List<T>>` inside `List<T>`) is degraded to `Param: Trait` bounds for the parameters it uses, because a self-referencing predicate would overflow the trait solver (E0275).
/// 5. Any other type produces the precise predicate `FieldType: Trait`, so the compiler verifies the real requirement even for types with unusual conditional impls.
pub(crate) fn create_where_predicates_from_field_types(
    params: &Punctuated<GenericParam, Comma>,
    bound_trait: &Path,
    types: &[&Type],
    self_ident: &Ident,
    exceptions: &BoundExceptions,
) -> WherePredicates {
    fn process_type(
        ty: &Type,
        params: &Punctuated<GenericParam, Comma>,
        bound_trait: &Path,
        self_ident: &Ident,
        exceptions: &BoundExceptions,
        where_predicates: &mut WherePredicates,
        seen: &mut HashSet<String>,
    ) {
        // Identical predicates can be produced by different fields, so deduplicate them by their token strings.
        let mut push = |where_predicates: &mut WherePredicates, predicate: WherePredicate| {
            if seen.insert(predicate.to_token_stream().to_string()) {
                where_predicates.push(predicate);
            }
        };

        // Rule 1: the trait is implemented for this type no matter what the type arguments are.
        if exceptions.type_is_unconditional(ty) {
            return;
        }

        // Rule 2: a type without generic type parameters would produce a constant predicate.
        if !type_uses_type_params(ty, params) {
            return;
        }

        // Rule 3: a forwarding type is replaced by its type arguments, processed recursively.
        if let Some(argument_types) = exceptions.forwarding_type_arguments(ty) {
            for ty in argument_types {
                process_type(
                    ty,
                    params,
                    bound_trait,
                    self_ident,
                    exceptions,
                    where_predicates,
                    seen,
                );
            }

            return;
        }

        if type_mentions_ident(ty, self_ident) {
            // Rule 4: degrade a self-referencing type to per-parameter bounds.
            let mut used = HashSet::new();

            find_idents_in_type(&mut used, ty, exceptions);

            for param in params {
                if let GenericParam::Type(param) = param
                    && used.contains(&param.ident)
                {
                    let ident = &param.ident;

                    push(where_predicates, syn::parse2(quote! { #ident: #bound_trait }).unwrap());
                }
            }
        } else {
            // Rule 5: the precise predicate.
            push(where_predicates, syn::parse2(quote! { #ty: #bound_trait }).unwrap());
        }
    }

    let mut where_predicates = Punctuated::new();

    let mut seen: HashSet<String> = HashSet::new();

    for ty in types {
        process_type(
            ty,
            params,
            bound_trait,
            self_ident,
            exceptions,
            &mut where_predicates,
            &mut seen,
        );
    }

    where_predicates
}

/// Creates a `Param: Trait` predicate for each generic type parameter that appears bare as one of the field types.
///
/// This is only used by the `Into` handler, where a `T: Into<Target>` bound is meaningful solely when a field's type is `T` itself.
pub(crate) fn create_where_predicates_from_bare_field_types(
    params: &Punctuated<GenericParam, Comma>,
    bound_trait: &Path,
    types: &[&Type],
) -> WherePredicates {
    let mut where_predicates = Punctuated::new();

    let mut set = HashSet::new();

    for t in types {
        find_bare_ident_in_type(&mut set, t);
    }

    for param in params {
        if let GenericParam::Type(ty) = param {
            let ident = &ty.ident;

            if set.contains(ident) {
                where_predicates.push(syn::parse2(quote! { #ident: #bound_trait }).unwrap());
            }
        }
    }

    where_predicates
}
