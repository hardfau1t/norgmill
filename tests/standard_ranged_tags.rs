use norgmill::renderer::parse_and_render_norg;

// Comment Tag Tests
#[test]
fn test_comment_tag_basic() {
    let norg = r#"|comment
This is a comment that should not appear in the rendered output.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic comment tag");
    // Comment should not appear in the output
    assert!(!result.contains("This is a comment that should not appear in the rendered output."));
    // Should not have any comment-related HTML elements visible
    assert!(!result.contains("comment"));
}

#[test]
fn test_comment_tag_multiline() {
    let norg = r#"|comment
This is a multi-line comment
that spans several lines
and should not appear in the output.

It can contain paragraph breaks too.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline comment tag");
    // None of the comment content should appear
    assert!(!result.contains("This is a multi-line comment"));
    assert!(!result.contains("that spans several lines"));
    assert!(!result.contains("and should not appear in the output."));
    assert!(!result.contains("It can contain paragraph breaks too."));
}

#[test]
fn test_comment_tag_with_markup() {
    let norg = r#"|comment
This comment contains *bold* and /italic/ markup.
It also has `code` and {https://example.com}[links].
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse comment tag with markup");
    // No markup should appear in the output
    assert!(!result.contains("bold"));
    assert!(!result.contains("italic"));
    assert!(!result.contains("code"));
    assert!(!result.contains("https://example.com"));
    assert!(!result.contains("links"));
}

#[test]
fn test_comment_tag_between_content() {
    let norg = r#"This is visible content before the comment.

|comment
This is a hidden comment.
|end

This is visible content after the comment."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse comment tag between content");
    assert!(result.contains("This is visible content before the comment."));
    assert!(result.contains("This is visible content after the comment."));
    assert!(!result.contains("This is a hidden comment."));
}

#[test]
fn test_multiple_comment_tags() {
    let norg = r#"|comment
First comment.
|end

Visible content.

|comment
Second comment.
|end

More visible content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple comment tags");
    assert!(result.contains("Visible content."));
    assert!(result.contains("More visible content."));
    assert!(!result.contains("First comment."));
    assert!(!result.contains("Second comment."));
}

// Example Tag Tests
#[test]
fn test_example_tag_basic() {
    let norg = r#"|example
This is an example that demonstrates the concept.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic example tag");
    assert!(result.contains("This is an example that demonstrates the concept."));
    // Should be wrapped in some kind of example container
    assert!(result.contains("<div") || result.contains("<section") || result.contains("<aside") || result.contains("example"));
}

