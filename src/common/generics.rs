use quote::{format_ident, quote};
use syn::{DeriveInput, GenericParam, Generics, Ident, Path, PathArguments, visit_mut::VisitMut};

use super::where_predicates_bool::WherePredicates;

pub(crate) fn unused_ident(generics: &Generics, name: &str) -> Ident {
    let mut ident = format_ident!("{name}");
    while generics.params.iter().any(|param| match param {
        GenericParam::Type(param) => param.ident == ident,
        GenericParam::Const(param) => param.ident == ident,
        GenericParam::Lifetime(_) => false,
    }) {
        ident = format_ident!("_{ident}");
    }
    ident
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
