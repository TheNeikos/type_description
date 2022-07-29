//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use indexmap::IndexSet;

use crate::{TypeDescription, TypeEnumKind};

fn get_list_of_types(desc: &TypeDescription) -> IndexSet<&TypeDescription> {
    let mut types = IndexSet::new();

    let mut remaining = vec![desc];

    while let Some(cur) = remaining.pop() {
        types.insert(cur);

        match cur.kind() {
            crate::TypeKind::Bool
            | crate::TypeKind::Integer { .. }
            | crate::TypeKind::Float { .. }
            | crate::TypeKind::Enum(_, _)
            | crate::TypeKind::String => (),
            crate::TypeKind::Wrapped(wrapped) => remaining.push(wrapped),
            crate::TypeKind::Array(arr) => remaining.push(arr),
            crate::TypeKind::HashMap { key, value } => {
                remaining.push(value);
                remaining.push(key);
            }
            crate::TypeKind::Struct(strt) => {
                remaining.extend(
                    strt.iter()
                        .rev()
                        .map(|(_, _, td): &(&str, Option<&str>, TypeDescription)| td),
                );
            }
        }
    }

    types
}

/// Render a [`struct@TypeDescription`] to a Markdown string
///
///
/// # Example
///
/// ```rust
///     use type_description::AsTypeDescription;
///     use type_description::render::render_to_markdown;
///
///     let ty_desc = std::collections::HashMap::<String, Vec<String>>::as_type_description();
///     let markdown = render_to_markdown(&ty_desc).unwrap();
///
///     println!("{markdown}");
/// ```
///
/// Renders:
///
/// # Table of 'String => Array of 'String's'
///
///
/// _Key: String, Values: Array of 'String's_
///
/// # String
///
/// An UTF-8 string
///
/// # Array of 'String's
///
///
/// _Array Elements of String_
pub fn render_to_markdown(desc: &TypeDescription) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let list_of_types = get_list_of_types(desc);

    let mut markdown = String::new();

    for ty in list_of_types {
        writeln!(markdown, "# {}", ty.name())?;
        writeln!(markdown)?;
        if let Some(doc) = ty.doc() {
            writeln!(markdown, "{}", doc)?;
        }
        match ty.kind() {
            crate::TypeKind::Bool
            | crate::TypeKind::Integer { .. }
            | crate::TypeKind::Float { .. }
            | crate::TypeKind::String => (),
            crate::TypeKind::Wrapped(wrapped_ty) => {
                writeln!(markdown)?;
                writeln!(markdown, "_Represented by {}_", wrapped_ty.name())?;
            }
            crate::TypeKind::Array(arr_ty) => {
                writeln!(markdown)?;
                writeln!(markdown, "_Array Elements of {}_", arr_ty.name())?;
            }
            crate::TypeKind::HashMap {
                key: key_ty,
                value: value_ty,
            } => {
                writeln!(markdown)?;
                writeln!(
                    markdown,
                    "_Key: {}, Values: {}_",
                    key_ty.name(),
                    value_ty.name()
                )?;
            }
            crate::TypeKind::Struct(strct) => {
                writeln!(markdown)?;
                writeln!(markdown, "**Fields:**")?;

                for (field_name, field_doc, field_ty) in strct {
                    writeln!(
                        markdown,
                        "- `{field_name}` ({}): {}",
                        field_ty.name(),
                        field_doc.unwrap_or("_No doc_")
                    )?;
                }
            }
            crate::TypeKind::Enum(tag_kind, variants) => {
                write!(markdown, "**Variants:** ")?;
                if let TypeEnumKind::Tagged(tag) = tag_kind {
                    writeln!(markdown, "(Tagged in field `{tag}`)")?;
                } else {
                    writeln!(markdown, "Untagged")?;
                };

                for (variant_name, variant_doc, variant_ty) in variants {
                    match variant_ty {
                        crate::EnumVariantRepresentation::String(_) => {
                            writeln!(
                                markdown,
                                "- `{variant_name}`: {}",
                                variant_doc.unwrap_or("")
                            )?;
                        }
                        crate::EnumVariantRepresentation::Wrapped(wrapped_ty) => {
                            writeln!(markdown, "- `{variant_name}` (): {}", wrapped_ty.name())?;
                        }
                    }
                }
            }
        }
        writeln!(markdown)?;
    }

    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use crate::AsTypeDescription;

    use super::render_to_markdown;

    #[test]
    fn render_simple() {
        let ty_desc = std::collections::HashMap::<String, Vec<String>>::as_type_description();
        let markdown = render_to_markdown(&ty_desc).unwrap();

        println!("{markdown}");
    }
}

// #[derive(TypeDescription)]
// /// Some docs
// struct Config {
//     /// The bind address
//     addr: std::net::SocketAddr,
//
//     /// The Port
//     port: Vec<Port>,
// }
//
// ->
//
// # Config
// Some docs
//
// ## Fields
//
// - `addr` ([SocketAddr]):  The bind address
// - `port` ([Port]): The port
//
//
// # Array of Port's
// A port is a number from 0-65565
//
// _Represented as [u16]_