#[test]
fn test_example_tag_with_code() {
    let norg = r#"|example
Here's how to use this function:

@code python
def greet(name):
    return f"Hello, {name}!"

print(greet("World"))
@end

This example shows basic usage.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse example tag with code");
    assert!(result.contains("Here's how to use this function:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def greet(name):"));
    assert!(result.contains("return f\"Hello, {name}!\""));
    assert!(result.contains("print(greet(\"World\"))"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("This example shows basic usage."));
}

#[test]
fn test_example_tag_with_lists() {
    let norg = r#"|example
Steps to follow:

- First step
- Second step
- Third step

Follow these steps in order.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse example tag with lists");
    assert!(result.contains("Steps to follow:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First step</li>"));
    assert!(result.contains("<li>Second step</li>"));
    assert!(result.contains("<li>Third step</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("Follow these steps in order."));
}

#[test]
fn test_example_tag_with_headings() {
    let norg = r#"|example
* Example Section
This is the example content.

** Sub-example
More detailed example information.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse example tag with headings");
    assert!(result.contains("<h1>Example Section</h1>"));
    assert!(result.contains("This is the example content."));
    assert!(result.contains("<h2>Sub-example</h2>"));
    assert!(result.contains("More detailed example information."));
}

#[test]
fn test_example_tag_with_markup() {
    let norg = r#"|example
This example shows *bold text*, /italic text/, and `inline code`.

You can also use _underline_ and -strikethrough-.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse example tag with markup");
    assert!(result.contains("This example shows <strong>bold text</strong>, <em>italic text</em>, and <code>inline code</code>."));
    assert!(result.contains("You can also use <u>underline</u> and <del>strikethrough</del>."));
}

#[test]
fn test_multiple_example_tags() {
    let norg = r#"|example
First example demonstrating concept A.
|end

Regular content between examples.

|example
Second example demonstrating concept B.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple example tags");
    assert!(result.contains("First example demonstrating concept A."));
    assert!(result.contains("Regular content between examples."));
    assert!(result.contains("Second example demonstrating concept B."));
}

// Details Tag Tests
#[test]
fn test_details_tag_basic() {
    let norg = r#"|details
This is detailed information that can be collapsed.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic details tag");
    assert!(result.contains("This is detailed information that can be collapsed."));
    // Should be wrapped in HTML details element or similar
    assert!(result.contains("<details") || result.contains("<summary") || result.contains("details"));
}

#[test]
fn test_details_tag_with_summary() {
    let norg = r#"|details Summary: Click to expand
This is the detailed content that is initially hidden.

It can contain multiple paragraphs and various formatting.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse details tag with summary");
    assert!(result.contains("Summary: Click to expand"));
    assert!(result.contains("This is the detailed content that is initially hidden."));
    assert!(result.contains("It can contain multiple paragraphs and various formatting."));
    assert!(result.contains("<details") || result.contains("<summary"));
}

#[test]
fn test_details_tag_with_complex_content() {
    let norg = r#"|details Technical Details
* Implementation
The system uses a modular architecture.

** Components
- Parser
- Renderer
- Utilities

@code rust
fn main() {
    println!("Hello, world!");
}
@end
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse details tag with complex content");
    assert!(result.contains("Technical Details"));
    assert!(result.contains("<h1>Implementation</h1>"));
    assert!(result.contains("The system uses a modular architecture."));
    assert!(result.contains("<h2>Components</h2>"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Parser</li>"));
    assert!(result.contains("<li>Renderer</li>"));
    assert!(result.contains("<li>Utilities</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("fn main() {"));
    assert!(result.contains("println!(\"Hello, world!\");"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_details_tag_nested() {
    let norg = r#"|details Outer Details
This is the outer details content.

|details Inner Details
This is nested details content.
|end

Back to outer details.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested details tags");
    assert!(result.contains("Outer Details"));
    assert!(result.contains("This is the outer details content."));
    assert!(result.contains("Inner Details"));
    assert!(result.contains("This is nested details content."));
    assert!(result.contains("Back to outer details."));
}

#[test]
fn test_multiple_details_tags() {
    let norg = r#"|details First Section
Details for the first section.
|end

|details Second Section
Details for the second section.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple details tags");
    assert!(result.contains("First Section"));
    assert!(result.contains("Details for the first section."));
    assert!(result.contains("Second Section"));
    assert!(result.contains("Details for the second section."));
}

// Group Tag Tests
#[test]
fn test_group_tag_basic() {
    let norg = r#"|group
This content is grouped together.
It forms a logical unit.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic group tag");
    assert!(result.contains("This content is grouped together."));
    assert!(result.contains("It forms a logical unit."));
    // Should be wrapped in some kind of group container
    assert!(result.contains("<div") || result.contains("<section") || result.contains("group"));
}

#[test]
fn test_group_tag_with_title() {
    let norg = r#"|group Related Information
This group contains related information.

All items in this group are connected.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse group tag with title");
    assert!(result.contains("Related Information"));
    assert!(result.contains("This group contains related information."));
    assert!(result.contains("All items in this group are connected."));
}

#[test]
fn test_group_tag_with_mixed_content() {
    let norg = r#"|group Documentation Group
* Overview
This section provides an overview.

- Feature 1
- Feature 2
- Feature 3

> Important note about the features.

@code example
example_code();
@end
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse group tag with mixed content");
    assert!(result.contains("Documentation Group"));
    assert!(result.contains("<h1>Overview</h1>"));
    assert!(result.contains("This section provides an overview."));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Feature 1</li>"));
    assert!(result.contains("<li>Feature 2</li>"));
    assert!(result.contains("<li>Feature 3</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Important note about the features."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("example_code();"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_group_tag_nested() {
    let norg = r#"|group Outer Group
This is the outer group.

|group Inner Group
This is the inner group.
|end

Back to the outer group.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested group tags");
    assert!(result.contains("Outer Group"));
    assert!(result.contains("This is the outer group."));
    assert!(result.contains("Inner Group"));
    assert!(result.contains("This is the inner group."));
    assert!(result.contains("Back to the outer group."));
}

#[test]
fn test_multiple_group_tags() {
    let norg = r#"|group First Group
Content for the first group.
|end

Regular content between groups.

|group Second Group
Content for the second group.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple group tags");
    assert!(result.contains("First Group"));
    assert!(result.contains("Content for the first group."));
    assert!(result.contains("Regular content between groups."));
    assert!(result.contains("Second Group"));
    assert!(result.contains("Content for the second group."));
}

// Mixed Standard Ranged Tags Tests
#[test]
fn test_mixed_standard_ranged_tags() {
    let norg = r#"|example
This is an example.
|end

|comment
This is a comment.
|end

|details More Information
This is detailed information.
|end

|group Related Content
This is grouped content.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed standard ranged tags");
    assert!(result.contains("This is an example."));
    assert!(!result.contains("This is a comment."));
    assert!(result.contains("More Information"));
    assert!(result.contains("This is detailed information."));
    assert!(result.contains("Related Content"));
    assert!(result.contains("This is grouped content."));
}

#[test]
fn test_standard_ranged_tags_in_other_contexts() {
    let norg = r#"* Main Section

|example
Example within a section.
|end

- List item
- Another item with details:
  |details Details for item
  Additional information about this item.
  |end
- Third item

> Quote with example:
  |example
  Example within a quote.
  |end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standard ranged tags in other contexts");
    assert!(result.contains("<h1>Main Section</h1>"));
    assert!(result.contains("Example within a section."));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>List item</li>"));
    assert!(result.contains("Another item with details:"));
    assert!(result.contains("Details for item"));
    assert!(result.contains("Additional information about this item."));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Quote with example:"));
    assert!(result.contains("Example within a quote."));
    assert!(result.contains("</blockquote>"));
}

// Complex Nesting Tests
#[test]
fn test_complex_nesting_standard_ranged_tags() {
    let norg = r#"|group Main Group
This is the main group.

|example
Example within the group.

|details Example Details
Details about the example.
|end

More example content.
|end

|comment
This comment is within the group but won't be visible.
|end

Back to main group content.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex nesting of standard ranged tags");
    assert!(result.contains("Main Group"));
    assert!(result.contains("This is the main group."));
    assert!(result.contains("Example within the group."));
    assert!(result.contains("Example Details"));
    assert!(result.contains("Details about the example."));
    assert!(result.contains("More example content."));
    assert!(!result.contains("This comment is within the group but won't be visible."));
    assert!(result.contains("Back to main group content."));
}

// Standard Ranged Tags with Attributes Tests
#[test]
fn test_example_tag_with_attributes() {
    let norg = r#"|example lang:python title:"Python Example"
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse example tag with attributes");
    assert!(result.contains("def fibonacci(n):"));
    assert!(result.contains("if n <= 1:"));
    assert!(result.contains("return n"));
    assert!(result.contains("return fibonacci(n-1) + fibonacci(n-2)"));
    assert!(result.contains("print(fibonacci(10))"));
    // Should handle attributes appropriately
    assert!(result.contains("python") || result.contains("Python Example"));
}

#[test]
fn test_details_tag_with_attributes() {
    let norg = r#"|details open:true class:"important"
This details section is open by default.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse details tag with attributes");
    assert!(result.contains("This details section is open by default."));
    // Should handle attributes appropriately
    assert!(result.contains("open") || result.contains("important"));
}

#[test]
fn test_group_tag_with_attributes() {
    let norg = r#"|group id:"main-group" class:"highlight"
This group has specific attributes.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse group tag with attributes");
    assert!(result.contains("This group has specific attributes."));
    // Should handle attributes appropriately
    assert!(result.contains("main-group") || result.contains("highlight"));
}

// Empty Tags Tests
#[test]
fn test_empty_example_tag() {
    let norg = r#"|example
|end"#;
    let _result = parse_and_render_norg(norg).expect("Failed to parse empty example tag");
    // Should handle empty tags gracefully
}

#[test]
fn test_empty_details_tag() {
    let norg = r#"|details Empty Details
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty details tag");
    assert!(result.contains("Empty Details"));
}

#[test]
fn test_empty_group_tag() {
    let norg = r#"|group Empty Group
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty group tag");
    assert!(result.contains("Empty Group"));
}

// Error Cases and Edge Cases
#[test]
fn test_unclosed_example_tag() {
    let norg = r#"|example
This example is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_unclosed_details_tag() {
    let norg = r#"|details Unclosed Details
This details section is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_unclosed_group_tag() {
    let norg = r#"|group Unclosed Group
This group is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_unclosed_comment_tag() {
    let norg = r#"|comment
This comment is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_malformed_tag_names() {
    let norg = r#"|exampl
This is a malformed tag name.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed tag name");
    // Should handle as regular text or custom tag
    assert!(result.contains("This is a malformed tag name."));
}

#[test]
fn test_standard_ranged_tags_with_special_characters() {
    let norg = r#"|example
Content with & < > " ' special characters.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standard ranged tags with special characters");
    assert!(result.contains("Content with &amp; &lt; &gt; &quot; &#x27; special characters."));
}

#[test]
fn test_standard_ranged_tags_with_unicode() {
    let norg = r#"|example
Content with émojis 🌟 and Unicode characters.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standard ranged tags with unicode");
    assert!(result.contains("Content with émojis 🌟 and Unicode characters."));
}

#[test]
fn test_standard_ranged_tags_with_line_breaks() {
    let norg = r#"|example
Content with
multiple lines
and line breaks.
|end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standard ranged tags with line breaks");
    assert!(result.contains("Content with"));
    assert!(result.contains("multiple lines"));
    assert!(result.contains("and line breaks."));
}

#[test]
fn test_standard_ranged_tags_case_sensitivity() {
    let norg = r#"|EXAMPLE
This tests case sensitivity.
|END"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse case sensitive standard ranged tags");
    assert!(result.contains("This tests case sensitivity."));
    // Should handle case sensitivity appropriately
}

#[test]
fn test_standard_ranged_tags_with_whitespace() {
    let norg = r#"|example   
   This has whitespace in the tag.   
|end   "#;
    let result = parse_and_render_norg(norg).expect("Failed to parse standard ranged tags with whitespace");
    assert!(result.contains("This has whitespace in the tag."));
}
