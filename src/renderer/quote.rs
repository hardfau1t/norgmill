//! module which does rendering of quotes
use crate::renderer::paragraph;
use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Serialize)]
struct Quote {
    content: String,
    level: u16,
    children: Vec<String>,
}

#[instrument(skip(extensions, inner_content, hbr))]
pub fn render_quote(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    hbr: &Handlebars,
) -> miette::Result<String> {
    debug!("Writing Quote with extensions:{extensions:?}, text: {text:?}, inner_content: {inner_content:?}");
    if !extensions.is_empty() {
        warn!("Quote has extensions which is not supposed be, if things have changed, then raise issue to fix this");
    }
    let rendered_quote_text = super::render_flat_ast(&text, hbr)?;
    let children = inner_content
        .into_iter()
        .filter_map(|inner| {
            let mut context = super::NorgContext::default();
            match super::render_ast(inner, &mut context, hbr) {
                Ok(rendered_inner) => Some(rendered_inner),
                Err(e) => {
                    warn!("Couldn't render quotes inner ast: {e:?}");
                    None
                }
            }
        })
        .collect();
    let quote = Quote {
        content: rendered_quote_text,
        level,
        children,
    };
    hbr.render("quote", &quote)
        .into_diagnostic()
        .wrap_err("Couldn't render quote")
}
