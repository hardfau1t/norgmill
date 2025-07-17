use norgmill::renderer::parse_and_render_norg;

// Link Descriptions Tests
#[test]
fn test_link_description_basic() {
    let norg = r#"Visit {https://example.com}[Example Site] for more info."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic link description");
    assert!(result.contains("<p>Visit"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Example Site</a>"));
    assert!(result.contains("for more info.</p>"));
}

#[test]
fn test_link_description_with_markup() {
    let norg = r#"See {https://example.com}[*Important* Site] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with markup");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("<strong>Important</strong> Site</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_link_description_italic_markup() {
    let norg = r#"Check {https://example.com}[/Italic/ Description] out."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with italic");
    assert!(result.contains("<p>Check"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("<em>Italic</em> Description</a>"));
    assert!(result.contains("out.</p>"));
}

#[test]
fn test_link_description_code_markup() {
    let norg = r#"Visit {https://example.com}[`Code` Example] site."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with code");
    assert!(result.contains("<p>Visit"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("<code>Code</code> Example</a>"));
    assert!(result.contains("site.</p>"));
}

#[test]
fn test_link_description_long_text() {
    let norg = r#"See {https://example.com}[This is a very long link description that spans multiple words] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse long link description");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("This is a very long link description that spans multiple words</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_link_description_with_numbers() {
    let norg = r#"Version {https://example.com}[2.0 Release] available."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with numbers");
    assert!(result.contains("<p>Version"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("2.0 Release</a>"));
    assert!(result.contains("available.</p>"));
}

#[test]
fn test_link_description_with_punctuation() {
    let norg = r#"See {https://example.com}[Site: Example & Demo!] now."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with punctuation");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Site: Example &amp; Demo!</a>"));
    assert!(result.contains("now.</p>"));
}

#[test]
fn test_link_description_empty() {
    let norg = r#"Empty description {https://example.com}[] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty link description");
    assert!(result.contains("<p>Empty description"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_link_description_file_link() {
    let norg = r#"Read {/ manual.pdf}[User Manual] for help."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link description");
    assert!(result.contains("<p>Read"));
    assert!(result.contains("<a href=\"manual.pdf\">"));
    assert!(result.contains("User Manual</a>"));
    assert!(result.contains("for help.</p>"));
}

#[test]
fn test_link_description_internal_link() {
    let norg = r#"* Introduction
See {* Introduction}[the intro section] above."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse internal link description");
    assert!(result.contains("<h1>Introduction</h1>"));
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"#introduction\">"));
    assert!(result.contains("the intro section</a>"));
    assert!(result.contains("above.</p>"));
}

#[test]
fn test_link_description_with_line_breaks() {
    let norg = r#"See {https://example.com}[Multi-line
description text] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with line breaks");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Multi-line"));
    assert!(result.contains("description text</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_link_description_unicode() {
    let norg = r#"Site {https://example.com}[Café & Naïve] with unicode."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with unicode");
    assert!(result.contains("<p>Site"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Café &amp; Naïve</a>"));
    assert!(result.contains("with unicode.</p>"));
}

#[test]
fn test_link_description_emoji() {
    let norg = r#"Fun {https://example.com}[🎉 Party Site 🎊] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link description with emoji");
    assert!(result.contains("<p>Fun"));
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("🎉 Party Site 🎊</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_link_description_multiple_links() {
    let norg = r#"Links: {https://first.com}[First Site], {https://second.com}[Second Site], {https://third.com}[Third Site]."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple link descriptions");
    assert!(result.contains("Links:"));
    assert!(result.contains("<a href=\"https://first.com\">First Site</a>"));
    assert!(result.contains("<a href=\"https://second.com\">Second Site</a>"));
    assert!(result.contains("<a href=\"https://third.com\">Third Site</a>"));
}

#[test]
fn test_link_description_malformed() {
    let norg = r#"Malformed {https://example.com}[description without closing"#;
    let result = parse_and_render_norg(norg);
    assert!(result.is_ok());
    // Should handle gracefully
}