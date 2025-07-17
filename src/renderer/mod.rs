use std::iter::Peekable;

use miette::{Context, IntoDiagnostic};
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
    output: &mut String,
) -> std::fmt::Result
where
    Tokens: Iterator<Item = norg::NorgAST>,
{
    trace!("rendering ast");
    output.push_str("<div class=\"content_block\">");
    while let Some(token) = tokens.next() {
        match token {
            norg::NorgAST::Paragraph(p) => {
                paragraph::render_paragraph(&p, output)?;
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
                        quote::render_quote(level, extensions, text, content, output)?;
                    }
                    norg::NestableDetachedModifier::UnorderedList => {
                        output.push_str("<ul>");
                        list::render_list_element(
                            text,
                            content,
                            level,
                            list::ListKind::Unordered,
                            extensions,
                            output,
                        )?;
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
                                    modifier_type: norg::NestableDetachedModifier::UnorderedList,
                                    ..
                                }
                            )
                        }) {
                            list::render_list_element(
                                n_text,
                                n_content,
                                n_level,
                                list::ListKind::Unordered,
                                n_extensions,
                                output,
                            )?;
                        }
                        output.push_str("</ul>");
                    }
                    norg::NestableDetachedModifier::OrderedList => {
                        output.push_str("<ol>");
                        list::render_list_element(
                            text,
                            content,
                            level,
                            list::ListKind::Ordered,
                            extensions,
                            output,
                        )?;
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
                            list::render_list_element(
                                n_text,
                                n_content,
                                n_level,
                                list::ListKind::Ordered,
                                n_extensions,
                                output,
                            )?;
                        }
                        output.push_str("</ul>");
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
                        output.push_str("<dl>");
                        definition::render_definition(title, extensions, content, output)?;
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
                            definition::render_definition(title, extensions, content, output)?;
                        }

                        output.push_str("</dl>");
                    }
                    norg::RangeableDetachedModifier::Footnote => {
                        footnotes.push((title, extensions, content));
                    }
                    norg::RangeableDetachedModifier::Table => {
                        table::render_table(title, extensions, content, output);
                    }
                };
            }
            norg::NorgAST::Heading {
                level,
                title,
                extensions,
                content,
            } => {
                heading::render_heading(level, title, extensions, content, footnotes, output)?;
            }
            //norg::NorgAST::CarryoverTag { tag_type, name, parameters, next_object } => todo!(),
            norg::NorgAST::VerbatimRangedTag {
                name,
                parameters,
                content,
            } => {
                // rendering code/document tag
                verbatim::render_paragraph(name, parameters, content, output)?;
            }
            //norg::NorgAST::RangedTag { name, parameters, content } => todo!(),
            //norg::NorgAST::InfirmTag { name, parameters } => todo!(),
            _ => {
                warn!("Rendering is not implemented for {token:?} item");
            }
        };
    }
    Ok(())
}

pub fn parse_and_render_norg(input: &str) -> miette::Result<String> {
    let tokens = norg::parse_tree(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("found tokens: {tokens:#?}");

    let mut footnotes = Vec::new();

    let mut token_iterator = tokens.into_iter().peekable();

    let mut output = String::with_capacity(input.len() * 2);
    output.push_str("<div class=norg_content>");

    render_ast(&mut token_iterator, &mut footnotes, &mut output)
        .into_diagnostic()
        .wrap_err("Rendering ast, with ignoring fmt errors")?;
    output.push_str("</div>");

    if !footnotes.is_empty() {
        output.push_str("<footer><ol>");
        footnotes
            .into_iter()
            .map(|(title, extensions, foot_note_paras)| {
                if !extensions.is_empty() {
                    warn!(?extensions, "extensions are not yet supported for footer");
                }
                let title_string = paragraph::render_segments(&title)?;
                output.push_str(&format!("<li id=\"{}_footnote\">", title_string));
                foot_note_paras
                    .into_iter()
                    .map(|fnote| render_flat_ast(&fnote, &mut output))
                    .collect::<std::fmt::Result>()?;
                // TODO: create a backref for each footnote
                let backref_tag = format!("#{}_footnote_backref", title_string);
                output.push_str(&format!(
                    "<a href=\"{}\" aria-label=\"{}\">↩</a>",
                    backref_tag, backref_tag
                ));
                output.push_str("</li>");
                Ok(())
            })
            .collect::<std::fmt::Result>()
            .into_diagnostic()
            .wrap_err("Couldn't add foooter")?;
        output.push_str("</ol></footer>")
    }
    Ok(output)
}

/// this currently used only in html list and definitions items so some of the items may not work
fn render_flat_ast(ast: &norg::NorgASTFlat, output: &mut String) -> std::fmt::Result {
    trace!(?ast, "rendering flat ast");
    match ast {
        norg::NorgASTFlat::Paragraph(paras) => {
            // Create a single paragraph for all content in list items
            paragraph::render_paragraph(paras, output)?;
        }
        _ => {
            error!(
                "Only paragraphs are supported in nested content, raise a issue if it is required"
            );
        }
    }
    Ok(())
}
