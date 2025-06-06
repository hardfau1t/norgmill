//! show page document in the top of the page

use tracing::{instrument, trace, warn};

fn parse_document<'a, 'd>(
    s: &'a str,
    dbuilder: &'d mut html::text_content::builders::DivisionBuilder,
) -> &'d mut html::text_content::builders::DivisionBuilder {
    for line in s.lines() {
        if let Some((key, value)) = line.trim().split_once(':') {
            match key.trim() {
                "title" => {
                    trace!(value, "title of the document");
                    dbuilder.division(|db| db.text(format!("Title: {}", value.trim())));
                }
                "description" => {
                    trace!(value, "description of the document");
                    dbuilder.division(|db| db.text(format!("Description: {}", value.trim())));
                }
                "authors" => {
                    trace!(value, "authors of the document");
                    dbuilder.division(|db| db.text(format!("👤 {}", value.trim())));
                }
                "categories" => {
                    trace!(value, "categories of the document");
                    dbuilder.division(|db| db.text(format!("📂 {}", value.trim())));
                }
                "created" => {
                    trace!(value, "created of the document");
                    dbuilder.division(|db| db.text(format!("🕒 {}", value.trim())));
                }
                "updated" => {
                    trace!(value, "updated of the document");
                    dbuilder.division(|db| db.text(format!("Last Updated {}", value.trim())));
                }
                "version" => {
                    trace!(value, "version of the document");
                    dbuilder.division(|db| db.text(format!("📌 {}", value.trim())));
                }
                _ => {
                    warn!(key, "Unknown key while parsing document meta, ignoring");
                }
            }
        } else {
            warn!(line, "Ignoring the line, since there is no `:`");
        }
    }
    dbuilder
}

#[instrument(skip(content, dbuilder))]
pub fn render_document<'n, 'd>(
    name: &'n [String],
    params: Vec<String>,
    content: String,
    dbuilder: &'d mut html::text_content::builders::DivisionBuilder,
) -> &'d mut html::text_content::builders::DivisionBuilder {
    match name.first() {
        Some(s) if s == "meta" => {
            if !params.is_empty() {
                warn!("not sure what to do with params")
            };
            dbuilder.division(|meta_builder| {
                meta_builder.class("metadata");
                parse_document(&content, meta_builder)
            });
        }
        None => {
            warn!(content, "Missing document type, skipping");
        }
        Some(document_type) => {
            warn!(document_type, "Unknown document type, skipping");
        }
    }
    dbuilder
}
