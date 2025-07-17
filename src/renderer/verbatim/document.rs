//! show page document in the top of the page

use std::fmt::Write;
use tracing::{instrument, trace, warn};

fn parse_document(s: &str, output: &mut String) -> std::fmt::Result {
    for line in s.lines() {
        if let Some((key, value)) = line.trim().split_once(':') {
            match key.trim() {
                "title" => {
                    trace!(value, "title of the document");
                    write!(output, "<div>Title: {}</div>", value.trim())?;
                }
                "description" => {
                    trace!(value, "description of the document");
                    write!(output, "<div>Description: {}</div>", value.trim())?;
                }
                "authors" => {
                    trace!(value, "authors of the document");
                    write!(output, "<div>👤 {}</div>", value.trim())?;
                }
                "categories" => {
                    trace!(value, "categories of the document");
                    write!(output, "<div>📂 {}</div>", value.trim())?;
                }
                "created" => {
                    trace!(value, "created of the document");
                    write!(output, "<div>🕒 {}</div>", value.trim())?;
                }
                "updated" => {
                    trace!(value, "updated of the document");
                    write!(output, "<div>Last Updated {}</div>", value.trim())?;
                }
                "version" => {
                    trace!(value, "version of the document");
                    write!(output, "<div>📌 {}</div>", value.trim())?;
                }
                _ => {
                    warn!(key, "Unknown key while parsing document meta, ignoring");
                }
            }
        } else {
            warn!(line, "Ignoring the line, since there is no `:`");
        }
    }
    Ok(())
}

#[instrument(skip(content, output))]
pub fn render_document(
    name: &[String],
    params: Vec<String>,
    content: String,
    output: &mut String,
) -> std::fmt::Result {
    trace!("rendering document");
    match name.first() {
        Some(s) if s == "meta" => {
            if !params.is_empty() {
                warn!("not sure what to do with params")
            };
            write!(output, "<div class=\"metadata\">")?;
            parse_document(&content, output)?;
            write!(output, "</div>")?;
        }
        None => {
            warn!(content, "Missing document type, skipping");
        }
        Some(document_type) => {
            warn!(document_type, "Unknown document type, skipping");
        }
    }
    Ok(())
}
