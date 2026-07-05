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
