// This is free and unencumbered software released into the public domain.

use super::r#type::Typed;
use alloc::string::{String, ToString};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{FnArg, Ident, Pat, PatType, Path, Type, TypePath};
use syn_match::path_match;

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub typ: ParamType,
}

impl Param {
    pub fn is_port(&self) -> bool {
        self.typ.is_port()
    }

    pub fn name(&self) -> Ident {
        Ident::new(&self.name, Span::call_site())
    }
}

impl ToTokens for Param {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = self.name();
        let field_type = &self.typ;

        // Determine visibility: `pub` for port types:
        let visibility = if self.is_port() {
            quote! { pub }
        } else {
            quote! {}
        };

        tokens.extend(quote! {
            #visibility #field_name: #field_type
        });
    }
}

impl TryFrom<&FnArg> for Param {
    type Error = ();

    fn try_from(input: &FnArg) -> Result<Self, Self::Error> {
        match input {
            FnArg::Typed(typed) => typed.try_into(),
            _ => return Err(()), // skip `self` parameters
        }
    }
}

impl TryFrom<&PatType> for Param {
    type Error = ();

    fn try_from(input: &PatType) -> Result<Self, Self::Error> {
        // Extract the identifier from the pattern, ignoring `mut`:
        let input_ty: &Type = &input.ty;
        let input_pat: &Pat = &input.pat;
        Ok(match input_pat {
            Pat::Ident(id) => Self {
                name: id.ident.to_string(),
                typ: input_ty.into(),
            },
            _ => return Err(()), // skip non-identifiers
        })
    }
}

#[derive(Clone, Debug)]
pub enum ParamType {
    Input(Path, isize),
    Output(Path, isize),
    Other(Typed),
}

impl ParamType {
    pub fn is_port(&self) -> bool {
        use ParamType::*;
        matches!(self, Input(_, _) | Output(_, _))
    }

    pub fn owned(&self) -> Self {
        use ParamType::*;
        match &self {
            Other(typed) => Other(typed.owned()),
            _ => self.clone(),
        }
    }
}

impl From<&Type> for ParamType {
    fn from(input: &Type) -> Self {
        match input {
            Type::Path(TypePath { path, .. }) => path_match!(&path,
                async_flow?::Input<$_t> => ParamType::Input(path.clone(), 1),
                async_flow?::Inputs<$_t> => ParamType::Input(path.clone(), -1),
                async_flow?::Output<$_t> => ParamType::Output(path.clone(), 1),
                async_flow?::Outputs<$_t> => ParamType::Output(path.clone(), -1),
                _ => ParamType::Other(input.into()),
            ),
            _ => ParamType::Other(input.into()),
        }
    }
}

impl ToTokens for ParamType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            ParamType::Input(path, _count) => {
                path_match!(&path,
                    async_flow?::Input<$t> => quote! { async_flow::model::Input<#t> },
                    async_flow?::Inputs<$t> => quote! { async_flow::model::Inputs<#t> },
                    _ => unreachable!(),
                )
            },
            ParamType::Output(path, _count) => {
                path_match!(&path,
                    async_flow?::Output<$t> => quote! { async_flow::model::Output<#t> },
                    async_flow?::Outputs<$t> => quote! { async_flow::model::Outputs<#t> },
                    _ => unreachable!(),
                )
            },
            ParamType::Other(type_) => {
                quote! { #type_ }
            },
        });
    }
}
