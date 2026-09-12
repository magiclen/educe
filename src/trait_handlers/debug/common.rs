use proc_macro2::Ident;
use syn::{DeriveInput, ExprPath, Type};

use crate::common::{marker::MethodMarker, quote_mixed};

pub(crate) struct HelperTypes {
    pub(crate) field:      Ident,
    pub(crate) raw_string: Ident,
}

impl HelperTypes {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let mut used = crate::common::generics::UsedIdents::new(ast);
        Self {
            field:      used.select("Educe__DebugField"),
            raw_string: used.select("Educe__RawString"),
        }
    }
}

/// Builds the helper type that prints a map key without the quotes a `str` would be formatted with.
///
/// A nameless struct or variant is formatted as a map, and its keys are the field names; the type is declared once per generated `fmt` body, so an enum with several such variants does not repeat it.
#[inline]
pub(crate) fn create_raw_string_type(ident: &Ident) -> proc_macro2::TokenStream {
    quote_mixed!(
        #[allow(non_camel_case_types)]
        struct #ident(&'static ::core::primitive::str);

        impl ::core::fmt::Debug for #ident {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.0)
            }
        }
    )
}

/// Builds the statement that starts a map builder; the caller has to emit [`create_raw_string_type`] once in the same block.
#[inline]
pub(crate) fn create_debug_map_builder() -> proc_macro2::TokenStream {
    quote_mixed!(let mut builder = f.debug_map();)
}

/// Wraps a field with a closure that keeps the source impl's bounds and `Self` scope.
#[inline]
pub(crate) fn create_format_arg(
    ident: &Ident,
    field_ty: &Type,
    format_method: &ExprPath,
    field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let value = proc_macro2::Ident::new("educe__value", proc_macro2::Span::mixed_site());
    let formatter = proc_macro2::Ident::new("educe__formatter", proc_macro2::Span::mixed_site());
    quote_mixed!(
        let arg = {
            #[allow(non_camel_case_types)]
            struct #ident<'a, V: ?Sized, F>(&'a V, F);

            impl<V: ?Sized, F> ::core::fmt::Debug for #ident<'_, V, F>
            where
                F: ::core::ops::Fn(&V, &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result,
            {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    (self.1)(self.0, f)
                }
            }

            #ident(#field_expr, |#value: &#field_ty, #formatter: &mut ::core::fmt::Formatter<'_>| #format_method(#value, #formatter))
        };
    )
}

/// Keeps custom formatting methods visible to dead-code analysis under the final impl bounds.
pub(crate) fn create_mark_method_used(
    ast: &DeriveInput,
    generics: &syn::Generics,
    field_ty: &Type,
    method: &ExprPath,
) -> proc_macro2::TokenStream {
    let marker = MethodMarker::new(ast, generics, field_ty, method);
    let (impl_generics, _, where_clause) = marker.generics.split_for_impl();
    let field_ty = &marker.field_ty;
    let method = &marker.method;

    marker.wrap(quote_mixed!(
        fn __educe_debug_method_used #impl_generics (
            educe__value: &#field_ty,
            educe__f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result #where_clause {
            #method(educe__value, educe__f)
        }
    ))
}
