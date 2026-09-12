use quote::quote;
use syn::{
    GenericParam, Ident, Meta, Path, Type, WherePredicate, punctuated::Punctuated, token::Comma,
};

use crate::common::{
    r#type::BoundExceptions,
    where_predicates_bool::{
        WherePredicates, WherePredicatesOrBool,
        create_where_predicates_from_all_generic_parameters,
        create_where_predicates_from_bare_field_types, create_where_predicates_from_field_types,
        meta_2_where_predicates,
    },
};

/// The exception table for `Clone`: `Arc`-like std types clone by duplicating a pointer or a marker, and the container types forward `Clone` to their type arguments.
pub(crate) const BOUND_EXCEPTIONS_CLONE: BoundExceptions = BoundExceptions {
    unconditional_types:               &["Arc", "Rc", "Weak", "NonNull", "Cow", "Discriminant"],
    forwarding_types:                  &[
        "Option",
        "Result",
        "Box",
        "Vec",
        "VecDeque",
        "LinkedList",
        "BTreeMap",
        "BTreeSet",
        "BinaryHeap",
        "HashMap",
        "HashSet",
        "RefCell",
        "Wrapping",
        "Reverse",
        "Saturating",
    ],
    shared_reference_is_unconditional: true,
};

/// The exception table for `Copy`: only the by-value std wrappers forward `Copy` to their type arguments, because the heap-owning containers are never `Copy`.
pub(crate) const BOUND_EXCEPTIONS_COPY: BoundExceptions = BoundExceptions {
    unconditional_types:               &["Arc", "Rc", "Weak", "NonNull", "Cow", "Discriminant"],
    forwarding_types:                  &["Option", "Result", "Wrapping", "Reverse", "Saturating"],
    shared_reference_is_unconditional: true,
};

/// The exception table for `Debug`: some std types print an address or a fixed placeholder instead of formatting their type arguments, and the container types forward `Debug` to them.
pub(crate) const BOUND_EXCEPTIONS_DEBUG: BoundExceptions = BoundExceptions {
    unconditional_types:               &["Weak", "NonNull", "AtomicPtr", "Discriminant"],
    forwarding_types:                  &[
        "Option",
        "Result",
        "Box",
        "Vec",
        "VecDeque",
        "LinkedList",
        "BTreeMap",
        "BTreeSet",
        "BinaryHeap",
        "HashMap",
        "HashSet",
        "Arc",
        "Rc",
        "RefCell",
        "Mutex",
        "RwLock",
        "Wrapping",
        "Reverse",
        "Saturating",
    ],
    shared_reference_is_unconditional: false,
};

/// The common forwarding list for the comparison traits: these std types implement `PartialEq`/`Eq`/`PartialOrd`/`Ord` whenever their type arguments do.
///
/// `HashMap` and `HashSet` are deliberately absent, because their comparison impls additionally require `K: Eq + Hash`, which a plain forwarded bound would not carry.
const FORWARDING_TYPES_COMPARISON: &[&str] = &[
    "Option",
    "Result",
    "Box",
    "Vec",
    "VecDeque",
    "LinkedList",
    "BTreeMap",
    "BTreeSet",
    "Arc",
    "Rc",
    "RefCell",
    "Wrapping",
    "Reverse",
    "Saturating",
];

/// The exception table for `PartialEq` and `Eq`: `NonNull` and `Discriminant` compare by address or by an opaque token.
pub(crate) const BOUND_EXCEPTIONS_EQUALITY: BoundExceptions = BoundExceptions {
    unconditional_types:               &["NonNull", "Discriminant"],
    forwarding_types:                  FORWARDING_TYPES_COMPARISON,
    shared_reference_is_unconditional: false,
};

/// The exception table for `Hash`: like the equality table, but `RefCell` has no `Hash` impl to forward.
pub(crate) const BOUND_EXCEPTIONS_HASH: BoundExceptions = BoundExceptions {
    unconditional_types:               &["NonNull", "Discriminant"],
    forwarding_types:                  &[
        "Option",
        "Result",
        "Box",
        "Vec",
        "VecDeque",
        "LinkedList",
        "BTreeMap",
        "BTreeSet",
        "Arc",
        "Rc",
        "Wrapping",
        "Reverse",
        "Saturating",
    ],
    shared_reference_is_unconditional: false,
};

/// The exception table for `PartialOrd` and `Ord`: `NonNull` orders by address.
pub(crate) const BOUND_EXCEPTIONS_ORDER: BoundExceptions = BoundExceptions {
    unconditional_types:               &["NonNull"],
    forwarding_types:                  FORWARDING_TYPES_COMPARISON,
    shared_reference_is_unconditional: false,
};

