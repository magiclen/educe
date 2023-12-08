#[cfg(not(any(
    feature = "Debug",
    feature = "Clone",
    feature = "Copy",
    feature = "PartialEq",
    feature = "Eq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash",
    feature = "Default",
    feature = "Deref",
    feature = "DerefMut",
)))]
compile_error!("at least one of the trait features must be enabled");

use enum_ordinalize::Ordinalize;
use syn::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(variants(pub(crate) const VARIANTS))]
pub(crate) enum Trait {
    #[cfg(feature = "Debug")]
    Debug,
    #[cfg(feature = "Clone")]
    Clone,
    #[cfg(feature = "Copy")]
    Copy,
    #[cfg(feature = "PartialEq")]
    PartialEq,
    #[cfg(feature = "Eq")]
    Eq,
    #[cfg(feature = "PartialOrd")]
    PartialOrd,
    #[cfg(feature = "Ord")]
    Ord,
    #[cfg(feature = "Hash")]
    Hash,
    #[cfg(feature = "Default")]
    Default,
    #[cfg(feature = "Deref")]
    Deref,
    #[cfg(feature = "DerefMut")]
    DerefMut,

    _Nothing,
}

impl Trait {
    #[inline]
    pub(crate) fn from_path(path: &Path) -> Option<Trait> {
        let ident_string = match path.get_ident() {
            Some(ident) => ident.to_string(),
            None => return None,
        };

        match ident_string.as_str() {
            #[cfg(feature = "Debug")]
            "Debug" => Some(Trait::Debug),
            #[cfg(feature = "Clone")]
            "Clone" => Some(Trait::Clone),
            #[cfg(feature = "Copy")]
            "Copy" => Some(Trait::Copy),
            #[cfg(feature = "PartialEq")]
            "PartialEq" => Some(Trait::PartialEq),
            #[cfg(feature = "Eq")]
            "Eq" => Some(Trait::Eq),
            #[cfg(feature = "PartialOrd")]
            "PartialOrd" => Some(Trait::PartialOrd),
            #[cfg(feature = "Ord")]
            "Ord" => Some(Trait::Ord),
            #[cfg(feature = "Hash")]
            "Hash" => Some(Trait::Hash),
            #[cfg(feature = "Default")]
            "Default" => Some(Trait::Default),
            #[cfg(feature = "Deref")]
            "Deref" => Some(Trait::Deref),
            #[cfg(feature = "DerefMut")]
            "DerefMut" => Some(Trait::DerefMut),
            _ => None,
        }
    }
}
