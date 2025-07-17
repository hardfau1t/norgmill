use tracing::{debug, instrument, warn};

#[instrument(skip(content, extensions, _output))]
pub fn render_table(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    _output: &mut String,
) {
    warn!("rendering table is not implemented");
    debug!("content of table: {content:?} and extension: {extensions:?}");
}
