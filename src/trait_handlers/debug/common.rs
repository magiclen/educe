use quote::quote;
use syn::{DeriveInput, Path, Type};

#[inline]
pub(crate) fn create_debug_map_builder() -> proc_macro2::TokenStream {
    quote!(
        #[allow(non_camel_case_types)] // We're using __ to help avoid clashes.
        struct Educe__RawString(&'static str);

        impl ::core::fmt::Debug for Educe__RawString {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.0)
            }
        }

        let mut builder = f.debug_map();
    )
}

/// Builds the `let arg = { ... };` statement that wraps a field so it is formatted with a custom method, together with a module-level marker that keeps the method counted as used.
///
/// The first returned token stream belongs inside the generated `fmt` body; the second one is a standalone item that must be emitted at module level, next to the impl.
#[inline]
pub(crate) fn create_format_arg(
    ast: &DeriveInput,
    field_ty: &Type,
    format_method: &Path,
    field_expr: proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ty_ident = &ast.ident;

    // We use the complete original generics, not filtered by field,
    // and include a PhantomData<Self> in our wrapper struct to use the generics.
    //
    // This avoids having to try to calculate the right *subset* of the generics
    // relevant for this field, which is nontrivial and maybe impossible.
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let arg = quote!(
        let arg = {
            #[allow(non_camel_case_types)] // We're using __ to help avoid clashes.
            struct Educe__DebugField<V, M>(V, ::core::marker::PhantomData<M>);

            impl #impl_generics ::core::fmt::Debug
                for Educe__DebugField<&#field_ty, #ty_ident #ty_generics>
                #where_clause
            {
                #[inline]
                fn fmt(&self, educe__f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #format_method(self.0, educe__f)
                }
            }

            Educe__DebugField(#field_expr, ::core::marker::PhantomData::<Self>)
        };
    );

    // The generated `Debug` impl is marked `#[automatically_derived]`, and `Debug` carries `#[rustc_trivial_field_reads]`, so the compiler skips its body during dead-code analysis. A custom formatting method used only inside that body would therefore be wrongly reported as unused. This marker references the method from an ordinary item that is still analyzed, so the method stays counted as used, and it mirrors the real call site so it compiles under exactly the same conditions.
    //
    // This function is generated glue whose only purpose is to reference the custom method, so its signature can look problematic in isolation (e.g. `&Vec<T>` would normally suggest `clippy::ptr_arg`, or an unused generic would trigger `clippy::extra_unused_type_parameters`). Lints like these already do not fire on code coming from an external proc-macro, but the `clippy::all` allow is kept here as a low-cost safeguard in case that exemption ever narrows.
    let mark = quote!(
        const _: () = {
            #[allow(dead_code, clippy::all)]
            fn __educe_debug_method_used #impl_generics (
                educe__value: &#field_ty,
                educe__f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result #where_clause {
                #format_method(educe__value, educe__f)
            }
        };
    );

    (arg, mark)
}
