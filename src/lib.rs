#![doc = include_str!("../README.md")]

use field_attribute_layout::AttributeLayout;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort_call_site, proc_macro_error, ResultExt};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput,
    Field, Meta, Token,
};

mod field_attribute_layout;
mod field_attributes;

/// Derives getters and setters.
#[proc_macro_derive(Getset, attributes(getset))]
#[proc_macro_error]
pub fn derive_getset(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = &ast;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(DataStruct { fields, .. }) = data else {
        abort_call_site!("#[derive(Getset)] is only supported for structs")
    };

    let mut impls = TokenStream2::new();
    for (idx, field) in fields.iter().enumerate() {
        let Field {
            attrs,
            ident: field_ident,
            ty: field_type,
            ..
        } = field;
        let getset_attrs = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("getset"))
            .collect::<Vec<_>>();
        if getset_attrs.is_empty() {
            continue;
        }

        #[allow(clippy::map_unwrap_or)]
        let ident_or_index = field_ident
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| idx.to_string());
        let ident_or_index = ident_or_index.as_str();

        let doc = attrs
            .iter()
            .filter(|v| {
                v.parse_meta()
                    .map(|meta| meta.path().is_ident("doc"))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        for attr in getset_attrs {
            let metas = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap_or_abort();

            let fn_def =
                AttributeLayout::from(metas)
                    .generate_fn_def(ident_or_index, field_type, || attr.span());

            let imp = quote! {
                #(#doc)*
                #[inline(always)]
                #fn_def
            };

            impls.extend(imp);
        }
    }

    let tokens = quote! {
        impl #impl_generics #ident #ty_generics #where_clause
        {
            #impls
        }
    };
    tokens.into()
}
