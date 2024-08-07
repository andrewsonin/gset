use crate::field_attributes::AttributeKind;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{spanned::Spanned, Lit, Meta, MetaNameValue, Type, Visibility};

#[derive(Default)]
pub(crate) struct AttributeLayout {
    fn_name_override: Option<Ident>,
    visibility: Option<Visibility>,
    kind: Option<AttributeKind>,
    type_override: Option<Type>,
}

impl<T> From<T> for AttributeLayout
where
    T: IntoIterator<Item = Meta>,
{
    fn from(metas: T) -> Self {
        let mut current_layout = Self::default();
        for meta in metas {
            match &meta {
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
                    let kind: AttributeKind = path.parse().unwrap_or_else(|()| {
                        abort!(
                            meta.span(),
                            "Unknown getset kind attribute: `{}`. Should be one of: {}",
                            path,
                            AttributeKind::all_kinds()
                        )
                    });
                    current_layout.kind = kind.into();
                }
                Meta::List(list) => {
                    abort!(list.span(), "Multiple attributes are not supported")
                }
                Meta::NameValue(MetaNameValue { path, lit, .. }) => {
                    let path = quote! { #path }.to_string();
                    let lit_str = match &lit {
                        Lit::Str(str) => str.value(),
                        _ => abort!(lit.span(), "Value type not supported. Should be string"),
                    };
                    match path.as_str() {
                        "name" => {
                            current_layout.fn_name_override = syn::parse_str::<Ident>(&lit_str)
                                .map_err(|e| syn::Error::new(lit.span(), e))
                                .expect_or_abort("invalid ident")
                                .into();
                        }
                        "vis" => {
                            current_layout.visibility = syn::parse_str::<Visibility>(&lit_str)
                                .map_err(|e| syn::Error::new(lit.span(), e))
                                .expect_or_abort("invalid visibility found")
                                .into();
                        }
                        "ty" => {
                            current_layout.type_override = syn::parse_str::<Type>(&lit_str)
                                .map_err(|e| syn::Error::new(lit.span(), e))
                                .expect_or_abort("invalid ty found")
                                .into();
                        }
                        _ => abort!(
                            lit.span(),
                            "Unknown named parameter. Should be one of: `name`, `vis`, `ty`"
                        ),
                    }
                }
            }
        }
        current_layout
    }
}

