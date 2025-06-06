use serde::Serialize;
use tracing::{debug, instrument, warn};

#[derive(Debug, Serialize)]
struct Code<'a> {
    language: &'a str,
    content: &'a str,
}
#[instrument(skip(content, dbuilder))]
pub fn render_code<'n, 'd>(
    _name: &'n [String],
    params: Vec<String>,
    content: String,
    dbuilder: &'d mut html::text_content::builders::DivisionBuilder,
) -> &'d mut html::text_content::builders::DivisionBuilder {
    let language = params.first().map_or("text", |l| l.as_str());
    debug!("found language: {language}");
    dbuilder.preformatted_text(|pt_builder| {
        pt_builder
            .code(|code_builder| code_builder.text(content))
            .class(format!("language-{language}"))
    })
}
