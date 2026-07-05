use std::collections::HashSet;

use syn::{
    GenericArgument, GenericParam, Ident, Meta, Path, PathArguments, ReturnType, Token, Type,
    TypeParamBound,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) struct TypeWithPunctuatedMeta {
    pub(crate) ty:   Type,
    pub(crate) list: Punctuated<Meta, Token![,]>,
}

impl Parse for TypeWithPunctuatedMeta {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<Type>()?;

        if input.is_empty() {
            return Ok(Self {
                ty,
                list: Punctuated::new(),
            });
        }

        input.parse::<Token![,]>()?;

        let list = input.parse_terminated(Meta::parse, Token![,])?;

        Ok(Self {
            ty,
            list,
        })
    }
}

/// Describes, for one trait, the places where the automatic bound engine can treat a field type specially instead of emitting a plain `FieldType: Trait` predicate.
///
/// An unconditional field type produces no where predicate at all, because the predicate would always hold and would only be noise in the generated code.
/// A forwarding field type produces the predicates of its type arguments instead, because the standard library guarantees that the arguments satisfying the trait is enough for the whole type.
/// The unconditional table is also used when a self-referencing field type is degraded to per-parameter bounds, to avoid collecting parameters from unconditional positions.
pub(crate) struct BoundExceptions {
    /// Type names, matched against the last path segment, whose implementation of the trait never depends on their type arguments, e.g. `Arc` for `Clone`.
    pub(crate) unconditional_types:               &'static [&'static str],
    /// Type names, matched against the last path segment, that implement the trait whenever their type arguments do, e.g. `Option` for `Copy` or `Vec` for `Clone`.
    ///
    /// Bounding the arguments instead of the whole type keeps the where clause simple and lets the compiler use supertrait elaboration on the parameters, e.g. deriving `T: Clone` from `T: Copy`.
    pub(crate) forwarding_types:                  &'static [&'static str],
    /// Whether a shared reference (`&T`) implements the trait unconditionally, which is the case for `Copy` and `Clone`.
    pub(crate) shared_reference_is_unconditional: bool,
}

impl BoundExceptions {
    /// Checks whether the last segment of the path names a type that implements the trait unconditionally.
    ///
    /// `PhantomData` implements every trait supported by Educe unconditionally, so it is always treated as an exception.
    #[inline]
    fn path_is_unconditional(&self, path: &Path) -> bool {
        if let Some(segment) = path.segments.last() {
            let ident = &segment.ident;

            if ident == "PhantomData" {
                return true;
            }

            self.unconditional_types.iter().any(|name| ident == name)
        } else {
            false
        }
    }

    /// Checks whether the whole type is known to implement the trait unconditionally, so its predicate can be omitted.
    ///
    /// Raw pointers and bare function pointers are unconditional for every trait supported by Educe, because their implementations only look at the address value.
    #[inline]
    pub(crate) fn type_is_unconditional(&self, ty: &Type) -> bool {
        match ty {
            Type::Path(ty) => ty.qself.is_none() && self.path_is_unconditional(&ty.path),
            Type::Ptr(_) | Type::BareFn(_) => true,
            Type::Reference(ty) => {
                ty.mutability.is_none() && self.shared_reference_is_unconditional
            },
            _ => false,
        }
    }

    /// Returns the type arguments of a type that forwards the trait to them, or `None` if the type is not in the forwarding table.
    pub(crate) fn forwarding_type_arguments<'a>(&self, ty: &'a Type) -> Option<Vec<&'a Type>> {
        if let Type::Path(ty) = ty
            && ty.qself.is_none()
            && let Some(segment) = ty.path.segments.last()
            && self.forwarding_types.iter().any(|name| segment.ident == name)
            && let PathArguments::AngleBracketed(args) = &segment.arguments
        {
            let mut types = Vec::new();

            for arg in &args.args {
                if let GenericArgument::Type(ty) = arg {
                    types.push(ty);
                }
            }

            return Some(types);
        }

        None
    }
}

