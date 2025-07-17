use norgmill::renderer::parse_and_render_norg;

// Standalone Links Tests
#[test]
fn test_standalone_heading_link() {
    let norg = r#"* My Heading

Link to {* My Heading}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone heading link");
    assert!(result.contains("<h1>My Heading</h1>"));
    assert!(result.contains("<a href=\"#my-heading\">") || result.contains("<a href=\"#My_Heading\">"));
    assert!(result.contains("My Heading</a>"));
}

#[test]
fn test_standalone_heading_link_level_2() {
    let norg = r#"** Second Level Heading

Link to {** Second Level Heading}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone heading link level 2");
    assert!(result.contains("<h2>Second Level Heading</h2>"));
    assert!(result.contains("<a href=\"#second-level-heading\">") || result.contains("<a href=\"#Second_Level_Heading\">"));
    assert!(result.contains("Second Level Heading</a>"));
}

#[test]
fn test_standalone_heading_link_level_3() {
    let norg = r#"*** Third Level Heading

Link to {*** Third Level Heading}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone heading link level 3");
    assert!(result.contains("<h3>Third Level Heading</h3>"));
    assert!(result.contains("<a href=\"#third-level-heading\">") || result.contains("<a href=\"#Third_Level_Heading\">"));
    assert!(result.contains("Third Level Heading</a>"));
}

#[test]
fn test_standalone_heading_link_with_special_chars() {
    let norg = r#"* Heading with & Special Chars!

Link to {* Heading with & Special Chars!}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone heading link with special chars");
    assert!(result.contains("<h1>Heading with &amp; Special Chars!</h1>"));
    assert!(result.contains("<a href=\"#heading-with-special-chars\">") || result.contains("<a href=\"#Heading_with_Special_Chars\">"));
    assert!(result.contains("Heading with &amp; Special Chars!</a>"));
}

#[test]
fn test_standalone_heading_link_with_numbers() {
    let norg = r#"* Section 1.2.3

Link to {* Section 1.2.3}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone heading link with numbers");
    assert!(result.contains("<h1>Section 1.2.3</h1>"));
    assert!(result.contains("<a href=\"#section-123\">") || result.contains("<a href=\"#Section_123\">"));
    assert!(result.contains("Section 1.2.3</a>"));
}

#[test]
fn test_standalone_definition_link() {
    let norg = r#"$ Term Definition
This is the definition of the term.

Link to {$ Term Definition}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone definition link");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Term Definition</dt>"));
    assert!(result.contains("<dd>This is the definition of the term.</dd>"));
    assert!(result.contains("<a href=\"#term-definition\">") || result.contains("<a href=\"#Term_Definition\">"));
    assert!(result.contains("Term Definition</a>"));
}

#[test]
fn test_standalone_footnote_link() {
    let norg = r#"^ Footnote Title
This is a footnote content.

Link to {^ Footnote Title}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone footnote link");
    assert!(result.contains("Footnote Title"));
    assert!(result.contains("This is a footnote content."));
    assert!(result.contains("<a href=\"#footnote-title\">") || result.contains("<a href=\"#Footnote_Title\">"));
    assert!(result.contains("Footnote Title</a>"));
}

#[test]
fn test_standalone_link_in_paragraph() {
    let norg = r#"* Important Section

This paragraph contains a link to {* Important Section} for reference."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone link in paragraph");
    assert!(result.contains("<h1>Important Section</h1>"));
    assert!(result.contains("<p>This paragraph contains a link to"));
    assert!(result.contains("<a href=\"#important-section\">") || result.contains("<a href=\"#Important_Section\">"));
    assert!(result.contains("Important Section</a> for reference.</p>"));
}

