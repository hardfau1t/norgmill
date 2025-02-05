use std::backtrace::Backtrace;

use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, warn};

mod basic;
mod definition;
mod footnote;
mod heading;
mod html_list;
mod link;
mod paragraph;
mod quote;
mod table;
mod verbatim;

#[derive(Debug, Default)]
struct NorgContext {
    /// active ordered/unordered entries list
    active_html_list: Option<html_list::HtmlList>,
}
impl NorgContext {
    fn flush(&mut self, write_to: &mut String, hbr: &Handlebars) -> miette::Result<()> {
        if let Some(html_list) = self.active_html_list.take() {
            html_list.render(write_to, hbr)?
        };
        Ok(())
    }
}

impl Drop for NorgContext {
    fn drop(&mut self) {
        if let Some(active_list) = self.active_html_list.take() {
            warn!("Dropping html list items: {active_list:?}");
            debug!("Backtrace: {}", Backtrace::capture());
        }
    }
}

pub fn parse_and_render_body<'h>(input: &str, hbr: &Handlebars<'h>) -> miette::Result<String> {
    let tokens = norg::parse_tree(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("found tokens: {tokens:#?}");

    let mut context = NorgContext::default();

    let mut rendered_string = tokens
        .into_iter()
        .map(|ast| render_ast(ast, &mut context, hbr))
        .collect::<Result<_, _>>()?;
    context.flush(&mut rendered_string, hbr)?;
    // cleanup context if there are any pending items
    Ok(rendered_string)
}

pub async fn dump_ast(path: std::path::PathBuf) -> miette::Result<()> {
    let input = tokio::fs::read_to_string(&path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Couldn't read {path:?}"))?;
    let tokens = norg::parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    println!("{tokens:#?}");
    Ok(())
}

#[derive(Serialize, Debug)]
struct Para {
    para: String,
}

fn render_ast(
    ast: norg::NorgAST,
    context: &mut NorgContext,
    hbr: &Handlebars,
) -> miette::Result<String> {
    let mut rendered_string = String::new();

    // first check if there are any liste items from the previous step are present
    if let Some(html_list) = context
        .active_html_list
        .take_if(|list| !list.expected(&ast))
    {
        // there exists a html list but current item is not part of that list so render that item
        html_list.render(&mut rendered_string, hbr)?
    };

    match ast {
        norg::NorgAST::Paragraph(p) => {
            let mut para = String::new();
            p.iter()
                .map(|segment| paragraph::render_paragraph(segment, &mut para, hbr))
                .collect::<Result<(), _>>()
                .into_diagnostic()
                .wrap_err("Failed to construct paragraph")?;
            let para = Para { para };
            let rendered_para = hbr
                .render("paragraph", &para)
                .into_diagnostic()
                .wrap_err("Failed to render paragraph")?;
            rendered_string.push_str(&rendered_para);
        }
        norg::NorgAST::NestableDetachedModifier {
            modifier_type,
            level,
            extensions,
            text,
            content,
        } => match modifier_type {
            norg::NestableDetachedModifier::Quote => {
                quote::render_quote(level, extensions, text, content, &mut rendered_string, hbr)
            }
            // no need to check if the item is of different type, if it is then it will be flushed at the beginning of the loop
            _ => context
                .active_html_list
                .get_or_insert(html_list::HtmlList::new(level, modifier_type))
                .push(text, content, extensions, hbr),
        }?,

        norg::NorgAST::RangeableDetachedModifier {
            modifier_type,
            title,
            extensions,
            content,
        } => {
            let output_string = match modifier_type {
                norg::RangeableDetachedModifier::Definition => {
                    definition::render_definition(title, extensions, content, hbr)?
                }
                norg::RangeableDetachedModifier::Footnote => {
                    footnote::render_footnote(title, extensions, content, hbr)?
                }
                norg::RangeableDetachedModifier::Table => {
                    table::render_table(title, extensions, content, hbr)?
                }
            };
            rendered_string.push_str(&output_string);
        }
        norg::NorgAST::Heading {
            level,
            title,
            extensions,
            content,
        } => {
            let mut heading_context = NorgContext::default();
            let mut heading_content = content.into_iter().try_fold(
                String::new(),
                |mut acc, content_ast| -> miette::Result<String> {
                    let rendered_item = render_ast(content_ast, &mut heading_context, hbr)?;
                    acc.push_str(&rendered_item);
                    Ok(acc)
                },
            )?;
            heading_context.flush(&mut heading_content, hbr)?;
            heading::render_heading(
                level,
                title,
                extensions,
                heading_content,
                &mut rendered_string,
                hbr,
            )
            .into_diagnostic()
            .wrap_err("Failed to construct paragraph")?;
        }
        //norg::NorgAST::CarryoverTag { tag_type, name, parameters, next_object } => todo!(),
        norg::NorgAST::VerbatimRangedTag {
            name,
            parameters,
            content,
        } => {
            let tag_content = verbatim::render_paragraph(name, parameters, content, hbr)?;
            rendered_string.push_str(&tag_content);
        }
        //norg::NorgAST::RangedTag { name, parameters, content } => todo!(),
        //norg::NorgAST::InfirmTag { name, parameters } => todo!(),
        _ => {
            warn!("Rendering is not implemented for {ast:?} item");
        }
    };
    Ok(rendered_string)
}

/// this currently used only in html list and definitions items so some of the items may not work
fn render_flat_ast(ast: &norg::NorgASTFlat, hbr: &Handlebars) -> miette::Result<String> {
    match ast {
        norg::NorgASTFlat::Paragraph(paras) => {
            let list_item = paras.into_iter().try_fold(
                String::new(),
                |mut acc, para| -> miette::Result<String> {
                    paragraph::render_paragraph(para, &mut acc, &hbr)
                        .into_diagnostic()
                        .wrap_err("Couldn't render list item")?;
                    miette::Result::Ok(acc)
                },
            )?;
            Ok(list_item)
        }
        _ => {
            warn!("Current this item is not supported in list items");
            miette::bail!("Unsupported item in list text")
        }
    }
}

/// register all the helpers from submodule
pub fn registser_helpers(hbr: &mut handlebars::Handlebars) {
    heading::registser_helpers(hbr);
}

#[cfg(test)]
mod tests {
    use super::parse_and_render_body;

    #[test]
    fn link_to_norg_file() {
        let content = "{:abc/def:}[link to def]";
        let expected = "<p><a href=\"abc/def.norg\">link to def</a></p>";
        let mut hbr = handlebars::Handlebars::new();
        let load_options = handlebars::DirectorySourceOptions::default();
        hbr.register_templates_directory("./templates", load_options)
            .expect("couldn't load handlebars");
        assert_eq!(
            parse_and_render_body(content, &hbr).expect("couldn't parse content"),
            expected,
            "html with a paragraph pointing to another norg file"
        );
    }
}
