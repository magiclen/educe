use proc_macro2::{Ident, TokenStream};

use super::quote_mixed;

/// Builds the statement that binds the whole storage of a union as a byte slice.
///
/// A union does not track its active field at runtime, so the unsafe union derives read its storage as bytes instead of reading a field.
pub(crate) fn union_bytes(binding: &Ident, receiver: &TokenStream) -> TokenStream {
    quote_mixed!(
        // SAFETY: The live union reference provides a valid pointer and size; the unsafe derive contract requires every byte, including padding and bytes outside the active field, to be initialized and unchanged during this call.
        // The user must preserve this condition after every construction, write, move, and copy; reading uninitialized bytes is undefined behavior.
        let #binding = unsafe {
            ::core::slice::from_raw_parts(
                #receiver as *const Self as *const ::core::primitive::u8,
                ::core::mem::size_of::<Self>(),
            )
        };
    )
}
