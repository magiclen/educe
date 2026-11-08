use proc_macro2::Ident;
use syn::{DeriveInput, ExprPath, Type};

use crate::{
    common::{generics::ValueBindings, marker::MethodMarker, quote_mixed},
    trait_handlers::TraitHandlerContext,
};

pub(crate) struct HelperTypes {
    pub(crate) field:      Ident,
    pub(crate) raw_string: Ident,
}

impl HelperTypes {
    pub(crate) fn new(bindings: &mut ValueBindings) -> Self {
        Self {
            field:      bindings.fresh("Educe__DebugField"),
            raw_string: bindings.fresh("Educe__RawString"),
        }
    }
}

/// Builds the helper type that prints a map key without the quotes a `str` would be formatted with.
///
/// A nameless struct or variant is formatted as a map, and its keys are the field names; the type is declared once per generated `fmt` body, so an enum with several such variants does not repeat it.
#[inline]
pub(crate) fn create_raw_string_type(ident: &Ident, formatter: &Ident) -> proc_macro2::TokenStream {
    quote_mixed!(
        #[allow(non_camel_case_types)]
        struct #ident(&'static ::core::primitive::str);

        impl ::core::fmt::Debug for #ident {
            #[inline]
            fn fmt(&self, #formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #formatter.write_str(self.0)
            }
        }
    )
}

/// Builds the statement that starts a map builder; the caller has to emit [`create_raw_string_type`] once in the same block.
#[inline]
pub(crate) fn create_debug_map_builder(
    formatter: &Ident,
    builder: &Ident,
) -> proc_macro2::TokenStream {
    quote_mixed!(let mut #builder = #formatter.debug_map();)
}

/// Wraps a field with a closure that keeps the source impl's bounds and `Self` scope.
#[inline]
pub(crate) fn create_format_arg(
    bindings: &mut ValueBindings,
    formatter: &Ident,
    ident: &Ident,
    field_ty: &Type,
    format_method: &ExprPath,
    field_expr: proc_macro2::TokenStream,
) -> (Ident, proc_macro2::TokenStream) {
    let value = bindings.binding("value");
    let arg = bindings.binding("arg");
    let tokens = quote_mixed!(
        let #arg = {
            #[allow(non_camel_case_types)]
            struct #ident<'a, V: ?::core::marker::Sized, F>(&'a V, F);

            impl<V: ?::core::marker::Sized, F> ::core::fmt::Debug for #ident<'_, V, F>
            where
                F: ::core::ops::Fn(&V, &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result,
            {
                #[inline]
                fn fmt(&self, #formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    (self.1)(self.0, #formatter)
                }
            }

            #ident(#field_expr, |#value: &#field_ty, #formatter: &mut ::core::fmt::Formatter<'_>| #format_method(#value, #formatter))
        };
    );
    (arg, tokens)
}

/// Keeps custom formatting methods visible to dead-code analysis under the final impl bounds.
pub(crate) fn create_mark_method_used(
    ast: &DeriveInput,
    ctx: &TraitHandlerContext,
    generics: &syn::Generics,
    field_ty: &Type,
    method: &ExprPath,
) -> proc_macro2::TokenStream {
    let marker = MethodMarker::new(ast, generics, field_ty, method);
    let (impl_generics, _, where_clause) = marker.generics.split_for_impl();
    let field_ty = &marker.field_ty;
    let method = &marker.method;
    let mut bindings = ctx.value_bindings(ast);
    let marker_name = bindings.fresh("__educe_debug_method_used");
    let value = bindings.binding("value");
    let formatter = bindings.binding("formatter");

    marker.wrap(
        quote_mixed!(
            fn #marker_name #impl_generics (
                #value: &#field_ty,
                #formatter: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result #where_clause {
                #method(#value, #formatter)
            }
        ),
        &bindings,
    )
}
