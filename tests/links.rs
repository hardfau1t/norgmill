#![recursion_limit = "512"]
use html::text_content::Division;
use norgmill::renderer; // For renderer::parse_and_render_body // For Body::builder()

// Utility function to render a Norg string to a full HTML body string.
fn test_link_rendering(norg_link_string: &str, _current_file_path: Option<&str>) -> String {
    // _current_file_path is now unused as parse_and_render_body doesn't take it directly.
    // This might affect tests requiring path context for link resolution.
    let mut body_builder = Division::builder();
    match renderer::parse_and_render_norg(norg_link_string, &mut body_builder) {
        Ok(_) => body_builder.build().to_string(),
        Err(e) => panic!("parse_and_render_body failed: {:?}", e),
    }
}

#[test]
fn test_link_with_url_target() {
    let html = test_link_rendering("{https://example.com}", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"https://example.com\">https://example.com</a></p></div></body>"
    );
}

#[test]
fn test_link_with_description() {
    let html = test_link_rendering("{https://example.com}[Example Website]", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"https://example.com\">Example Website</a></p></div></body>"
    );
}

#[test]
fn test_link_with_heading_target_no_file_path() {
    let html = test_link_rendering("{* Test Heading}", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"#Test_Heading_h1\">#Test_Heading_h1</a></p></div></body>"
    );
}

#[test]
fn test_link_with_workspace_file_and_description() {
    let html = test_link_rendering("{:$/folder/file:}[description]", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"/view/current/folder/file\">description</a></p></div></body>"
    );
}

#[test]
fn test_link_with_footnote_target_no_file_path() {
    let html = test_link_rendering("{^ footnote1}", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"#footnote1_f\">#footnote1_f</a></p></div></body>"
    );
}

#[test]
fn test_link_with_definition_target_no_file_path() {
    let html = test_link_rendering("{$ term}", None);
    assert_eq!(
        html,
        "<body><div><p><a href=\"#term_d\">#term_d</a></p></div></body>"
    );
}

// Tests for file_path prefixing with targets that use it (e.g., Heading)
#[test]
fn test_link_to_heading_with_file_path_system_root() {
    let norg_input = "{:/docs/file.norg:** TargetHeading}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/root/docs/file.norg#TargetHeading_h2\">/view/root/docs/file.norg#TargetHeading_h2</a></p></div></body>");
}

#[test]
fn test_link_to_heading_with_file_path_home() {
    let norg_input = "{:~/docs/file.norg:** TargetHeading}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/home/docs/file.norg#TargetHeading_h2\">/view/home/docs/file.norg#TargetHeading_h2</a></p></div></body>");
}

#[test]
fn test_link_to_heading_with_file_path_workspace() {
    let norg_input = "{:$/docs/file.norg:** TargetHeading}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/current/docs/file.norg#TargetHeading_h2\">/view/current/docs/file.norg#TargetHeading_h2</a></p></div></body>");
}

#[test]
fn test_link_to_heading_with_file_path_different_workspace() {
    let norg_input = "{:$other_ws/docs/file:** TargetHeading}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/workspace/other_ws/docs/file#TargetHeading_h2\">/view/workspace/other_ws/docs/file#TargetHeading_h2</a></p></div></body>");
}

// Tests for LinkTarget::Path variants
#[test]
fn test_link_with_path_target_system_root() {
    let norg_input = "{/ /path/to/file.txt}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/root/path/to/file.txt?raw=1\">/view/root/path/to/file.txt?raw=1</a></p></div></body>");
}

#[test]
fn test_link_with_path_target_home() {
    let norg_input = "{:~/path/to/file:* heading}[description]";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"/view/home/path/to/file#heading_h1\">description</a></p></div></body>");
}

#[test]
fn test_link_with_path_target_relative() {
    let norg_input = "{:relative/path:** heading2}";
    let html = test_link_rendering(norg_input, None);
    assert_eq!(html, "<body><div><p><a href=\"relative/path#heading2_h2\">relative/path#heading2_h2</a></p></div></body>");
}

#[test]
fn test_link_with_empty_targets() {
    let html = test_link_rendering("{}", None);
    assert_eq!(html, "<body><div><p><a href=\"#\">#</a></p></div></body>");
}

// Tests for currently unsupported LinkTarget types (should default to href="#" or file_path + "#")
#[test]
fn test_link_with_line_number_target() {
    let html = test_link_rendering("{123}", None); // Correct Norg syntax for line number link
    assert_eq!(html, "<body><div><p><a href=\"#\">#</a></p></div></body>");
}

#[test]
fn test_link_with_line_number_target_with_file() {
    let html = test_link_rendering("{:file:123}", None); // Correct Norg syntax for line number link in a file
    assert_eq!(
        html,
        "<body><div><p><a href=\"file#\">file#</a></p></div></body>"
    );
}

#[test]
fn test_link_with_wiki_target() {
    let html = test_link_rendering("{@ wikipage}", None); // Correct Norg syntax for a wiki link
    assert_eq!(html, "<body><div><p><a href=\"#\">#</a></p></div></body>");
}

// #[test]
// fn test_link_with_generic_target() {
//     let html = test_link_rendering("{& generic_target}", None); // Correct Norg syntax for a generic link
//     assert_eq!(
//         html,
//         "<body><div><p><a href=\"#\">#</a></p></div></body>"
//     );
// }

// #[test]
// fn test_link_with_timestamp_target() {
//     let html = test_link_rendering("{<2024-01-01>}", None);
//     assert_eq!(
//         html,
//         "<body><div><p><a href=\"#\">#</a></p></div></body>"
//     );
// }
//
