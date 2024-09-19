//! this module handles rendering of links

use serde::Serialize;
use tracing::debug;
use tracing::instrument;

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
    ) -> Self {
        if let Some(norgfile_path) = file_path {
            debug!("link to another norg file");
            let target = format!("{}.norg", norgfile_path);
            let description = target.clone();
            Link {
                target,
                description,
            }
        } else {
            debug!("not yet parsed link");
            todo!()
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
    let link = Link::new(file_path, targets, description);
    hbr.render("link", &link)
        .expect("change this to error type")
}
