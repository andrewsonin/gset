use {
    printable::AsPrintable,
    proc_macro2::TokenStream as TokenStream2,
    proc_macro::TokenStream,
    proc_macro_error::{abort, abort_call_site, proc_macro_error, ResultExt},
    quote::quote,
    std::fmt::{Display, Formatter},
    syn::{
        Data,
        DataStruct,
        DeriveInput,
        Field,
        Lit,
        Meta,
        MetaNameValue,
        parse_macro_input,
        punctuated::Punctuated,
        spanned::Spanned,
        Token,
        Visibility,
    },
};

mod printable;

#[derive(Debug)]
enum GetSetKind {
    Get,
    GetMut,
    GetCopy,
    DerefGet,
    DerefGetMut,
    DerefGetCopy,
    Set,
}

const ALL_KINDS: [&str; 7] = [
    "get",
    "get_mut",
    "get_copy",
    "deref_get",
    "deref_get_mut",
    "deref_get_copy",
    "set"
];

impl Display for GetSetKind
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let msg = match self
        {
            GetSetKind::Get => "get",
            GetSetKind::GetMut => "get_mut",
            GetSetKind::GetCopy => "get_copy",
            GetSetKind::DerefGet => "deref_get",
            GetSetKind::DerefGetMut => "deref_get_mut",
            GetSetKind::DerefGetCopy => "deref_get_copy",
            GetSetKind::Set => "set",
        };
        write!(f, "{msg}")
    }
}

#[proc_macro_derive(Getset, attributes(getset))]
#[proc_macro_error]
pub fn derive_getset(input: TokenStream) -> TokenStream
{
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = &ast;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => abort_call_site!("#[derive(Getset)] is only supported for structs")
    };

    #[derive(Default)]
    struct GetSetLayout {
        fn_name: Option<proc_macro2::Ident>,
        visibility: Option<Visibility>,
        kind: Option<GetSetKind>,
    }

    let mut impls = TokenStream2::new();
    for field in fields
    {
        let Field { attrs, ident, ty, .. } = field;
        let ident = if let Some(ident) = ident {
            ident
        } else {
            continue;
        };
        let doc = field.attrs.iter()
            .filter(
                |v| {
                    v.parse_meta()
                        .map(|meta| meta.path().is_ident("doc"))
                        .unwrap_or(false)
                }
            )
            .collect::<Vec<_>>();
        for attr in attrs
        {
            if !attr.path.is_ident("getset") {
                continue;
            }
            let metas = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap_or_abort();
            let mut current_layout = GetSetLayout::default();
            for meta in metas
            {
                match &meta
                {
                    Meta::Path(path) => {
                        let path = quote! { #path }.to_string();
                        if let Some(kind) = &current_layout.kind {
                            abort!(
                                meta.span(),
                                "Duplicate getset kind attributes: `{}` and `{}`",
                                kind,
                                path
                            )
                        }
                        let kind = match path.as_str() {
                            "get" => GetSetKind::Get,
                            "get_mut" => GetSetKind::GetMut,
                            "get_copy" => GetSetKind::GetCopy,
                            "deref_get" => GetSetKind::DerefGet,
                            "deref_get_mut" => GetSetKind::DerefGetMut,
                            "deref_get_copy" => GetSetKind::DerefGetCopy,
                            "set" => GetSetKind::Set,
                            _ => abort!(
                                meta.span(),
                                "Unknown getset kind attribute: `{}`. Should be one of: {}",
                                path,
                                ALL_KINDS.printable()
                            )
                        };
                        current_layout.kind = kind.into()
                    }
                    Meta::List(list) => {
                        abort!(list.span(), "Multiple attributes are not supported")
                    }
                    Meta::NameValue(MetaNameValue { path, lit, .. }) => {
                        let path = quote! { #path }.to_string();
                        let lit_str = match &lit {
                            Lit::Str(str) => str.value(),
                            _ => abort!(lit.span(), "Value type not supported. Should be string")
                        };
                        match path.as_str() {
                            "name" => {
                                current_layout.fn_name = syn::parse_str::<proc_macro2::Ident>(&lit_str)
                                    .map_err(|e| syn::Error::new(lit.span(), e))
                                    .expect_or_abort("invalid ident")
                                    .into()
                            }
                            "vis" => {
                                current_layout.visibility = syn::parse_str::<Visibility>(&lit_str)
                                    .map_err(|e| syn::Error::new(lit.span(), e))
                                    .expect_or_abort("invalid visibility found")
                                    .into()
                            }
                            _ => abort!(
                                lit.span(),
                                "Unknown named attribute. Should be one of: [name, vis]"
                            )
                        }
                    }
                }
            }

            let GetSetLayout { fn_name, visibility, kind } = current_layout;

            let kind = if let Some(kind) = kind {
                kind
            } else {
                abort!(
                    attr.span(),
                    "Missed getset kind attribute. Should be one of: {}",
                    ALL_KINDS.printable()
                )
            };

            let (signature, body, ty) = match kind
            {
                GetSetKind::Get => {
                    let fn_name = fn_name.as_ref().unwrap_or(ident);
                    (
                        quote! { #fn_name(&self) },
                        quote! { &self.#ident },
                        quote! { &#ty }
                    )
                }
                GetSetKind::GetMut => {
                    let fn_name = fn_name
                        .unwrap_or_else(|| syn::parse_str(&format!("{ident}_mut")).unwrap());
                    (
                        quote! { #fn_name(&mut self) },
                        quote! { &mut self.#ident },
                        quote! { &mut #ty }
                    )
                }
                GetSetKind::GetCopy => {
                    let fn_name = fn_name.as_ref().unwrap_or(ident);
                    (
                        quote! { #fn_name(&self) },
                        quote! { self.#ident },
                        quote! { #ty }
                    )
                }
                GetSetKind::DerefGet => {
                    let fn_name = fn_name.as_ref().unwrap_or(ident);
                    (
                        quote! { #fn_name(&self) },
                        quote! { &self.#ident },
                        quote! { &<#ty as ::std::ops::Deref>::Target }
                    )
                }
                GetSetKind::DerefGetMut => {
                    let fn_name = fn_name
                        .unwrap_or_else(|| syn::parse_str(&format!("{ident}_mut")).unwrap());
                    (
                        quote! { #fn_name(&mut self) },
                        quote! { &mut self.#ident },
                        quote! { &mut <#ty as ::std::ops::Deref>::Target }
                    )
                }
                GetSetKind::DerefGetCopy => {
                    let fn_name = fn_name.as_ref().unwrap_or(ident);
                    (
                        quote! { #fn_name(&self) },
                        quote! { *self.#ident },
                        quote! { <#ty as ::std::ops::Deref>::Target }
                    )
                }
                GetSetKind::Set => {
                    let fn_name = fn_name
                        .unwrap_or_else(|| syn::parse_str(&format!("set_{ident}")).unwrap());
                    (
                        quote! { #fn_name(&mut self, value: #ty) },
                        quote! { self.#ident = value },
                        quote! { () }
                    )
                }
            };

            let imp = quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #signature -> #ty {
                    #body
                }
            };

            impls.extend(imp)
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