use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, LitStr, Meta};

/// Parsed `#[jumo(...)]` attributes on a type.
#[derive(Default, Debug)]
struct TypeAttr {
    id: Option<String>,
    kind: String,
    domain: String,
    module: Option<String>,
    role: Option<String>,
    identity: Option<String>,
    tag: Option<String>,
    storage_kind: Option<String>,
    durability: Option<String>,
    parent: Option<String>,
    description: Option<String>,
}

/// Extract `#[jumo(...)]` attributes from the type-level attribute list.
///
/// Unknown keys and malformed values are hard errors: a silently dropped key
/// would emit incomplete metadata that no compiler check would ever catch.
/// `kind` and `domain` are required, because `JumoItem` declares them without a
/// default and an empty string is not a usable model fact.
fn parse_type_attrs(
    attrs: &[syn::Attribute],
    fallback_span: proc_macro2::Span,
) -> syn::Result<TypeAttr> {
    let mut result = TypeAttr::default();
    let mut seen_any = false;

    for attr in attrs {
        if !attr.path().is_ident("jumo") {
            continue;
        }
        seen_any = true;

        if let Meta::List(list) = &attr.meta {
            list.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        "kind" => result.kind = meta.value()?.parse::<LitStr>()?.value(),
                        "id" => result.id = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "domain" => result.domain = meta.value()?.parse::<LitStr>()?.value(),
                        "module" => result.module = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "role" => result.role = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "identity" => {
                            result.identity = Some(meta.value()?.parse::<LitStr>()?.value())
                        }
                        "tag" => result.tag = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "storage_kind" => {
                            result.storage_kind = Some(meta.value()?.parse::<LitStr>()?.value())
                        }
                        "durability" => {
                            result.durability = Some(meta.value()?.parse::<LitStr>()?.value())
                        }
                        "parent" => result.parent = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "description" => {
                            result.description = Some(meta.value()?.parse::<LitStr>()?.value())
                        }
                        other => {
                            return Err(syn::Error::new(
                                meta.path.span(),
                                format!("unknown jumo attr: `{other}`"),
                            ));
                        }
                    }
                }
                Ok(())
            })?;
        }
    }

    if !seen_any {
        return Err(syn::Error::new(
            fallback_span,
            "`#[derive(Jumo)]` requires a `#[jumo(kind = \"...\", domain = \"...\")]` attribute",
        ));
    }

    let missing: Vec<&str> = [
        ("kind", result.kind.as_str()),
        ("domain", result.domain.as_str()),
    ]
    .into_iter()
    .filter(|(_, value)| value.is_empty())
    .map(|(key, _)| key)
    .collect();

    if !missing.is_empty() {
        let names = missing
            .iter()
            .map(|key| format!("`{key}`"))
            .collect::<Vec<_>>()
            .join(" and ");
        return Err(syn::Error::new(
            fallback_span,
            format!("`#[jumo(...)]` requires {names}, but the value is empty"),
        ));
    }

    Ok(result)
}

/// Collect field idents that have `#[jumo(unique)]`.
///
/// Uninterpreted field-level flags are deliberately tolerated instead of
/// rejected: `#[jumo(skip)]` marks fields the extractor ignores, and the derive
/// macro must not fail a build over metadata it does not itself consume.
fn parse_field_unique_attrs(fields: &Fields) -> Vec<Ident> {
    let mut unique = Vec::new();

    let named = match fields {
        Fields::Named(named) => named,
        _ => return unique,
    };

    for field in &named.named {
        for attr in &field.attrs {
            if !attr.path().is_ident("jumo") {
                continue;
            }
            if let Meta::List(list) = &attr.meta {
                let _ = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("unique") {
                        if let Some(ident) = &field.ident {
                            unique.push(ident.clone());
                        }
                    }
                    Ok(())
                });
            }
        }
    }

    unique
}

fn to_lit_strs(idents: &[Ident]) -> Vec<LitStr> {
    idents
        .iter()
        .map(|i| LitStr::new(&i.to_string(), i.span()))
        .collect()
}

