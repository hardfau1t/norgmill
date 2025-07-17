use norgmill::renderer::parse_and_render_norg;

#[test]
fn test_heading_level_1() {
    let norg = "* Heading Level 1";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 1");
    assert!(result.contains("<h1>Heading Level 1</h1>"));
}

#[test]
fn test_heading_level_2() {
    let norg = "** Heading Level 2";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 2");
    assert!(result.contains("<h2>Heading Level 2</h2>"));
}

#[test]
fn test_heading_level_3() {
    let norg = "*** Heading Level 3";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 3");
    assert!(result.contains("<h3>Heading Level 3</h3>"));
}

#[test]
fn test_heading_level_4() {
    let norg = "**** Heading Level 4";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 4");
    assert!(result.contains("<h4>Heading Level 4</h4>"));
}

#[test]
fn test_heading_level_5() {
    let norg = "***** Heading Level 5";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 5");
    assert!(result.contains("<h5>Heading Level 5</h5>"));
}

#[test]
fn test_heading_level_6() {
    let norg = "****** Heading Level 6";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 6");
    assert!(result.contains("<h6>Heading Level 6</h6>"));
}

#[test]
fn test_heading_level_7_fallback() {
    let norg = "******* Heading Level 7";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading level 7");
    // Level 7 should fall back to level 6
    assert!(result.contains("<h6>Heading Level 7</h6>"));
}

#[test]
fn test_heading_with_markup() {
    let norg = "* Heading with *bold* and /italic/ text";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading with markup");
    assert!(result.contains("<h1>Heading with <strong>bold</strong> and <em>italic</em> text</h1>"));
}

#[test]
fn test_multiple_headings() {
    let norg = r#"* First Heading
** Second Heading
*** Third Heading"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple headings");
    assert!(result.contains("<h1>First Heading</h1>"));
    assert!(result.contains("<h2>Second Heading</h2>"));
    assert!(result.contains("<h3>Third Heading</h3>"));
}

#[test]
fn test_heading_with_special_characters() {
    let norg = "* Heading with 123 & symbols!";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading with special chars");
    assert!(result.contains("<h1>Heading with 123 &amp; symbols!</h1>"));
}

#[test]
fn test_heading_with_unicode() {
    let norg = "* Heading with émojis 🌟 and Unicode";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading with unicode");
    assert!(result.contains("<h1>Heading with émojis 🌟 and Unicode</h1>"));
}

#[test]
fn test_heading_empty() {
    let norg = "*";
    let result = parse_and_render_norg(norg).expect("Failed to parse empty heading");
    // Should handle empty heading gracefully
    assert!(result.contains("<h1></h1>") || result.contains("<h1> </h1>"));
}

#[test]
fn test_heading_whitespace_handling() {
    let norg = "*   Heading with extra spaces   ";
    let result = parse_and_render_norg(norg).expect("Failed to parse heading with whitespace");
    // Should trim whitespace appropriately
    assert!(result.contains("<h1>Heading with extra spaces</h1>"));
}

#[test]
fn test_heading_followed_by_content() {
    let norg = r#"* Main Heading
This is content under the heading.

** Sub Heading
More content here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse heading with content");
    assert!(result.contains("<h1>Main Heading</h1>"));
    assert!(result.contains("<h2>Sub Heading</h2>"));
    assert!(result.contains("This is content under the heading."));
    assert!(result.contains("More content here."));
}