use nu_ansi_term::Color;
use pretty::Arena;
use pretty::Doc;
use pretty::DocAllocator;
use pretty::Pretty;
use pretty::RefDoc;
use termimad::MadSkin;

use crate::EnumVariantRepresentation;
use crate::TypeDescription;
use crate::TypeEnumKind;
use crate::TypeKind;

/// Get a [`RefDoc`](pretty::RefDoc) which can be converted into a String to print to a terminal
/// with colors
///
/// ## Example
///
/// ```rust,no_run
/// let input: type_description::TypeDescription = todo!();
/// let terminal_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
/// let arena = pretty::Arena::new();
///
/// let rendered_doc = type_description::render::render_to_terminal(&input, &arena);
///
/// let mut output = String::new();
/// rendered_doc.render_fmt(terminal_width, &mut output).unwrap();
/// println!("{}", output);
/// ```
pub fn render_to_terminal<'a>(desc: &'a TypeDescription, arena: &'a Arena<'a>) -> RefDoc<'a> {
    let mut doc = arena.nil();

    if !matches!(desc.kind(), TypeKind::Wrapped(_)) && desc.doc().is_none() {
        doc = doc
            .append(Color::LightBlue.bold().paint(desc.name()).to_string())
            .append(arena.space())
            .append(match desc.kind() {
                TypeKind::Bool
                | TypeKind::Integer { .. }
                | TypeKind::Float { .. }
                | TypeKind::String
                | TypeKind::Wrapped(_)
                | TypeKind::Array(_)
                | TypeKind::HashMap { .. } => arena.nil(),
                TypeKind::Struct(_) => {
                    arena.text(Color::Blue.dimmed().paint("[Table]").to_string())
                }
                TypeKind::Enum(_, _) => {
                    arena.text(Color::Green.dimmed().paint("[Enum]").to_string())
                }
            })
            .append(arena.hardline());
    }

    let skin = MadSkin::default_dark();
    let render_markdown = |text: &str| {
        let rendered = skin.text(text, None).to_string();
        arena.intersperse(
            rendered.split('\n').map(|t| {
                arena.intersperse(
                    t.split(char::is_whitespace).map(|t| t.to_string()),
                    arena.softline(),
                )
            }),
            arena.hardline(),
        )
    };

    if let Some(conf_doc) = desc.doc() {
        doc = doc.append(render_markdown(conf_doc));
    }

    match desc.kind() {
        TypeKind::Bool | TypeKind::Integer { .. } | TypeKind::Float { .. } | TypeKind::String => (),
        TypeKind::Struct(stc) => {
            doc = doc
                .append(arena.hardline())
                .append(Color::Blue.paint("[Members]").to_string())
                .append(arena.hardline())
                .append(arena.intersperse(
                    stc.iter().map(|ev| {
                        let member_name = ev.name();
                        let member_doc = ev.doc();
                        let member_conf = ev.kind();
                        let mut doc = arena.nil();

                        if let Some(member_doc) = member_doc {
                            doc = doc.append(render_markdown(member_doc));
                        }
                        doc.append(arena.text(Color::Blue.bold().paint(member_name).to_string()))
                            .append(": ")
                            .append(
                                Pretty::pretty(render_to_terminal(member_conf, arena), arena)
                                    .nest(4),
                            )
                    }),
                    Doc::hardline(),
                ))
        }
        TypeKind::Enum(enum_kind, variants) => {
            doc = doc
                .append(arena.hardline())
                .append(Color::Green.paint("One of:").to_string())
                .append(arena.space())
                .append(match enum_kind {
                    TypeEnumKind::Tagged(tag) => arena.text(
                        Color::White
                            .dimmed()
                            .paint(format!(
                                "[Tagged with {}]",
                                Color::LightGreen
                                    .italic()
                                    .dimmed()
                                    .paint(format!("'{}'", tag))
                            ))
                            .to_string(),
                    ),
                    TypeEnumKind::Untagged => {
                        arena.text(Color::White.dimmed().paint("[Untagged]").to_string())
                    }
                })
                .append(arena.hardline())
                .append(arena.intersperse(
                    variants.iter().map(|ev| {
                        let member_name = ev.name();
                        let member_doc = ev.doc();
                        let member_conf = ev.repr();
                        arena.text("-").append(arena.space()).append({
                            let mut doc = arena
                                .nil()
                                .append(match member_conf {
                                    EnumVariantRepresentation::String(rep) => arena.text(
                                        Color::Green
                                            .bold()
                                            .paint(&format!("{:?}", rep.to_lowercase()))
                                            .to_string(),
                                    ),
                                    EnumVariantRepresentation::Wrapped(_) => arena
                                        .text(Color::Green.bold().paint(member_name).to_string()),
                                })
                                .append(": ");

                            if let Some(member_doc) = member_doc {
                                doc = doc.append(render_markdown(member_doc));
                            }

                            doc.append(
                                Pretty::pretty(
                                    match member_conf {
                                        EnumVariantRepresentation::String(_) => {
                                            arena.nil().into_doc()
                                        }

                                        EnumVariantRepresentation::Wrapped(member_conf) => arena
                                            .text(Color::LightRed.paint("Is a: ").to_string())
                                            .append(render_to_terminal(member_conf, arena))
                                            .into_doc(),
                                    },
                                    arena,
                                )
                                .nest(4),
                            )
                            .nest(2)
                        })
                    }),
                    Doc::hardline(),
                ));
        }
        TypeKind::Array(conf) => {
            doc = doc
                .append(Color::LightRed.paint("Many of:").to_string())
                .append(arena.space())
                .append(render_to_terminal(conf, arena));
        }
        TypeKind::HashMap { key, value } => {
            doc = doc
                .append(Color::LightRed.paint("Hashmap of").to_string())
                .append(arena.space())
                .append(key.name())
                .append(arena.space())
                .append(Color::LightRed.paint("to").to_string())
                .append(arena.space())
                .append(value.name());
        }
        TypeKind::Wrapped(conf) => {
            doc = doc.append(render_to_terminal(conf, arena));
        }
    };

    doc.into_doc()
}
