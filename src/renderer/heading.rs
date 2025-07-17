//! module which does rendering of headings
use crate::renderer::paragraph;
use std::fmt::Write;
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(content, footnotes, output))]
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
    output: &mut String,
) {
    trace!("rendering heading");

    write!(output, "<div class=\"heading_block\">");

    let title_text = paragraph::render_segments(&title);
    debug!(?title, "adding heading");

    // Apply extensions first since modifiers which are applied at the end should not be applied to inner lists
    for extension in extensions {
        warn!(?extension, "unimplemented for extensions of headings");
    }

    let heading_class = format!("heading_{level}");
    let sanitized_title = crate::html::sanitize_html(&title_text);
    match level {
        1..5 => write!(
            output,
            "<h{level} class=\"{}\">{}</h{level}>",
            heading_class, sanitized_title
        ),
        _ => write!(
            output,
            "<h6 class=\"{}\">{}</h6>",
            heading_class, sanitized_title
        ),
    };

    if !content.is_empty() {
        let mut content_iter = content.into_iter().peekable();
        super::render_ast(&mut content_iter, footnotes, output);
    }

    write!(output, "</div>");
}
