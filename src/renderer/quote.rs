//! module which does rendering of quotes
use crate::renderer::paragraph;
use handlebars::Handlebars;
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Serialize)]
struct Quote {
    title: String,
    level: u16,
    content: Vec<String>,
}

pub fn render_quote(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    write_to: &mut String,
    hbr: &Handlebars,
) -> miette::Result<()> {
    todo!()
}
