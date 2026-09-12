use quote::ToTokens;
use syn::{Data, DeriveInput, Fields, Meta, Type};

use super::models::{FieldAttributeBuilder, FieldName, TypeAttributeBuilder, TypeName};
use crate::{
    common::{
        bound::BOUND_EXCEPTIONS_DEBUG, ident_index::IdentOrIndex, path::path_to_string, quote_mixed,
    },
    supported_traits::Trait,
    trait_handlers::{TraitHandler, TraitHandlerContext},
};

/// Generates the `Debug` implementation for an enum.
pub(crate) struct DebugEnumHandler;

impl TraitHandler for DebugEnumHandler {
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        _ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_flag:        true,
            enable_unsafe:      false,
            enable_name:        true,
            enable_named_field: false,
            enable_bound:       true,
            name:               TypeName::Disable,
            named_field:        false,
        }
        .build_from_debug_meta(meta)?;

        let name = type_attribute.name.to_ident_by_ident(&ast.ident);
        let helper_types = super::common::HelperTypes::new(ast);
        let raw_string_ident = &helper_types.raw_string;

        let mut debug_types: Vec<&Type> = Vec::new();

        let mut builder_token_stream = proc_macro2::TokenStream::new();

        let mut arms_token_stream = proc_macro2::TokenStream::new();

        let mut mark_fields = Vec::new();

        // Every nameless variant formatted as a map needs the raw-string helper type, which is declared once for the whole `fmt` body.
        let mut uses_raw_string = false;

        if let Data::Enum(data) = &ast.data {
            for variant in data.variants.iter() {
                let type_attribute = TypeAttributeBuilder {
                    enable_flag:        false,
                    enable_unsafe:      false,
                    enable_name:        true,
                    enable_named_field: true,
                    enable_bound:       false,
                    name:               TypeName::Default,
                    named_field:        matches!(&variant.fields, Fields::Named(_)),
                }
                .build_from_attributes(&variant.attrs, traits)?;

                let variant_ident = &variant.ident;

                let variant_name = type_attribute.name.to_ident_by_ident(variant_ident);

                let named_field = type_attribute.named_field;

                let name_string = if let Some(name) = name {
                    if let Some(variant_name) = variant_name {
                        Some(path_to_string(
                            &syn::parse2(quote_mixed!(#name::#variant_name)).unwrap(),
                        ))
                    } else {
                        Some(name.into_token_stream().to_string())
                    }
                } else {
                    variant_name.map(|variant_name| variant_name.into_token_stream().to_string())
                };

                if let Fields::Unit = &variant.fields {
                    if name_string.is_none() {
                        return Err(super::panic::unit_variant_need_name(variant));
                    }

                    arms_token_stream
                        .extend(quote_mixed!( Self::#variant_ident => f.write_str(#name_string), ));

                    continue;
                }

                let mut has_fields = false;

                let mut pattern_token_stream = proc_macro2::TokenStream::new();
                let mut block_token_stream = proc_macro2::TokenStream::new();

                if named_field {
                    uses_raw_string |= name_string.is_none();

                    block_token_stream.extend(create_named_field_builder(name_string.as_deref()));
                } else {
                    // A variant without a name is formatted like a nameless tuple struct, which the standard builder spells with an empty name.
                    let tuple_name = name_string.as_deref().unwrap_or("");

                    block_token_stream
                        .extend(quote_mixed!(let mut builder = f.debug_tuple(#tuple_name);));
                }

                for (index, field) in variant.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_name:   named_field,
                        enable_ignore: true,
                        enable_method: true,
                        name:          FieldName::Default,
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    let field_name =
                        IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                    if field_attribute.ignore {
                        pattern_token_stream.extend(field_name.to_field(&quote_mixed!(_)));

                        continue;
                    }

                    let field_name_var = field_name.to_binding("_");

                    pattern_token_stream
                        .extend(field_name.to_field(&quote_mixed!(#field_name_var)));

                    // The displayed field name is a plain string, so a tuple field can be shown with its real name `0`, which is not a valid ident.
                    let key = if named_field {
                        let key = match field_attribute.name {
                            FieldName::Custom(name) => name.to_string(),
                            FieldName::Default => field_name.to_token_stream().to_string(),
                        };

                        Some(syn::LitStr::new(&key, proc_macro2::Span::call_site()))
                    } else {
                        None
                    };

                    let ty = &field.ty;

                    let value = if let Some(method) = field_attribute.method {
                        let arg = super::common::create_format_arg(
                            &helper_types.field,
                            ty,
                            &method,
                            quote_mixed!(#field_name_var),
                        );

                        block_token_stream.extend(arg);
                        mark_fields.push((ty, method));

                        quote_mixed!(&arg)
                    } else {
                        debug_types.push(ty);

                        quote_mixed!(#field_name_var)
                    };

                    block_token_stream.extend(match (&key, name_string.is_some()) {
                        (Some(key), true) => quote_mixed!(builder.field(#key, #value);),
                        (Some(key), false) => {
                            quote_mixed!(builder.entry(&#raw_string_ident(#key), #value);)
                        },
                        (None, _) => quote_mixed!(builder.field(#value);),
                    });

                    has_fields = true;
                }

                if !has_fields && name_string.is_none() {
                    return Err(super::panic::unit_struct_need_name(variant_ident));
                }

                let pattern = if let Fields::Named(_) = &variant.fields {
                    quote_mixed!(Self::#variant_ident { #pattern_token_stream })
                } else {
                    quote_mixed!(Self::#variant_ident ( #pattern_token_stream ))
                };

                arms_token_stream.extend(quote_mixed! {
                    #pattern => {
                        #block_token_stream

                        builder.finish()
                    },
                });
            }
        }

        let ident = &ast.ident;

        if arms_token_stream.is_empty() {
            if let Some(ident) = name {
                builder_token_stream.extend(quote_mixed! {
                    f.write_str(stringify!(#ident))
                });
            } else {
                return Err(super::panic::unit_enum_need_name(ident));
            }
        } else {
            let raw_string_type = if uses_raw_string {
                Some(super::common::create_raw_string_type(raw_string_ident))
            } else {
                None
            };

            builder_token_stream.extend(quote_mixed! {
                #raw_string_type

                match self {
                    #arms_token_stream
                }
            });
        }

        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::fmt::Debug)).unwrap(),
            &debug_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_DEBUG,
        );

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #builder_token_stream
                }
            }
        });

        for (field_ty, method) in &mark_fields {
            token_stream
                .extend(super::common::create_mark_method_used(ast, &generics, field_ty, method));
        }

        Ok(())
    }
}

#[inline]
fn create_named_field_builder(name_string: Option<&str>) -> proc_macro2::TokenStream {
    if let Some(name_string) = name_string {
        quote_mixed!(let mut builder = f.debug_struct(#name_string);)
    } else {
        super::common::create_debug_map_builder()
    }
}
