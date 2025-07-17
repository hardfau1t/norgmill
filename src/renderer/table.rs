use tracing::{debug, instrument, trace, warn};

#[instrument(skip(content, extensions, output))]
pub fn render_table(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    output: &mut String,
) {
    warn!("rendering table is not implemented");
    debug!("content of table: {content:?} and extension: {extensions:?}");
}
