//! basic markup tokens rendering

use std::fmt::Write;
use tracing::{instrument, trace, warn};

use super::paragraph;

type Modifier = char;

#[instrument(skip(content, output))]
pub fn render_attached(
    modifier: Modifier,
    content: &[norg::ParagraphSegment],
    output: &mut String,
) -> std::fmt::Result {
    // render segments first
    let segments_collector = String::new();
    content
        .iter()
        .try_for_each(|seg| paragraph::render_segment(seg, output))?;

    // sanitize the content before writing to output
    let sanitized_content = crate::html::sanitize_html(&segments_collector);

    // apply modifiers for rendered segments
    match modifier {
        '*' => {
            trace!("rendering bold text");
            write!(output, "<strong>{}</strong>", sanitized_content)?;
        }
        '/' => {
            trace!("rendering italic text");
            write!(output, "<em>{}</em>", sanitized_content)?;
        }
        '-' => {
            trace!("rendering striked text");
            write!(output, "<s>{}</s>", sanitized_content)?;
        }
        '_' => {
            trace!("rendering underlined text");
            write!(output, "<u>{}</u>", sanitized_content)?;
        }
        '!' => {
            trace!("rendering spoiler text");
            write!(
                output,
                "<span class=\"spoiler\">{}</span>",
                sanitized_content
            )?;
        }
        '`' => {
            trace!("rendering inline code");
            write!(output, "<code>{}</code>", sanitized_content)?;
        }
        '^' => {
            trace!("rendering superscript");
            write!(output, "<sup>{}</sup>", sanitized_content)?;
        }
        ',' => {
            trace!("rendering subscript");
            write!(output, "<sub>{}</sub>", sanitized_content)?;
        }
        '$' => {
            trace!("rendering math equation");
            write!(output, "<span class=\"math\">{}</span>", sanitized_content)?;
        }
        '&' => {
            trace!("rendering variable");
            write!(
                output,
                "<span class=\"variable\">{}</span>",
                sanitized_content
            )?;
        }
        '%' => {
            trace!("rendering commented text");
            write!(
                output,
                "<span class=\"comment\">{}</span>",
                sanitized_content
            )?;
        }

        _ => {
            warn!("unknown modifier");
        }
    };
    Ok(())
}
