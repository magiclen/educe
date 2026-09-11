use quote::{ToTokens, quote};
use syn::{Attribute, Meta, Token, punctuated::Punctuated};

use super::quote_mixed;

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

/// Returns true if the input type is `#[repr(packed)]` or `#[repr(packed(N))]`.
///
/// Every packing level is treated the same way, matching the built-in derives: a field's alignment cannot be worked out from the type syntax, so the generated code has to assume that it exceeds the packing.
pub(crate) fn is_packed(attributes: &[Attribute]) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident("repr")
            && let Meta::List(list) = &attribute.meta
            && let Ok(items) = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        {
            // A malformed `repr` is reported by the compiler on the user's own item, so a parse failure is simply skipped here.
            for item in items {
                if item.path().is_ident("packed") {
                    return true;
                }
            }
        }
    }

    false
}

/// Builds an expression that borrows a field for reading.
///
/// A field of a `#[repr(packed)]` type cannot be borrowed in place, so its value is copied into a temporary first, which requires the field type to implement `Copy` exactly like the built-in derives do.
pub(crate) fn borrow_field<T: ToTokens>(
    is_packed: bool,
    receiver: &proc_macro2::TokenStream,
    field_name: &T,
) -> proc_macro2::TokenStream {
    if is_packed {
        quote_mixed!(&{ #receiver.#field_name })
    } else {
        quote_mixed!(&#receiver.#field_name)
    }
}
