// This is free and unencumbered software released into the public domain.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_quote;
use syn::{Type, TypePath, TypeSlice};
use syn_match::path_match;
use syn_path::ty;

#[derive(Clone, Debug)]
pub struct Typed {
    pub(crate) inner: Type,
    pub(crate) xform: Option<TokenStream>,
}

impl Typed {
    pub fn owned(&self) -> Self {
        use syn::Type::*;
        let (ty, xform) = match &self.inner {
            ImplTrait(impl_trait) => {
                for bound in &impl_trait.bounds {
                    if let syn::TypeParamBound::Trait(trait_bound) = bound {
                        // Map common traits to concrete, owned types:
                        let (inner, xform) = path_match!(&trait_bound.path,
                            AsRef<str> => (ty!(::alloc::string::String), Some(quote! { as_ref().into() })),
                            Into<String> => (ty!(::alloc::string::String), Some(quote! { into() })),
                            ToString | Display => (ty!(::alloc::string::String), Some(quote! { to_string() })),
                            _ => unreachable!(), // TODO
                        );
                        return Typed { inner, xform };
                    }
                }
                (ty!(::alloc::string::String), None)
            },

            // Handle `&[T]` -> `Vec<T>`
            Reference(type_ref) => match type_ref.elem.as_ref() {
                // `&[T]` -> `Vec<T>`
                Slice(TypeSlice { elem, .. }) => {
                    let inner: &Type = elem; // TODO: recursively convert inner type
                    (
                        parse_quote! { ::alloc::vec::Vec<#inner> },
                        Some(quote! { into() }),
                    )
                },

                // `&str` -> `String`, etc
                Path(TypePath { path, .. }) => path_match!(&path,
                    str => (ty!(::alloc::string::String), Some(quote! { into() })), // `&str` -> `String`
                    _ => unreachable!(), // TODO
                ),

                // Other references: try to convert inner type, keep as-is if unchanged
                other => (other.clone(), None),
            },

            _ => (self.inner.clone(), self.xform.clone()),
        };
        Typed { inner: ty, xform }
    }
}

impl From<syn::Type> for Typed {
    fn from(input: syn::Type) -> Self {
        Self {
            inner: input,
            xform: None,
        }
    }
}

impl From<&syn::Type> for Typed {
    fn from(input: &syn::Type) -> Self {
        Self {
            inner: input.clone(),
            xform: None,
        }
    }
}

impl AsRef<syn::Type> for Typed {
    fn as_ref(&self) -> &syn::Type {
        &self.inner
    }
}

impl ToTokens for Typed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.inner.to_tokens(tokens)
    }
}
