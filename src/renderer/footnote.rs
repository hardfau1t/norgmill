use handlebars::Handlebars;
use tracing::{debug, instrument, warn, trace};

#[instrument(skip(content, extensions, hbr))]
pub fn render_footnote(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering footnote");
    warn!("rendering footnote is not implemented");
    debug!("content of footnote: {content:?} and extension: {extensions:?}");
    miette::bail!("Not yet implemented")
}
