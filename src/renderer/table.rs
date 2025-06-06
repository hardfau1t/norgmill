use tracing::{debug, instrument, trace, warn};

#[instrument(skip(content, extensions, tbl_builder))]
pub fn render_table(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    tbl_builder: &mut html::tables::builders::TableBuilder,
) -> &mut html::tables::builders::TableBuilder {
    trace!("rendering table");
    warn!("rendering table is not implemented");
    debug!("content of table: {content:?} and extension: {extensions:?}");
    tbl_builder
}
