//! basic markup tokens rendering

use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

use super::paragraph;

type Modifier = char;

#[derive(Serialize)]
struct Strike {
    text: String,
}
#[derive(Serialize)]
struct Underline {
    text: String,
}
#[derive(Serialize)]
struct Bold {
    text: String,
}

#[derive(Serialize)]
struct Italic {
    text: String,
}

#[instrument(skip(write_to, hbr, content))]
pub fn render_attached(
    modifier: Modifier,
    content: Vec<norg::ParagraphSegment>,
    write_to: &mut String,
    hbr: &handlebars::Handlebars,
) {
    // render segments first
    let mut segments_collector = String::new();
    content.into_iter().for_each(|segment| {
        paragraph::render_paragraph(segment, &mut segments_collector, hbr)
            .expect("Couldn't render string")
    });

    // apply modifiers for rendered segments
    let rendered_string = match modifier {
        '*' => {
            trace!("rendering bold text");
            let bold = Bold {
                text: segments_collector,
            };
            hbr.render("bold", &bold)
                .expect("Couldn't render bold text")
        }
        '/' => {
            trace!("rendering italic text");
            let bold = Italic {
                text: segments_collector,
            };
            hbr.render("italic", &bold)
                .expect("Couldn't render bold text")
        }
        '-' => {
            trace!("rendering striked text");
            let bold = Strike {
                text: segments_collector,
            };
            hbr.render("strike", &bold)
                .expect("Couldn't render bold text")
        }
        '_' => {
            trace!("rendering underlined text");
            let bold = Underline {
                text: segments_collector,
            };
            hbr.render("underline", &bold)
                .expect("Couldn't render bold text")
        }
        _ => {
            warn!("unknown modifier");
            return;
        }
    };
    write_to.push_str(&rendered_string)
}
