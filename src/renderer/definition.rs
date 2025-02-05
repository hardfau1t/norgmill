use super::paragraph;
use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Debug, Serialize)]
struct Definition<'a> {
    title: &'a str,
    content: &'a str,
}

#[instrument(skip(extensions, content, hbr))]
pub fn render_definition(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering footnote");
    debug!("content of definition: {content:?} and extension: {extensions:?}");
    if !extensions.is_empty() {
        warn!(extensions=?extensions, "extensions are not supported for definition" );
    }

    let title_text = title.iter().try_fold(
        String::new(),
        |mut acc, segment| -> miette::Result<String> {
            paragraph::render_paragraph(segment, &mut acc, hbr)
                .into_diagnostic()
                .wrap_err("Couldn't render definition title")?;
            Ok(acc)
        },
    )?;
    let definition_content = content.into_iter().try_fold(
        String::new(),
        |mut acc, content_ast| -> miette::Result<String> {
            let rendered_item = super::render_flat_ast(&content_ast, hbr)?;
            acc.push_str(&rendered_item);
            Ok(acc)
        },
    )?;
    let definition = Definition {
        title: &title_text,
        content: &definition_content,
    };
    let rendered_string = hbr
        .render("definition", &definition)
        .into_diagnostic()
        .wrap_err("Couldn't render definition")?;
    Ok(rendered_string)
}
