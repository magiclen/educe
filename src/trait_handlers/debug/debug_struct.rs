use std::str::FromStr;

use super::super::{TraitHandler, create_path_string_from_lit_str};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, NestedMeta, Lit, Data, export::ToTokens};
use crate::panic;

pub struct DebugStructHandler;

impl TraitHandler for DebugStructHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let is_tuple = {
            let mut is_tuple = true;

            if let Data::Struct(data) = &ast.data {
                if let Some(field) = data.fields.iter().next() {
                    if let Some(_) = field.ident {
                        is_tuple = false;
                    }
                }
            }

            is_tuple
        };

        let mut name: Option<Option<String>> = Some(None);

        let mut named_field = !is_tuple;

        match meta {
            Meta::List(list) => {
                let mut name_is_set = false;
                let mut named_field_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "name" | "rename" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Str(s) => {
                                                        if name_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        name_is_set = true;

                                                        let s = create_path_string_from_lit_str(s);

                                                        if s.is_some() {
                                                            name = Some(s);
                                                        } else {
                                                            name = None;
                                                        }
                                                    }
                                                    Lit::Bool(s) => {
                                                        if name_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        name_is_set = true;

                                                        if !s.value {
                                                            name = None;
                                                        }
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))])])
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))])])
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Str(s) => {
                                                if name_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                name_is_set = true;

                                                let s = create_path_string_from_lit_str(s);

                                                if s.is_some() {
                                                    name = Some(s);
                                                } else {
                                                    name = None;
                                                }
                                            }
                                            Lit::Bool(s) => {
                                                if name_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                name_is_set = true;

                                                if !s.value {
                                                    name = None;
                                                }
                                            }
                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = false))])])
                                        }
                                    }
                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))]), stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = false))])])
                                }
                                "named_field" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Bool(s) => {
                                                        if named_field_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        named_field_is_set = true;

                                                        named_field = s.value;
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(named_field(false)))])])
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(named_field(false)))])])
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Bool(s) => {
                                                if named_field_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                named_field_is_set = true;

                                                named_field = s.value;
                                            }
                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(named_field = false))])])
                                        }
                                    }
                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(named_field(false)))]), stringify!(#[educe(Debug(named_field = false))])])
                                }
                                _ => panic::unknown_parameter("Debug", meta_name.as_str())
                            }
                        }
                        NestedMeta::Literal(lit) => match lit {
                            Lit::Str(s) => {
                                if name_is_set {
                                    panic::reset_parameter("name");
                                }

                                name_is_set = true;

                                let s = create_path_string_from_lit_str(s);

                                if s.is_some() {
                                    name = Some(s);
                                } else {
                                    name = None;
                                }
                            }
                            Lit::Bool(s) => {
                                if name_is_set {
                                    panic::reset_parameter("name");
                                }

                                name_is_set = true;

                                if !s.value {
                                    name = None;
                                }
                            }
                            _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))]), stringify!(#[educe(Debug(false))])])
                        }
                    }
                }
            }
            Meta::NameValue(named_value) => {
                let lit = &named_value.lit;

                match lit {
                    Lit::Str(s) => {
                        let s = create_path_string_from_lit_str(s);

                        if s.is_some() {
                            name = Some(s);
                        } else {
                            name = None;
                        }
                    }
                    __ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug = "new_name")])])
                }
            }
            Meta::Word(_) => ()
        }

        let name = match name {
            Some(name) => {
                match name {
                    Some(name) => name,
                    None => {
                        ast.ident.to_string()
                    }
                }
            }
            None => String::new()
        };

        let mut builder_tokens = TokenStream::new();
        let mut has_fields = false;

        if named_field {
            if name.is_empty() {
                builder_tokens.extend(quote!(
                    struct RawString(&'static str);

                    impl core::fmt::Debug for RawString {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            f.write_str(self.0)
                        }
                    }
                ));
                builder_tokens.extend(quote!(let mut builder = formatter.debug_map();));
            } else {
                builder_tokens.extend(quote!(let mut builder = formatter.debug_struct(#name);));
            }

            if let Data::Struct(data) = &ast.data {
                for (index, field) in data.fields.iter().enumerate() {
                    let mut rename: Option<Option<String>> = None;
                    let mut ignore: Option<bool> = None;
                    let mut format_method: Option<String> = None;
                    let mut format_trait: Option<String> = None;

                    for attr in field.attrs.iter() {
                        let attr_meta = attr.parse_meta().unwrap();

                        let attr_meta_name = attr_meta.name().to_string();

                        match attr_meta_name.as_str() {
                            "educe" => match attr_meta {
                                Meta::List(list) => {
                                    for p in list.nested {
                                        match p {
                                            NestedMeta::Meta(meta) => {
                                                let meta_name = meta.name().to_string();

                                                let t = Trait::from_str(meta_name);

                                                if let Err(_) = traits.binary_search(&t) {
                                                    panic::trait_not_used(t.as_str());
                                                }

                                                if t == Trait::Debug {
                                                    match meta {
                                                        Meta::List(list) => {
                                                            for p in list.nested.iter() {
                                                                match p {
                                                                    NestedMeta::Meta(meta) => {
                                                                        let meta_name = meta.name().to_string();

                                                                        match meta_name.as_str() {
                                                                            "name" | "rename" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if rename.is_some() {
                                                                                                        panic::reset_parameter(meta_name.as_str());
                                                                                                    }

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if s.is_some() {
                                                                                                        rename = Some(s);
                                                                                                    } else {
                                                                                                        panic::disable_named_field_name();
                                                                                                    }
                                                                                                }
                                                                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))])])
                                                                                            }
                                                                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))])])
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if rename.is_some() {
                                                                                                panic::reset_parameter(meta_name.as_str());
                                                                                            }

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if s.is_some() {
                                                                                                rename = Some(s);
                                                                                            } else {
                                                                                                panic::disable_named_field_name();
                                                                                            }
                                                                                        }
                                                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name = "new_name"))])])
                                                                                    }
                                                                                }
                                                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name = "new_name"))])])
                                                                            }
                                                                            "ignore" => match meta {
                                                                                Meta::Word(_) => {
                                                                                    if ignore.is_some() {
                                                                                        panic::reset_parameter(meta_name.as_str());
                                                                                    }

                                                                                    ignore = Some(true);
                                                                                }
                                                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(ignore))])])
                                                                            }
                                                                            "format" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Meta(meta) => {
                                                                                                let meta_name = meta.name().to_string();

                                                                                                match meta_name.as_str() {
                                                                                                    "method" => match meta {
                                                                                                        Meta::List(list) => {
                                                                                                            for p in list.nested.iter() {
                                                                                                                match p {
                                                                                                                    NestedMeta::Literal(lit) => match lit {
                                                                                                                        Lit::Str(s) => {
                                                                                                                            if format_method.is_some() {
                                                                                                                                panic::reset_parameter("format_method");
                                                                                                                            }

                                                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                                                            if let Some(s) = s {
                                                                                                                                format_method = Some(s);
                                                                                                                            } else {
                                                                                                                                panic::empty_parameter("format_method");
                                                                                                                            }
                                                                                                                        }
                                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                                    }
                                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        Meta::NameValue(named_value) => {
                                                                                                            let lit = &named_value.lit;

                                                                                                            match lit {
                                                                                                                Lit::Str(s) => {
                                                                                                                    if format_method.is_some() {
                                                                                                                        panic::reset_parameter("format_method");
                                                                                                                    }

                                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                                    if let Some(s) = s {
                                                                                                                        format_method = Some(s);
                                                                                                                    } else {
                                                                                                                        panic::empty_parameter("format_method");
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => panic::debug_format_incorrect()
                                                                                                            }
                                                                                                        }
                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                    }
                                                                                                    "trait" => match meta {
                                                                                                        Meta::List(list) => {
                                                                                                            for p in list.nested.iter() {
                                                                                                                match p {
                                                                                                                    NestedMeta::Literal(lit) => match lit {
                                                                                                                        Lit::Str(s) => {
                                                                                                                            if format_trait.is_some() {
                                                                                                                                panic::reset_parameter("format_trait");
                                                                                                                            }

                                                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                                                            if let Some(s) = s {
                                                                                                                                format_trait = Some(s);
                                                                                                                            } else {
                                                                                                                                panic::empty_parameter("format_trait");
                                                                                                                            }
                                                                                                                        }
                                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                                    }
                                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        Meta::NameValue(named_value) => {
                                                                                                            let lit = &named_value.lit;

                                                                                                            match lit {
                                                                                                                Lit::Str(s) => {
                                                                                                                    if format_trait.is_some() {
                                                                                                                        panic::reset_parameter("format_trait");
                                                                                                                    }

                                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                                    if let Some(s) = s {
                                                                                                                        format_trait = Some(s);
                                                                                                                    } else {
                                                                                                                        panic::empty_parameter("format_trait");
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => panic::debug_format_incorrect()
                                                                                                            }
                                                                                                        }
                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                    }
                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                }
                                                                                            }
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if format_method.is_some() {
                                                                                                        panic::reset_parameter("format_method");
                                                                                                    }

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if let Some(s) = s {
                                                                                                        format_method = Some(s);
                                                                                                    } else {
                                                                                                        panic::empty_parameter("format_method");
                                                                                                    }
                                                                                                }
                                                                                                _ => panic::debug_format_incorrect()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if format_method.is_some() {
                                                                                                panic::reset_parameter("format_method");
                                                                                            }

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if let Some(s) = s {
                                                                                                format_method = Some(s);
                                                                                            } else {
                                                                                                panic::empty_parameter("format_method");
                                                                                            }
                                                                                        }
                                                                                        _ => panic::debug_format_incorrect()
                                                                                    }
                                                                                }
                                                                                _ => panic::debug_format_incorrect()
                                                                            }
                                                                            _ => panic::unknown_parameter("Debug", meta_name.as_str())
                                                                        }
                                                                    }
                                                                    NestedMeta::Literal(lit) => match lit {
                                                                        Lit::Str(s) => {
                                                                            if rename.is_some() {
                                                                                panic::reset_parameter("name");
                                                                            }

                                                                            let s = create_path_string_from_lit_str(s);

                                                                            if s.is_some() {
                                                                                rename = Some(s);
                                                                            } else {
                                                                                panic::disable_named_field_name();
                                                                            }
                                                                        }
                                                                        _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))])])
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Meta::NameValue(named_value) => {
                                                            let lit = &named_value.lit;

                                                            match lit {
                                                                Lit::Str(s) => {
                                                                    if rename.is_some() {
                                                                        panic::reset_parameter("name");
                                                                    }

                                                                    let s = create_path_string_from_lit_str(s);

                                                                    if s.is_some() {
                                                                        rename = Some(s);
                                                                    } else {
                                                                        panic::disable_named_field_name();
                                                                    }
                                                                }
                                                                Lit::Bool(b) => {
                                                                    if ignore.is_some() {
                                                                        panic::reset_parameter("ignore");
                                                                    }

                                                                    ignore = Some(!b.value);
                                                                }
                                                                _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug = "new_name")]), stringify!(#[educe(Debug = false)])])
                                                            }
                                                        }
                                                        _ => panic::attribute_incorrect_format_without_correct_usage("Debug")
                                                    }
                                                }
                                            }
                                            _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                        }
                                    }
                                }
                                _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                            }
                            _ => ()
                        }
                    }

                    let rename = match rename {
                        Some(rename) => rename,
                        None => None
                    };

                    let format_method = match format_trait.as_ref() {
                        Some(_) => match format_method {
                            Some(format_method) => Some(format_method),
                            None => Some("fmt".to_string())
                        }
                        None => format_method
                    };

                    if let Some(true) = ignore {
                        continue;
                    }

                    let (key, field_name) = match rename {
                        Some(rename) => {
                            if let Some(ident) = field.ident.as_ref() {
                                (rename, ident.to_string())
                            } else {
                                (rename, format!("{}", index))
                            }
                        }
                        None => {
                            if let Some(ident) = field.ident.as_ref() {
                                (ident.to_string(), ident.to_string())
                            } else {
                                (format!("_{}", index), format!("{}", index))
                            }
                        }
                    };

                    match format_trait {
                        Some(format_trait) => {
                            let format_method = format_method.unwrap();

                            builder_tokens.extend(TokenStream::from_str(&format!("
                                let arg = {{
                                    struct MyDebug<'a, T: {format_trait}>(&'a T);

                                    impl<'a, T: {format_trait}> core::fmt::Debug for MyDebug<'a, T> {{
                                        fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                            {format_trait}::{format_method}(self.0, formatter)
                                        }}
                                    }}

                                    MyDebug(&self.{field_name})
                                }};
                            ", format_trait = format_trait, format_method = format_method, field_name = field_name)).unwrap());

                            let statement = if name.is_empty() {
                                format!("builder.entry(&RawString({key:?}), &arg);", key = key)
                            } else {
                                format!("builder.field({key:?}, &arg);", key = key)
                            };

                            builder_tokens.extend(TokenStream::from_str(&statement).unwrap());
                        }
                        None => {
                            match format_method {
                                Some(format_method) => {
                                    let ty = field.ty.clone().into_token_stream().to_string();

                                    builder_tokens.extend(TokenStream::from_str(&format!("
                                        let arg = {{
                                            struct MyDebug<'a>(&'a {ty});

                                            impl<'a> core::fmt::Debug for MyDebug<'a> {{
                                                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                                    {format_method}(self.0, formatter)
                                                }}
                                            }}

                                            MyDebug(&self.{field_name})
                                        }};
                                    ", ty = ty, format_method = format_method, field_name = field_name)).unwrap());

                                    let statement = if name.is_empty() {
                                        format!("builder.entry(&RawString({key:?}), &arg);", key = key)
                                    } else {
                                        format!("builder.field({key:?}, &arg);", key = key)
                                    };

                                    builder_tokens.extend(TokenStream::from_str(&statement).unwrap());
                                }
                                None => {
                                    let statement = if name.is_empty() {
                                        format!("builder.entry(&RawString({key:?}), &self.{field_name});", key = key, field_name = field_name)
                                    } else {
                                        format!("builder.field({key:?}, &self.{field_name});", key = key, field_name = field_name)
                                    };

                                    builder_tokens.extend(TokenStream::from_str(&statement).unwrap());
                                }
                            }
                        }
                    }

                    has_fields = true;
                }
            }
        } else {
            builder_tokens.extend(quote!(let mut builder = formatter.debug_tuple(#name);));

            if let Data::Struct(data) = &ast.data {
                for (index, field) in data.fields.iter().enumerate() {
                    let mut ignore: Option<bool> = None;
                    let mut format_method: Option<String> = None;
                    let mut format_trait: Option<String> = None;

                    for attr in field.attrs.iter() {
                        let attr_meta = attr.parse_meta().unwrap();

                        let attr_meta_name = attr_meta.name().to_string();

                        match attr_meta_name.as_str() {
                            "educe" => match attr_meta {
                                Meta::List(list) => {
                                    for p in list.nested {
                                        match p {
                                            NestedMeta::Meta(meta) => {
                                                let meta_name = meta.name().to_string();

                                                let t = Trait::from_str(meta_name);

                                                if let Err(_) = traits.binary_search(&t) {
                                                    panic::trait_not_used(t.as_str());
                                                }

                                                if t == Trait::Debug {
                                                    match meta {
                                                        Meta::List(list) => {
                                                            for p in list.nested.iter() {
                                                                match p {
                                                                    NestedMeta::Meta(meta) => {
                                                                        let meta_name = meta.name().to_string();

                                                                        match meta_name.as_str() {
                                                                            "ignore" => match meta {
                                                                                Meta::Word(_) => {
                                                                                    if ignore.is_some() {
                                                                                        panic::reset_parameter(meta_name.as_str());
                                                                                    }

                                                                                    ignore = Some(true);
                                                                                }
                                                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(ignore))])])
                                                                            }
                                                                            "format" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Meta(meta) => {
                                                                                                let meta_name = meta.name().to_string();

                                                                                                match meta_name.as_str() {
                                                                                                    "method" => match meta {
                                                                                                        Meta::List(list) => {
                                                                                                            for p in list.nested.iter() {
                                                                                                                match p {
                                                                                                                    NestedMeta::Literal(lit) => match lit {
                                                                                                                        Lit::Str(s) => {
                                                                                                                            if format_method.is_some() {
                                                                                                                                panic::reset_parameter("format_method");
                                                                                                                            }

                                                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                                                            if let Some(s) = s {
                                                                                                                                format_method = Some(s);
                                                                                                                            } else {
                                                                                                                                panic::empty_parameter("format_method");
                                                                                                                            }
                                                                                                                        }
                                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                                    }
                                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        Meta::NameValue(named_value) => {
                                                                                                            let lit = &named_value.lit;

                                                                                                            match lit {
                                                                                                                Lit::Str(s) => {
                                                                                                                    if format_method.is_some() {
                                                                                                                        panic::reset_parameter("format_method");
                                                                                                                    }

                                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                                    if let Some(s) = s {
                                                                                                                        format_method = Some(s);
                                                                                                                    } else {
                                                                                                                        panic::empty_parameter("format_method");
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => panic::debug_format_incorrect()
                                                                                                            }
                                                                                                        }
                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                    }
                                                                                                    "trait" => match meta {
                                                                                                        Meta::List(list) => {
                                                                                                            for p in list.nested.iter() {
                                                                                                                match p {
                                                                                                                    NestedMeta::Literal(lit) => match lit {
                                                                                                                        Lit::Str(s) => {
                                                                                                                            if format_trait.is_some() {
                                                                                                                                panic::reset_parameter("format_trait");
                                                                                                                            }

                                                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                                                            if let Some(s) = s {
                                                                                                                                format_trait = Some(s);
                                                                                                                            } else {
                                                                                                                                panic::empty_parameter("format_trait");
                                                                                                                            }
                                                                                                                        }
                                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                                    }
                                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        Meta::NameValue(named_value) => {
                                                                                                            let lit = &named_value.lit;

                                                                                                            match lit {
                                                                                                                Lit::Str(s) => {
                                                                                                                    if format_trait.is_some() {
                                                                                                                        panic::reset_parameter("format_trait");
                                                                                                                    }

                                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                                    if let Some(s) = s {
                                                                                                                        format_trait = Some(s);
                                                                                                                    } else {
                                                                                                                        panic::empty_parameter("format_trait");
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => panic::debug_format_incorrect()
                                                                                                            }
                                                                                                        }
                                                                                                        _ => panic::debug_format_incorrect()
                                                                                                    }
                                                                                                    _ => panic::debug_format_incorrect()
                                                                                                }
                                                                                            }
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if format_method.is_some() {
                                                                                                        panic::reset_parameter("format_method");
                                                                                                    }

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if let Some(s) = s {
                                                                                                        format_method = Some(s);
                                                                                                    } else {
                                                                                                        panic::empty_parameter("format_method");
                                                                                                    }
                                                                                                }
                                                                                                _ => panic::debug_format_incorrect()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if format_method.is_some() {
                                                                                                panic::reset_parameter("format_method");
                                                                                            }

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if let Some(s) = s {
                                                                                                format_method = Some(s);
                                                                                            } else {
                                                                                                panic::empty_parameter("format_method");
                                                                                            }
                                                                                        }
                                                                                        _ => panic::debug_format_incorrect()
                                                                                    }
                                                                                }
                                                                                _ => panic::debug_format_incorrect()
                                                                            }
                                                                            _ => panic::unknown_parameter("Debug", meta_name.as_str())
                                                                        }
                                                                    }
                                                                    _ => panic::attribute_incorrect_format_without_correct_usage("Debug")
                                                                }
                                                            }
                                                        }
                                                        Meta::NameValue(named_value) => {
                                                            let lit = &named_value.lit;

                                                            match lit {
                                                                Lit::Bool(b) => {
                                                                    if ignore.is_some() {
                                                                        panic::reset_parameter("ignore");
                                                                    }

                                                                    ignore = Some(!b.value);
                                                                }
                                                                _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug = false)])])
                                                            }
                                                        }
                                                        _ => panic::attribute_incorrect_format_without_correct_usage("Debug")
                                                    }
                                                }
                                            }
                                            _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                        }
                                    }
                                }
                                _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                            }
                            _ => ()
                        }
                    }

                    let format_method = match format_trait.as_ref() {
                        Some(_) => match format_method {
                            Some(format_method) => Some(format_method),
                            None => Some("fmt".to_string())
                        }
                        None => format_method
                    };

                    if let Some(true) = ignore {
                        continue;
                    }

                    let field_name = if let Some(ident) = field.ident.as_ref() {
                        ident.to_string()
                    } else {
                        format!("{}", index)
                    };

                    match format_trait {
                        Some(format_trait) => {
                            let format_method = format_method.unwrap();

                            builder_tokens.extend(TokenStream::from_str(&format!("
                                let arg = {{
                                    struct MyDebug<'a, T: {format_trait}>(&'a T);

                                    impl<'a, T: {format_trait}> core::fmt::Debug for MyDebug<'a, T> {{
                                        fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                            {format_trait}::{format_method}(self.0, formatter)
                                        }}
                                    }}

                                    MyDebug(&self.{field_name})
                                }};
                            ", format_trait = format_trait, format_method = format_method, field_name = field_name)).unwrap());

                            builder_tokens.extend(TokenStream::from_str("builder.field(&arg);").unwrap());
                        }
                        None => {
                            match format_method {
                                Some(format_method) => {
                                    let ty = field.ty.clone().into_token_stream().to_string();

                                    builder_tokens.extend(TokenStream::from_str(&format!("
                                        let arg = {{
                                            struct MyDebug<'a>(&'a {ty});

                                            impl<'a> core::fmt::Debug for MyDebug<'a> {{
                                                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                                    {format_method}(self.0, formatter)
                                                }}
                                            }}

                                            MyDebug(&self.{field_name})
                                        }};
                                    ", ty = ty, format_method = format_method, field_name = field_name)).unwrap());

                                    builder_tokens.extend(TokenStream::from_str("builder.field(&arg);").unwrap());
                                }
                                None => {
                                    let statement = format!("builder.field(&self.{field_name});", field_name = field_name);

                                    builder_tokens.extend(TokenStream::from_str(&statement).unwrap());
                                }
                            }
                        }
                    }

                    has_fields = true;
                }
            }
        }

        if name.is_empty() && !has_fields {
            panic::unit_struct_need_name();
        }

        let ident = &ast.ident;

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let debug_impl = quote! {
            impl #impl_generics core::fmt::Debug for #ident #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #builder_tokens
                    builder.finish()
                }
            }
        };

        tokens.extend(debug_impl);
    }
}