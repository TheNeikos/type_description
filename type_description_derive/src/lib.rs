//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error, OptionExt, ResultExt};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, Attribute, DeriveInput, Ident, Lit, LitStr, Meta, MetaNameValue, NestedMeta,
    Type,
};

#[derive(Debug)]
struct TypeField<'q> {
    ident: &'q Ident,
    ty: &'q Type,
    docs: Option<Vec<LitStr>>,
}

#[derive(Debug)]
enum TypeVariantKind<'q> {
    String(&'q Ident),
    Wrapped(&'q Ident, TypeField<'q>),
    Struct(&'q Ident, Vec<TypeField<'q>>),
}

#[derive(Debug)]
struct TypeVariant<'q> {
    kind: TypeVariantKind<'q>,
    docs: Option<Vec<LitStr>>,
}

#[derive(Debug)]
enum TypeEnumKind {
    Tagged(LitStr),
    Untagged,
}

#[derive(Debug)]
enum TypeQuoteKind<'q> {
    Wrapped(&'q Type),
    Struct(Vec<TypeField<'q>>),
    Enum(TypeEnumKind, Vec<TypeVariant<'q>>),
}

#[derive(Debug)]
struct TypeQuote<'q> {
    ident: &'q Ident,
    docs: Option<Vec<LitStr>>,
    kind: TypeQuoteKind<'q>,
}

fn lit_strings_to_string_quoted(docs: &Option<Vec<LitStr>>) -> TokenStream {
    if let Some(docs) = docs {
        let docs = docs
            .iter()
            .map(|litstr| litstr.value().trim().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        quote!(Some(#docs))
    } else {
        quote!(None)
    }
}

fn extract_docs_from_attributes<'a>(
    attrs: impl Iterator<Item = &'a Attribute>,
) -> Option<Vec<LitStr>> {
    let attrs = attrs
        .filter_map(|attr| {
            if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                if meta.path.is_ident("doc") {
                    if let Lit::Str(litstr) = meta.lit {
                        return Some(litstr);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    if attrs.is_empty() {
        None
    } else {
        Some(attrs)
    }
}

impl<'q> ToTokens for TypeQuote<'q> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident_name = self.ident.to_string();
        let outer_docs = lit_strings_to_string_quoted(&self.docs);

        tokens.append_all(match &self.kind {
            TypeQuoteKind::Wrapped(ty) => {
                quote! {
                    ::type_description::TypeDescription::new(
                        ::std::string::String::from(#ident_name),
                        ::type_description::TypeKind::Wrapped(
                            ::std::boxed::Box::new(<#ty as ::type_description::AsTypeDescription>::as_type_description())
                        ),
                        #outer_docs
                    )
                }
            }
            TypeQuoteKind::Struct(fields) => {
                let ident = fields.iter().map(|f| f.ident.to_string());
                let ty = fields.iter().map(|f| f.ty);
                let docs = fields.iter().map(|f| lit_strings_to_string_quoted(&f.docs));

                quote! {
                    ::type_description::TypeDescription::new(
                        ::std::string::String::from(#ident_name),
                        ::type_description::TypeKind::Struct(
                            vec![
                                #(
                                    ::type_description::StructField::new(#ident, #docs, <#ty as ::type_description::AsTypeDescription>::as_type_description())
                                ),*
                            ]
                        ),
                        #outer_docs
                    )
                }
            }
            TypeQuoteKind::Enum(kind, variants) => {
                let kind = match kind {
                    TypeEnumKind::Tagged(tag) => {
                        quote! {
                            ::type_description::TypeEnumKind::Tagged(::std::borrow::Cow::Borrowed(#tag))
                        }
                    }
                    TypeEnumKind::Untagged => {
                        quote! {
                            ::type_description::TypeEnumKind::Untagged
                        }
                    }
                };

                let variants = variants.iter().map(|var| {
                    let docs = lit_strings_to_string_quoted(&var.docs);
                    match &var.kind {
                        TypeVariantKind::Wrapped(ident, TypeField { ty, .. }) => {
                            // we ignore the above docs since the outer docs are the important ones
                            // TODO: Emit an error if an inner type in a enum is annotated
                            let ident = ident.to_string();
                            quote!{
                                ::type_description::EnumVariant::new(
                                    #ident,
                                    #docs,
                                    ::type_description::EnumVariantRepresentation::Wrapped(
                                        std::boxed::Box::new(::type_description::TypeDescription::new(
                                            ::std::string::String::from(#ident),
                                            ::type_description::TypeKind::Wrapped(
                                                std::boxed::Box::new(<#ty as ::type_description::AsTypeDescription>::as_type_description())
                                            ),
                                            None,
                                        ))
                                    )
                                )
                            }
                        }
                        TypeVariantKind::Struct(ident, fields) => {
                            let ident = ident.to_string();
                            let idents = fields.iter().map(|f| f.ident.to_string());
                            let field_docs = fields.iter().map(|f| lit_strings_to_string_quoted(&f.docs));
                            let tys = fields.iter().map(|f| f.ty);

                            quote! {
                                ::type_description::EnumVariant::new(
                                    #ident,
                                    #docs,
                                    ::type_description::EnumVariantRepresentation::Wrapped(
                                        std::boxed::Box::new(::type_description::TypeDescription::new(
                                            ::std::string::String::from(#ident),
                                            ::type_description::TypeKind::Struct(
                                                vec![
                                                    #(
                                                        (#idents, #field_docs, <#tys as ::type_description::AsTypeDescription>::as_type_description())
                                                     ),*
                                                ]
                                            ),
                                            None
                                        ))
                                    )
                                )
                            }
                        }
                        TypeVariantKind::String(ident) => {
                            let ident = ident.to_string();
                            quote!{
                                ::type_description::EnumVariant::new(
                                    #ident,
                                    #docs,
                                    ::type_description::EnumVariantRepresentation::String(
                                        ::std::borrow::Cow::Borrowed(#ident)
                                    )
                                )
                            }
                        }
                    }
                });

                quote! {
                    ::type_description::TypeDescription::new(
                        ::std::string::String::from(#ident_name),
                        ::type_description::TypeKind::Enum(
                            #kind,
                            vec![#(#variants),*]
                        ),
                        #outer_docs
                    )
                }
            }
        });
    }
}

#[proc_macro_derive(TypeDescription, attributes(description))]
#[proc_macro_error]
pub fn derive_type_description(input: TS) -> TS {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let type_desc_kind: TypeQuoteKind = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => TypeQuoteKind::Struct(
                fields
                    .named
                    .iter()
                    .map(|f| TypeField {
                        ident: f.ident.as_ref().unwrap(),
                        ty: &f.ty,
                        docs: extract_docs_from_attributes(f.attrs.iter()),
                    })
                    .collect(),
            ),
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    abort!(
                        fields,
                        "Tuple structs should only contain a single variant."
                    )
                }
                TypeQuoteKind::Wrapped(&fields.unnamed.first().unwrap().ty)
            }
            syn::Fields::Unit => abort!(
                ident,
                "Unit structs are not supported as they cannot be represented"
            ),
        },
        syn::Data::Enum(data) => {
            let enum_kind: TypeEnumKind = {
                let potential_kinds = input
                    .attrs
                    .iter()
                    .filter(|attr| attr.path.is_ident("description"))
                    .collect::<Vec<_>>();

                let error_no_kind = || abort!(ident, "Enums need to specify what kind of tagging they use"; help = "Use #[description(untagged)] for untagged enums, and #[description(tag = \"type\")] for internally tagged variants. Other kinds are not supported.");

                if potential_kinds.is_empty() {
                    error_no_kind()
                }

                let mut found_enum_kind = None;

                for potential_kind in potential_kinds {
                    match potential_kind
                        .parse_meta()
                        .expect_or_abort("Could not parse #[description] meta attribute.")
                    {
                        syn::Meta::List(kind) => {
                            if kind.nested.len() != 1 {
                                continue;
                            }

                            match kind.nested.first() {
                                Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(lit_str),
                                    ..
                                }))) => {
                                    if path.is_ident("tag") {
                                        found_enum_kind =
                                            Some(TypeEnumKind::Tagged(lit_str.clone()));
                                    }
                                }
                                Some(NestedMeta::Meta(Meta::Path(path))) => {
                                    if path.is_ident("untagged") {
                                        found_enum_kind = Some(TypeEnumKind::Untagged);
                                    }
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                        _ => continue,
                    }
                }

                if found_enum_kind.is_none() {
                    error_no_kind()
                }

                found_enum_kind.unwrap()
            };

            let variants = data
                .variants
                .iter()
                .map(|var| {
                    let kind = match &var.fields {
                        syn::Fields::Named(fields) => TypeVariantKind::Struct(
                            &var.ident,
                            fields
                                .named
                                .iter()
                                .map(|f| TypeField {
                                    ident: f.ident.as_ref().unwrap(),
                                    ty: &f.ty,
                                    docs: extract_docs_from_attributes(f.attrs.iter()),
                                })
                                .collect(),
                        ),
                        syn::Fields::Unnamed(fields) => {
                            if fields.unnamed.len() != 1 {
                                abort!(
                                    fields,
                                    "Tuple structs should only contain a single variant."
                                )
                            }
                            TypeVariantKind::Wrapped(
                                &var.ident,
                                TypeField {
                                    ident: &var.ident,
                                    ty: &fields.unnamed.first().unwrap().ty,
                                    docs: extract_docs_from_attributes(var.attrs.iter()),
                                },
                            )
                        }
                        syn::Fields::Unit => TypeVariantKind::String(&var.ident),
                    };
                    let docs = extract_docs_from_attributes(var.attrs.iter());
                    Some(TypeVariant { kind, docs })
                })
                .collect::<Option<_>>();

            TypeQuoteKind::Enum(
                enum_kind,
                variants.expect_or_abort("Enum contains invalid variants"),
            )
        }
        syn::Data::Union(_) => {
            abort!(
                ident,
                "Untagged unions are not supported. Consider using an enum instead."
            );
        }
    };

    let docs = extract_docs_from_attributes(input.attrs.iter());

    let type_desc = TypeQuote {
        kind: type_desc_kind,
        docs,
        ident,
    };

    let expanded = quote! {
        impl ::type_description::AsTypeDescription for #ident {
            fn as_type_description() -> ::type_description::TypeDescription {
                #type_desc
            }
        }
    };

    TS::from(expanded)
}
