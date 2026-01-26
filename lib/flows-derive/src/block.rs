// This is free and unencumbered software released into the public domain.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, Pat, Type, TypeImplTrait, parse_macro_input};

/// Optional arguments for the `#[block]` attribute
#[derive(Debug, Default, FromMeta)]
struct BlockArgs {
    /// Override the generated struct name
    #[darling(default)]
    name: Option<String>,
}

pub fn block(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Parse attributes using Darling
    let args = if attr.is_empty() {
        BlockArgs::default()
    } else {
        match NestedMeta::parse_meta_list(attr.into()) {
            Ok(meta_list) => match BlockArgs::from_list(&meta_list) {
                Ok(args) => args,
                Err(e) => return TokenStream::from(e.write_errors()),
            },
            Err(e) => return TokenStream::from(e.to_compile_error()),
        }
    };

    // Generate struct name: snake_case -> PascalCase
    let struct_name = args
        .name
        .map(|n| Ident::new(&n, Span::call_site()))
        .unwrap_or_else(|| snake_to_pascal(&input_fn.sig.ident));

    // Process function parameters into struct fields
    let fields: Vec<_> = input_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| process_arg(arg))
        .collect();

    // Generate the struct
    let struct_def = quote! {
        pub struct #struct_name {
            #(#fields),*
        }
    };

    TokenStream::from(quote! {
        #struct_def
        #input_fn
    })
}

/// Process a function argument into a struct field
fn process_arg(arg: &FnArg) -> Option<proc_macro2::TokenStream> {
    let FnArg::Typed(pat_type) = arg else {
        return None; // Skip `self` parameters
    };

    // Extract the field name from the pattern
    let field_name = extract_ident(&pat_type.pat)?;

    // Convert the type (handle `impl Trait` -> concrete type)
    let field_type = convert_type(&pat_type.ty);

    // Determine visibility: `pub` for Inputs/Outputs types
    let visibility = if is_io_type(&pat_type.ty) {
        quote! { pub }
    } else {
        quote! {}
    };

    Some(quote! {
        #visibility #field_name: #field_type
    })
}

/// Extract identifier from a pattern, ignoring `mut`
fn extract_ident(pat: &Pat) -> Option<&Ident> {
    match pat {
        Pat::Ident(pat_ident) => Some(&pat_ident.ident),
        _ => None,
    }
}

/// Convert `impl AsRef<str>` and similar to concrete types
fn convert_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::ImplTrait(impl_trait) => convert_impl_trait(impl_trait),
        other => quote! { #other },
    }
}

/// Convert impl trait bounds to a concrete type
fn convert_impl_trait(impl_trait: &TypeImplTrait) -> proc_macro2::TokenStream {
    for bound in &impl_trait.bounds {
        if let syn::TypeParamBound::Trait(trait_bound) = bound {
            let path = &trait_bound.path;
            let last_segment = path.segments.last();

            if let Some(segment) = last_segment {
                let trait_name = segment.ident.to_string();

                // Map common traits to concrete types
                return match trait_name.as_str() {
                    "AsRef" => quote! { String },
                    "Into" => extract_generic_arg(segment).unwrap_or_else(|| quote! { String }),
                    "ToString" => quote! { String },
                    "Display" => quote! { String },
                    "Iterator" => quote! { Vec<_> },
                    _ => quote! { String }, // Default fallback
                };
            }
        }
    }
    quote! { String }
}

/// Extract the generic argument from a path segment (e.g., `AsRef<str>` -> `str`)
fn extract_generic_arg(segment: &syn::PathSegment) -> Option<proc_macro2::TokenStream> {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return Some(quote! { #ty });
        }
    }
    None
}

/// Check if type is `Inputs<T>` or `Outputs<T>` (should be `pub`)
fn is_io_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string();
    type_str.contains("Inputs") || type_str.contains("Outputs")
}

/// Convert snake_case to PascalCase
fn snake_to_pascal(ident: &Ident) -> Ident {
    let snake = ident.to_string();
    let pascal: String = snake
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect();

    Ident::new(&pascal, ident.span())
}
