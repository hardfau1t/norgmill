use super::paragraph;
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(extensions, content, dl_builder))]
pub fn render_definition<'n, 'd>(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    dl_builder: &'d mut html::text_content::builders::DescriptionListBuilder,
) -> &'d mut html::text_content::builders::DescriptionListBuilder {
    // FIX: this renders headings in separate lines for separate words
    trace!("rendering description list");
    if !extensions.is_empty() {
        warn!(extensions=?extensions, "extensions are not supported for definition" );
    }

    dl_builder.description_term(|dt_builder| {
        debug!(
            num_title_segments = title.len(),
            "Rendering definition term"
        );
        dt_builder.push(paragraph::render_paragraph_to_string(&title))
    });

    dl_builder.description_details(|dd_builder| {
        content.into_iter().for_each(|cont_ast| {
            dd_builder.push(super::render_flat_ast(&cont_ast));
        });
        dd_builder
    });

    debug!("Finished rendering definition term and details");
    dl_builder
}