impl AttributeLayout {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn generate_fn_def(
        self,
        field_ident_or_idx: &str,
        field_type: &Type,
        attr_span: impl Fn() -> Span,
    ) -> TokenStream2 {
        let Self {
            fn_name_override,
            visibility,
            kind,
            type_override,
        } = self;
        let fn_name_override = fn_name_override.as_ref();
        let get_ty_override = || type_override.map(|ty| quote! { #ty });

        let Some(kind) = kind else {
            abort!(
                attr_span(),
                "Missed getset `kind` attribute. Should be one of: {}",
                AttributeKind::all_kinds()
            )
        };

        let getter_fn_name =
            || generate_fn_name(fn_name_override, || field_ident_or_idx, &attr_span, kind);
        let mut_getter_fn_name = || {
            generate_fn_name(
                fn_name_override,
                || format!("{field_ident_or_idx}_mut"),
                &attr_span,
                kind,
            )
        };
        let setter_fn_name = || {
            generate_fn_name(
                fn_name_override,
                || format!("set_{field_ident_or_idx}"),
                &attr_span,
                kind,
            )
        };

        let field_ident_or_idx: TokenStream2 = field_ident_or_idx.parse().unwrap();

        let (signature, body, ty) = match kind {
            AttributeKind::Get => {
                let fn_name = getter_fn_name();
                (
                    quote! { #fn_name(&self) },
                    quote! { &self.#field_ident_or_idx },
                    get_ty_override().unwrap_or_else(|| quote! { &#field_type }),
                )
            }
            AttributeKind::GetMut => {
                let fn_name = mut_getter_fn_name();
                (
                    quote! { #fn_name(&mut self) },
                    quote! { &mut self.#field_ident_or_idx },
                    get_ty_override().unwrap_or_else(|| quote! { &mut #field_type }),
                )
            }
            AttributeKind::GetCopy => {
                let fn_name = getter_fn_name();
                (
                    quote! { #fn_name(&self) },
                    quote! { self.#field_ident_or_idx },
                    get_ty_override().unwrap_or_else(|| quote! { #field_type }),
                )
            }
            AttributeKind::GetDeref => {
                let fn_name = getter_fn_name();
                (
                    quote! { #fn_name(&self) },
                    quote! { &self.#field_ident_or_idx },
                    get_ty_override()
                        .unwrap_or_else(|| quote! { &<#field_type as ::std::ops::Deref>::Target }),
                )
            }
            AttributeKind::GetDerefMut => {
                let fn_name = mut_getter_fn_name();
                (
                    quote! { #fn_name(&mut self) },
                    quote! { &mut self.#field_ident_or_idx },
                    get_ty_override().unwrap_or_else(
                        || quote! { &mut <#field_type as ::std::ops::Deref>::Target },
                    ),
                )
            }
            AttributeKind::GetDerefCopy => {
                let fn_name = getter_fn_name();
                (
                    quote! { #fn_name(&self) },
                    quote! { *self.#field_ident_or_idx },
                    get_ty_override()
                        .unwrap_or_else(|| quote! { <#field_type as ::std::ops::Deref>::Target }),
                )
            }
            AttributeKind::GetAsRef => {
                let fn_name = getter_fn_name();
                let ty = get_ty_override().unwrap_or_else(|| {
                    abort!(
                        attr_span(),
                        "Missed `ty` named parameter. Should be set for `get_as_ref` getset kind",
                    )
                });
                (
                    quote! { #fn_name(&self) },
                    quote! { self.#field_ident_or_idx.as_ref() },
                    ty,
                )
            }
            AttributeKind::GetAsDeref => {
                let fn_name = getter_fn_name();
                let ty = get_ty_override().unwrap_or_else(|| {
                    abort!(
                        attr_span(),
                        "Missed `ty` named parameter. Should be set for `get_as_deref` getset kind",
                    )
                });
                (
                    quote! { #fn_name(&self) },
                    quote! { self.#field_ident_or_idx.as_deref() },
                    ty,
                )
            }
            AttributeKind::GetAsDerefMut => {
                let fn_name = mut_getter_fn_name();
                let ty = get_ty_override().unwrap_or_else(|| {
                    abort!(
                        attr_span(),
                        "Missed `ty` named parameter. Should be set for `get_as_deref_mut` getset kind",
                    )
                });
                (
                    quote! { #fn_name(&mut self) },
                    quote! { self.#field_ident_or_idx.as_deref_mut() },
                    ty,
                )
            }
            AttributeKind::Set => {
                let fn_name = setter_fn_name();
                (
                    quote! { #fn_name(&mut self, value: #field_type) },
                    quote! { self.#field_ident_or_idx = value },
                    get_ty_override().unwrap_or_else(|| quote! { () }),
                )
            }
            AttributeKind::SetBorrowed => {
                let fn_name = setter_fn_name();
                (
                    quote! { #fn_name(&mut self, value: #field_type) },
                    quote! { self.#field_ident_or_idx = value; self },
                    get_ty_override().unwrap_or_else(|| quote! { &mut Self }),
                )
            }
            AttributeKind::SetOwned => {
                let fn_name = setter_fn_name();
                (
                    quote! { #fn_name(mut self, value: #field_type) },
                    quote! { self.#field_ident_or_idx = value; self },
                    get_ty_override().unwrap_or_else(|| quote! { Self }),
                )
            }
        };
        quote! {
            #visibility fn #signature -> #ty {
                #body
            }
        }
    }
}

fn generate_fn_name<F>(
    fn_name_override: Option<&Ident>,
    fn_name_fallback: impl FnOnce() -> F,
    attr_span: impl FnOnce() -> Span,
    attr_kind: AttributeKind,
) -> Ident
where
    F: AsRef<str>,
{
    fn_name_override
        .cloned()
        .or_else(|| syn::parse_str(fn_name_fallback().as_ref()).ok())
        .unwrap_or_else(|| {
            abort!(
                attr_span(),
                "Missed `name` named parameter. \
                Should be set for `{}` getset kind when struct ",
                attr_kind
            )
        })
}
