//! this module handles rendering of links

use crate::renderer::paragraph;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, instrument, warn, trace};

#[derive(Debug, Serialize)]
struct Link {
    target: String,
    description: String,
}

impl Link {
    fn new(
        file_path: Option<&str>,
        targets: &[norg::LinkTarget],
        description: Option<&[norg::ParagraphSegment]>,
        hbr: &handlebars::Handlebars,
    ) -> miette::Result<Self> {
        let mut rendered_target = String::new();
        render_target(targets, &mut rendered_target, hbr)?;
        // if description is found use it else calculate target as description
        let link_name = if let Some(description) = description {
            description
                .into_iter()
                .try_fold(String::new(), |mut acc, para| -> miette::Result<_> {
                    paragraph::render_paragraph(&para, &mut acc, hbr)
                        .wrap_err("Couldn't render link description")?;
                    Ok(acc)
                })
        } else {
            Ok(rendered_target.clone())
        }?;
        if let Some(norgfile_path) = file_path {
            let mut target_link = parse_norg_path(norgfile_path.to_string());
            // if there is only link to file then target will be empty
            if !rendered_target.is_empty() {
                target_link.push('#');
                target_link.push_str(&rendered_target);
            }
            Ok(Link {
                target: target_link,
                description: link_name,
            })
        } else {
            // if target is empty then empty link is generated
            Ok(Link {
                target: rendered_target,
                description: link_name,
            })
        }
    }
}

#[instrument(skip(hbr))]
pub fn render_link<'a>(
    file_path: Option<&str>,
    targets: &[norg::LinkTarget],
    description: Option<&[norg::ParagraphSegment]>,
    hbr: &handlebars::Handlebars,
) -> miette::Result<String> {
    let link = Link::new(file_path, targets, description, hbr)?;
    hbr.render("link", &link)
        .into_diagnostic()
        .wrap_err("Couldn't generate link")
}

fn render_target(
    targets: &[norg::LinkTarget],
    write_to: &mut String,
    hbr: &handlebars::Handlebars,
) -> miette::Result<()> {
    let render_paras = |title: &[norg::ParagraphSegment], dest: &mut String| {
        title
            .iter()
            .map(|segment| {
                paragraph::render_paragraph(&segment, dest, hbr)
                    .wrap_err("Couldn't render heading link")
            })
            .collect::<miette::Result<()>>()
    };

    for target in targets.into_iter() {
        match target {
            norg::LinkTarget::Heading { level, title } => {
                render_paras(title, write_to)?;
                write_to.push('_');
                write_to.push_str(level.to_string().as_str());
                write_to.push('h');
            }
            norg::LinkTarget::Footnote(titles) => {
                render_paras(titles, write_to)?;
                write_to.push_str("_f");
            }
            norg::LinkTarget::Definition(defs) => {
                render_paras(defs, write_to)?;
                write_to.push_str("_d");
            }
            norg::LinkTarget::Generic(generics) => {
                warn!("Generics links are not supported");
                render_paras(generics, write_to)?;
            }
            norg::LinkTarget::Wiki(wiki) => {
                render_paras(wiki, write_to)?;
                write_to.push_str("_w");
            }
            norg::LinkTarget::Extendable(extendable) => {
                warn!("Unwknown extendables, raise issue");
                render_paras(extendable, write_to)?;
                write_to.push_str("_w");
            }
            norg::LinkTarget::Path(p) => write_to.push_str(&p),
            norg::LinkTarget::Url(u) => write_to.push_str(&u),
            norg::LinkTarget::Timestamp(ts) => write_to.push_str(&ts),
        }
    }
    Ok(())
}

/// Parses file links and generates respective link
fn parse_norg_path(mut original_path: String) -> String {
    debug!("link to another norg file");
    let mut original_path_chars = original_path.chars();
    match original_path_chars.next() {
        Some('$') => {
            if !original_path_chars.next().is_some_and(|c| c == '/') {
                warn!("linking to different workspace is not yet supported, assuming as current workspace");
            }
            // start from workspace root
            let mut path = format!("/{}/", crate::constants::WORKSPACE_PATH);
            path.push_str(
                original_path
                    .trim_start_matches('$')
                    .trim_start_matches('/'),
            );
            path.push_str(".norg");
            path
        }
        Some('/') => {
            // start from workspace root
            let mut path = format!("/{}/", crate::constants::SYSTEM_PATH.to_string());
            path.push_str(original_path.trim_start_matches('/'));
            if !original_path_chars
                .next()
                .is_some_and(|c| c.is_whitespace())
            {
                // this is a link to system level norg file
                path.push_str(".norg");
            } else {
                // if there was a space then its non norg file so no need to append anything
                trace!("system level non norg file link: {path} generated");
            }
            path
        }
        Some(c) => {
            debug!("file path starts from {c}, should be relative to current file");
            original_path.push_str(".norg");
            original_path
        }
        None => {
            warn!("empty file link found, don't know what to do");
            original_path
        }
    }
}
