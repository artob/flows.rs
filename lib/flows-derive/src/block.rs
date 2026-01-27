// This is free and unencumbered software released into the public domain.

use super::params::{Param, ParamType};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use darling::{FromMeta, ast::NestedMeta};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{FnArg, Ident, ItemFn};

/// Optional arguments for the `#[block]` attribute
#[derive(Debug, Default, FromMeta)]
struct BlockOptions {
    /// Override the generated struct name
    #[darling(default)]
    name: Option<String>,
}

pub fn block(attr: TokenStream, input_fn: ItemFn) -> TokenStream {
    let fn_vis = input_fn.vis.clone();

    // Parse attributes using Darling
    let options = if attr.is_empty() {
        BlockOptions::default()
    } else {
        match NestedMeta::parse_meta_list(attr.into()) {
            Ok(meta_list) => match BlockOptions::from_list(&meta_list) {
                Ok(args) => args,
                Err(e) => return TokenStream::from(e.write_errors()),
            },
            Err(e) => return TokenStream::from(e.to_compile_error()),
        }
    };

    // Generate the trait name: snake_case->PascalCase:
    let trait_name = options
        .name
        .map(|n| Ident::new(&n, Span::call_site()))
        .unwrap_or_else(|| {
            Ident::new(
                &snake_to_pascal(&input_fn.sig.ident.to_string()).to_string(),
                Span::call_site(),
            )
        });

    // Generate the struct name: snake_case->PascalCase + "Block":
    let struct_name = Ident::new(
        &format!("{}Block", trait_name.to_string()),
        Span::call_site(),
    );

    // Extract generics and where clause from the function:
    let generics = &input_fn.sig.generics;
    let where_clause = &input_fn.sig.generics.where_clause;

    let inputs = input_fn.sig.inputs.clone();

    let params: Vec<_> = inputs.iter().filter_map(extract_param).collect();

    // Process function parameters into struct fields:
    let struct_fields: Vec<_> = params.iter().filter_map(param_to_struct_field).collect(); // TODO

    // Process function parameters into constructor parameters:
    let new_params: Vec<_> = params.iter().filter_map(param_name_and_type).collect();

    // Process function parameters into constructor initializers:
    let new_args: Vec<_> = params.iter().filter_map(param_name).collect();

    // Process function parameters into trait method parameters:
    // let trait_params: Vec<_> = inputs.iter().filter_map(fn_param_to_new_param).collect();

    // Generate the struct with generics and where clause:
    let struct_def = quote! {
        #[allow(unused)]
        #[automatically_derived]
        #fn_vis struct #struct_name #generics
        #where_clause
        {
            #(#struct_fields),*
        }
        impl #generics #struct_name #generics
        #where_clause
        {
            #fn_vis fn new(#(#new_params),*) -> Self {
                Self { #(#new_args),* }
            }
        }
    };

    // Generate the trait with generics and where clause:
    // let _trait_def = quote! {
    //     #[allow(unused)]
    //     #[automatically_derived]
    //     #fn_vis trait #trait_name
    //     {
    //         fn new #generics (&self, #(#trait_params),*) -> #struct_name #generics {
    //             todo!()
    //         }
    //     }
    // };

    quote! {
        #struct_def
        #input_fn
    }
}

/// Process a function argument into a constructor parameter
fn extract_param(param: &FnArg) -> Option<Param> {
    param.try_into().ok()
}

/// Process a function argument into a constructor parameter
fn param_name(param: &Param) -> Option<TokenStream> {
    let field_name = param.name();
    match &param.typ.owned() {
        ParamType::Other(t) => Some(match &t.xform {
            None => quote! { #field_name },
            Some(xform) => quote! { #field_name: #field_name.#xform },
        }),
        _ => Some(quote! { #field_name: Default::default() }),
    }
}

/// Process a function argument into a constructor parameter
fn param_name_and_type(param: &Param) -> Option<TokenStream> {
    match &param.typ {
        ParamType::Other(_) => {
            let field_name = param.name();
            let field_type = &param.typ;
            Some(quote! { #field_name: #field_type })
        },
        _ => None,
    }
}

/// Process a function argument into a struct field
fn param_to_struct_field(param: &Param) -> Option<TokenStream> {
    // Convert the type (handle `impl Trait` -> concrete type, `&[T]` -> `Vec<T>`):
    let field_name = param.name();
    let field_type = &param.typ.owned();
    Some(quote! { #field_name: #field_type })
}

/// Convert snake_case to PascalCase
fn snake_to_pascal(input: &str) -> String {
    input
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}
