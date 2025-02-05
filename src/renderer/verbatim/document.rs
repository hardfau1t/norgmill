//! show page document in the top of the page

use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, trace, warn};

#[derive(Debug, Serialize)]
struct DocumentMeta<'a> {
    title: &'a str,
    description: &'a str,
    authors: &'a str,
    categories: &'a str,
    created: &'a str,
    updated: &'a str,
    version: &'a str,
}

impl<'a> DocumentMeta<'a> {
    fn from_str(s: &'a str) -> miette::Result<Self> {
        let mut title = None;
        let mut description = None;
        let mut authors = None;
        let mut categories = None;
        let mut created = None;
        let mut updated = None;
        let mut version = None;
        for line in s.lines() {
            if let Some((key, value)) = line.trim().split_once(':') {
                match key.trim() {
                    "title" => {
                        trace!(value, "title of the document");
                        title = Some(value.trim())
                    }
                    "description" => {
                        trace!(value, "description of the document");
                        description = Some(value.trim())
                    }
                    "authors" => {
                        trace!(value, "authors of the document");
                        authors = Some(value.trim())
                    }
                    "categories" => {
                        trace!(value, "categories of the document");
                        categories = Some(value.trim())
                    }
                    "created" => {
                        trace!(value, "created of the document");
                        created = Some(value.trim())
                    }
                    "updated" => {
                        trace!(value, "updated of the document");
                        updated = Some(value.trim())
                    }
                    "version" => {
                        trace!(value, "version of the document");
                        version = Some(value.trim())
                    }
                    _ => {
                        warn!(key, "Unknown key while parsing document meta, ignoring");
                    }
                }
            } else {
                warn!(line, "Ignoring the line, since there is no `:`");
            }
        }
        Ok(Self {
            title: title.ok_or(miette::miette!(
                "Couldn't find the title in the document meta"
            ))?,
            description: description.ok_or(miette::miette!(
                "Couldn't find the description in the document meta"
            ))?,
            authors: authors.ok_or(miette::miette!(
                "Couldn't find the authors in the document meta"
            ))?,
            categories: categories.ok_or(miette::miette!(
                "Couldn't find the categories in the document meta"
            ))?,
            created: created.ok_or(miette::miette!(
                "Couldn't find the created in the document meta"
            ))?,
            updated: updated.ok_or(miette::miette!(
                "Couldn't find the updated in the document meta"
            ))?,
            version: version.ok_or(miette::miette!(
                "Couldn't find the version in the document meta"
            ))?,
        })
    }
}

#[instrument(skip(content, hbr))]
pub fn render_document(
    name: &[String],
    params: Vec<String>,
    content: String,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("rendering document");
    debug!("document content: {content}");
    match name.first() {
        Some(s) if s == "meta" => {
            if !params.is_empty() {};
            let document = DocumentMeta::from_str(&content)?;
            let rendered_metadata = hbr
                .render("document_meta", &document)
                .into_diagnostic()
                .wrap_err("couldn't render code block")?;
            Ok(rendered_metadata)
        }
        None => {
            warn!("Missing document type, skipping");
            miette::bail!("missing document type");
        }
        Some(_) => {
            warn!("Unknown document type, skipping");
            miette::bail!("unknown document type");
        }
    }
}