#[proc_macro_derive(Jumo, attributes(jumo))]
pub fn derive_jumo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let attrs = match parse_type_attrs(&input.attrs, name.span()) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    // syn 2.x: fields are inside `data`.  Enum has per-variant fields so we
    // skip field-level unique collection for enums; struct/union have top-level
    // named fields.
    let unique_fields = match &input.data {
        Data::Struct(s) => parse_field_unique_attrs(&s.fields),
        Data::Enum(_) => {
            // No field-level unique for enums — emit impl and return early.
            let kind = &attrs.kind;
            let domain = &attrs.domain;
            let id = opt_str(&attrs.id);
            let module = opt_str(&attrs.module);
            let role = opt_str(&attrs.role);
            let identity = opt_str(&attrs.identity);
            let tag = opt_str(&attrs.tag);
            let storage_kind = opt_str(&attrs.storage_kind);
            let durability = opt_str(&attrs.durability);
            let parent = opt_str(&attrs.parent);
            let description = opt_str(&attrs.description);

            let expanded = quote! {
                impl #impl_generics ::jumo_derive::JumoItem for #name #type_generics #where_clause {
                    fn jumo_id() -> ::core::option::Option<&'static str> { #id }
                    fn jumo_kind() -> &'static str { #kind }
                    fn jumo_domain() -> &'static str { #domain }
                    fn jumo_module() -> ::core::option::Option<&'static str> { #module }
                    fn jumo_role() -> ::core::option::Option<&'static str> { #role }
                    fn jumo_identity() -> ::core::option::Option<&'static str> { #identity }
                    fn jumo_tag() -> ::core::option::Option<&'static str> { #tag }
                    fn jumo_storage_kind() -> ::core::option::Option<&'static str> { #storage_kind }
                    fn jumo_durability() -> ::core::option::Option<&'static str> { #durability }
                    fn jumo_parent() -> ::core::option::Option<&'static str> { #parent }
                    fn jumo_description() -> ::core::option::Option<&'static str> { #description }
                }
            };
            return TokenStream::from(expanded);
        }
        Data::Union(u) => parse_field_unique_attrs(&Fields::Named(u.fields.clone())),
    };

    let unique_strs = to_lit_strs(&unique_fields);

    let kind = &attrs.kind;
    let domain = &attrs.domain;
    let id = opt_str(&attrs.id);
    let module = opt_str(&attrs.module);
    let role = opt_str(&attrs.role);
    let identity = opt_str(&attrs.identity);
    let tag = opt_str(&attrs.tag);
    let storage_kind = opt_str(&attrs.storage_kind);
    let durability = opt_str(&attrs.durability);
    let parent = opt_str(&attrs.parent);
    let description = opt_str(&attrs.description);

    let unique_tokens = if unique_strs.is_empty() {
        quote! { &[] }
    } else {
        quote! { &[#(#unique_strs),*] }
    };

    let expanded = quote! {
        impl #impl_generics ::jumo_derive::JumoItem for #name #type_generics #where_clause {
            fn jumo_id() -> ::core::option::Option<&'static str> { #id }

            fn jumo_kind() -> &'static str { #kind }

            fn jumo_domain() -> &'static str { #domain }

            fn jumo_module() -> ::core::option::Option<&'static str> { #module }

            fn jumo_role() -> ::core::option::Option<&'static str> { #role }

            fn jumo_identity() -> ::core::option::Option<&'static str> { #identity }

            fn jumo_tag() -> ::core::option::Option<&'static str> { #tag }

            fn jumo_storage_kind() -> ::core::option::Option<&'static str> { #storage_kind }

            fn jumo_durability() -> ::core::option::Option<&'static str> { #durability }

            fn jumo_parent() -> ::core::option::Option<&'static str> { #parent }

            fn jumo_description() -> ::core::option::Option<&'static str> { #description }

            fn jumo_unique_fields() -> &'static [&'static str] { #unique_tokens }
        }
    };

    TokenStream::from(expanded)
}

