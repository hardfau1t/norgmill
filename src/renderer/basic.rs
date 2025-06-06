//! basic markup tokens rendering

use tracing::{instrument, trace, warn};

use super::paragraph;

type Modifier = char;

#[instrument(skip(content, builder))]
pub fn render_attached(
    modifier: Modifier,
    content: &[norg::ParagraphSegment],
    builder: &mut html::text_content::builders::ParagraphBuilder,
) {
    // render segments first
    let segments_collector = paragraph::render_paragraph_to_string(content);

    // apply modifiers for rendered segments
    match modifier {
        '*' => {
            trace!("rendering bold text");
            builder.bold(|bb| bb.text(segments_collector));
        }
        '/' => {
            trace!("rendering italic text");
            builder.italic(|ib| ib.text(segments_collector));
        }
        '-' => {
            trace!("rendering striked text");
            builder.strike_through(|b| b.text(segments_collector));
        }
        '_' => {
            trace!("rendering underlined text");
            builder.underline(|b| b.text(segments_collector));
        }
        '!' => {
            trace!("rendering spoiler text");
            builder.span(|b| b.text(segments_collector).class("spoiler"));
        }
        '`' => {
            trace!("rendering inline code");
            builder.code(|b| b.text(segments_collector));
        }
        '^' => {
            trace!("rendering superscript");
            builder.super_script(|b| b.text(segments_collector));
        }
        ',' => {
            trace!("rendering subscript");
            builder.sub_script(|b| b.text(segments_collector));
        }
        '$' => {
            trace!("rendering math equation");
            builder.span(|b| b.text(segments_collector).class("math"));
        }
        '&' => {
            trace!("rendering variable");
            builder.span(|b| b.text(segments_collector).class("variable"));
        }
        '%' => {
            trace!("rendering commented text");
            builder.span(|b| b.text(segments_collector).class("comment"));
        }

        _ => {
            warn!("unknown modifier");
            return;
        }
    };
}
