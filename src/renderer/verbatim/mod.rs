use tracing::{instrument, trace, warn};

mod code;
mod document;

#[instrument(skip(params, content, target))]
pub fn render_paragraph(
    name: Vec<String>,
    params: Vec<String>,
    content: String,
    target: &mut String,
) {
    if let Some((first_name, name_etc)) = name.split_first() {
        match first_name.as_str() {
            "code" => {
                trace!("rendering code block");
                code::render_code(name_etc, params, content, target);
            }
            "document" => {
                trace!("rendering document");
                document::render_document(name_etc, params, content, target);
            }
            _ => {
                warn!("unknown tag: {first_name}, so just pushing the content as it is");
            }
        }
    } else {
        warn!("tag without name found don't know what to do so just pushing the content as it is");
    };
}
