//! module which does rendering of headings
use crate::renderer::paragraph;
use tracing::{debug, instrument, trace};

#[instrument(skip(content, footnotes))]
pub fn render_heading(
    level: u16,
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgAST>,
    footnotes: &mut Vec<(
        Vec<norg::ParagraphSegment>,
        Vec<norg::DetachedModifierExtension>,
        Vec<norg::NorgASTFlat>,
    )>,
) -> html::text_content::Division {
    trace!("rendering heading");

    let mut div_builder = html::text_content::Division::builder();
    div_builder.class("heading_block");

    let title_text = paragraph::render_paragraph_to_string(&title);
    debug!(?title, "adding heading");
    // this needs to be applied first since modifiers which are applied at the end should not be applied to inner lists
    extensions.into_iter().for_each(|extension| {
        div_builder.span(|spb| super::extensions::apply_extension(extension, spb));
    });
    let heading_class = format!("heading_{level}");
    match level {
        1 => div_builder.heading_1(|hb| hb.class(heading_class).text(title_text)),
        2 => div_builder.heading_2(|hb| hb.class(heading_class).text(title_text)),
        3 => div_builder.heading_3(|hb| hb.class(heading_class).text(title_text)),
        4 => div_builder.heading_4(|hb| hb.class(heading_class).text(title_text)),
        5 => div_builder.heading_5(|hb| hb.class(heading_class).text(title_text)),
        _ => div_builder.heading_6(|hb| hb.class(heading_class).text(title_text)),
    };

    if !content.is_empty() {
        let mut content_iter = content.into_iter().peekable();
        // this has to be converted to string.
        // when there are lot of nested divisions it causes stack overflow.
        // usually heading contents will have lots of nested content, so better to convert it to string and push
        // FIX: fix needs to be done in the html library, if this is not converted to string then it causes stack overflow
        div_builder.push(super::render_ast(&mut content_iter, footnotes).to_string());
    }
    div_builder.build()
}