/// The exception table for `Default`: the std containers default to an empty value regardless of their type arguments, and the by-value wrappers forward `Default` to them.
pub(crate) const BOUND_EXCEPTIONS_DEFAULT: BoundExceptions = BoundExceptions {
    unconditional_types:               &[
        "Option",
        "Vec",
        "VecDeque",
        "LinkedList",
        "HashMap",
        "HashSet",
        "BTreeMap",
        "BTreeSet",
        "Weak",
    ],
    forwarding_types:                  &[
        "Box",
        "Arc",
        "Rc",
        "Cell",
        "RefCell",
        "Mutex",
        "RwLock",
        "Wrapping",
        "Reverse",
        "Saturating",
    ],
    shared_reference_is_unconditional: false,
};

/// How the where clause of a generated impl should be determined, parsed from the `bound` parameter of an `#[educe(...)]` attribute.
pub(crate) enum Bound {
    /// `bound(false)`: add no predicates at all.
    Disabled,
    /// No `bound` parameter: let Educe work the predicates out from the field types, and inherit the predicates of prerequisite traits.
    Auto,
    /// `bound(*)`: add `Param: Trait` for every generic type parameter, matching the built-in derives.
    All,
    /// `bound(...)` / `bound = "..."`: use exactly the given predicates.
    Custom(WherePredicates),
}

impl Bound {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("bound"));

        Ok(match meta_2_where_predicates(meta)? {
            WherePredicatesOrBool::WherePredicates(where_predicates) => {
                Self::Custom(where_predicates)
            },
            WherePredicatesOrBool::Bool(b) => {
                if b {
                    Self::Auto
                } else {
                    Self::Disabled
                }
            },
            WherePredicatesOrBool::All => Self::All,
        })
    }
}

impl Bound {
    /// Turns the parsed bound setting into concrete where predicates for a generated impl.
    ///
    /// `types` are the field types that the generated code actually touches, `self_ident` is the name of the type being derived, and `exceptions` is the trait's table of unconditional implementations.
    #[inline]
    pub(crate) fn into_where_predicates_by_generic_parameters_check_types(
        self,
        params: &Punctuated<GenericParam, Comma>,
        bound_trait: &Path,
        types: &[&Type],
        self_ident: &Ident,
        exceptions: &BoundExceptions,
    ) -> Punctuated<WherePredicate, Comma> {
        match self {
            Self::Disabled => Punctuated::new(),
            Self::Auto => create_where_predicates_from_field_types(
                params,
                bound_trait,
                types,
                self_ident,
                exceptions,
            ),
            Self::All => create_where_predicates_from_all_generic_parameters(params, bound_trait),
            Self::Custom(where_predicates) => where_predicates,
        }
    }

    /// Builds the extra `Copy` predicates that a `#[repr(packed)]` type needs, because its fields have to be copied out before they can be borrowed.
    ///
    /// `types` are the field types that the generated code reads, including the ones handled by a custom method. Explicit predicates are used verbatim, so only the automatic modes contribute anything here.
    #[inline]
    pub(crate) fn packed_copy_predicates(
        &self,
        params: &Punctuated<GenericParam, Comma>,
        types: &[&Type],
        self_ident: &Ident,
    ) -> WherePredicates {
        let copy_trait: Path = syn::parse2(quote!(::core::marker::Copy)).unwrap();

        match self {
            Self::Auto => create_where_predicates_from_field_types(
                params,
                &copy_trait,
                types,
                self_ident,
                &BOUND_EXCEPTIONS_COPY,
            ),
            Self::All => create_where_predicates_from_all_generic_parameters(params, &copy_trait),
            Self::Disabled | Self::Custom(_) => Punctuated::new(),
        }
    }

    /// A shallow variant used by the `Into` handler, where only field types that are a bare generic parameter can meaningfully receive a bound.
    #[inline]
    pub(crate) fn into_where_predicates_by_generic_parameters_check_types_shallow(
        self,
        params: &Punctuated<GenericParam, Comma>,
        bound_trait: &Path,
        types: &[&Type],
    ) -> Punctuated<WherePredicate, Comma> {
        match self {
            Self::Disabled => Punctuated::new(),
            Self::Auto => create_where_predicates_from_bare_field_types(params, bound_trait, types),
            Self::All => create_where_predicates_from_all_generic_parameters(params, bound_trait),
            Self::Custom(where_predicates) => where_predicates,
        }
    }
}

