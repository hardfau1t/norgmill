use norgmill::renderer::parse_and_render_norg;

// Single Footnote Tests
#[test]
fn test_footnote_basic() {
    let norg = r#"^ Basic Footnote
This is a basic footnote content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic footnote");
    assert!(result.contains("Basic Footnote"));
    assert!(result.contains("This is a basic footnote content."));
    // Should contain footnote structure (could be aside, section, or div with footnote class)
    assert!(result.contains("footnote") || result.contains("<aside") || result.contains("id="));
}

#[test]
fn test_footnote_single_word_title() {
    let norg = r#"^ Note
Important note content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single word footnote");
    assert!(result.contains("Note"));
    assert!(result.contains("Important note content."));
}

#[test]
fn test_footnote_multi_word_title() {
    let norg = r#"^ Important Reference Note
This footnote has a multi-word title."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multi-word footnote");
    assert!(result.contains("Important Reference Note"));
    assert!(result.contains("This footnote has a multi-word title."));
}

#[test]
fn test_footnote_with_markup_in_title() {
    let norg = r#"^ *Important* Note
This footnote has markup in the title."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with markup in title");
    assert!(result.contains("<strong>Important</strong> Note"));
    assert!(result.contains("This footnote has markup in the title."));
}

#[test]
fn test_footnote_with_markup_in_content() {
    let norg = r#"^ Technical Note
This footnote has *bold* and /italic/ text in the content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with markup in content");
    assert!(result.contains("Technical Note"));
    assert!(result.contains("This footnote has <strong>bold</strong> and <em>italic</em> text in the content."));
}

#[test]
fn test_footnote_with_code_in_content() {
    let norg = r#"^ Code Note
This footnote contains `inline code` for reference."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with code in content");
    assert!(result.contains("Code Note"));
    assert!(result.contains("This footnote contains <code>inline code</code> for reference."));
}

#[test]
fn test_footnote_with_link_in_content() {
    let norg = r#"^ Reference Note
See {https://example.com}[the documentation] for more details."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with link in content");
    assert!(result.contains("Reference Note"));
    assert!(result.contains("See"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("the documentation</a> for more details."));
}

#[test]
fn test_footnote_multiline_content() {
    let norg = r#"^ Extended Note
This footnote spans multiple lines
and contains detailed information
about the referenced topic."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline footnote");
    assert!(result.contains("Extended Note"));
    assert!(result.contains("This footnote spans multiple lines"));
    assert!(result.contains("and contains detailed information"));
    assert!(result.contains("about the referenced topic."));
}

#[test]
fn test_footnote_with_special_characters() {
    let norg = r#"^ Special Characters Note
This footnote contains & < > " ' special characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with special characters");
    assert!(result.contains("Special Characters Note"));
    assert!(result.contains("This footnote contains &amp; &lt; &gt; &quot; &#x27; special characters."));
}

#[test]
fn test_footnote_with_unicode() {
    let norg = r#"^ Unicode Note
This footnote contains émojis 🌟 and Unicode characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with unicode");
    assert!(result.contains("Unicode Note"));
    assert!(result.contains("This footnote contains émojis 🌟 and Unicode characters."));
}

#[test]
fn test_footnote_with_numbers() {
    let norg = r#"^ Version 2.0 Note
This footnote refers to version 2.0 of the software."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with numbers");
    assert!(result.contains("Version 2.0 Note"));
    assert!(result.contains("This footnote refers to version 2.0 of the software."));
}

// Multiple Footnotes Tests
#[test]
fn test_multiple_footnotes_grouped() {
    let norg = r#"^ First Note
Content of the first footnote.
^ Second Note
Content of the second footnote.
^ Third Note
Content of the third footnote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple footnotes");
    assert!(result.contains("First Note"));
    assert!(result.contains("Content of the first footnote."));
    assert!(result.contains("Second Note"));
    assert!(result.contains("Content of the second footnote."));
    assert!(result.contains("Third Note"));
    assert!(result.contains("Content of the third footnote."));
}

#[test]
fn test_footnotes_separated_by_paragraph_break() {
    let norg = r#"^ First Note
Content of the first footnote.

^ Second Note
Content of the second footnote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnotes separated by paragraph break");
    assert!(result.contains("First Note"));
    assert!(result.contains("Content of the first footnote."));
    assert!(result.contains("Second Note"));
    assert!(result.contains("Content of the second footnote."));
}

