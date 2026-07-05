use syn::{
    Meta, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// The parsed content of an attribute list that may start with the `unsafe` keyword, e.g. `Debug(unsafe, name = false)`.
///
/// The unsafe marker is how a user opts in to the byte-based union implementations.
pub(crate) struct UnsafePunctuatedMeta {
    pub(crate) list:       Punctuated<Meta, Token![,]>,
    pub(crate) has_unsafe: bool,
}

impl Parse for UnsafePunctuatedMeta {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let has_unsafe = input.parse::<Token![unsafe]>().is_ok();

        if input.is_empty() {
            return Ok(Self {
                list: Punctuated::new(),
                has_unsafe,
            });
        }

        if has_unsafe {
            input.parse::<Token![,]>()?;
        }

        let list = input.parse_terminated(Meta::parse, Token![,])?;

        Ok(Self {
            list,
            has_unsafe,
        })
    }
}
