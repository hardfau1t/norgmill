use std::iter::Peekable;

use tracing::{debug, error, trace, warn};

mod basic;
mod definition;
mod extensions;
mod heading;
mod link;
mod list;
mod paragraph;
mod quote;
mod table;
mod verbatim;

fn render_ast<'t, 'd, 's, Tokens>(
    tokens: &mut Peekable<Tokens>,
    footnotes: &mut Vec<(
        Vec<norg::ParagraphSegment>,
        Vec<norg::DetachedModifierExtension>,
        Vec<norg::NorgASTFlat>,
    )>,
) -> html::text_content::Division
where
    Tokens: Iterator<Item = norg::NorgAST>,
{
    trace!("rendering ast");
    let mut div_builder = html::text_content::Division::builder();
    div_builder.class("content_block");
    while let Some(token) = tokens.next() {
        match token {
            norg::NorgAST::Paragraph(p) => {
                div_builder.push(paragraph::render_paragraph(&p));
            }
            norg::NorgAST::NestableDetachedModifier {
                modifier_type,
                level,
                extensions,
                text,
                content,
            } => {
                match modifier_type {
                    norg::NestableDetachedModifier::Quote => {
                        div_builder.push(quote::render_quote(level, extensions, text, content));
                    }
                    norg::NestableDetachedModifier::UnorderedList => {
                        div_builder.unordered_list(|builder| {
                            list::render_unordered_list(level, extensions, text, content, builder);
                            // check if next tokens are also belongs to this list
                            while let Some(norg::NorgAST::NestableDetachedModifier {
                                level: n_level,
                                extensions: n_extensions,
                                text: n_text,
                                content: n_content,
                                ..
                            }) = tokens.next_if(|t| {
                                matches!(
                                    t,
                                    norg::NorgAST::NestableDetachedModifier {
                                        modifier_type:
                                            norg::NestableDetachedModifier::UnorderedList,
                                        ..
                                    }
                                )
                            }) {
                                list::render_unordered_list(
                                    n_level,
                                    n_extensions,
                                    n_text,
                                    n_content,
                                    builder,
                                );
                            }
                            builder
                        });
                    }
                    norg::NestableDetachedModifier::OrderedList => {
                        div_builder.ordered_list(|builder| {
                            list::render_ordered_list(level, extensions, text, content, builder);
                            // check if the next items are also part of list
                            while let Some(norg::NorgAST::NestableDetachedModifier {
                                level: n_level,
                                extensions: n_extensions,
                                text: n_text,
                                content: n_content,
                                ..
                            }) = tokens.next_if(|t| {
                                matches!(
                                    t,
                                    norg::NorgAST::NestableDetachedModifier {
                                        modifier_type: norg::NestableDetachedModifier::OrderedList,
                                        ..
                                    }
                                )
                            }) {
                                list::render_ordered_list(
                                    n_level,
                                    n_extensions,
                                    n_text,
                                    n_content,
                                    builder,
                                );
                            }
                            builder
                        });
                    } // no need to check if the item is of different type, if it is then it will be flushed at the beginning of the loop
                };
            }

            norg::NorgAST::RangeableDetachedModifier {
                modifier_type,
                title,
                extensions,
                content,
            } => {
                match modifier_type {
                    norg::RangeableDetachedModifier::Definition => {
                        let mut dl_builder = html::text_content::DescriptionList::builder();

                        definition::render_definition(title, extensions, content, &mut dl_builder);
                        // if there are more definitions then add it to the same list
                        // next_if cannot be replaced with next(), if you do that then when let fails to match that token is lost
                        while let Some(norg::NorgAST::RangeableDetachedModifier {
                            title,
                            extensions,
                            content,
                            ..
                        }) = tokens.next_if(|tkn| {
                            matches!(
                                tkn,
                                norg::NorgAST::RangeableDetachedModifier {
                                    modifier_type: norg::RangeableDetachedModifier::Definition,
                                    ..
                                }
                            )
                        }) {
                            definition::render_definition(
                                title,
                                extensions,
                                content,
                                &mut dl_builder,
                            );
                        }

                        div_builder.push(dl_builder.build());
                    }
                    norg::RangeableDetachedModifier::Footnote => {
                        footnotes.push((title, extensions, content));
                    }
                    norg::RangeableDetachedModifier::Table => {
                        div_builder
                            .table(|tbl_b| table::render_table(title, extensions, content, tbl_b));
                    }
                };
            }
            norg::NorgAST::Heading {
                level,
                title,
                extensions,
                content,
            } => {
                div_builder.push(heading::render_heading(
                    level, title, extensions, content, footnotes,
                ));
            }
            //norg::NorgAST::CarryoverTag { tag_type, name, parameters, next_object } => todo!(),
            norg::NorgAST::VerbatimRangedTag {
                name,
                parameters,
                content,
            } => {
                div_builder.division(|dbuilder| {
                    verbatim::render_paragraph(name, parameters, content, dbuilder)
                });
            }
            //norg::NorgAST::RangedTag { name, parameters, content } => todo!(),
            //norg::NorgAST::InfirmTag { name, parameters } => todo!(),
            _ => {
                warn!("Rendering is not implemented for {token:?} item");
            }
        };
    }
    div_builder.build()
}

pub fn parse_and_render_norg(input: &str) -> miette::Result<html::text_content::Division> {
    let mut div_builder = html::text_content::Division::builder();
    div_builder.class("norg_content");

    let tokens = norg::parse_tree(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("found tokens: {tokens:#?}");

    let mut footnotes = Vec::new();

    let mut token_iterator = tokens.into_iter().peekable();

    div_builder.push(render_ast(&mut token_iterator, &mut footnotes));

    if !footnotes.is_empty() {
        div_builder.footer(|fb| {
            fb.ordered_list(|ol_builder| {
                footnotes
                    .into_iter()
                    .for_each(|(title, _, foot_note_paras)| {
                        let title_string = paragraph::render_paragraph_to_string(&title);
                        ol_builder.list_item(|list_item_builder| {
                            list_item_builder
                                .id(format!("{title_string}_footnote"))
                                .division(|divb| {
                                    foot_note_paras.into_iter().for_each(|fnote| {
                                        divb.division(|pdiv| {
                                            pdiv.push(render_flat_ast(&fnote));
                                            pdiv
                                        });
                                    });
                                    divb
                                })
                                .anchor(|ab| {
                                    // TODO: create a backref for each footnote
                                    let backref_tag = format!("#{title_string}_footnote_backref");
                                    ab.href(backref_tag.clone())
                                        .aria_label(backref_tag)
                                        .text("↩")
                                })
                        });
                    });
                ol_builder
            })
        });
    }
    Ok(div_builder.build())
}

/// this currently used only in html list and definitions items so some of the items may not work
fn render_flat_ast(ast: &norg::NorgASTFlat) -> html::text_content::Division {
    trace!(?ast, "rendering flat ast");
    let mut dbuilder = html::text_content::Division::builder();
    match ast {
        norg::NorgASTFlat::Paragraph(paras) => {
            // Create a single paragraph for all content in list items
            dbuilder.push(paragraph::render_paragraph(paras));
        }
        _ => {
            error!(
                "Only paragraphs are supported in nested content, raise a issue if it is required"
            );
        }
    }
    dbuilder.build()
}