#[test]
fn test_footnotes_mixed_with_other_elements() {
    let norg = r#"^ Technical Note
This is a footnote.

This is a regular paragraph.

^ Another Note
Another footnote here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnotes mixed with other elements");
    assert!(result.contains("Technical Note"));
    assert!(result.contains("This is a footnote."));
    assert!(result.contains("<p>This is a regular paragraph.</p>"));
    assert!(result.contains("Another Note"));
    assert!(result.contains("Another footnote here."));
}

// Ranged Footnote Tests
#[test]
fn test_ranged_footnote_basic() {
    let norg = r#"^^ Complex Note
This is a ranged footnote
that can span multiple paragraphs.

It can contain more complex content.
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic ranged footnote");
    assert!(result.contains("Complex Note"));
    assert!(result.contains("This is a ranged footnote"));
    assert!(result.contains("that can span multiple paragraphs."));
    assert!(result.contains("It can contain more complex content."));
}

#[test]
fn test_ranged_footnote_with_lists() {
    let norg = r#"^^ Note with List
This footnote contains:
- First item
- Second item
- Third item
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged footnote with lists");
    assert!(result.contains("Note with List"));
    assert!(result.contains("This footnote contains:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_ranged_footnote_with_code_block() {
    let norg = r#"^^ Code Example Note
Here's a code example:

@code python
def hello():
    print("Hello, world!")
@end
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged footnote with code block");
    assert!(result.contains("Code Example Note"));
    assert!(result.contains("Here's a code example:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def hello():"));
    assert!(result.contains("print(\"Hello, world!\")"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_ranged_footnote_with_quotes() {
    let norg = r#"^^ Important Quote Note
According to the author:

> This is a quote within the footnote
  that provides additional context.
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged footnote with quotes");
    assert!(result.contains("Important Quote Note"));
    assert!(result.contains("According to the author:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote within the footnote"));
    assert!(result.contains("that provides additional context."));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_ranged_footnote_with_headings() {
    let norg = r#"^^ Comprehensive Note
* Overview
This section provides an overview.

** Details
More detailed information here.
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged footnote with headings");
    assert!(result.contains("Comprehensive Note"));
    assert!(result.contains("<h1>Overview</h1>"));
    assert!(result.contains("This section provides an overview."));
    assert!(result.contains("<h2>Details</h2>"));
    assert!(result.contains("More detailed information here."));
}

#[test]
fn test_ranged_footnote_empty_content() {
    let norg = r#"^^ Empty Note
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged footnote with empty content");
    assert!(result.contains("Empty Note"));
}

#[test]
fn test_multiple_ranged_footnotes() {
    let norg = r#"^^ First Complex Note
This is the first ranged footnote.

It has multiple paragraphs.
^^

^^ Second Complex Note
This is the second ranged footnote.

It also has multiple paragraphs.
^^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple ranged footnotes");
    assert!(result.contains("First Complex Note"));
    assert!(result.contains("This is the first ranged footnote."));
    assert!(result.contains("It has multiple paragraphs."));
    assert!(result.contains("Second Complex Note"));
    assert!(result.contains("This is the second ranged footnote."));
    assert!(result.contains("It also has multiple paragraphs."));
}

// Mixed Single and Ranged Footnotes
#[test]
fn test_mixed_single_and_ranged_footnotes() {
    let norg = r#"^ Simple Note
Simple footnote content.

^^ Complex Note
Complex footnote with multiple paragraphs.

More content here.
^^

^ Another Simple Note
Another simple footnote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed single and ranged footnotes");
    assert!(result.contains("Simple Note"));
    assert!(result.contains("Simple footnote content."));
    assert!(result.contains("Complex Note"));
    assert!(result.contains("Complex footnote with multiple paragraphs."));
    assert!(result.contains("More content here."));
    assert!(result.contains("Another Simple Note"));
    assert!(result.contains("Another simple footnote."));
}

// Footnotes in Other Contexts
#[test]
fn test_footnote_in_list() {
    let norg = r#"- First item
- Item with footnote:
  ^ List Note
  Footnote content within a list.
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Item with footnote:"));
    assert!(result.contains("List Note"));
    assert!(result.contains("Footnote content within a list."));
    assert!(result.contains("</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_footnote_in_quote() {
    let norg = r#"> This quote contains a footnote:
  ^ Quoted Note
  Footnote within a quote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains a footnote:"));
    assert!(result.contains("Quoted Note"));
    assert!(result.contains("Footnote within a quote."));
    assert!(result.contains("</blockquote>"));
}

// Links to Footnotes
#[test]
fn test_link_to_footnote() {
    let norg = r#"^ Important Note
This is an important footnote.

See {^ Important Note}[the note] for details."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link to footnote");
    assert!(result.contains("Important Note"));
    assert!(result.contains("This is an important footnote."));
    assert!(result.contains("See"));
    assert!(result.contains("<a href=\"#important-note\">") || result.contains("<a href=\"#Important_Note\">"));
    assert!(result.contains("the note</a> for details."));
}

// Footnote References in Text
#[test]
fn test_footnote_reference_in_text() {
    let norg = r#"This is some text with a footnote reference.^[See note below]

^ See note below
This is the footnote content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote reference in text");
    assert!(result.contains("This is some text with a footnote reference."));
    assert!(result.contains("See note below"));
    assert!(result.contains("This is the footnote content."));
}

#[test]
fn test_multiple_footnote_references() {
    let norg = r#"First reference^[Note 1] and second reference^[Note 2].

^ Note 1
Content of first footnote.

^ Note 2
Content of second footnote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple footnote references");
    assert!(result.contains("First reference"));
    assert!(result.contains("and second reference"));
    assert!(result.contains("Note 1"));
    assert!(result.contains("Content of first footnote."));
    assert!(result.contains("Note 2"));
    assert!(result.contains("Content of second footnote."));
}

// Footnotes with Anchors
#[test]
fn test_footnote_with_anchor() {
    let norg = r#"^ My Footnote
This footnote has an anchor for easy referencing.

[footnote-anchor]{^ My Footnote}

Link to the [footnote-anchor] here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with anchor");
    assert!(result.contains("My Footnote"));
    assert!(result.contains("This footnote has an anchor for easy referencing."));
    assert!(result.contains("footnote-anchor"));
    assert!(result.contains("Link to the"));
}

// Error Cases
#[test]
fn test_footnote_missing_content() {
    let norg = "^ Note Without Content";
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote without content");
    assert!(result.contains("Note Without Content"));
}

#[test]
fn test_ranged_footnote_missing_closing() {
    let norg = r#"^^ Unclosed Note
This ranged footnote has no closing marker."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_footnote_empty_title() {
    let norg = r#"^ 
This footnote has an empty title."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with empty title");
    assert!(result.contains("This footnote has an empty title."));
}

#[test]
fn test_footnote_whitespace_only_title() {
    let norg = r#"^    
This footnote has a whitespace-only title."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with whitespace-only title");
    assert!(result.contains("This footnote has a whitespace-only title."));
}

#[test]
fn test_footnote_with_inline_link_target() {
    let norg = r#"^ <footnote-target>Important Note
This footnote can be referenced by its inline target.

Link to {# footnote-target}[the footnote]."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse footnote with inline link target");
    assert!(result.contains("Important Note"));
    assert!(result.contains("This footnote can be referenced by its inline target."));
    assert!(result.contains("Link to"));
    assert!(result.contains("the footnote</a>"));
}

#[test]
fn test_footnote_with_bibliography_style() {
    let norg = r#"^ Smith, J. (2023)
Smith, J. (2023). /Title of Article/. *Journal Name*, 15(3), 123-145.

Referenced work by {^ Smith, J. (2023)}[Smith (2023)]."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse bibliography-style footnote");
    assert!(result.contains("Smith, J. (2023)"));
    assert!(result.contains("<em>Title of Article</em>"));
    assert!(result.contains("<strong>Journal Name</strong>"));
    assert!(result.contains("Referenced work by"));
    assert!(result.contains("Smith (2023)</a>"));
}

#[test]
fn test_footnote_numbering_sequence() {
    let norg = r#"^ 1
First footnote.

^ 2
Second footnote.

^ 3
Third footnote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse numbered footnotes");
    assert!(result.contains("First footnote."));
    assert!(result.contains("Second footnote."));
    assert!(result.contains("Third footnote."));
}