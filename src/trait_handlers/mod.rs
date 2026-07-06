#[cfg(any(
    feature = "Clone",
    feature = "Copy",
    feature = "Eq",
    feature = "Ord",
    feature = "PartialEq",
    feature = "PartialOrd"
))]
use std::collections::HashMap;
#[cfg(any(feature = "Copy", feature = "Eq", feature = "Ord", feature = "PartialOrd"))]
use std::collections::HashSet;

#[cfg(any(feature = "Copy", feature = "Eq", feature = "Ord", feature = "PartialOrd"))]
use quote::ToTokens;
use syn::{DeriveInput, Meta};

use crate::Trait;
#[cfg(any(
    feature = "Clone",
    feature = "Copy",
    feature = "Eq",
    feature = "Ord",
    feature = "PartialEq",
    feature = "PartialOrd"
))]
use crate::common::where_predicates_bool::WherePredicates;

#[cfg(feature = "Clone")]
pub(crate) mod clone;
#[cfg(feature = "Copy")]
pub(crate) mod copy;
#[cfg(feature = "Debug")]
pub(crate) mod debug;
#[cfg(feature = "Default")]
pub(crate) mod default;
#[cfg(feature = "Deref")]
pub(crate) mod deref;
#[cfg(feature = "DerefMut")]
pub(crate) mod deref_mut;
#[cfg(feature = "Eq")]
pub(crate) mod eq;
#[cfg(feature = "Hash")]
pub(crate) mod hash;
#[cfg(feature = "Into")]
pub(crate) mod into;
#[cfg(feature = "Ord")]
pub(crate) mod ord;
#[cfg(feature = "PartialEq")]
pub(crate) mod partial_eq;
#[cfg(feature = "PartialOrd")]
pub(crate) mod partial_ord;

/// Shared state that flows through all trait handlers during a single `#[derive(Educe)]` expansion.
///
/// Its main job is to let a trait inherit the where predicates of its prerequisite traits, e.g. `Ord` inherits the predicates of `Eq` and `PartialOrd`.
#[derive(Default)]
pub(crate) struct TraitHandlerContext {
    /// The final where predicates that each handled trait has actually emitted, keyed by trait.
    #[cfg(any(
        feature = "Clone",
        feature = "Copy",
        feature = "Eq",
        feature = "Ord",
        feature = "PartialEq",
        feature = "PartialOrd"
    ))]
    final_predicates: HashMap<Trait, WherePredicates>,
}

impl TraitHandlerContext {
    /// Records the where predicates that a trait impl has emitted, so that traits handled later can inherit them.
    #[cfg(any(
        feature = "Clone",
        feature = "Copy",
        feature = "Eq",
        feature = "Ord",
        feature = "PartialEq",
        feature = "PartialOrd"
    ))]
    pub(crate) fn record(&mut self, t: Trait, predicates: &WherePredicates) {
        self.final_predicates.insert(t, predicates.clone());
    }

    /// Appends the recorded predicates of every prerequisite trait to `own`, skipping predicates that are already present.
    ///
    /// Prerequisites that were not handled by Educe (e.g. implemented manually by the user) simply have no record and contribute nothing.
    #[cfg(any(feature = "Copy", feature = "Eq", feature = "Ord", feature = "PartialOrd"))]
    pub(crate) fn inherit_from(&self, prerequisites: &[Trait], own: &mut WherePredicates) {
        // Compare predicates by their token strings because `WherePredicate` implements neither `Eq` nor `Hash`.
        let mut seen: HashSet<String> =
            own.iter().map(|predicate| predicate.to_token_stream().to_string()).collect();

        for prerequisite in prerequisites {
            if let Some(predicates) = self.final_predicates.get(prerequisite) {
                for predicate in predicates {
                    if seen.insert(predicate.to_token_stream().to_string()) {
                        own.push(predicate.clone());
                    }
                }
            }
        }
    }
}

// Every single-meta handler implements this trait; when `Into` is the only enabled feature none of them are compiled, so the trait would look unused.
#[cfg(any(
    feature = "Clone",
    feature = "Copy",
    feature = "Debug",
    feature = "Default",
    feature = "Deref",
    feature = "DerefMut",
    feature = "Eq",
    feature = "Hash",
    feature = "Ord",
    feature = "PartialEq",
    feature = "PartialOrd"
))]
pub(crate) trait TraitHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()>;
}

#[cfg(feature = "Into")]
pub(crate) trait TraitHandlerMultiple {
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &[Meta],
    ) -> syn::Result<()>;
}
