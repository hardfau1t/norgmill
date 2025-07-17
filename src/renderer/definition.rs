use super::paragraph;
use std::fmt::Write;
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(extensions, content, output))]
pub fn render_definition(
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: Vec<norg::NorgASTFlat>,
    output: &mut String,
) {
    // FIX: this renders headings in separate lines for separate words
    trace!("rendering description list");
    if !extensions.is_empty() {
        warn!(extensions=?extensions, "extensions are not supported for definition" );
    }

    write!(output, "<dt>");
    debug!(
        num_title_segments = title.len(),
        "Rendering definition term"
    );
    for segment in &title {
        paragraph::render_segment(segment, output);
    }
    write!(output, "</dt>");

    write!(output, "<dd>");
    for cont_ast in content {
        match cont_ast {
            norg::NorgASTFlat::Paragraph(paras) => paras
                .iter()
                .for_each(|seg| paragraph::render_segment(seg, output)),
            _ => {
                warn!(?cont_ast, "Unsupported token found inside definition");
            }
        }
    }
    write!(output, "</dd>");

    debug!("Finished rendering definition term and details");
}
