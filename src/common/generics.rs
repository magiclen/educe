use std::{
    collections::{BTreeMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Expr, Generics, Ident, Lit, LitStr, MetaNameValue, Path, PathArguments,
    ext::IdentExt, visit_mut::VisitMut,
};

use super::{quote_mixed, where_predicates_bool::WherePredicates};

/// Names in the input that generated identifiers must avoid.
#[derive(Clone)]
pub(crate) struct UsedIdents {
    input:     Rc<HashSet<String>>,
    selected:  HashSet<String>,
    macro_key: Option<u64>,
}

impl UsedIdents {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        fn collect(tokens: TokenStream, used: &mut HashSet<String>, may_expand: &mut bool) {
            for token in tokens {
                match token {
                    TokenTree::Ident(ident) => {
                        used.insert(ident.unraw().to_string());
                    },
                    TokenTree::Group(group) => {
                        // A doc comment is never expanded as code, so its words are reserved without treating a `!` in it as a macro call.
                        if let Some(text) = doc_text(&group) {
                            reserve_words(&text, used);
                        } else {
                            collect(group.stream(), used, may_expand);
                        }
                    },
                    TokenTree::Literal(literal) => {
                        // Method paths can also be written inside string literals, so the words of a string are reserved as well.
                        // The text is scanned instead of being parsed, because a string can hold prose such as `the node's title`, which is not valid Rust.
                        if let Ok(literal) = syn::parse2::<LitStr>(literal.into_token_stream()) {
                            let value = literal.value();
                            *may_expand |= value.contains('!');
                            reserve_words(&value, used);
                        }
                    },
                    TokenTree::Punct(punct) => *may_expand |= punct.as_char() == '!',
                }
            }
        }

        /// Returns the text of a `#[doc = "..."]` attribute from its bracketed body.
        fn doc_text(group: &Group) -> Option<String> {
            if group.delimiter() != Delimiter::Bracket {
                return None;
            }

            let attribute = syn::parse2::<MetaNameValue>(group.stream()).ok()?;

            if attribute.path.is_ident("doc")
                && let Expr::Lit(value) = &attribute.value
                && let Lit::Str(text) = &value.lit
            {
                Some(text.value())
            } else {
                None
            }
        }

        fn reserve_words(text: &str, used: &mut HashSet<String>) {
            for word in text
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .filter(|word| !word.is_empty())
            {
                used.insert(word.to_owned());
            }
        }

        let mut used = HashSet::new();
        let tokens = ast.to_token_stream();
        let mut may_expand = false;
        collect(tokens.clone(), &mut used, &mut may_expand);

        // A macro can produce names we cannot see, so use an input-specific prefix for its scope.
        let macro_key = may_expand.then(|| {
            let mut hasher = DefaultHasher::new();
            tokens.to_string().hash(&mut hasher);
            hasher.finish()
        });
        Self {
            input: Rc::new(used),
            selected: HashSet::new(),
            macro_key,
        }
    }

    pub(crate) fn select(&mut self, name: &str) -> Ident {
        let mut name = name.to_owned();
        while self.input.contains(&name) || !self.selected.insert(name.clone()) {
            name.insert(0, '_');
        }
        Ident::new(&name, Span::mixed_site())
    }
}

/// The names that one generated item introduces, chosen to avoid every identifier in the input.
pub(crate) struct ValueBindings {
    idents:  UsedIdents,
    aliases: BTreeMap<String, Ident>,
}

impl ValueBindings {
    pub(crate) fn new(idents: UsedIdents) -> Self {
        Self {
            idents,
            aliases: BTreeMap::new(),
        }
    }

    /// Picks a name for a generated type, function, or generic parameter.
    pub(crate) fn fresh(&mut self, name: &str) -> Ident {
        self.idents.select(name)
    }

    /// Picks the name of a local value, such as a parameter or a pattern binding, and records it for [`aliases`](Self::aliases).
    pub(crate) fn binding(&mut self, name: &str) -> Ident {
        if let Some(ident) = self.aliases.get(name) {
            return ident.clone();
        }

        let name_in_scope = match self.idents.macro_key {
            Some(key) => format!("educe__{key:x}_{name}"),
            None => format!("educe__{name}"),
        };
        let ident = self.fresh(&name_in_scope);
        self.aliases.insert(name.to_owned(), ident.clone());
        ident
    }

    /// A function alias stops Rust from reading a binding as a constant pattern.
    /// One anonymous constant uses every alias, because `allow(unused_imports)` would be rejected under a `forbid` in the user's crate.
    pub(crate) fn aliases(&self) -> TokenStream {
        if self.aliases.is_empty() {
            return TokenStream::new();
        }

        let aliases = self.aliases.values();
        let used = aliases.clone();
        quote_mixed! {
            use ::core::mem::{ #(drop as #aliases),* };
            const _: &[fn(())] = &[#(#used::<()>),*];
        }
    }

    /// Puts the aliases and the item into an anonymous constant, so that the aliases are only visible to the item.
    pub(crate) fn wrap(&self, item: TokenStream) -> TokenStream {
        if self.aliases.is_empty() {
            return item;
        }

        let aliases = self.aliases();
        quote_mixed! {
            const _: () = {
                #aliases
                #item
            };
        }
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