fn opt_str(opt: &Option<String>) -> proc_macro2::TokenStream {
    match opt {
        Some(s) => {
            let lit = LitStr::new(s, proc_macro2::Span::call_site());
            quote! { ::core::option::Option::Some(#lit) }
        }
        None => quote! { ::core::option::Option::None },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn parse(attr: syn::Attribute) -> syn::Result<TypeAttr> {
        parse_type_attrs(&[attr], proc_macro2::Span::call_site())
    }

    fn message(err: &syn::Error) -> String {
        err.to_string()
    }

    fn named_fields(src: proc_macro2::TokenStream) -> Fields {
        let input: DeriveInput = syn::parse2(src).expect("should parse as a type");
        match input.data {
            Data::Struct(data) => data.fields,
            _ => panic!("expected a struct"),
        }
    }

    #[test]
    fn parses_id_module_and_optional_metadata() {
        let parsed = parse(parse_quote!(#[jumo(
            id = "Business.Order.Root",
            kind = "struct",
            domain = "Business",
            module = "Business.Order",
            role = "command"
        )]))
        .expect("should parse");

        assert_eq!(parsed.id.as_deref(), Some("Business.Order.Root"));
        assert_eq!(parsed.kind, "struct");
        assert_eq!(parsed.domain, "Business");
        assert_eq!(parsed.module.as_deref(), Some("Business.Order"));
        assert_eq!(parsed.role.as_deref(), Some("command"));
        assert_eq!(parsed.identity, None);
        assert_eq!(parsed.parent, None);
    }

    #[test]
    fn unknown_key_is_rejected_instead_of_silently_dropped() {
        // Regression: the error used to be discarded by `let _ =`, and because
        // `parse_nested_meta` stops at the first error every key after the
        // unknown one was dropped too.
        let err = parse(parse_quote!(#[jumo(
            kind = "struct",
            bogus = "x",
            domain = "Business"
        )]))
        .expect_err("unknown key must fail");

        assert!(
            message(&err).contains("unknown jumo attr: `bogus`"),
            "{err}"
        );
    }

    #[test]
    fn non_string_value_is_rejected() {
        let err = parse(parse_quote!(#[jumo(kind = 12, domain = "Business")]))
            .expect_err("non-string value must fail");

        assert!(message(&err).contains("expected string literal"), "{err}");
    }

    #[test]
    fn missing_attribute_is_rejected() {
        let err = parse_type_attrs(&[], proc_macro2::Span::call_site())
            .expect_err("missing attribute must fail");

        assert!(message(&err).contains("requires a `#[jumo("), "{err}");
    }

    #[test]
    fn empty_attribute_is_rejected() {
        let err = parse(parse_quote!(#[jumo()])).expect_err("empty attribute must fail");

        assert!(message(&err).contains("`kind` and `domain`"), "{err}");
    }

    #[test]
    fn missing_domain_is_rejected() {
        let err =
            parse(parse_quote!(#[jumo(kind = "struct")])).expect_err("missing domain must fail");

        assert!(message(&err).contains("`domain`"), "{err}");
    }

    #[test]
    fn empty_value_is_rejected() {
        let err = parse(parse_quote!(#[jumo(kind = "", domain = "Business")]))
            .expect_err("empty value must fail");

        assert!(message(&err).contains("`kind`"), "{err}");
    }

    #[test]
    fn duplicate_key_last_one_wins() {
        let parsed = parse(parse_quote!(#[jumo(
            kind = "struct",
            kind = "message",
            domain = "Business"
        )]))
        .expect("should parse");

        assert_eq!(parsed.kind, "message");
    }

    #[test]
    fn accumulates_across_multiple_attributes_before_validating() {
        let attrs = vec![
            parse_quote!(#[jumo(kind = "struct")]),
            parse_quote!(#[jumo(domain = "Business", role = "query")]),
        ];
        let parsed = parse_type_attrs(&attrs, proc_macro2::Span::call_site())
            .expect("kind and domain may be split across attributes");

        assert_eq!(parsed.kind, "struct");
        assert_eq!(parsed.domain, "Business");
        assert_eq!(parsed.role.as_deref(), Some("query"));
    }

    #[test]
    fn uninterpreted_field_flags_are_tolerated() {
        // `#[jumo(skip)]` is used by downstream models; the derive macro must
        // ignore flags it does not interpret rather than failing the build.
        let fields = named_fields(quote! {
            struct Sample {
                #[jumo(skip)]
                internal: String,
                #[jumo(unique)]
                id: String,
            }
        });

        assert_eq!(
            parse_field_unique_attrs(&fields),
            vec![Ident::new("id", proc_macro2::Span::call_site())]
        );
    }

    #[test]
    fn unique_requires_a_named_field() {
        let fields = named_fields(quote! { struct Sample(String, u32); });

        assert!(parse_field_unique_attrs(&fields).is_empty());
    }
}