/// Walks a type and collects every ident that could refer to a generic type parameter.
///
/// When `exceptions` is provided, positions that the exception table marks as unconditional are not descended into, so parameters that only appear there are not collected.
fn walk_type<'a>(set: &mut HashSet<&'a Ident>, ty: &'a Type, exceptions: Option<&BoundExceptions>) {
    match ty {
        Type::Array(ty) => walk_type(set, ty.elem.as_ref(), exceptions),
        Type::Group(ty) => walk_type(set, ty.elem.as_ref(), exceptions),
        Type::Paren(ty) => walk_type(set, ty.elem.as_ref(), exceptions),
        Type::Slice(ty) => walk_type(set, ty.elem.as_ref(), exceptions),
        Type::Tuple(ty) => {
            for ty in &ty.elems {
                walk_type(set, ty, exceptions);
            }
        },
        Type::Reference(ty) => {
            // A shared reference implements `Copy`/`Clone` no matter what it points to, so those traits do not need the pointee's parameters.
            if let Some(exceptions) = exceptions
                && ty.mutability.is_none()
                && exceptions.shared_reference_is_unconditional
            {
                return;
            }

            walk_type(set, ty.elem.as_ref(), exceptions);
        },
        Type::Ptr(ty) => {
            // Raw pointer impls only compare or print the address, so no trait supported by Educe needs the pointee's parameters.
            if exceptions.is_some() {
                return;
            }

            walk_type(set, ty.elem.as_ref(), exceptions);
        },
        Type::BareFn(ty) => {
            // Function pointer impls also only look at the address.
            if exceptions.is_some() {
                return;
            }

            for arg in &ty.inputs {
                walk_type(set, &arg.ty, exceptions);
            }

            if let ReturnType::Type(_, ty) = &ty.output {
                walk_type(set, ty, exceptions);
            }
        },
        Type::ImplTrait(ty) => {
            for b in &ty.bounds {
                if let TypeParamBound::Trait(b) = b {
                    walk_path(set, &b.path, exceptions);
                }
            }
        },
        Type::TraitObject(ty) => {
            for b in &ty.bounds {
                if let TypeParamBound::Trait(b) = b {
                    walk_path(set, &b.path, exceptions);
                }
            }
        },
        Type::Path(ty) => {
            if let Some(qself) = &ty.qself {
                walk_type(set, qself.ty.as_ref(), exceptions);
            }

            walk_path(set, &ty.path, exceptions);
        },
        _ => (),
    }
}

/// Walks a path, collecting the first segment ident (the only position where a generic type parameter can appear) and recursing into every segment's arguments.
fn walk_path<'a>(
    set: &mut HashSet<&'a Ident>,
    path: &'a Path,
    exceptions: Option<&BoundExceptions>,
) {
    if let Some(exceptions) = exceptions
        && exceptions.path_is_unconditional(path)
    {
        return;
    }

    // A path that starts with `::` can never begin with a generic type parameter.
    if path.leading_colon.is_none()
        && let Some(segment) = path.segments.first()
    {
        // This may also collect idents like `Vec` or `std`, but the callers intersect the set with the real generic parameters, so false candidates are harmless.
        set.insert(&segment.ident);
    }

    for segment in &path.segments {
        match &segment.arguments {
            PathArguments::AngleBracketed(args) => {
                for arg in &args.args {
                    match arg {
                        GenericArgument::Type(ty) => walk_type(set, ty, exceptions),
                        GenericArgument::AssocType(ty) => walk_type(set, &ty.ty, exceptions),
                        _ => (),
                    }
                }
            },
            PathArguments::Parenthesized(args) => {
                for ty in &args.inputs {
                    walk_type(set, ty, exceptions);
                }

                if let ReturnType::Type(_, ty) = &args.output {
                    walk_type(set, ty, exceptions);
                }
            },
            PathArguments::None => (),
        }
    }
}

/// Collects every ident in the type that could refer to a generic type parameter, without applying any exceptions.
#[inline]
pub(crate) fn find_all_idents_in_type<'a>(set: &mut HashSet<&'a Ident>, ty: &'a Type) {
    walk_type(set, ty, None);
}

