//! basic markup tokens rendering

use serde::Serialize;
use tracing::{instrument, trace, warn};

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

#[derive(Serialize)]
struct Spoiler {
    text: String,
}

#[derive(Serialize)]
struct InlineCode {
    text: String,
}

#[derive(Serialize)]
struct SuperScript {
    text: String,
}

#[derive(Serialize)]
struct SubScript {
    text: String,
}

#[derive(Serialize)]
struct Math {
    text: String,
}

#[derive(Serialize)]
struct Variable {
    text: String,
}

#[derive(Serialize)]
struct Comment {
    text: String,
}

pub fn render_attached(
    modifier: Modifier,
    content: &[norg::ParagraphSegment],
    write_to: &mut String,
    hbr: &handlebars::Handlebars,
) {
    // render segments first
    let mut segments_collector = String::new();
    content.iter().for_each(|segment| {
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
            let italic = Italic {
                text: segments_collector,
            };
            hbr.render("italic", &italic)
                .expect("Couldn't render italic text")
        }
        '-' => {
            trace!("rendering striked text");
            let striked = Strike {
                text: segments_collector,
            };
            hbr.render("strike", &striked)
                .expect("Couldn't render striked text")
        }
        '_' => {
            trace!("rendering underlined text");
            let underline = Underline {
                text: segments_collector,
            };
            hbr.render("underline", &underline)
                .expect("Couldn't render underlined text")
        }
        '!' => {
            trace!("rendering underlined text");
            let spoiler = Spoiler {
                text: segments_collector,
            };
            hbr.render("spoiler", &spoiler)
                .expect("Couldn't render spoiler text")
        }
        '`' => {
            trace!("rendering inline code");
            let inline = InlineCode {
                text: segments_collector,
            };
            hbr.render("inline-code", &inline)
                .expect("Couldn't render inline code")
        }
        '^' => {
            trace!("rendering superscript");
            let superscript = SuperScript {
                text: segments_collector,
            };
            hbr.render("superscript", &superscript)
                .expect("Couldn't render superscript")
        }
        ',' => {
            trace!("rendering subscript");
            let subscript = SubScript {
                text: segments_collector,
            };
            hbr.render("subscript", &subscript)
                .expect("Couldn't render subscript")
        }
        '$' => {
            trace!("rendering math equation");
            let math = Math {
                text: segments_collector,
            };
            hbr.render("math", &math)
                .expect("Couldn't render math equation")
        }
        '&' => {
            trace!("rendering variable");
            let variable = Variable {
                text: segments_collector,
            };
            hbr.render("variable", &variable)
                .expect("Couldn't render variable text")
        }
        '%' => {
            trace!("rendering commented text");
            let comment = Comment {
                text: segments_collector,
            };
            hbr.render("comment", &comment)
                .expect("Couldn't render comment text")
        }

        _ => {
            warn!("unknown modifier");
            return;
        }
    };
    write_to.push_str(&rendered_string)
}
