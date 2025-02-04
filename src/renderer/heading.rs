//! module which does rendering of headings
use crate::renderer::paragraph;
use handlebars::Handlebars;
use serde::Serialize;
use tracing::{instrument, trace, warn, debug};

#[derive(Serialize)]
struct Heading {
    title: String,
    level: u16,
}

#[instrument(skip(extensions, title, write_to, hbr))]
pub fn render_heading(
    level: u16,
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    write_to: &mut String,
    hbr: &Handlebars,
) -> std::fmt::Result {
    if !extensions.is_empty() {
        warn!("not rendering extensions: {extensions:?}, its not supported yet");
    }
    trace!("rendering heading");
    let title_text = title
        .into_iter()
        .try_fold(String::new(), |mut acc, segment| {
            paragraph::render_paragraph(segment, &mut acc, hbr)?;
            Ok(acc)
        })?;
    let heading = Heading {
        title: title_text,
        level,
    };
    let rendered_string = hbr
        .render("heading", &heading)
        .expect("Couldn't render heading");
    debug!("writing heading: {rendered_string}");
    write_to.push_str(&rendered_string);
    Ok(())
}
