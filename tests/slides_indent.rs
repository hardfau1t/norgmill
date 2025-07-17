use norgmill::renderer::parse_and_render_norg;

// Basic Slide Tests
#[test]
fn test_slide_basic() {
    let norg = r#":: Slide 1
This is the content of the first slide.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic slide");
    assert!(result.contains("Slide 1"));
    assert!(result.contains("This is the content of the first slide."));
    // Should be wrapped in some kind of slide container (section, div, etc.)
    assert!(result.contains("<section") || result.contains("<div") || result.contains("slide"));
}

#[test]
fn test_slide_with_title() {
    let norg = r#":: Introduction to Norg
Welcome to the Norg markup language presentation.

Key features:
- Hierarchical structure
- Rich formatting
- Easy to learn
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with title");
    assert!(result.contains("Introduction to Norg"));
    assert!(result.contains("Welcome to the Norg markup language presentation."));
    assert!(result.contains("Key features:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Hierarchical structure</li>"));
    assert!(result.contains("<li>Rich formatting</li>"));
    assert!(result.contains("<li>Easy to learn</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_slide_with_headings() {
    let norg = r#":: Main Slide
* Section 1
Content for section 1.

** Subsection 1.1
Content for subsection 1.1.

* Section 2
Content for section 2.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with headings");
    assert!(result.contains("Main Slide"));
    assert!(result.contains("<h1>Section 1</h1>"));
    assert!(result.contains("Content for section 1."));
    assert!(result.contains("<h2>Subsection 1.1</h2>"));
    assert!(result.contains("Content for subsection 1.1."));
    assert!(result.contains("<h1>Section 2</h1>"));
    assert!(result.contains("Content for section 2."));
}

#[test]
fn test_slide_with_lists() {
    let norg = r#":: List Examples
Unordered list:
- First item
- Second item
- Third item

Ordered list:
~ First step
~ Second step
~ Third step
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with lists");
    assert!(result.contains("List Examples"));
    assert!(result.contains("Unordered list:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("Ordered list:"));
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>First step</li>"));
    assert!(result.contains("<li>Second step</li>"));
    assert!(result.contains("<li>Third step</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_slide_with_code() {
    let norg = r#":: Code Example
Here's a Python example:

@code python
def hello_world():
    print("Hello, World!")
    return "success"
@end

That's the basic structure.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with code");
    assert!(result.contains("Code Example"));
    assert!(result.contains("Here's a Python example:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def hello_world():"));
    assert!(result.contains("print(\"Hello, World!\")"));
    assert!(result.contains("return \"success\""));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("That's the basic structure."));
}

#[test]
fn test_slide_with_quotes() {
    let norg = r#":: Inspirational Quote
As someone once said:

> The best way to predict the future
  is to create it.

That's why we build tools.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with quotes");
    assert!(result.contains("Inspirational Quote"));
    assert!(result.contains("As someone once said:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("The best way to predict the future"));
    assert!(result.contains("is to create it."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("That's why we build tools."));
}

#[test]
fn test_slide_with_markup() {
    let norg = r#":: Formatting Examples
This slide shows *bold text*, /italic text/, and `code`.

You can also use _underlined_ and -strikethrough- text.

Links work too: {https://example.com}[Example Site]
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with markup");
    assert!(result.contains("Formatting Examples"));
    assert!(result.contains("This slide shows <strong>bold text</strong>, <em>italic text</em>, and <code>code</code>."));
    assert!(result.contains("You can also use <u>underlined</u> and <del>strikethrough</del> text."));
    assert!(result.contains("Links work too:"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("Example Site</a>"));
}

#[test]
fn test_slide_empty() {
    let norg = r#":: Empty Slide
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty slide");
    assert!(result.contains("Empty Slide"));
    // Should handle empty slides gracefully
    assert!(result.contains("<section") || result.contains("<div") || result.contains("slide"));
}

// Multiple Slides Tests
#[test]
fn test_multiple_slides() {
    let norg = r#":: First Slide
Content of the first slide.
::

:: Second Slide
Content of the second slide.
::

:: Third Slide
Content of the third slide.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple slides");
    assert!(result.contains("First Slide"));
    assert!(result.contains("Content of the first slide."));
    assert!(result.contains("Second Slide"));
    assert!(result.contains("Content of the second slide."));
    assert!(result.contains("Third Slide"));
    assert!(result.contains("Content of the third slide."));
    // Should have multiple slide containers
    let slide_count = result.matches("<section").count() + result.matches("<div").count();
    assert!(slide_count >= 3);
}

#[test]
fn test_slides_with_content_between() {
    let norg = r#":: First Slide
Content of the first slide.
::

This is regular content between slides.

:: Second Slide
Content of the second slide.
::

More regular content.

:: Third Slide
Content of the third slide.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slides with content between");
    assert!(result.contains("First Slide"));
    assert!(result.contains("Content of the first slide."));
    assert!(result.contains("This is regular content between slides."));
    assert!(result.contains("Second Slide"));
    assert!(result.contains("Content of the second slide."));
    assert!(result.contains("More regular content."));
    assert!(result.contains("Third Slide"));
    assert!(result.contains("Content of the third slide."));
}

// Nested Slides Tests
#[test]
fn test_nested_slides() {
    let norg = r#":: Main Slide
This is the main slide content.

:: Nested Slide
This is nested inside the main slide.
::

Back to main slide content.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested slides");
    assert!(result.contains("Main Slide"));
    assert!(result.contains("This is the main slide content."));
    assert!(result.contains("Nested Slide"));
    assert!(result.contains("This is nested inside the main slide."));
    assert!(result.contains("Back to main slide content."));
}

// Basic Indent Segment Tests
#[test]
fn test_indent_segment_basic() {
    let norg = r#": Indented Content
This content is indented and grouped together.
It forms a logical unit.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic indent segment");
    assert!(result.contains("Indented Content"));
    assert!(result.contains("This content is indented and grouped together."));
    assert!(result.contains("It forms a logical unit."));
    // Should be wrapped in some kind of indent container (div, section, etc.)
    assert!(result.contains("<div") || result.contains("<section") || result.contains("indent"));
}

#[test]
fn test_indent_segment_with_title() {
    let norg = r#": Important Note
This is an important note that needs to be highlighted.

It can contain multiple paragraphs and various formatting.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with title");
    assert!(result.contains("Important Note"));
    assert!(result.contains("This is an important note that needs to be highlighted."));
    assert!(result.contains("It can contain multiple paragraphs and various formatting."));
}

#[test]
fn test_indent_segment_with_lists() {
    let norg = r#": Task List
Here are the tasks to complete:

- Review documentation
- Update examples
- Test functionality
- Deploy changes

All tasks should be completed by end of week.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with lists");
    assert!(result.contains("Task List"));
    assert!(result.contains("Here are the tasks to complete:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Review documentation</li>"));
    assert!(result.contains("<li>Update examples</li>"));
    assert!(result.contains("<li>Test functionality</li>"));
    assert!(result.contains("<li>Deploy changes</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("All tasks should be completed by end of week."));
}

#[test]
fn test_indent_segment_with_code() {
    let norg = r#": Code Example
Here's how to use this function:

@code javascript
function greet(name) {
    return `Hello, ${name}!`;
}
@end

This function returns a greeting string.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with code");
    assert!(result.contains("Code Example"));
    assert!(result.contains("Here's how to use this function:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("function greet(name) {"));
    assert!(result.contains("return `Hello, ${name}!`;"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("This function returns a greeting string."));
}

#[test]
fn test_indent_segment_with_headings() {
    let norg = r#": Documentation Section
* Overview
This section provides an overview of the system.

** Architecture
The system follows a modular architecture.

** Components
Key components include:
- Parser
- Renderer
- Utilities
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with headings");
    assert!(result.contains("Documentation Section"));
    assert!(result.contains("<h1>Overview</h1>"));
    assert!(result.contains("This section provides an overview of the system."));
    assert!(result.contains("<h2>Architecture</h2>"));
    assert!(result.contains("The system follows a modular architecture."));
    assert!(result.contains("<h2>Components</h2>"));
    assert!(result.contains("Key components include:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Parser</li>"));
    assert!(result.contains("<li>Renderer</li>"));
    assert!(result.contains("<li>Utilities</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_indent_segment_with_quotes() {
    let norg = r#": Quotation Section
As the author states:

> This is a significant development
  in the field of markup languages.

The quote emphasizes the importance of the innovation.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with quotes");
    assert!(result.contains("Quotation Section"));
    assert!(result.contains("As the author states:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a significant development"));
    assert!(result.contains("in the field of markup languages."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("The quote emphasizes the importance of the innovation."));
}

#[test]
fn test_indent_segment_empty() {
    let norg = r#": Empty Segment
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty indent segment");
    assert!(result.contains("Empty Segment"));
    // Should handle empty segments gracefully
}

// Multiple Indent Segments Tests
#[test]
fn test_multiple_indent_segments() {
    let norg = r#": First Section
Content of the first section.
:

: Second Section
Content of the second section.
:

: Third Section
Content of the third section.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple indent segments");
    assert!(result.contains("First Section"));
    assert!(result.contains("Content of the first section."));
    assert!(result.contains("Second Section"));
    assert!(result.contains("Content of the second section."));
    assert!(result.contains("Third Section"));
    assert!(result.contains("Content of the third section."));
}

#[test]
fn test_indent_segments_with_content_between() {
    let norg = r#": First Section
Content of the first section.
:

This is regular content between sections.

: Second Section
Content of the second section.
:

More regular content.

: Third Section
Content of the third section.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segments with content between");
    assert!(result.contains("First Section"));
    assert!(result.contains("Content of the first section."));
    assert!(result.contains("This is regular content between sections."));
    assert!(result.contains("Second Section"));
    assert!(result.contains("Content of the second section."));
    assert!(result.contains("More regular content."));
    assert!(result.contains("Third Section"));
    assert!(result.contains("Content of the third section."));
}

// Nested Indent Segments Tests
#[test]
fn test_nested_indent_segments() {
    let norg = r#": Main Section
This is the main section content.

: Nested Section
This is nested inside the main section.
:

Back to main section content.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested indent segments");
    assert!(result.contains("Main Section"));
    assert!(result.contains("This is the main section content."));
    assert!(result.contains("Nested Section"));
    assert!(result.contains("This is nested inside the main section."));
    assert!(result.contains("Back to main section content."));
}

// Mixed Slides and Indent Segments Tests
#[test]
fn test_mixed_slides_and_indent_segments() {
    let norg = r#":: Presentation Slide
This is a presentation slide.
::

: Important Note
This is an important note section.
:

:: Another Slide
This is another slide.
::

: Another Section
This is another section.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed slides and indent segments");
    assert!(result.contains("Presentation Slide"));
    assert!(result.contains("This is a presentation slide."));
    assert!(result.contains("Important Note"));
    assert!(result.contains("This is an important note section."));
    assert!(result.contains("Another Slide"));
    assert!(result.contains("This is another slide."));
    assert!(result.contains("Another Section"));
    assert!(result.contains("This is another section."));
}

#[test]
fn test_slide_containing_indent_segment() {
    let norg = r#":: Main Slide
This slide contains an indent segment:

: Indented Content
This content is indented within the slide.
It provides additional details.
:

Back to the main slide content.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide containing indent segment");
    assert!(result.contains("Main Slide"));
    assert!(result.contains("This slide contains an indent segment:"));
    assert!(result.contains("Indented Content"));
    assert!(result.contains("This content is indented within the slide."));
    assert!(result.contains("It provides additional details."));
    assert!(result.contains("Back to the main slide content."));
}

#[test]
fn test_indent_segment_containing_slide() {
    let norg = r#": Main Section
This section contains a slide:

:: Embedded Slide
This is a slide embedded within the section.
::

Back to the main section content.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment containing slide");
    assert!(result.contains("Main Section"));
    assert!(result.contains("This section contains a slide:"));
    assert!(result.contains("Embedded Slide"));
    assert!(result.contains("This is a slide embedded within the section."));
    assert!(result.contains("Back to the main section content."));
}

// Complex Nesting Tests
#[test]
fn test_complex_nesting() {
    let norg = r#":: Outer Slide
This is the outer slide.

: Indent Section
This is an indent section within the slide.

:: Inner Slide
This is a slide within the indent section.
::

Back to the indent section.
:

Back to the outer slide.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex nesting");
    assert!(result.contains("Outer Slide"));
    assert!(result.contains("This is the outer slide."));
    assert!(result.contains("Indent Section"));
    assert!(result.contains("This is an indent section within the slide."));
    assert!(result.contains("Inner Slide"));
    assert!(result.contains("This is a slide within the indent section."));
    assert!(result.contains("Back to the indent section."));
    assert!(result.contains("Back to the outer slide."));
}

// Advanced Features Tests
#[test]
fn test_slide_with_metadata() {
    let norg = r#":: Slide with Metadata
@document.meta
title: My Presentation
author: John Doe
date: 2023-12-15
@end

This slide has metadata.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with metadata");
    assert!(result.contains("Slide with Metadata"));
    assert!(result.contains("This slide has metadata."));
    // Metadata should be processed appropriately
}

#[test]
fn test_indent_segment_with_definitions() {
    let norg = r#": Definitions Section
Here are some important definitions:

$ API
Application Programming Interface

$ REST
Representational State Transfer

$ JSON
JavaScript Object Notation

These are fundamental concepts.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with definitions");
    assert!(result.contains("Definitions Section"));
    assert!(result.contains("Here are some important definitions:"));
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>API</dt>"));
    assert!(result.contains("<dd>Application Programming Interface</dd>"));
    assert!(result.contains("<dt>REST</dt>"));
    assert!(result.contains("<dd>Representational State Transfer</dd>"));
    assert!(result.contains("<dt>JSON</dt>"));
    assert!(result.contains("<dd>JavaScript Object Notation</dd>"));
    assert!(result.contains("</dl>"));
    assert!(result.contains("These are fundamental concepts."));
}

#[test]
fn test_slide_with_footnotes() {
    let norg = r#":: Slide with Footnotes
This slide has footnotes.^[See note 1]

More content here.^[See note 2]

^ See note 1
This is the first footnote.

^ See note 2
This is the second footnote.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with footnotes");
    assert!(result.contains("Slide with Footnotes"));
    assert!(result.contains("This slide has footnotes."));
    assert!(result.contains("More content here."));
    assert!(result.contains("See note 1"));
    assert!(result.contains("This is the first footnote."));
    assert!(result.contains("See note 2"));
    assert!(result.contains("This is the second footnote."));
}

// Error Cases and Edge Cases
#[test]
fn test_unclosed_slide() {
    let norg = r#":: Unclosed Slide
This slide is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_unclosed_indent_segment() {
    let norg = r#": Unclosed Segment
This segment is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_slide_with_special_characters() {
    let norg = r#":: Special Characters & Symbols
This slide contains & < > " ' special characters.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with special characters");
    assert!(result.contains("Special Characters &amp; Symbols"));
    assert!(result.contains("This slide contains &amp; &lt; &gt; &quot; &#x27; special characters."));
}

#[test]
fn test_indent_segment_with_unicode() {
    let norg = r#": Unicode Content
This section contains émojis 🌟 and Unicode characters.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with unicode");
    assert!(result.contains("Unicode Content"));
    assert!(result.contains("This section contains émojis 🌟 and Unicode characters."));
}

#[test]
fn test_slide_with_line_breaks() {
    let norg = r#":: Multi-line
Title
This slide has
multiple lines
of content.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse slide with line breaks");
    assert!(result.contains("Multi-line"));
    assert!(result.contains("Title"));
    assert!(result.contains("This slide has"));
    assert!(result.contains("multiple lines"));
    assert!(result.contains("of content."));
}

#[test]
fn test_indent_segment_with_mixed_content() {
    let norg = r#": Mixed Content Section
This section contains various elements:

* Heading
Some text under the heading.

- List item 1
- List item 2

> A quote within the section.

@code rust
fn main() {
    println!("Hello, world!");
}
@end

Final paragraph.
:"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse indent segment with mixed content");
    assert!(result.contains("Mixed Content Section"));
    assert!(result.contains("This section contains various elements:"));
    assert!(result.contains("<h1>Heading</h1>"));
    assert!(result.contains("Some text under the heading."));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>List item 1</li>"));
    assert!(result.contains("<li>List item 2</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("A quote within the section."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("fn main() {"));
    assert!(result.contains("println!(\"Hello, world!\");"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("Final paragraph."));
}