/// Collects the idents in the type that could refer to a generic type parameter, skipping positions that the exception table marks as unconditional.
#[inline]
pub(crate) fn find_idents_in_type<'a>(
    set: &mut HashSet<&'a Ident>,
    ty: &'a Type,
    exceptions: &BoundExceptions,
) {
    walk_type(set, ty, Some(exceptions));
}

/// Collects only a bare, single-segment type ident such as `T`, without descending into any structure.
///
/// This is used by the `Into` handler: a bound like `T: Into<Target>` only makes sense when the field type is the parameter itself, so nested parameters like the `T` in `Option<T>` must not be collected.
#[inline]
pub(crate) fn find_bare_ident_in_type<'a>(set: &mut HashSet<&'a Ident>, ty: &'a Type) {
    if let Type::Path(ty) = ty
        && ty.qself.is_none()
        && let Some(ident) = ty.path.get_ident()
    {
        set.insert(ident);
    }
}

/// Returns true if the type syntactically uses any of the generic type parameters.
#[inline]
pub(crate) fn type_uses_type_params(ty: &Type, params: &Punctuated<GenericParam, Comma>) -> bool {
    let mut set = HashSet::new();

    find_all_idents_in_type(&mut set, ty);

    params.iter().any(|param| {
        if let GenericParam::Type(param) = param { set.contains(&param.ident) } else { false }
    })
}

/// Returns true if any path segment in the type is exactly the given ident.
///
/// This is used to detect field types that refer to the type currently being derived, e.g. `Box<List<T>>` inside `List<T>`, so that the bound engine can avoid generating a self-referencing predicate that would overflow the trait solver.
pub(crate) fn type_mentions_ident(ty: &Type, ident: &Ident) -> bool {
    fn path_mentions_ident(path: &Path, ident: &Ident) -> bool {
        for segment in &path.segments {
            if segment.ident == *ident {
                return true;
            }

            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    for arg in &args.args {
                        match arg {
                            GenericArgument::Type(ty) => {
                                if type_mentions_ident(ty, ident) {
                                    return true;
                                }
                            },
                            GenericArgument::AssocType(ty)
                                if type_mentions_ident(&ty.ty, ident) =>
                            {
                                return true;
                            },
                            _ => (),
                        }
                    }
                },
                PathArguments::Parenthesized(args) => {
                    for ty in &args.inputs {
                        if type_mentions_ident(ty, ident) {
                            return true;
                        }
                    }

                    if let ReturnType::Type(_, ty) = &args.output
                        && type_mentions_ident(ty, ident)
                    {
                        return true;
                    }
                },
                PathArguments::None => (),
            }
        }

        false
    }

    match ty {
        Type::Array(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Group(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Paren(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Slice(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Ptr(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Reference(ty) => type_mentions_ident(ty.elem.as_ref(), ident),
        Type::Tuple(ty) => ty.elems.iter().any(|ty| type_mentions_ident(ty, ident)),
        Type::BareFn(ty) => {
            ty.inputs.iter().any(|arg| type_mentions_ident(&arg.ty, ident))
                || matches!(&ty.output, ReturnType::Type(_, ty) if type_mentions_ident(ty, ident))
        },
        Type::ImplTrait(ty) => ty
            .bounds
            .iter()
            .any(|b| matches!(b, TypeParamBound::Trait(b) if path_mentions_ident(&b.path, ident))),
        Type::TraitObject(ty) => ty
            .bounds
            .iter()
            .any(|b| matches!(b, TypeParamBound::Trait(b) if path_mentions_ident(&b.path, ident))),
        Type::Path(ty) => {
            (match &ty.qself {
                Some(qself) => type_mentions_ident(qself.ty.as_ref(), ident),
                None => false,
            }) || path_mentions_ident(&ty.path, ident)
        },
        _ => false,
    }
}

#[inline]
pub(crate) fn dereference(ty: &Type) -> &Type {
    if let Type::Reference(ty) = ty { dereference(ty.elem.as_ref()) } else { ty }
}

#[inline]
pub(crate) fn dereference_changed(ty: &Type) -> (&Type, bool) {
    if let Type::Reference(ty) = ty { (dereference(ty.elem.as_ref()), true) } else { (ty, false) }
}
