use proc_macro2::TokenStream;
use syn::{DeriveInput, ExprPath, Generics, Type, visit_mut::VisitMut};

use super::{attributes::generated_lint_attributes, generics::ReplaceSelf, quote_mixed};

/// The pieces a custom method marker is built from, with `Self` rewritten so that it still refers to the source type.
///
/// The generated trait impls are `#[automatically_derived]`, and `Clone` also carries `#[rustc_trivial_field_reads]`, so the compiler skips their bodies during dead-code analysis. A custom method used only inside such a body would therefore be wrongly reported as unused. A marker calls the method from an ordinary item that is still analyzed, and it takes the same generics and where clause as the impl so it compiles under exactly the same conditions.
pub(crate) struct MethodMarker {
    lint_attributes:     TokenStream,
    pub(crate) generics: Generics,
    pub(crate) field_ty: Type,
    pub(crate) method:   ExprPath,
}

impl MethodMarker {
    pub(crate) fn new(
        ast: &DeriveInput,
        generics: &Generics,
        field_ty: &Type,
        method: &ExprPath,
    ) -> Self {
        let mut replace_self = ReplaceSelf::new(ast);
        let mut generics = generics.clone();
        let mut field_ty = field_ty.clone();
        let mut method = method.clone();

        replace_self.visit_generics_mut(&mut generics);
        replace_self.visit_type_mut(&mut field_ty);
        replace_self.visit_expr_path_mut(&mut method);

        Self {
            lint_attributes: generated_lint_attributes(&ast.attrs),
            generics,
            field_ty,
            method,
        }
    }

    /// Wraps the marker function in an anonymous constant.
    ///
    /// The function is generated glue whose only purpose is to reference the custom method, so its signature can look problematic in isolation (e.g. `&Vec<T>` would normally suggest `clippy::ptr_arg`, or an unused generic would trigger `clippy::extra_unused_type_parameters`). Lints like these already do not fire on code coming from an external proc-macro, but the `clippy::all` allow is kept here as a low-cost safeguard in case that exemption ever narrows.
    pub(crate) fn wrap(&self, function: TokenStream) -> TokenStream {
        let lint_attributes = &self.lint_attributes;

        quote_mixed!(
            #lint_attributes
            const _: () = {
                #[allow(dead_code, clippy::all)]
                #function
            };
        )
    }
}
