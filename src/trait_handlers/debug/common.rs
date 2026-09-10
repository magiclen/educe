use syn::{DeriveInput, ExprPath, Type};

use crate::common::quote_mixed;

/// Builds the helper type that prints a map key without the quotes a `str` would be formatted with.
///
/// A nameless struct or variant is formatted as a map, and its keys are the field names; the type is declared once per generated `fmt` body, so an enum with several such variants does not repeat it.
#[inline]
pub(crate) fn create_raw_string_type() -> proc_macro2::TokenStream {
    quote_mixed!(
        #[allow(non_camel_case_types)] // We're using __ to help avoid clashes.
        struct Educe__RawString(&'static str);

        impl ::core::fmt::Debug for Educe__RawString {
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
    field_ty: &Type,
    format_method: &ExprPath,
    field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let value = proc_macro2::Ident::new("educe__value", proc_macro2::Span::mixed_site());
    let formatter = proc_macro2::Ident::new("educe__formatter", proc_macro2::Span::mixed_site());
    quote_mixed!(
        let arg = {
            #[allow(non_camel_case_types)]
            struct Educe__DebugField<'a, V: ?Sized, F>(&'a V, F);

            impl<V: ?Sized, F> ::core::fmt::Debug for Educe__DebugField<'_, V, F>
            where
                F: ::core::ops::Fn(&V, &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result,
            {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    (self.1)(self.0, f)
                }
            }

            Educe__DebugField(#field_expr, |#value: &#field_ty, #formatter: &mut ::core::fmt::Formatter<'_>| #format_method(#value, #formatter))
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
    use syn::visit_mut::VisitMut;
    let lint_attributes = crate::common::attributes::generated_lint_attributes(&ast.attrs);
    let mut replace_self = crate::common::generics::ReplaceSelf::new(ast);
    let mut generics = generics.clone();
    let mut field_ty = field_ty.clone();
    let mut method = method.clone();
    replace_self.visit_generics_mut(&mut generics);
    replace_self.visit_type_mut(&mut field_ty);
    replace_self.visit_expr_path_mut(&mut method);
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    quote_mixed!(
        #lint_attributes
        const _: () = {
            #[allow(dead_code, clippy::all)]
            fn __educe_debug_method_used #impl_generics (
                educe__value: &#field_ty,
                educe__f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result #where_clause {
                #method(educe__value, educe__f)
            }
        };
    )
}
