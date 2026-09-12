use std::collections::HashSet;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Generics, Ident, LitStr, Path, PathArguments, ext::IdentExt, visit_mut::VisitMut,
};

use super::where_predicates_bool::WherePredicates;

/// Names in the input that generated identifiers must avoid.
pub(crate) struct UsedIdents(HashSet<String>);

impl UsedIdents {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        fn collect(tokens: TokenStream, used: &mut HashSet<String>) {
            for token in tokens {
                match token {
                    TokenTree::Ident(ident) => {
                        used.insert(ident.unraw().to_string());
                    },
                    TokenTree::Group(group) => collect(group.stream(), used),
                    TokenTree::Literal(literal) => {
                        // Method paths can also be written inside string literals.
                        if let Ok(literal) = syn::parse2::<LitStr>(literal.into_token_stream())
                            && let Ok(tokens) = literal.value().parse::<TokenStream>()
                        {
                            collect(tokens, used);
                        }
                    },
                    TokenTree::Punct(_) => (),
                }
            }
        }

        let mut used = HashSet::new();
        collect(ast.to_token_stream(), &mut used);
        Self(used)
    }

    pub(crate) fn select(&mut self, name: &str) -> Ident {
        let mut name = name.to_owned();
        while !self.0.insert(name.clone()) {
            name.insert(0, '_');
        }
        Ident::new(&name, Span::mixed_site())
    }
}

/// Keeps `Self` tied to the source type when code moves into a helper or a `From` impl.
pub(crate) struct ReplaceSelf(Path);

impl ReplaceSelf {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let ident = &ast.ident;
        let (_, ty_generics, _) = ast.generics.split_for_impl();
        let mut path: Path = syn::parse2(quote!(#ident #ty_generics)).unwrap();
        if let PathArguments::AngleBracketed(args) =
            &mut path.segments.last_mut().unwrap().arguments
        {
            args.colon2_token = Some(Default::default());
        }
        Self(path)
    }
}
impl VisitMut for ReplaceSelf {
    fn visit_path_mut(&mut self, path: &mut Path) {
        syn::visit_mut::visit_path_mut(self, path);
        if path.leading_colon.is_none()
            && path.segments.first().is_some_and(|segment| segment.ident == "Self")
        {
            let mut replacement = self.0.clone();
            replacement.segments.extend(path.segments.iter().skip(1).cloned());
            *path = replacement;
        }
    }
}

/// Appends the predicates a generated impl needs to the generics the input declares.
pub(crate) fn with_predicates(mut generics: Generics, predicates: WherePredicates) -> Generics {
    let where_clause = generics.make_where_clause();

    for predicate in predicates {
        where_clause.predicates.push(predicate);
    }

    generics
}
