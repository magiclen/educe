use syn::{Data, DeriveInput, Meta};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
#[cfg(feature = "Copy")]
use crate::common::{
    bound::Bound,
    where_predicates_bool::{
        create_where_predicates_from_all_generic_parameters, extend_where_predicates,
    },
};
use crate::{
    common::{bound::BOUND_EXCEPTIONS_COPY, quote_mixed},
    supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Clone` implementation for a union.
pub(crate) struct CloneUnionHandler;

impl TraitHandler for CloneUnionHandler {
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_flag: true, enable_bound: true
        }
        .build_from_clone_meta(meta)?;

        let mut field_types = Vec::new();

        if let Data::Union(data) = &ast.data {
            for field in data.fields.named.iter() {
                let _ = FieldAttributeBuilder {
                    enable_method: false
                }
                .build_from_attributes(&field.attrs, traits)?;

                field_types.push(&field.ty);
            }
        }

        let ident = &ast.ident;

        let copy_trait = syn::parse2(quote_mixed!(::core::marker::Copy)).unwrap();
        #[cfg(feature = "Copy")]
        let copy_bound = if matches!(type_attribute.bound, Bound::Auto) {
            super::super::copy::configured_bound(ctx)?
        } else {
            None
        };

        // A union can only be cloned by a bitwise copy, so the fields must satisfy `Copy` no matter whether `Copy` is derived together or not.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics,
            &copy_trait,
            &field_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_COPY,
        );

        #[cfg(feature = "Copy")]
        let bound = {
            let mut bound = bound;
            let extra = match copy_bound {
                Some(Bound::All) => create_where_predicates_from_all_generic_parameters(
                    &ast.generics.params,
                    &copy_trait,
                ),
                Some(Bound::Custom(predicates)) => predicates,
                _ => Default::default(),
            };
            extend_where_predicates(&mut bound, extra);
            bound
        };

        ctx.record(Trait::Clone, &bound);

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::clone::Clone for #ident #ty_generics #where_clause {
                #[inline]
                fn clone(&self) -> Self {
                    *self
                }
            }
        });

        Ok(())
    }
}
