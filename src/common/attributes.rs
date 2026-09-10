use quote::{ToTokens, quote};
use syn::{Attribute, Meta};

/// Builds the attributes that every generated impl carries.
pub(crate) fn generated_impl_attributes(attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let lints = generated_lint_attributes(attributes);
    quote!(#[automatically_derived] #lints)
}

/// Copies `allow`, `warn`, and `deny` from the input type.
/// An `expect` becomes `allow` because a generated item may not trigger the original lint.
pub(crate) fn generated_lint_attributes(attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let mut token_stream = proc_macro2::TokenStream::new();

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
