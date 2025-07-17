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
    let mut segments_collector = String::new();
    content
        .iter()
        .try_for_each(|seg| paragraph::render_segment(seg, &mut segments_collector))?;


    // apply modifiers for rendered segments
    match modifier {
        '*' => {
            trace!("rendering bold text");
            write!(output, "<strong>{}</strong>", segments_collector)?;
        }
        '/' => {
            trace!("rendering italic text");
            write!(output, "<em>{}</em>", segments_collector)?;
        }
        '-' => {
            trace!("rendering striked text");
            write!(output, "<s>{}</s>", segments_collector)?;
        }
        '_' => {
            trace!("rendering underlined text");
            write!(output, "<u>{}</u>", segments_collector)?;
        }
        '!' => {
            trace!("rendering spoiler text");
            write!(
                output,
                "<span class=\"spoiler\">{}</span>",
                segments_collector
            )?;
        }
        '`' => {
            trace!("rendering inline code");
            write!(output, "<code>{}</code>", segments_collector)?;
        }
        '^' => {
            trace!("rendering superscript");
            write!(output, "<sup>{}</sup>", segments_collector)?;
        }
        ',' => {
            trace!("rendering subscript");
            write!(output, "<sub>{}</sub>", segments_collector)?;
        }
        '$' => {
            trace!("rendering math equation");
            write!(output, "<span class=\"math\">{}</span>", segments_collector)?;
        }
        '&' => {
            trace!("rendering variable");
            write!(
                output,
                "<span class=\"variable\">{}</span>",
                segments_collector
            )?;
        }
        '%' => {
            trace!("rendering commented text");
            write!(
                output,
                "<!-- {} -->",
                segments_collector
            )?;
        }

        _ => {
            warn!("unknown modifier");
        }
    };
    Ok(())
}
