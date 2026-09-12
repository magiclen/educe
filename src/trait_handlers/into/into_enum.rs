use quote::format_ident;
use syn::{Data, DeriveInput, ExprPath, Fields, Ident, Meta, Type, visit_mut::VisitMut};

use super::{
    TraitHandlerMultiple,
    models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{Trait, common::quote_mixed, panic, trait_handlers::TraitHandlerContext};

/// Generates the `Into` implementation for an enum.
pub(crate) struct IntoEnumHandler;

impl TraitHandlerMultiple for IntoEnumHandler {
    #[inline]
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        _ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &[Meta],
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_types: true
        }
        .build_from_into_meta(meta)?;

        if let Data::Enum(data) = &ast.data {
            let field_attributes: Vec<Vec<FieldAttribute>> = {
                let mut variant_attributes = Vec::with_capacity(data.variants.len());

                for variant in data.variants.iter() {
                    let mut field_attributes = Vec::with_capacity(variant.fields.len());

                    let _ = TypeAttributeBuilder {
                        enable_types: false
                    }
                    .build_from_attributes(&variant.attrs, traits)?;

                    for field in variant.fields.iter() {
                        let field_attribute = FieldAttributeBuilder {
                            enable_types: true
                        }
                        .build_from_attributes(&field.attrs, traits)?;

                        for ty in field_attribute.types.keys() {
                            if !type_attribute.types.contains_key(ty) {
                                return Err(super::panic::no_into_impl(ty));
                            }
                        }

                        field_attributes.push(field_attribute);
                    }

                    variant_attributes.push(field_attributes);
                }

                variant_attributes
            };

            for (target_key, target) in type_attribute.types {
                let target_ty = &target.ty;
                let target_matcher = super::common::TargetMatcher::new(target_ty);
                // By default a `From` impl is generated because it provides `Into` for free; the `into` flag asks for a direct `Into` impl instead.
                let generate_from = !target.force_into;

                let bound = target.bound;

                let mut into_types: Vec<&Type> = Vec::new();

                let mut arms_token_stream = proc_macro2::TokenStream::new();

                let enum_ident = &ast.ident;

                type Variants<'a> =
                    Vec<(&'a Ident, bool, usize, Ident, &'a Type, Option<&'a ExprPath>)>;

                let mut variants: Variants = Vec::new();

                for (variant, field_attributes) in data.variants.iter().zip(field_attributes.iter())
                {
                    if let Fields::Unit = &variant.fields {
                        return Err(panic::trait_not_support_unit_variant(
                            meta[0].path().get_ident().unwrap(),
                            variant,
                        ));
                    }

                    let (index, field, method) = super::common::select_field(
                        &variant.fields,
                        field_attributes,
                        &target_key,
                        &target_matcher,
                    )?;

                    let (field_name, is_tuple): (Ident, bool) = match field.ident.as_ref() {
                        Some(ident) => (ident.clone(), false),
                        None => (
                            format_ident!("_{}", index, span = proc_macro2::Span::mixed_site()),
                            true,
                        ),
                    };

                    variants.push((&variant.ident, is_tuple, index, field_name, &field.ty, method));
                }

                if variants.is_empty() {
                    return Err(super::panic::no_into_field(&target_key));
                }

                for (variant_ident, is_tuple, index, field_name, ty, method) in variants {
                    let field_binding =
                        format_ident!("_{}", index, span = proc_macro2::Span::mixed_site());
                    let mut pattern_token_stream = proc_macro2::TokenStream::new();
                    let mut body_token_stream = proc_macro2::TokenStream::new();

                    if let Some(method) = method {
                        let mut method = method.clone();
                        crate::common::generics::ReplaceSelf::new(ast)
                            .visit_expr_path_mut(&mut method);
                        body_token_stream.extend(quote_mixed!( #method(#field_binding) ));
                    } else if target_matcher.matches(ty) {
                        body_token_stream.extend(quote_mixed!( #field_binding ));
                    } else {
                        into_types.push(ty);

                        body_token_stream
                            .extend(quote_mixed!( ::core::convert::Into::into(#field_binding) ));
                    }

                    if is_tuple {
                        for _ in 0..index {
                            pattern_token_stream.extend(quote_mixed!(_,));
                        }

                        pattern_token_stream.extend(quote_mixed!( #field_binding, .. ));

                        arms_token_stream.extend(
                            quote_mixed!( #enum_ident::#variant_ident ( #pattern_token_stream ) => #body_token_stream, ),
                        );
                    } else {
                        pattern_token_stream
                            .extend(quote_mixed!( #field_name: #field_binding, .. ));

                        arms_token_stream.extend(
                            quote_mixed!( #enum_ident::#variant_ident { #pattern_token_stream } => #body_token_stream, ),
                        );
                    }
                }

                let ident = &ast.ident;

                let bound = bound.into_where_predicates_by_generic_parameters_check_types_shallow(
                    &ast.generics.params,
                    &syn::parse2(quote_mixed!(::core::convert::Into<#target_ty>)).unwrap(),
                    &into_types,
                );

                // The generics are cloned so that this target does not affect the other `Into` implementations.
                let mut generics = ast.generics.clone();
                if generate_from {
                    crate::common::generics::ReplaceSelf::new(ast)
                        .visit_generics_mut(&mut generics);
                }

                let generics = crate::common::generics::with_predicates(generics, bound);

                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

                token_stream.extend(if generate_from {
                    quote_mixed! {
                        #generated_impl_attributes
                        impl #impl_generics ::core::convert::From<#ident #ty_generics> for #target_ty #where_clause {
                            #[inline]
                            fn from(value: #ident #ty_generics) -> Self {
                                match value {
                                    #arms_token_stream
                                }
                            }
                        }
                    }
                } else {
                    quote_mixed! {
                        #generated_impl_attributes
                        impl #impl_generics ::core::convert::Into<#target_ty> for #ident #ty_generics #where_clause {
                            #[inline]
                            fn into(self) -> #target_ty {
                                match self {
                                    #arms_token_stream
                                }
                            }
                        }
                    }
                });
            }
        }

        Ok(())
    }
}
