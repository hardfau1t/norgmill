//! module which does rendering of headings
use crate::renderer::paragraph;
use handlebars::Handlebars;
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Serialize)]
struct Heading {
    title: String,
    level: u16,
    content: String,
}

pub fn render_heading(
    level: u16,
    title: Vec<norg::ParagraphSegment>,
    extensions: Vec<norg::DetachedModifierExtension>,
    content: String,
    write_to: &mut String,
    hbr: &Handlebars,
) -> std::fmt::Result {
    if !extensions.is_empty() {
        warn!("not rendering extensions: {extensions:?}, its not supported yet");
    }
    debug!("rendering heading: title {title:?}, with _content: {content:?}, extensions: {extensions:?}");
    let title_text = title
        .iter()
        .try_fold(String::new(), |mut acc, segment| {
            paragraph::render_paragraph(segment, &mut acc, hbr)?;
            Ok(acc)
        })?;
    let heading = Heading {
        title: title_text,
        level,
        content,
    };
    let rendered_string = hbr
        .render("heading", &heading)
        .expect("Couldn't render heading");
    debug!("writing heading: {rendered_string}");
    write_to.push_str(&rendered_string);
    Ok(())
}

/// helper for rendering heading level
fn heading_level_calculate(
    helper: &handlebars::Helper,
    _hbr: &handlebars::Handlebars,
    _context: &handlebars::Context,
    _rc: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    // get the indentation level
    let serde_json::Value::Number(indent_level) = helper.param(0).map(|i| i.value()).ok_or(
        handlebars::RenderErrorReason::ParamNotFoundForIndex("Couldn't get level", 0),
    )?
    else {
        // value is not integer
        return Err(handlebars::RenderErrorReason::InvalidParamType(
            "Expected integer indentation level",
        )
        .into());
    };
    let indent_level =
        indent_level
            .as_i64()
            .ok_or(handlebars::RenderErrorReason::InvalidParamType(
                "Too big to hold in integer",
            ))?;
    // heading should start with 1, but if it is not then
    let indent_level = indent_level.saturating_sub(1) * 2;
    out.write(&indent_level.to_string())?;
    Ok(())
}

/// register all the helper functions from this module
pub fn registser_helpers(hbr: &mut handlebars::Handlebars) {
    hbr.register_helper("heading_indent_level", Box::new(heading_level_calculate));
}
