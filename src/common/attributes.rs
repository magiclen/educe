use quote::{ToTokens, quote};
use syn::Attribute;

/// Builds the attributes that every generated impl carries.
///
/// `#[automatically_derived]` tells the compiler and lints such as clippy that the impl is machine generated, and the lint-level attributes (`allow`/`expect`/`warn`/`deny`) written on the derive input are copied onto the impl, so user lint settings also cover the generated code just like with the built-in derives.
pub(crate) fn generated_impl_attributes(attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let mut token_stream = quote!(#[automatically_derived]);

    for attribute in attributes {
        let path = attribute.path();

        if path.is_ident("allow")
            || path.is_ident("expect")
            || path.is_ident("warn")
            || path.is_ident("deny")
        {
            attribute.to_tokens(&mut token_stream);
        }
    }

    token_stream
}
