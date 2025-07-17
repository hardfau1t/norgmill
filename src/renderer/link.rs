//! this module handles rendering of links

use crate::{constants, renderer::paragraph};
use std::fmt::Write;
use tracing::{debug, error, instrument, trace, warn};

#[instrument(skip(output, description_segments))]
pub fn render_link(
    file_path: Option<&str>,
    targets: &[norg::LinkTarget],
    description_segments: Option<&[norg::ParagraphSegment]>,
    output: &mut String,
) -> std::fmt::Result {
    trace!("rendering link");
    let norg_file_path = file_path
        .map(|norg_path| {
            let mut norg_path_iter = norg_path.trim_start().chars();
            match norg_path_iter
                .next()
                .expect("there should be some character found if the norg file path is specified")
            {
                '/' => {
                    // this is norg file from root of the file system
                    Some(format!(
                        "{}/{}",
                        constants::SYSTEM_PATH,
                        norg_path_iter.as_str().trim_start(),
                    ))
                }
                '$' => {
                    // this is norg file from root of workspace
                    // get next character to see if it is space or not
                    let work_dir_path = norg_path_iter.as_str().trim_start();
                    match work_dir_path.chars().next() {
                        Some('/') => {
                            // this path is from root of the workspace
                            Some(format!(
                                "{}{}",
                                constants::CURRENT_WORKSPACE_PATH,
                                norg_path_iter.as_str().trim_start(),
                            ))
                        }
                        None => {
                            error!("invalid File path found, empty path");
                            None
                        }
                        _ => {
                            // path is from different workspace
                            Some(format!("{}/{}", constants::WORKSPACE_PATH, work_dir_path,))
                        }
                    }
                }
                '~' => {
                    // this is norg file from home directory
                    Some(format!(
                        "{}{}",
                        constants::HOME_PATH,
                        norg_path_iter.as_str().trim_start(),
                    ))
                }
                _ => {
                    // this is the relative path to file
                    Some(norg_path.trim_start().to_string())
                }
            }
        })
        .flatten();
    debug!(?norg_file_path, "norg file found?");

    let fragment_or_external_link = targets
        .first()
        .map(|target| {
            match target {
                norg::LinkTarget::Heading { level, title } => {
                    let link = format!(
                        "#{}_h{}",
                        paragraph::render_segments(title)
                            .expect("string formatting is infallible")
                            .replace(' ', "_"),
                        level
                    );
                    Some(link)
                }
                norg::LinkTarget::LineNumber(_) => {
                    error!("<!-- Unsupported feature: line number on anchor -->");
                    Some("#".to_string())
                }
                norg::LinkTarget::Footnote(title) => Some(format!(
                    "#{}_f",
                    paragraph::render_segments(title)
                        .expect("string formatting is infallible")
                        .replace(' ', "_")
                )),
                norg::LinkTarget::Definition(title) => Some(format!(
                    "#{}_d",
                    paragraph::render_segments(title)
                        .expect("string formatting is infallible")
                        .replace(' ', "_")
                )),
                norg::LinkTarget::Wiki(title) => {
                    error!(target = ?title, "wiki links are not yet supported");
                    None
                }
                norg::LinkTarget::Generic(generics) => {
                    error!(target=?generics, "wiki links are not yet supported");
                    None
                }
                norg::LinkTarget::Extendable(extendable) => {
                    error!(target=?extendable, "extendable links are not yet supported");
                    None
                }
                norg::LinkTarget::Path(raw_path) => {
                    let mut raw_path_iter = raw_path.trim_start().chars();
                    match raw_path_iter.next() {
                        None => {
                            warn!("empty raw file path found as a link");
                            None
                        }
                        Some('/') => {
                            // root raw file path
                            Some(format!(
                                "{}/{}?raw=1",
                                constants::SYSTEM_PATH,
                                raw_path_iter.as_str()
                            ))
                        }
                        Some('~') => {
                            // home raw file path
                            Some(format!(
                                "{}{}?raw=1",
                                constants::HOME_PATH,
                                raw_path_iter.as_str()
                            ))
                        }
                        Some(_) => Some(raw_path.trim_start().to_string()),
                    }
                }
                norg::LinkTarget::Url(text) => Some(text.to_string()),
                norg::LinkTarget::Timestamp(text) => {
                    error!(target=?text, "timestamp links are not yet supported");
                    None
                }
            }
        })
        .flatten();
    debug!(?fragment_or_external_link, "href found?");

    let href = if let Some(mut norg_file_path) = norg_file_path {
        if let Some(fragment) = fragment_or_external_link {
            norg_file_path.push_str(&fragment);
            norg_file_path
        } else {
            norg_file_path
        }
    } else {
        fragment_or_external_link.unwrap_or("#".to_string())
    };

    let title = description_segments
        .map(paragraph::render_segments)
        .unwrap_or_else(|| Ok(href.clone()))?;
    write!(output, "<a href={href}>{title}</a>")
}

// TODO: Create a function to generate fragment tags, that will be used to create anchor tags and link to that tag
