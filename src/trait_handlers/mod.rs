#[cfg(any(
    feature = "Clone",
    feature = "Copy",
    feature = "Eq",
    feature = "Ord",
    feature = "PartialEq",
    feature = "PartialOrd"
))]
use std::collections::HashMap;

use syn::{DeriveInput, Meta, Type};

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
#[cfg(any(feature = "Copy", feature = "Eq", feature = "Ord", feature = "PartialOrd"))]
use crate::common::where_predicates_bool::extend_where_predicates;

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
pub(crate) struct TraitHandlerContext<'a> {
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
    /// The `Copy` meta of the input, so that `Clone` can look at the `Copy` settings without scanning the attributes again.
    #[cfg(all(feature = "Clone", feature = "Copy"))]
    copy_meta:        Option<Meta>,
    /// The field types that the `PartialEq` impl compares, so that `Eq` does not have to parse the field attributes again.
    ///
    /// Only `Eq` reads this, but the field stays declared whichever traits are enabled, so that the lifetime of the input is always part of the type.
    #[allow(dead_code)]
    partial_eq_types: Option<Vec<&'a Type>>,
}

impl<'a> TraitHandlerContext<'a> {
    /// Stores the `Copy` meta that the entry point has already collected.
    #[cfg(all(feature = "Clone", feature = "Copy"))]
    pub(crate) fn set_copy_meta(&mut self, meta: Option<&Meta>) {
        self.copy_meta = meta.cloned();
    }

    /// Returns the `Copy` meta of the input, or `None` when `Copy` is not derived by Educe.
    #[cfg(all(feature = "Clone", feature = "Copy"))]
    pub(crate) fn copy_meta(&self) -> Option<&Meta> {
        self.copy_meta.as_ref()
    }

    /// Records the field types that the `PartialEq` impl compares, which are the fields that are neither ignored nor handled by a custom method.
    #[cfg(all(feature = "Eq", feature = "PartialEq"))]
    pub(crate) fn record_partial_eq_types(&mut self, types: &[&'a Type]) {
        self.partial_eq_types = Some(types.to_vec());
    }

    /// Returns the field types recorded by the `PartialEq` handler, or `None` when `PartialEq` is not derived by Educe.
    #[cfg(all(feature = "Eq", feature = "PartialEq"))]
    pub(crate) fn partial_eq_types(&self) -> Option<&[&'a Type]> {
        self.partial_eq_types.as_deref()
    }

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
        for prerequisite in prerequisites {
            if let Some(predicates) = self.final_predicates.get(prerequisite) {
                extend_where_predicates(own, predicates.clone());
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
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()>;
}

#[cfg(feature = "Into")]
pub(crate) trait TraitHandlerMultiple {
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &[Meta],
    ) -> syn::Result<()>;
}