#[test]
fn test_standalone_link_in_list() {
    let norg = r#"* Main Topic

- First item
- Link to {* Main Topic}
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone link in list");
    assert!(result.contains("<h1>Main Topic</h1>"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Link to"));
    assert!(result.contains("<a href=\"#main-topic\">") || result.contains("<a href=\"#Main_Topic\">"));
    assert!(result.contains("Main Topic</a></li>"));
    assert!(result.contains("<li>Third item</li>"));
}

#[test]
fn test_standalone_link_in_quote() {
    let norg = r#"* Reference Section

> This quote references {* Reference Section} for more details."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standalone link in quote");
    assert!(result.contains("<h1>Reference Section</h1>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote references"));
    assert!(result.contains("<a href=\"#reference-section\">") || result.contains("<a href=\"#Reference_Section\">"));
    assert!(result.contains("Reference Section</a> for more details."));
}

// Links with Description Tests
#[test]
fn test_link_with_description_basic() {
    let norg = r#"* Target Heading

Link to {* Target Heading}[click here]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with description");
    assert!(result.contains("<h1>Target Heading</h1>"));
    assert!(result.contains("<a href=\"#target-heading\">") || result.contains("<a href=\"#Target_Heading\">"));
    assert!(result.contains("click here</a>"));
}

#[test]
fn test_link_with_description_different_text() {
    let norg = r#"* Technical Documentation

See the {* Technical Documentation}[docs] for more information."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with different description text");
    assert!(result.contains("<h1>Technical Documentation</h1>"));
    assert!(result.contains("See the"));
    assert!(result.contains("<a href=\"#technical-documentation\">") || result.contains("<a href=\"#Technical_Documentation\">"));
    assert!(result.contains("docs</a> for more information."));
}

#[test]
fn test_link_with_description_markup() {
    let norg = r#"* Important Section

{* Important Section}[*Important* /Section/]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with markup in description");
    assert!(result.contains("<h1>Important Section</h1>"));
    assert!(result.contains("<a href=\"#important-section\">") || result.contains("<a href=\"#Important_Section\">"));
    assert!(result.contains("<strong>Important</strong> <em>Section</em></a>"));
}

#[test]
fn test_link_with_description_level_2() {
    let norg = r#"** Subsection Title

{** Subsection Title}[Go to subsection]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse level 2 link with description");
    assert!(result.contains("<h2>Subsection Title</h2>"));
    assert!(result.contains("<a href=\"#subsection-title\">") || result.contains("<a href=\"#Subsection_Title\">"));
    assert!(result.contains("Go to subsection</a>"));
}

#[test]
fn test_link_with_description_level_3() {
    let norg = r#"*** Deep Section

{*** Deep Section}[Deep link]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse level 3 link with description");
    assert!(result.contains("<h3>Deep Section</h3>"));
    assert!(result.contains("<a href=\"#deep-section\">") || result.contains("<a href=\"#Deep_Section\">"));
    assert!(result.contains("Deep link</a>"));
}

#[test]
fn test_link_with_description_definition() {
    let norg = r#"$ Important Term
This is an important definition.

{$ Important Term}[See definition]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition link with description");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Important Term</dt>"));
    assert!(result.contains("<dd>This is an important definition.</dd>"));
    assert!(result.contains("<a href=\"#important-term\">") || result.contains("<a href=\"#Important_Term\">"));
    assert!(result.contains("See definition</a>"));
}

#[test]
fn test_link_with_description_footnote() {
    let norg = r#"^ Note Reference
This is a footnote.

{^ Note Reference}[See note]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote link with description");
    assert!(result.contains("Note Reference"));
    assert!(result.contains("This is a footnote."));
    assert!(result.contains("<a href=\"#note-reference\">") || result.contains("<a href=\"#Note_Reference\">"));
    assert!(result.contains("See note</a>"));
}

#[test]
fn test_link_with_description_empty() {
    let norg = r#"* Target Section

{* Target Section}[]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with empty description");
    assert!(result.contains("<h1>Target Section</h1>"));
    assert!(result.contains("<a href=\"#target-section\">") || result.contains("<a href=\"#Target_Section\">"));
    assert!(result.contains("</a>"));
}

#[test]
fn test_link_with_description_long_text() {
    let norg = r#"* Reference Material

{* Reference Material}[Click here to access the comprehensive reference material]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with long description");
    assert!(result.contains("<h1>Reference Material</h1>"));
    assert!(result.contains("<a href=\"#reference-material\">") || result.contains("<a href=\"#Reference_Material\">"));
    assert!(result.contains("Click here to access the comprehensive reference material</a>"));
}

#[test]
fn test_link_with_description_special_chars() {
    let norg = r#"* Code & Examples

{* Code & Examples}[View code & examples]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with special chars in description");
    assert!(result.contains("<h1>Code &amp; Examples</h1>"));
    assert!(result.contains("<a href=\"#code-examples\">") || result.contains("<a href=\"#Code_Examples\">"));
    assert!(result.contains("View code &amp; examples</a>"));
}

#[test]
fn test_link_with_description_unicode() {
    let norg = r#"* Café Documentation

{* Café Documentation}[Go to café docs]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with unicode in description");
    assert!(result.contains("<h1>Café Documentation</h1>"));
    assert!(result.contains("<a href=\"#café-documentation\">") || result.contains("<a href=\"#Café_Documentation\">"));
    assert!(result.contains("Go to café docs</a>"));
}

