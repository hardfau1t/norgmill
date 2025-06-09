//! module which does rendering of headings
use crate::renderer::paragraph;
use tracing::{debug, instrument, trace};

#[instrument(skip(div_builder, content))]
pub fn render_heading<'f, 'd>(
    level: u16,
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgAST>,
    footnotes: &'f mut Vec<(
        Vec<norg::ParagraphSegment>,
        Vec<norg::DetachedModifierExtension>,
        Vec<norg::NorgASTFlat>,
    )>,
    div_builder: &'d mut html::text_content::builders::DivisionBuilder,
) -> &'d mut html::text_content::builders::DivisionBuilder {
    trace!("rendering heading");

    let title_text = paragraph::render_paragraph_to_string(&title);
    debug!(?title, "adding heading");
    // this needs to be applied first since modifiers which are applied at the end should not be applied to inner lists
    extensions.into_iter().for_each(|extension| {
        div_builder.span(|spb| super::extensions::apply_extension(extension, spb));
    });
    div_builder.division(|hdiv| match level {
        1 => hdiv.heading_1(|hb| hb.class("heading").text(title_text)),
        2 => hdiv.heading_2(|hb| hb.class("heading").text(title_text)),
        3 => hdiv.heading_3(|hb| hb.class("heading").text(title_text)),
        4 => hdiv.heading_4(|hb| hb.class("heading").text(title_text)),
        5 => hdiv.heading_5(|hb| hb.class("heading").text(title_text)),
        _ => hdiv.heading_6(|hb| hb.class("heading").text(title_text)),
    });

    if !content.is_empty() {
        div_builder.division(|cdiv| {
            cdiv.class("content-wrapper");
            let mut content_iter = content.into_iter().peekable();
            super::render_ast(&mut content_iter, footnotes, cdiv);
            cdiv
        });
    }
    div_builder
}
