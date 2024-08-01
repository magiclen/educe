use std::iter;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, Ident, Variant};

pub(crate) struct VariantInfo<'v> {
    pub(crate) variant: Option<&'v Variant>,
    pub(crate) fields:  &'v syn::Fields,
    pub(crate) attrs:   &'v [syn::Attribute],
}

pub(crate) struct VariantSelector<'v> {
    pub(crate) name: Option<&'v Ident>,
}

impl<'v> VariantInfo<'v> {
    pub(crate) fn iter_from_data(data: &'v syn::Data) -> Box<dyn Iterator<Item = Self> + 'v> {
        match data {
            Data::Struct(s) => Box::new(iter::once(VariantInfo {
                variant: None,
                fields:  &s.fields,
                attrs:   &[],
            })),

            Data::Enum(e) => Box::new(e.variants.iter().map(|v| VariantInfo {
                variant: Some(v),
                fields:  &v.fields,
                attrs:   &v.attrs,
            })),

            Data::Union(_) => panic!("VariantInfo cannot be used for unions"),
        }
    }

    pub(crate) fn selector(&self) -> VariantSelector<'v> {
        let name = self.variant.as_ref().map(|v| &v.ident);
        VariantSelector {
            name,
        }
    }
}

impl ToTokens for VariantSelector<'_> {
    fn to_tokens(&self, out: &mut TokenStream) {
        if let Some(name) = &self.name {
            quote! { :: #name }.to_tokens(out)
        }
    }
}
