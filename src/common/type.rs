use std::collections::HashSet;

use syn::{Ident, Type, TypeParamBound};

#[inline]
pub(crate) fn find_idents_in_type<'a>(set: &mut HashSet<&'a Ident>, ty: &'a Type) {
    match ty {
        Type::Array(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::Group(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::ImplTrait(ty) => {
            for b in &ty.bounds {
                if let TypeParamBound::Trait(ty) = b {
                    if let Some(ty) = ty.path.get_ident() {
                        set.insert(ty);
                    }
                }
            }
        },
        Type::Macro(ty) => {
            if let Some(ty) = ty.mac.path.get_ident() {
                set.insert(ty);
            }
        },
        Type::Paren(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::Path(ty) => {
            if let Some(ty) = ty.path.get_ident() {
                set.insert(ty);
            }
        },
        Type::Ptr(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::Reference(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::Slice(ty) => find_idents_in_type(set, ty.elem.as_ref()),
        Type::TraitObject(ty) => {
            for b in &ty.bounds {
                if let TypeParamBound::Trait(ty) = b {
                    if let Some(ty) = ty.path.get_ident() {
                        set.insert(ty);
                    }
                }
            }
        },
        Type::Tuple(ty) => {
            for ty in &ty.elems {
                find_idents_in_type(set, ty)
            }
        },
        _ => (),
    }
}

#[inline]
pub(crate) fn dereference(ty: &Type) -> &Type {
    if let Type::Reference(ty) = ty {
        dereference(ty.elem.as_ref())
    } else {
        ty
    }
}

#[inline]
pub(crate) fn dereference_changed(ty: &Type) -> (&Type, bool) {
    if let Type::Reference(ty) = ty {
        (dereference(ty.elem.as_ref()), true)
    } else {
        (ty, false)
    }
}
