use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Debug, Serialize)]
struct Code<'a> {
    language: &'a str,
    content: &'a str,
}
#[instrument(skip(content, hbr))]
pub fn render_code(
    name: &[String],
    params: Vec<String>,
    content: String,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering code");
    debug!("code content: {content}");
    let language = params.first().map_or("text", |l| l.as_str());
    let code = Code {
        language,
        content: content.as_str(),
    };
    let rendered_string = hbr
        .render("code", &code)
        .into_diagnostic()
        .wrap_err("couldn't render code block")?;
    Ok(rendered_string)
}
