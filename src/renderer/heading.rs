//! module which does rendering of headings
use handlebars::Handlebars;
use tracing::instrument;

#[instrument(skip(extensions, title, write_to, hbr))]
pub fn render_heading(
    level: u16,
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    write_to: &mut String,
    hbr: &Handlebars,
) -> std::fmt::Result {
    Ok(())
}
