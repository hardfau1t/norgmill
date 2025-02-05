use handlebars::Handlebars;
use tracing::{debug, instrument, warn, trace};

#[instrument(skip(content, extensions, hbr))]
pub fn render_table(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering table");
    warn!("rendering table is not implemented");
    debug!("content of table: {content:?} and extension: {extensions:?}");
    miette::bail!("Not yet implemented")
}
