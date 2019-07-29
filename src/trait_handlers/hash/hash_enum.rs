use std::str::FromStr;
use std::fmt::Write;

use super::super::TraitHandler;
use super::models::TypeAttributeBuilder;
use super::models::FieldAttributeBuilder;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data, Generics, Fields};

pub struct HashEnumHandler;

impl TraitHandler for HashEnumHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let type_attribute = TypeAttributeBuilder {
            enable_bound: true,
        }.from_hash_meta(meta);

        let enum_name = ast.ident.to_string();

        let bound = type_attribute.bound.into_punctuated_where_predicates_by_generic_parameters(&ast.generics.params);

        let mut hasher_tokens = TokenStream::new();

        let mut match_tokens = String::from("match self { ");

        if let Data::Enum(data) = &ast.data {
            let has_non_unit = {
                let mut non_unit = false;

                for variant in data.variants.iter() {
                    match &variant.fields {
                        Fields::Named(_) | Fields::Unnamed(_) => {
                            non_unit = true;

                            break;
                        }
                        _ => ()
                    }
                }

                non_unit
            };

            for (index, variant) in data.variants.iter().enumerate() {
                let variant_ident = variant.ident.to_string();

                match &variant.fields {
                    Fields::Unit => { // TODO Unit
                        if has_non_unit {
                            match_tokens.write_fmt(format_args!("{enum_name}::{variant_ident} => {{ core::hash::Hash::hash(&{index}, state); }}", enum_name = enum_name, variant_ident = variant_ident, index = index)).unwrap();
                        } else {
                            match_tokens.write_fmt(format_args!("{enum_name}::{variant_ident} => {{ core::hash::Hash::hash(&({enum_name}::{variant_ident} as isize), state); }}", enum_name = enum_name, variant_ident = variant_ident)).unwrap();
                        }
                    }
                    Fields::Named(fields) => {  // TODO Struct
                        let mut pattern_tokens = String::new();
                        let mut block_tokens = String::new();

                        block_tokens.write_fmt(format_args!("core::hash::Hash::hash(&{index}, state);", index = index)).unwrap();

                        for field in fields.named.iter() {
                            let field_attribute = FieldAttributeBuilder {
                                enable_ignore: true,
                                enable_hash: true,
                            }.from_attributes(&field.attrs, traits);

                            let field_name = field.ident.as_ref().unwrap().to_string();

                            if field_attribute.ignore {
                                pattern_tokens.write_fmt(format_args!("{field_name}: _,", field_name = field_name)).unwrap();
                                continue;
                            }

                            let hash_trait = field_attribute.hash_trait;

                            let hash_method = match hash_trait.as_ref() {
                                Some(_) => match field_attribute.hash_method {
                                    Some(hash_method) => Some(hash_method),
                                    None => Some("hash".to_string())
                                }
                                None => field_attribute.hash_method
                            };

                            pattern_tokens.write_fmt(format_args!("{field_name},", field_name = field_name)).unwrap();

                            match hash_trait {
                                Some(hash_trait) => {
                                    let hash_method = hash_method.unwrap();

                                    block_tokens.write_fmt(format_args!("{hash_trait}::{hash_method}({field_name}, state);", hash_trait = hash_trait, hash_method = hash_method, field_name = field_name)).unwrap();
                                }
                                None => {
                                    match hash_method {
                                        Some(hash_method) => {
                                            block_tokens.write_fmt(format_args!("{hash_method}({field_name}, state);", hash_method = hash_method, field_name = field_name)).unwrap();
                                        }
                                        None => {
                                            block_tokens.write_fmt(format_args!("core::hash::Hash::hash({field_name}, state);", field_name = field_name)).unwrap();
                                        }
                                    }
                                }
                            }
                        }

                        match_tokens.write_fmt(format_args!("{enum_name}::{variant_ident}{{ {pattern_tokens} }} => {{ {block_tokens} }}", enum_name = enum_name, variant_ident = variant_ident, pattern_tokens = pattern_tokens, block_tokens = block_tokens)).unwrap();
                    }
                    Fields::Unnamed(fields) => { // TODO Tuple
                        let mut pattern_tokens = String::new();
                        let mut block_tokens = String::new();

                        block_tokens.write_fmt(format_args!("core::hash::Hash::hash(&{index}, state);", index = index)).unwrap();

                        for (index, field) in fields.unnamed.iter().enumerate() {
                            let field_attribute = FieldAttributeBuilder {
                                enable_ignore: true,
                                enable_hash: true,
                            }.from_attributes(&field.attrs, traits);

                            if field_attribute.ignore {
                                pattern_tokens.push_str("_,");
                                continue;
                            }

                            let hash_trait = field_attribute.hash_trait;

                            let hash_method = match hash_trait.as_ref() {
                                Some(_) => match field_attribute.hash_method {
                                    Some(hash_method) => Some(hash_method),
                                    None => Some("hash".to_string())
                                }
                                None => field_attribute.hash_method
                            };

                            let field_name = format!("{}", index);

                            pattern_tokens.write_fmt(format_args!("_{field_name},", field_name = field_name)).unwrap();

                            match hash_trait {
                                Some(hash_trait) => {
                                    let hash_method = hash_method.unwrap();

                                    block_tokens.write_fmt(format_args!("{hash_trait}::{hash_method}(_{field_name}, state);", hash_trait = hash_trait, hash_method = hash_method, field_name = field_name)).unwrap();
                                }
                                None => {
                                    match hash_method {
                                        Some(hash_method) => {
                                            block_tokens.write_fmt(format_args!("{hash_method}(_{field_name}, state);", hash_method = hash_method, field_name = field_name)).unwrap();
                                        }
                                        None => {
                                            block_tokens.write_fmt(format_args!("core::hash::Hash::hash(_{field_name}, state);", field_name = field_name)).unwrap();
                                        }
                                    }
                                }
                            }
                        }

                        match_tokens.write_fmt(format_args!("{enum_name}::{variant_ident}( {pattern_tokens} ) => {{ {block_tokens} }}", enum_name = enum_name, variant_ident = variant_ident, pattern_tokens = pattern_tokens, block_tokens = block_tokens)).unwrap();
                    }
                }
            }
        }

        match_tokens.push_str(" }");

        hasher_tokens.extend(TokenStream::from_str(&match_tokens).unwrap());

        let ident = &ast.ident;

        let mut generics_cloned: Generics = ast.generics.clone();

        let where_clause = generics_cloned.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics_cloned.split_for_impl();

        let hash_impl = quote! {
            impl #impl_generics core::hash::Hash for #ident #ty_generics #where_clause {
                #[allow(dead_code)]
                fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                    #hasher_tokens
                }
            }
        };

        tokens.extend(hash_impl);
    }
}