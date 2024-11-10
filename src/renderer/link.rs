//! this module handles rendering of links

use serde::Serialize;
use tracing::debug;
use tracing::instrument;

use crate::renderer::paragraph;

#[derive(Debug, Serialize)]
struct Link {
    target: String,
    description: String,
}

impl Link {
    fn new(
        file_path: Option<String>,
        targets: Vec<norg::LinkTarget>,
        description: Option<Vec<norg::ParagraphSegment>>,
        hbr: &handlebars::Handlebars,
    ) -> Self {
        if let Some(norgfile_path) = file_path {
            debug!("link to another norg file");
            let target = format!("{}.norg", norgfile_path);
            let description = description
                .map(|paras| {
                    let mut write_to = String::new();
                    paras
                        .into_iter()
                        .map(|para| paragraph::render_paragraph(para, &mut write_to, hbr))
                        .collect::<Result<(), _>>()
                        .expect("Couldn't deserialize paragraph description");
                    write_to
                })
                .unwrap_or_else(|| target.clone());
            Link {
                target,
                description,
            }
        } else {
            todo!("not yet parsed link")
        }
    }
}

#[instrument]
pub fn render_link<'a>(
    file_path: Option<String>,
    targets: Vec<norg::LinkTarget>,
    description: Option<Vec<norg::ParagraphSegment>>,
    hbr: &handlebars::Handlebars,
) -> String {
    let link = Link::new(file_path, targets, description, hbr);
    hbr.render("link", &link)
        .expect("change this to error type")
}
