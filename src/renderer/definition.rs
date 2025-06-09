
use super::paragraph;
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(extensions, content, dl_builder))]
pub fn render_definition<'n, 'd>(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    dl_builder: &'d mut html::text_content::builders::DescriptionListBuilder,
) -> &'d mut html::text_content::builders::DescriptionListBuilder {
    trace!("rendering description list");
    if !extensions.is_empty() {
        warn!(extensions=?extensions, "extensions are not supported for definition" );
    }

    dl_builder.description_term(|dt_builder| {
        debug!(num_title_segments = title.len(), "Rendering definition term");
        title.into_iter().for_each(|tseg| {
            dt_builder.paragraph(|pb| {
                paragraph::render_paragraph(&tseg, pb);
                pb
            });
        });
        dt_builder
    });

    dl_builder.description_details(|dd_builder| {
        content.into_iter().for_each(|cont_ast| {
            dd_builder.division(|dv_builder| super::render_flat_ast(&cont_ast, dv_builder));
        });
        dd_builder
    });

    debug!("Finished rendering definition term and details");
    dl_builder
}
