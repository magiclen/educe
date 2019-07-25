use std::str::FromStr;
use std::collections::HashSet;

use super::super::{TraitHandler, create_path_string_from_lit_str, create_where_predicates_from_lit_str};

use crate::Trait;
use crate::proc_macro2::{TokenStream, Span};
use crate::syn::{DeriveInput, Meta, NestedMeta, Lit, Data, WherePredicate, Generics, GenericParam, Lifetime, LifetimeDef, TypeParamBound, PathArguments, GenericArgument, punctuated::Punctuated, token::Comma};

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
                let mut bound_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "name" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Str(s) => {
                                                        if name_is_set {
                                                            panic!("Try to reset the `name` parameter.");
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
                                                            panic!("Try to reset the `name` parameter.");
                                                        }

                                                        name_is_set = true;

                                                        if !s.value {
                                                            name = None;
                                                        }
                                                    }
                                                    _ => panic!("The literal for the `name` parameter should be a string or a bool value.")
                                                }
                                                _ => panic!("The format of the `name` parameter is incorrect.")
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Str(s) => {
                                                if name_is_set {
                                                    panic!("Try to reset the `name` parameter.");
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
                                                    panic!("Try to reset the `name` parameter.");
                                                }

                                                name_is_set = true;

                                                if !s.value {
                                                    name = None;
                                                }
                                            }
                                            _ => panic!("The literal for the `name` parameter should be a string or a bool value.")
                                        }
                                    }
                                    _ => panic!("You are using an incorrect format of the `name` parameter. The format should be `#[educe(Debug(name = \"new_name\"))]`, `#[educe(Debug(name = false))]`, `#[educe(Debug(name(\"new_name\"))]`, or `#[educe(Debug(name(false))]`.")
                                }
                                "named_field" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Bool(s) => {
                                                        if named_field_is_set {
                                                            panic!("Try to reset the `named_field` parameter.");
                                                        }

                                                        named_field_is_set = true;

                                                        named_field = s.value;
                                                    }
                                                    _ => panic!("The literal for the `named_field` parameter should be a bool value.")
                                                }
                                                _ => panic!("The format of the `named_field` parameter is incorrect.")
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Bool(s) => {
                                                if named_field_is_set {
                                                    panic!("Try to reset the `named_field` parameter.");
                                                }

                                                named_field_is_set = true;

                                                named_field = s.value;
                                            }
                                            _ => panic!("The literal for the `named_field` parameter should be a bool value.")
                                        }
                                    }
                                    _ => panic!("You are using an incorrect format of the `named_field` parameter. The format should be `#[educe(Debug(named_field = false))]` or `#[educe(Debug(named_field(false)))]`.")
                                }
                                "bound" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Str(s) => {
                                                        if bound_is_set {
                                                            panic!("Try to reset the `bound` parameter.");
                                                        }

                                                        bound_is_set = true;

                                                        let s = create_where_predicates_from_lit_str(s);

                                                        if s.is_some() {
                                                            bound = Some(s);
                                                        } else {
                                                            bound = None;
                                                        }
                                                    }
                                                    Lit::Bool(s) => {
                                                        if bound_is_set {
                                                            panic!("Try to reset the `bound` parameter.");
                                                        }

                                                        bound_is_set = true;

                                                        if !s.value {
                                                            bound = None;
                                                        }
                                                    }
                                                    _ => panic!("The literal for the `bound` parameter should be a string or a bool value.")
                                                }
                                                _ => panic!("The format of the `bound` parameter is incorrect.")
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Str(s) => {
                                                if bound_is_set {
                                                    panic!("Try to reset the `bound` parameter.");
                                                }

                                                bound_is_set = true;

                                                let s = create_where_predicates_from_lit_str(s);

                                                if s.is_some() {
                                                    bound = Some(s);
                                                } else {
                                                    bound = None;
                                                }
                                            }
                                            Lit::Bool(s) => {
                                                if bound_is_set {
                                                    panic!("Try to reset the `bound` parameter.");
                                                }

                                                bound_is_set = true;

                                                if !s.value {
                                                    bound = None;
                                                }
                                            }
                                            _ => panic!("The literal for the `bound` parameter should be a string or a bool value.")
                                        }
                                    }
                                    _ => panic!("You are using an incorrect format of the `bound` parameter. The format should be `#[educe(Debug(bound = \"where_predicate\"))]`, `#[educe(Debug(bound = false))]`, `#[educe(Debug(bound(\"where_predicate\"))]`, or `#[educe(Debug(bound(false))]`.")
                                }
                                _ => panic!("Unknown parameter `{}` for the `Debug` meta.", meta_name)
                            }
                        }
                        NestedMeta::Literal(lit) => match lit {
                            Lit::Str(s) => {
                                if name_is_set {
                                    panic!("Try to reset the `name` parameter.");
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
                                    panic!("Try to reset the `name` parameter.");
                                }

                                name_is_set = true;

                                if !s.value {
                                    name = None;
                                }
                            }
                            _ => panic!("The literal for the `Debug` meta should be a string or a bool value.")
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
                    _ => panic!("You are using an incorrect format of the `Debug` meta. If you want to change the name of the type by a named value, the format should be `#[educe(Debug = \"new_name\")]`.")
                }
            }
            Meta::Word(_) => ()
        }


        let mut generics: Generics = ast.generics.clone();

        if let Some(bound) = bound {
            match bound {
                Some(bound) => {
                    let mut lifetimes = HashSet::new();

                    for where_predicate in bound.iter() {
                        match where_predicate {
                            WherePredicate::Lifetime(pl) => {
                                lifetimes.insert(pl.lifetime.ident.to_string());
                            }
                            WherePredicate::Type(pt) => {
                                if let Some(bl) = pt.lifetimes.as_ref() {
                                    for l in bl.lifetimes.iter() {
                                        lifetimes.insert(l.lifetime.ident.to_string());
                                    }
                                }
                                for tp in pt.bounds.iter() {
                                    match tp {
                                        TypeParamBound::Lifetime(lifetime) => {
                                            lifetimes.insert(lifetime.ident.to_string());
                                        }
                                        TypeParamBound::Trait(tb) => {
                                            if let Some(bl) = tb.lifetimes.as_ref() {
                                                for l in bl.lifetimes.iter() {
                                                    lifetimes.insert(l.lifetime.ident.to_string());
                                                }
                                            }

                                            let path = &tb.path;

                                            for segment in path.segments.iter() {
                                                if let PathArguments::AngleBracketed(arg) = &segment.arguments {
                                                    for arg in arg.args.iter() {
                                                        if let GenericArgument::Lifetime(lifetime) = arg {
                                                            lifetimes.insert(lifetime.ident.to_string());
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => ()
                        }
                    }

                    for lifetime in lifetimes {
                        let l = Lifetime::new(&format!("'{}", lifetime), Span::call_site());

                        let l = GenericParam::Lifetime(LifetimeDef::new(l));

                        generics.params.push(l);
                    }

                    let mut where_clause = generics.make_where_clause();

                    where_clause.predicates.extend(bound);
                }
                None => {
                    let mut where_clause = generics.make_where_clause();

                    for param in ast.generics.params.iter() {
                        if let GenericParam::Type(typ) = param {
                            let ident = &typ.ident;

                            let where_predicate: WherePredicate = syn::parse(quote! { #ident: core::fmt::Debug }.into()).unwrap();

                            where_clause.predicates.push(where_predicate);
                        }
                    }
                }
            }
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
                    let mut rename: Option<String> = None;
                    let mut ignore: Option<bool> = None;
                    let mut format_with: Option<String> = None;

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
                                                    panic!("The `{}` trait is not used.", t.as_str());
                                                }

                                                if t == Trait::Debug {
                                                    match meta {
                                                        Meta::List(list) => {
                                                            let mut name_is_set = false;

                                                            for p in list.nested.iter() {
                                                                match p {
                                                                    NestedMeta::Meta(meta) => {
                                                                        let meta_name = meta.name().to_string();

                                                                        match meta_name.as_str() {
                                                                            "name" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if name_is_set {
                                                                                                        panic!("Try to reset the `name` parameter.");
                                                                                                    }

                                                                                                    name_is_set = true;

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if s.is_some() {
                                                                                                        rename = s;
                                                                                                    } else {
                                                                                                        panic!("You can't disable a field name.");
                                                                                                    }
                                                                                                }
                                                                                                _ => panic!("The literal for the `name` parameter should be a string.")
                                                                                            }
                                                                                            _ => panic!("The format of the `name` parameter is incorrect.")
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if name_is_set {
                                                                                                panic!("Try to reset the `name` parameter.");
                                                                                            }

                                                                                            name_is_set = true;

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if s.is_some() {
                                                                                                rename = s;
                                                                                            } else {
                                                                                                panic!("You can't disable a field name.");
                                                                                            }
                                                                                        }
                                                                                        _ => panic!("The literal for the `name` parameter should be a string.")
                                                                                    }
                                                                                }
                                                                                _ => panic!("You are using an incorrect format of the `name` parameter. The format should be `#[educe(Debug(name = \"new_name\"))]`, or `#[educe(Debug(name(\"new_name\"))]`.")
                                                                            }
                                                                            "ignore" => match meta {
                                                                                Meta::Word(_) => {
                                                                                    if ignore.is_some() {
                                                                                        panic!("Try to reset the `ignore` parameter.");
                                                                                    }

                                                                                    ignore = Some(true);
                                                                                }
                                                                                _ => panic!("You are using an incorrect format of the `ignore` parameter. The format should be `#[educe(Debug(ignore))]` or `#[educe(Debug = false)]`.")
                                                                            }
                                                                            "format_with" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if format_with.is_some() {
                                                                                                        panic!("Try to reset the `format_with` parameter.");
                                                                                                    }

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if s.is_some() {
                                                                                                        format_with = s;
                                                                                                    } else {
                                                                                                        panic!("The path can't be empty.");
                                                                                                    }
                                                                                                }
                                                                                                _ => panic!("The literal for the `format_with` parameter should be a string.")
                                                                                            }
                                                                                            _ => panic!("The format of the `name` parameter is incorrect.")
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if format_with.is_some() {
                                                                                                panic!("Try to reset the `format_with` parameter.");
                                                                                            }

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if s.is_some() {
                                                                                                format_with = s;
                                                                                            } else {
                                                                                                panic!("The path can't be empty.");
                                                                                            }
                                                                                        }
                                                                                        _ => panic!("The literal for the `format_with` parameter should be a string.")
                                                                                    }
                                                                                }
                                                                                _ => panic!("You are using an incorrect format of the `format_with` parameter. The format should be `#[educe(Debug(format_with = \"path\"))]`, or `#[educe(Debug(format_with(\"path\"))]`.")
                                                                            }
                                                                            _ => panic!("Unknown parameter `{}` for the `Debug` meta.", meta_name)
                                                                        }
                                                                    }
                                                                    NestedMeta::Literal(lit) => match lit {
                                                                        Lit::Str(s) => {
                                                                            if name_is_set {
                                                                                panic!("Try to reset the `name` parameter.");
                                                                            }

                                                                            name_is_set = true;

                                                                            let s = create_path_string_from_lit_str(s);

                                                                            if s.is_some() {
                                                                                rename = s;
                                                                            } else {
                                                                                panic!("You can't disable a field name.");
                                                                            }
                                                                        }
                                                                        _ => panic!("The literal for the `Debug` meta should be a string.")
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
                                                                        rename = s;
                                                                    } else {
                                                                        panic!("You can't disable a field name.");
                                                                    }
                                                                }
                                                                Lit::Bool(b) => {
                                                                    if ignore.is_some() {
                                                                        panic!("Try to reset the `ignore` parameter.");
                                                                    }

                                                                    ignore = Some(!b.value);
                                                                }
                                                                _ => panic!("You are using an incorrect format of the `Debug` meta. If you want to change the name of the type by a named value, the format should be `#[educe(Debug = \"new_name\")]`. If you want to ignore a field, the format should be `#[educe(Debug = false)]`.")
                                                            }
                                                        }
                                                        Meta::Word(_) => ()
                                                    }
                                                }
                                            }
                                            NestedMeta::Literal(_) => panic!("You are using an incorrect format of the `educe` attribute. Literals are not allowed here.")
                                        }
                                    }
                                }
                                _ => panic!("You are using an incorrect format of the `educe` attribute. It needs to be formed into `#[educe(Trait1, Trait2, ..., TraitN)]`")
                            }
                            _ => ()
                        }
                    }

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

                    match format_with {
                        Some(format_with) => {
                            builder_tokens.extend(TokenStream::from_str(&format!("
                                    let arg = {{
                                        struct MyDebug<T>(T);

                                        impl<T> core::fmt::Debug for MyDebug<T> {{
                                            fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                                {format_with}(&self, formatter)
                                            }}
                                        }}

                                        MyDebug(&self.{field_name})
                                    }};
                            ", format_with = format_with, field_name = field_name)).unwrap());

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

                    has_fields = true;
                }
            }
        } else {
            builder_tokens.extend(quote!(let mut builder = formatter.debug_tuple(#name);));

            if let Data::Struct(data) = &ast.data {
                for (index, field) in data.fields.iter().enumerate() {
                    let mut ignore: Option<bool> = None;
                    let mut format_with: Option<String> = None;

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
                                                    panic!("The `{}` trait is not used.", t.as_str());
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
                                                                                        panic!("Try to reset the `ignore` parameter.");
                                                                                    }

                                                                                    ignore = Some(true);
                                                                                }
                                                                                _ => panic!("You are using an incorrect format of the `ignore` parameter. The format should be `#[educe(Debug(ignore))]`.")
                                                                            }
                                                                            "format_with" => match meta {
                                                                                Meta::List(list) => {
                                                                                    for p in list.nested.iter() {
                                                                                        match p {
                                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                                Lit::Str(s) => {
                                                                                                    if format_with.is_some() {
                                                                                                        panic!("Try to reset the `format_with` parameter.");
                                                                                                    }

                                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                                    if s.is_some() {
                                                                                                        format_with = s;
                                                                                                    } else {
                                                                                                        panic!("The path can't be empty.");
                                                                                                    }
                                                                                                }
                                                                                                _ => panic!("The literal for the `format_with` parameter should be a string.")
                                                                                            }
                                                                                            _ => panic!("The format of the `name` parameter is incorrect.")
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Meta::NameValue(named_value) => {
                                                                                    let lit = &named_value.lit;

                                                                                    match lit {
                                                                                        Lit::Str(s) => {
                                                                                            if format_with.is_some() {
                                                                                                panic!("Try to reset the `format_with` parameter.");
                                                                                            }

                                                                                            let s = create_path_string_from_lit_str(s);

                                                                                            if s.is_some() {
                                                                                                format_with = s;
                                                                                            } else {
                                                                                                panic!("The path can't be empty.");
                                                                                            }
                                                                                        }
                                                                                        _ => panic!("The literal for the `format_with` parameter should be a string.")
                                                                                    }
                                                                                }
                                                                                _ => panic!("You are using an incorrect format of the `format_with` parameter. The format should be `#[educe(Debug(format_with = \"path\"))]`, or `#[educe(Debug(format_with(\"path\"))]`.")
                                                                            }
                                                                            _ => panic!("Unknown parameter `{}` for the `Debug` meta.", meta_name)
                                                                        }
                                                                    }
                                                                    NestedMeta::Literal(_) => panic!("You are using an incorrect format of the `educe` attribute. Literals are not allowed here.")
                                                                }
                                                            }
                                                        }
                                                        Meta::NameValue(named_value) => {
                                                            let lit = &named_value.lit;

                                                            match lit {
                                                                Lit::Bool(b) => {
                                                                    if ignore.is_some() {
                                                                        panic!("Try to reset the `ignore` parameter.");
                                                                    }

                                                                    ignore = Some(!b.value);
                                                                }
                                                                _ => panic!("You are using an incorrect format of the `Debug` meta. If you want to ignore a field, the format should be `#[educe(Debug = false)]`.")
                                                            }
                                                        }
                                                        Meta::Word(_) => ()
                                                    }
                                                }
                                            }
                                            NestedMeta::Literal(_) => panic!("You are using an incorrect format of the `educe` attribute. Literals are not allowed here.")
                                        }
                                    }
                                }
                                _ => panic!("You are using an incorrect format of the `educe` attribute. It needs to be formed into `#[educe(Trait1, Trait2, ..., TraitN)]`")
                            }
                            _ => ()
                        }
                    }

                    if let Some(true) = ignore {
                        continue;
                    }

                    let field_name = if let Some(ident) = field.ident.as_ref() {
                        ident.to_string()
                    } else {
                        format!("{}", index)
                    };

                    match format_with {
                        Some(format_with) => {
                            builder_tokens.extend(TokenStream::from_str(&format!("
                                    let arg = {{
                                        struct MyDebug<T>(T);

                                        impl<T> core::fmt::Debug for MyDebug<T> {{
                                            fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {{
                                                {format_with}(&self, formatter)
                                            }}
                                        }}

                                        MyDebug(&self.{field_name})
                                    }};
                            ", format_with = format_with, field_name = field_name)).unwrap());

                            builder_tokens.extend(TokenStream::from_str("builder.field(&arg);").unwrap());
                        }
                        None => {
                            let statement = format!("builder.field(&self.{field_name});", field_name = field_name);

                            builder_tokens.extend(TokenStream::from_str(&statement).unwrap());
                        }
                    }

                    has_fields = true;
                }
            }
        }

        if name.is_empty() && !has_fields {
            panic!("A unit struct needs to have a name.");
        }

        let ident = &ast.ident;

        let (_, ty_generics, _) = ast.generics.split_for_impl();
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let debug_impl = quote! {
            impl #impl_generics core::fmt::Debug for #ident #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #builder_tokens
                    builder.finish()
                }
            }
        };

        tokens.extend(debug_impl);

        println!("{}", tokens.to_string());
    }
}