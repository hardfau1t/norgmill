//! this module handles rendering of links

use crate::{constants, renderer::paragraph};
use std::fmt::Write;
use tracing::{error, warn};

pub fn render_link<'t, 'p>(
    file_path: Option<&str>,
    targets: &'t [norg::LinkTarget],
    description_segments: Option<&'t [norg::ParagraphSegment]>,
    para_builder: &'p mut html::text_content::builders::ParagraphBuilder,
) -> &'p mut html::text_content::builders::ParagraphBuilder {
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
                                "{}/{}",
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
                            Some(format!(
                                "{}/{}",
                                constants::WORKSPACE_PATH,
                                work_dir_path,
                            ))
                        }
                    }
                }
                '~' => {
                    // this is norg file from home directory
                    Some(format!(
                        "{}/{}",
                        constants::HOME_PATH,
                        norg_path_iter.as_str().trim_start(),
                    ))
                }
                _ => {
                    // this is the relative path to file
                    Some(norg_path_iter.as_str().trim_start().to_string())
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
                        paragraph::render_paragraph_to_string(title),
                        level
                    ).unwrap();
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
                    write!(link, "#{}_f", paragraph::render_paragraph_to_string(title)).unwrap();
                    Some(link)
                }
                norg::LinkTarget::Definition(title) => {
                    let mut link = norg_file_path.unwrap_or_default();
                    write!(link, "#{}_d", paragraph::render_paragraph_to_string(title)).unwrap();
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
                                "{}/{}",
                                constants::SYSTEM_PATH,
                                raw_path_iter.as_str()
                            ))
                        }
                        Some('~') => {
                            // home raw file path
                            Some(format!(
                                "{}/{}",
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
    para_builder.anchor(|ab| ab.href(href).title(|tb| tb.text(title)))
}

// TODO: Create a function to generate fragment tags, that will be used to create anchor tags and link to that tag

#[cfg(test)]
mod tests {
    use super::*;
    use html::text_content::builders::ParagraphBuilder;
    use html::text_content::Paragraph;
    use crate::constants; // Ensure constants are in scope

    // Helper function to create a paragraph builder for testing
    fn create_paragraph_builder() -> ParagraphBuilder {
        Paragraph::builder()
    }

    // Helper function to extract the HTML string from a paragraph builder
    fn get_html_from_builder(builder: &mut ParagraphBuilder) -> String {
        builder.build().to_string()
    }

    // Helper function to create paragraph segments for link descriptions/titles
    fn create_paragraph_segments(text: &str) -> Vec<norg::ParagraphSegment> {
        vec![norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(text.to_string()))]
    }

    #[test]
    fn test_link_with_url_target() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Url("https://example.com".to_string())];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"https://example.com\""), "HTML: {}", html);
    }

    #[test]
    fn test_link_with_description() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Url("https://example.com".to_string())];
        let description = create_paragraph_segments("Example Website");
        render_link(None, &targets, Some(&description), &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"https://example.com\""), "HTML: {}", html);
        assert!(html.contains("Example Website"), "HTML: {}", html);
    }

    #[test]
    fn test_link_with_heading_target_no_file_path() {
        let mut builder = create_paragraph_builder();
        let title_segments = create_paragraph_segments("Test Heading");
        let targets = vec![norg::LinkTarget::Heading { 
            level: 1, 
            title: title_segments // Pass Vec directly
        }];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#Test_Heading_h1\""), "HTML: {}", html);
    }

    #[test]
    fn test_link_with_footnote_target_no_file_path() {
        let mut builder = create_paragraph_builder();
        let title_segments = create_paragraph_segments("footnote1");
        let targets = vec![norg::LinkTarget::Footnote(title_segments)]; // Pass Vec directly
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#footnote1_f\""), "HTML: {}", html);
    }

    #[test]
    fn test_link_with_definition_target_no_file_path() {
        let mut builder = create_paragraph_builder();
        let title_segments = create_paragraph_segments("term");
        let targets = vec![norg::LinkTarget::Definition(title_segments)]; // Pass Vec directly
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#term_d\""), "HTML: {}", html);
    }

    // Tests for file_path prefixing with targets that use it (e.g., Heading)
    #[test]
    fn test_link_to_heading_with_file_path_system_root() {
        let mut builder = create_paragraph_builder();
        let heading_title = create_paragraph_segments("TargetHeading");
        let targets = vec![norg::LinkTarget::Heading { level: 2, title: heading_title }];
        render_link(Some("/docs/file.norg"), &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/docs/file.norg#TargetHeading_h2\"", constants::SYSTEM_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }

    #[test]
    fn test_link_to_heading_with_file_path_home() {
        let mut builder = create_paragraph_builder();
        let heading_title = create_paragraph_segments("TargetHeading");
        let targets = vec![norg::LinkTarget::Heading { level: 2, title: heading_title }];
        render_link(Some("~/docs/file.norg"), &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/docs/file.norg#TargetHeading_h2\"", constants::HOME_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }

    #[test]
    fn test_link_to_heading_with_file_path_workspace() {
        let mut builder = create_paragraph_builder();
        let heading_title = create_paragraph_segments("TargetHeading");
        let targets = vec![norg::LinkTarget::Heading { level: 2, title: heading_title }];
        render_link(Some("$/docs/file.norg"), &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/docs/file.norg#TargetHeading_h2\"", constants::CURRENT_WORKSPACE_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }

    #[test]
    fn test_link_to_heading_with_file_path_different_workspace() {
        let mut builder = create_paragraph_builder();
        let heading_title = create_paragraph_segments("TargetHeading");
        let targets = vec![norg::LinkTarget::Heading { level: 2, title: heading_title }];
        render_link(Some("$other_ws/docs/file.norg"), &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/other_ws/docs/file.norg#TargetHeading_h2\"", constants::WORKSPACE_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }
    
    // Tests for LinkTarget::Path variants
    #[test]
    fn test_link_with_path_target_system_root() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Path("/path/to/file.txt".to_string())];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/path/to/file.txt\"", constants::SYSTEM_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }

    #[test]
    fn test_link_with_path_target_home() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Path("~/path/to/file.txt".to_string())];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        let expected_href = format!("href=\"{}/path/to/file.txt\"", constants::HOME_PATH);
        assert!(html.contains(&expected_href), "HTML: {}\nExpected: {}", html, expected_href);
    }

    #[test]
    fn test_link_with_path_target_relative() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Path("relative/path.txt".to_string())];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"relative/path.txt\""), "HTML: {}", html);
    }

    #[test]
    fn test_link_with_empty_targets() {
        let mut builder = create_paragraph_builder();
        let targets: Vec<norg::LinkTarget> = vec![];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#\""), "HTML: {}", html);
    }

    // Tests for currently unsupported LinkTarget types (should default to href="#" or file_path + "#")
    #[test]
    fn test_link_with_line_number_target() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::LineNumber(123)];
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        // Current behavior is to append '#' to norg_file_path. If norg_file_path is empty, it's just "#".
        assert!(html.contains("href=\"#\""), "LineNumber without file_path. HTML: {}", html);

        let mut builder2 = create_paragraph_builder();
        render_link(Some("file.norg"), &targets, None, &mut builder2);
        let html2 = get_html_from_builder(&mut builder2);
        assert!(html2.contains("href=\"file.norg#\""), "LineNumber with file_path. HTML: {}", html2);
    }

    #[test]
    fn test_link_with_wiki_target() {
        let mut builder = create_paragraph_builder();
        let title_segments = create_paragraph_segments("wikipage");
        let targets = vec![norg::LinkTarget::Wiki(title_segments)]; // Pass Vec directly
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#\""), "Wiki should default to #. HTML: {}", html);
    }

    #[test]
    fn test_link_with_generic_target() {
        let mut builder = create_paragraph_builder();
        let title_segments = create_paragraph_segments("generic_target");
        let targets = vec![norg::LinkTarget::Generic(title_segments)]; // Pass Vec directly
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#\""), "Generic should default to #. HTML: {}", html);
    }

    #[test]
    fn test_link_with_timestamp_target() {
        let mut builder = create_paragraph_builder();
        let targets = vec![norg::LinkTarget::Timestamp("2024-01-01".to_string())]; // Pass Vec directly
        render_link(None, &targets, None, &mut builder);
        let html = get_html_from_builder(&mut builder);
        assert!(html.contains("href=\"#\""), "Timestamp should default to #. HTML: {}", html);
    }

}
