use std::fmt::Write;
use tracing::{debug, instrument, warn};

#[instrument(skip(content, target))]
pub fn render_code<'n, 'd>(
    _name: &'n [String],
    params: Vec<String>,
    content: String,
    target: &mut String,
)->std::fmt::Result {
    let language = params.first().map_or("text", |l| l.as_str());
    debug!("found language: {language}");
    write!(target, "<pre class={language}>{content}</pre>")
}
