use syn::Meta;

#[inline]
pub(crate) fn union_without_unsafe(meta: &Meta) -> syn::Error {
    let path = meta.path();
    let suggestion = if let Meta::List(list) = meta
        && !list.tokens.is_empty()
    {
        let arguments = &list.tokens;
        quote::quote!(#[educe(#path(unsafe, #arguments))])
    } else {
        quote::quote!(#[educe(#path(unsafe))])
    };
    syn::Error::new_spanned(
        meta,
        format!(
            "a union's `Hash` implementation reads its entire storage as bytes; reading \
             uninitialized bytes is undefined behavior\n* Every byte must be initialized and \
             readable during each call, including padding and bytes outside the active field.\n* \
             The storage must not change during a call. Initialization must hold after \
             construction, writes, moves, and copies.\n* Only if you can uphold this safety \
             contract, use `{suggestion}`."
        ),
    )
}