/// Generates the type-level attribute model of a trait whose only parameter is `bound`.
///
/// The optional fallback names a trait whose attribute is reused when the derived trait has none of its own at the same position.
#[allow(unused_macros)]
macro_rules! bound_only_type_attribute {
    ($trait:ident, $build:ident $(, $fallback:ident, $feature:literal)?) => {
        #[doc = concat!("The parsed settings of a type-level (or variant-level) `", stringify!($trait), "` attribute.")]
        pub(crate) struct TypeAttribute {
            pub(crate) bound: crate::common::bound::Bound,
        }

        #[derive(Debug)]
        #[doc = concat!("Parses `", stringify!($trait), "` metas; the `enable_*` switches describe which parameters are allowed at the current position.")]
        pub(crate) struct TypeAttributeBuilder {
            pub(crate) enable_flag:  bool,
            pub(crate) enable_bound: bool,
        }

        impl TypeAttributeBuilder {
            #[doc = concat!("Parses one `", stringify!($trait), "` meta into a `TypeAttribute`, rejecting parameters that are not enabled here.")]
            pub(crate) fn $build(&self, meta: &syn::Meta) -> syn::Result<TypeAttribute> {
                debug_assert!(
                    meta.path().is_ident(stringify!($trait))
                        $(|| meta.path().is_ident(stringify!($fallback)))?
                );

                let mut bound = crate::common::bound::Bound::Auto;

                let correct_usage = {
                    let mut usage = vec![];

                    if self.enable_flag {
                        usage.push(stringify!(#[educe($trait)]));
                    }

                    if self.enable_bound {
                        usage.push(stringify!(#[educe($trait(bound(where_predicates)))]));
                        usage.push(stringify!(#[educe($trait(bound = false))]));
                    }

                    usage
                };

                match meta {
                    syn::Meta::Path(_) => {
                        if !self.enable_flag {
                            return Err(crate::panic::attribute_incorrect_format(
                                meta.path().get_ident().unwrap(),
                                &correct_usage,
                            ));
                        }
                    },
                    syn::Meta::NameValue(_) => {
                        return Err(crate::panic::attribute_incorrect_format(
                            meta.path().get_ident().unwrap(),
                            &correct_usage,
                        ));
                    },
                    syn::Meta::List(list) => {
                        let result = list.parse_args_with(
                            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                        )?;

                        // An empty parameter list means the same as the bare attribute, so it is checked the same way.
                        if result.is_empty() && !self.enable_flag {
                            return Err(crate::panic::attribute_incorrect_format(
                                meta.path().get_ident().unwrap(),
                                &correct_usage,
                            ));
                        }

                        let mut bound_is_set = false;

                        let mut handler = |meta: syn::Meta| -> syn::Result<bool> {
                            if let Some(ident) = meta.path().get_ident()
                                && ident == "bound"
                            {
                                if !self.enable_bound {
                                    return Ok(false);
                                }

                                let v = crate::common::bound::Bound::from_meta(&meta)?;

                                if bound_is_set {
                                    return Err(crate::panic::parameter_reset(ident));
                                }

                                bound_is_set = true;

                                bound = v;

                                return Ok(true);
                            }

                            Ok(false)
                        };

                        for p in result {
                            if !handler(p)? {
                                return Err(crate::panic::attribute_incorrect_format(
                                    meta.path().get_ident().unwrap(),
                                    &correct_usage,
                                ));
                            }
                        }
                    },
                }

                Ok(TypeAttribute {
                    bound,
                })
            }

            #[doc = concat!("Scans the `#[educe(...)]` attributes of an item (typically an enum variant) and parses its `", stringify!($trait), "` meta if present.")]
            pub(crate) fn build_from_attributes(
                &self,
                attributes: &[syn::Attribute],
                traits: &[crate::Trait],
            ) -> syn::Result<TypeAttribute> {
                let mut output = None;

                $(
                    // The fallback is kept apart from `output` so that an attribute appearing after the fallback one is not mistaken for a repeated trait.
                    #[cfg(feature = $feature)]
                    let mut fallback = None;
                )?

                for attribute in attributes.iter() {
                    let path = attribute.path();

                    if path.is_ident("educe")
                        && let syn::Meta::List(list) = &attribute.meta
                    {
                        let result = list.parse_args_with(
                            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                        )?;

                        for meta in result {
                            let path = meta.path();

                            let t = match crate::Trait::from_path(path) {
                                Some(t) => t,
                                None => return Err(crate::panic::unsupported_trait(meta.path())),
                            };

                            if !traits.contains(&t) {
                                return Err(crate::panic::trait_not_used(path.get_ident().unwrap()));
                            }

                            if t == crate::Trait::$trait {
                                if output.is_some() {
                                    return Err(crate::panic::reuse_a_trait(
                                        path.get_ident().unwrap(),
                                    ));
                                }

                                output = Some(self.$build(&meta)?);
                            }

                            $(
                                // The fallback attribute is validated by its own handler, so a malformed one is simply skipped here.
                                #[cfg(feature = $feature)]
                                if t == crate::Trait::$fallback
                                    && fallback.is_none()
                                    && let Ok(type_attribute) = self.$build(&meta)
                                {
                                    fallback = Some(type_attribute);
                                }
                            )?
                        }
                    }
                }

                $(
                    #[cfg(feature = $feature)]
                    let output = output.or(fallback);
                )?

                Ok(output.unwrap_or(TypeAttribute {
                    bound: crate::common::bound::Bound::Auto,
                }))
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use bound_only_type_attribute;