// Multiple Links Tests
#[test]
fn test_multiple_links_same_target() {
    let norg = r#"* Main Section

First reference: {* Main Section}
Second reference: {* Main Section}[main section]
Third reference: {* Main Section}[click here]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple links to same target");
    assert!(result.contains("<h1>Main Section</h1>"));
    let link_count = result.matches("href=\"#main-section\"").count() + result.matches("href=\"#Main_Section\"").count();
    assert!(link_count >= 3);
    assert!(result.contains("Main Section</a>"));
    assert!(result.contains("main section</a>"));
    assert!(result.contains("click here</a>"));
}

#[test]
fn test_multiple_links_different_targets() {
    let norg = r#"* First Section
** Second Section

Links: {* First Section}[first] and {** Second Section}[second]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple links to different targets");
    assert!(result.contains("<h1>First Section</h1>"));
    assert!(result.contains("<h2>Second Section</h2>"));
    assert!(result.contains("Links:"));
    assert!(result.contains("first</a> and"));
    assert!(result.contains("second</a>"));
}

#[test]
fn test_link_in_heading() {
    let norg = r#"* Target Section
* Heading with {* Target Section}[link]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link in heading");
    assert!(result.contains("<h1>Target Section</h1>"));
    assert!(result.contains("<h1>Heading with"));
    assert!(result.contains("<a href=\"#target-section\">") || result.contains("<a href=\"#Target_Section\">"));
    assert!(result.contains("link</a></h1>"));
}

// Cross-reference Tests
#[test]
fn test_forward_reference() {
    let norg = r#"See {* Later Section}[later section] for details.

* Later Section
Content here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse forward reference");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"#later-section\">") || result.contains("<a href=\"#Later_Section\">"));
    assert!(result.contains("later section</a> for details.</p>"));
    assert!(result.contains("<h1>Later Section</h1>"));
}

#[test]
fn test_backward_reference() {
    let norg = r#"* Earlier Section
Content here.

Reference to {* Earlier Section}[earlier section]."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse backward reference");
    assert!(result.contains("<h1>Earlier Section</h1>"));
    assert!(result.contains("Reference to"));
    assert!(result.contains("<a href=\"#earlier-section\">") || result.contains("<a href=\"#Earlier_Section\">"));
    assert!(result.contains("earlier section</a>."));
}

// Nested Links Tests
#[test]
fn test_link_in_list_item() {
    let norg = r#"* Reference Section

- Item 1
- Item with {* Reference Section}[link] in it
- Item 3"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link in list item");
    assert!(result.contains("<h1>Reference Section</h1>"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Item with"));
    assert!(result.contains("<a href=\"#reference-section\">") || result.contains("<a href=\"#Reference_Section\">"));
    assert!(result.contains("link</a> in it</li>"));
}

#[test]
fn test_link_in_quote_block() {
    let norg = r#"* Important Note

> This is a quote that references {* Important Note}[the note] above."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link in quote block");
    assert!(result.contains("<h1>Important Note</h1>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote that references"));
    assert!(result.contains("<a href=\"#important-note\">") || result.contains("<a href=\"#Important_Note\">"));
    assert!(result.contains("the note</a> above."));
}

// Error Cases
#[test]
fn test_link_to_nonexistent_target() {
    let norg = "{* Nonexistent Section}[broken link]";
    let result = parse_and_render_norg(norg).expect("Failed to parse link to nonexistent target");
    // Should still create a link, even if target doesn't exist
    assert!(result.contains("<a href=\"#nonexistent-section\">") || result.contains("<a href=\"#Nonexistent_Section\">"));
    assert!(result.contains("broken link</a>"));
}

#[test]
fn test_malformed_link_missing_closing_brace() {
    let norg = r#"* Target Section

{* Target Section"#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_malformed_link_missing_description_closing() {
    let norg = r#"* Target Section

{* Target Section}[description"#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_link_with_line_break_in_description() {
    let norg = r#"* Target Section

{* Target Section}[description
with line break]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link with line break in description");
    assert!(result.contains("<h1>Target Section</h1>"));
    assert!(result.contains("<a href=\"#target-section\">") || result.contains("<a href=\"#Target_Section\">"));
    assert!(result.contains("description"));
    assert!(result.contains("with line break</a>"));
}