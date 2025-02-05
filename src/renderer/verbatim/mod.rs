use handlebars::Handlebars;
use tracing::{trace, warn};

mod code;
mod document;

pub fn render_paragraph(
    name: Vec<String>,
    params: Vec<String>,
    content: String,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering verbatim tags: {name:?}");
    if let Some((first_name, name_etc)) = name.split_first() {
        match first_name.as_str() {
            "code" => code::render_code(name_etc, params, content, hbr),
            "document"=> document::render_document(name_etc, params, content, hbr),
            _ => {
                warn!("unknown tag: {first_name}, so just pushing the content as it is");
                Ok(content)
            }
        }
    } else {
        warn!("tag without name found don't know what to do so just pushing the content as it is");
        Ok(content)
    }
}
