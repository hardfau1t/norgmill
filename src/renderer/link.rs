//! this module handles rendering of links

use crate::{constants, renderer::paragraph};
use std::fmt::Write;
use tracing::{error, instrument, trace, warn};

#[instrument(skip(para_builder, description_segments))]
pub fn render_link<'t, 'p>(
    file_path: Option<&str>,
    targets: &'t [norg::LinkTarget],
    description_segments: Option<&'t [norg::ParagraphSegment]>,
    para_builder: &'p mut html::text_content::builders::ParagraphBuilder,
) -> &'p mut html::text_content::builders::ParagraphBuilder {
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

    let href = targets
        .first()
        .map(|target| {
            match target {
                norg::LinkTarget::Heading { level, title } => {
                    let mut link = norg_file_path.unwrap_or_default();
                    write!(
                        link,
                        "#{}_h{}",
                        paragraph::render_paragraph_to_string(title).replace(' ', "_"),
                        level
                    )
                    .unwrap();
                    Some(link)
                }
                norg::LinkTarget::LineNumber(_) => {
                    let mut link = norg_file_path.unwrap_or_default();
                    link.push('#');
                    error!("<!-- Unsupported feature: line number on anchor -->");
                    Some(link)
                }
                norg::LinkTarget::Footnote(title) => {
                    let mut link = norg_file_path.unwrap_or_default();
                    write!(
                        link,
                        "#{}_f",
                        paragraph::render_paragraph_to_string(title).replace(' ', "_")
                    )
                    .unwrap();
                    Some(link)
                }
                norg::LinkTarget::Definition(title) => {
                    let mut link = norg_file_path.unwrap_or_default();
                    write!(
                        link,
                        "#{}_d",
                        paragraph::render_paragraph_to_string(title).replace(' ', "_")
                    )
                    .unwrap();
                    Some(link)
                }
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

    let href = href.unwrap_or("#".to_string());

    let title = description_segments
        .map(paragraph::render_paragraph_to_string)
        .unwrap_or_else(|| href.clone());
    para_builder.anchor(|ab| ab.href(href).text(title))
}

// TODO: Create a function to generate fragment tags, that will be used to create anchor tags and link to that tag
