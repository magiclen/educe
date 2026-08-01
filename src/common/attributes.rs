use quote::{ToTokens, quote};
use syn::{Attribute, Meta};

/// Builds the attributes that every generated impl carries.
///
/// `#[automatically_derived]` tells the compiler and lints such as clippy that the impl is machine generated, and the lint-level attributes (`allow`/`expect`/`warn`/`deny`) written on the derive input are copied onto the impl, so user lint settings also cover the generated code just like with the built-in derives.
///
/// An `expect` is copied as an `allow`, because the copy would otherwise carry its own expectation that the lint fires inside the generated impl as well. A lint that only applies to the type, e.g. `non_camel_case_types`, never fires there, and the compiler would report the user's attribute as an unfulfilled expectation. An `allow` suppresses the lint in the generated impl just the same, without demanding that it fire.
pub(crate) fn generated_impl_attributes(attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let mut token_stream = quote!(#[automatically_derived]);

    for attribute in attributes {
        let path = attribute.path();

        if path.is_ident("expect") {
            // A malformed `expect` is reported by the compiler on the user's own item, so there is nothing to copy here.
            if let Meta::List(list) = &attribute.meta {
                let lints = &list.tokens;

                // The `allow` ident is created here instead of being taken from the user's attribute, so that lints looking for hand-written `allow` attributes, such as `clippy::allow_attributes`, still see this one as macro generated and skip it.
                token_stream.extend(quote!(#[allow(#lints)]));
            }
        } else if path.is_ident("allow") || path.is_ident("warn") || path.is_ident("deny") {
            attribute.to_tokens(&mut token_stream);
        }
    }

    token_stream
}
