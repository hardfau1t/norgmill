use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Debug, Serialize)]
struct Footnote<'a> {
    id: &'a str,
    content: &'a str,
}

#[instrument(skip(content, extensions, hbr))]
pub fn render_footnote(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    hbr: &Handlebars,
) -> miette::Result<String> {
    debug!("content of footnote: {content:?} and extension: {extensions:?}");
    if !extensions.is_empty() {
        warn!(
            ?extensions,
            "footnote doesn't have extensions in specifications, yet!!!"
        );
    }
    let footnote_content = content
        .into_iter()
        .map(|ast| super::render_flat_ast(&ast, hbr))
        .collect::<Result<String, _>>()
        .wrap_err("Couldn't render footnote content")?;
    let mut id = String::new();
    title
        .into_iter()
        .map(|p| super::paragraph::render_paragraph(&p, &mut id, hbr))
        .collect::<Result<(), _>>()
        .wrap_err("Couldn't render footnote id")?;
    let footnote = Footnote {
        id: &id,
        content: &footnote_content,
    };
    hbr.render("footnote", &footnote)
        .into_diagnostic()
        .wrap_err("Couldn't render footnote")
}
