use norgmill::renderer::parse_and_render_norg;

// Detached Modifier Links Tests
#[test]
fn test_detached_modifier_link_heading() {
    let norg = r#"* Main Section
Reference to {* Main Section} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link to heading");
    assert!(result.contains("<h1>Main Section</h1>"));
    assert!(result.contains("<p>Reference to"));
    assert!(result.contains("<a href=\"#main-section\">"));
    assert!(result.contains("Main Section</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_detached_modifier_link_definition() {
    let norg = r#"$ Important Term
Definition of the term.

Link to {$ Important Term} definition."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link to definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Important Term</dt>"));
    assert!(result.contains("<dd>Definition of the term.</dd>"));
    assert!(result.contains("Link to"));
    assert!(result.contains("<a href=\"#important-term\">"));
    assert!(result.contains("Important Term</a>"));
    assert!(result.contains("definition."));
}

#[test]
fn test_detached_modifier_link_footnote() {
    let norg = r#"^ Technical Note
This is a technical note.

Reference to {^ Technical Note} above."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link to footnote");
    assert!(result.contains("Technical Note"));
    assert!(result.contains("This is a technical note."));
    assert!(result.contains("Reference to"));
    assert!(result.contains("<a href=\"#technical-note\">"));
    assert!(result.contains("Technical Note</a>"));
    assert!(result.contains("above."));
}

#[test]
fn test_detached_modifier_link_multi_level_heading() {
    let norg = r#"* Level 1
** Level 2
*** Level 3

Links: {* Level 1}, {** Level 2}, {*** Level 3}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier links to multi-level headings");
    assert!(result.contains("<h1>Level 1</h1>"));
    assert!(result.contains("<h2>Level 2</h2>"));
    assert!(result.contains("<h3>Level 3</h3>"));
    assert!(result.contains("Links:"));
    assert!(result.contains("<a href=\"#level-1\">Level 1</a>"));
    assert!(result.contains("<a href=\"#level-2\">Level 2</a>"));
    assert!(result.contains("<a href=\"#level-3\">Level 3</a>"));
}

#[test]
fn test_detached_modifier_link_with_description() {
    let norg = r#"* Getting Started
Introduction content.

See {* Getting Started}[the introduction] for details."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link with description");
    assert!(result.contains("<h1>Getting Started</h1>"));
    assert!(result.contains("Introduction content."));
    assert!(result.contains("See"));
    assert!(result.contains("<a href=\"#getting-started\">"));
    assert!(result.contains("the introduction</a>"));
    assert!(result.contains("for details."));
}

#[test]
fn test_detached_modifier_link_in_list() {
    let norg = r#"* Overview
* Installation
* Configuration

Table of contents:
- {* Overview}
- {* Installation}
- {* Configuration}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier links in list");
    assert!(result.contains("<h1>Overview</h1>"));
    assert!(result.contains("<h1>Installation</h1>"));
    assert!(result.contains("<h1>Configuration</h1>"));
    assert!(result.contains("Table of contents:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li><a href=\"#overview\">Overview</a></li>"));
    assert!(result.contains("<li><a href=\"#installation\">Installation</a></li>"));
    assert!(result.contains("<li><a href=\"#configuration\">Configuration</a></li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_detached_modifier_link_forward_reference() {
    let norg = r#"See {* Later Section} for more information.

* Later Section
Content of the later section."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse forward reference detached modifier link");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"#later-section\">"));
    assert!(result.contains("Later Section</a>"));
    assert!(result.contains("for more information.</p>"));
    assert!(result.contains("<h1>Later Section</h1>"));
    assert!(result.contains("Content of the later section."));
}

#[test]
fn test_detached_modifier_link_backward_reference() {
    let norg = r#"* Earlier Section
Content of the earlier section.

Back to {* Earlier Section} above."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse backward reference detached modifier link");
    assert!(result.contains("<h1>Earlier Section</h1>"));
    assert!(result.contains("Content of the earlier section."));
    assert!(result.contains("Back to"));
    assert!(result.contains("<a href=\"#earlier-section\">"));
    assert!(result.contains("Earlier Section</a>"));
    assert!(result.contains("above."));
}

#[test]
fn test_detached_modifier_link_special_characters() {
    let norg = r#"* Section with Special & Characters!
Content here.

Link to {* Section with Special & Characters!} section."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link with special characters");
    assert!(result.contains("<h1>Section with Special &amp; Characters!</h1>"));
    assert!(result.contains("Content here."));
    assert!(result.contains("Link to"));
    assert!(result.contains("<a href=\"#section-with-special-characters\">"));
    assert!(result.contains("Section with Special &amp; Characters!</a>"));
    assert!(result.contains("section."));
}

#[test]
fn test_detached_modifier_link_multiple_in_paragraph() {
    let norg = r#"* Introduction
* Installation
* Configuration

Navigate between {* Introduction}, {* Installation}, and {* Configuration}."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple detached modifier links");
    assert!(result.contains("<h1>Introduction</h1>"));
    assert!(result.contains("<h1>Installation</h1>"));
    assert!(result.contains("<h1>Configuration</h1>"));
    assert!(result.contains("Navigate between"));
    assert!(result.contains("<a href=\"#introduction\">Introduction</a>"));
    assert!(result.contains("<a href=\"#installation\">Installation</a>"));
    assert!(result.contains("<a href=\"#configuration\">Configuration</a>"));
}

#[test]
fn test_detached_modifier_link_nested_in_quote() {
    let norg = r#"* Important Note
This is important.

> Reference to {* Important Note} in this quote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link in quote");
    assert!(result.contains("<h1>Important Note</h1>"));
    assert!(result.contains("This is important."));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Reference to"));
    assert!(result.contains("<a href=\"#important-note\">"));
    assert!(result.contains("Important Note</a>"));
    assert!(result.contains("in this quote."));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_detached_modifier_link_to_nonexistent() {
    let norg = r#"Link to {* Nonexistent Section} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link to nonexistent");
    assert!(result.contains("Link to"));
    assert!(result.contains("<a href=\"#nonexistent-section\">"));
    assert!(result.contains("Nonexistent Section</a>"));
    assert!(result.contains("here."));
}

#[test]
fn test_detached_modifier_link_with_numbers() {
    let norg = r#"* Section 1.2.3
Content here.

Reference to {* Section 1.2.3} above."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse detached modifier link with numbers");
    assert!(result.contains("<h1>Section 1.2.3</h1>"));
    assert!(result.contains("Content here."));
    assert!(result.contains("Reference to"));
    assert!(result.contains("<a href=\"#section-123\">"));
    assert!(result.contains("Section 1.2.3</a>"));
    assert!(result.contains("above."));
}

#[test]
fn test_detached_modifier_link_malformed() {
    let norg = r#"* Test Section
Malformed link {* Test Section without closing brace"#;
    let result = parse_and_render_norg(norg);
    assert!(result.is_ok());
    // Should handle gracefully
}

#[test]
fn test_detached_modifier_link_empty() {
    let norg = r#"Empty link {*} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty detached modifier link");
    assert!(result.contains("Empty link"));
    assert!(result.contains("here."));
}