use norgmill::renderer::parse_and_render_norg;

#[test]
fn test_single_paragraph() {
    let norg = "This is a single paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse single paragraph");
    assert!(result.contains("<p>This is a single paragraph.</p>"));
}

#[test]
fn test_multiple_paragraphs() {
    let norg = r#"This is the first paragraph.

This is the second paragraph.

This is the third paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple paragraphs");
    assert!(result.contains("<p>This is the first paragraph.</p>"));
    assert!(result.contains("<p>This is the second paragraph.</p>"));
    assert!(result.contains("<p>This is the third paragraph.</p>"));
}

#[test]
fn test_paragraph_with_line_breaks() {
    let norg = r#"This is a paragraph
that continues on the next line
and has multiple lines."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with line breaks");
    assert!(result.contains("<p>This is a paragraph"));
    assert!(result.contains("that continues on the next line"));
    assert!(result.contains("and has multiple lines.</p>"));
}

#[test]
fn test_paragraph_with_markup() {
    let norg = "This paragraph has *bold* and /italic/ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with markup");
    assert!(result.contains("<p>This paragraph has <strong>bold</strong> and <em>italic</em> text.</p>"));
}

#[test]
fn test_paragraph_with_links() {
    let norg = "This paragraph has a {https://example.com}[link].";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with links");
    assert!(result.contains("<p>This paragraph has a"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains(">link</a>.</p>"));
}

#[test]
fn test_paragraph_with_inline_code() {
    let norg = "This paragraph has `inline code` in it.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with inline code");
    assert!(result.contains("<p>This paragraph has <code>inline code</code> in it.</p>"));
}

#[test]
fn test_paragraph_with_special_characters() {
    let norg = "This paragraph has & < > \" ' special characters.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with special characters");
    assert!(result.contains("<p>This paragraph has &amp; &lt; &gt; &quot; &#x27; special characters.</p>"));
}

#[test]
fn test_paragraph_with_unicode() {
    let norg = "This paragraph has émojis 🌟 and Unicode characters.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with unicode");
    assert!(result.contains("<p>This paragraph has émojis 🌟 and Unicode characters.</p>"));
}

#[test]
fn test_paragraph_with_numbers() {
    let norg = "This paragraph has numbers 123 and symbols #@$.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with numbers");
    assert!(result.contains("<p>This paragraph has numbers 123 and symbols #@$.</p>"));
}

#[test]
fn test_paragraph_very_long() {
    let long_text = "This is a very long paragraph that contains many words and should test how the parser handles long content. ".repeat(10);
    let result = parse_and_render_norg(&long_text).expect("Failed to parse very long paragraph");
    assert!(result.contains("<p>This is a very long paragraph"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_paragraph_empty_lines_between() {
    let norg = r#"First paragraph.


Second paragraph after multiple empty lines.



Third paragraph after even more empty lines."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with empty lines");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph after multiple empty lines.</p>"));
    assert!(result.contains("<p>Third paragraph after even more empty lines.</p>"));
}

#[test]
fn test_paragraph_with_whitespace_only_lines() {
    let norg = r#"First paragraph.
   
Second paragraph after whitespace-only line.
		
Third paragraph after tab-only line."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with whitespace lines");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph after whitespace-only line.</p>"));
    assert!(result.contains("<p>Third paragraph after tab-only line.</p>"));
}

#[test]
fn test_paragraph_leading_trailing_whitespace() {
    let norg = "   This paragraph has leading and trailing whitespace.   ";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with whitespace");
    assert!(result.contains("<p>This paragraph has leading and trailing whitespace.</p>"));
}

#[test]
fn test_paragraph_with_tabs() {
    let norg = "This paragraph\thas\ttabs\tin\tit.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with tabs");
    assert!(result.contains("<p>This paragraph"));
    assert!(result.contains("has"));
    assert!(result.contains("tabs"));
    assert!(result.contains("in"));
    assert!(result.contains("it.</p>"));
}

#[test]
fn test_paragraph_mixed_with_headings() {
    let norg = r#"This is a paragraph before a heading.

* This is a heading

This is a paragraph after a heading.

** This is a subheading

This is another paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs mixed with headings");
    assert!(result.contains("<p>This is a paragraph before a heading.</p>"));
    assert!(result.contains("<h1>This is a heading</h1>"));
    assert!(result.contains("<p>This is a paragraph after a heading.</p>"));
    assert!(result.contains("<h2>This is a subheading</h2>"));
    assert!(result.contains("<p>This is another paragraph.</p>"));
}

#[test]
fn test_paragraph_mixed_with_lists() {
    let norg = r#"This is a paragraph before a list.

- First item
- Second item
- Third item

This is a paragraph after a list."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs mixed with lists");
    assert!(result.contains("<p>This is a paragraph before a list.</p>"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("<p>This is a paragraph after a list.</p>"));
}

#[test]
fn test_paragraph_mixed_with_quotes() {
    let norg = r#"This is a paragraph before a quote.

> This is a quote
  with multiple lines.

This is a paragraph after a quote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs mixed with quotes");
    assert!(result.contains("<p>This is a paragraph before a quote.</p>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote"));
    assert!(result.contains("with multiple lines."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("<p>This is a paragraph after a quote.</p>"));
}

#[test]
fn test_paragraph_mixed_with_code_blocks() {
    let norg = r#"This is a paragraph before a code block.

@code
println!("Hello, world!");
@end

This is a paragraph after a code block."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs mixed with code blocks");
    assert!(result.contains("<p>This is a paragraph before a code block.</p>"));
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("println!(\"Hello, world!\");"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("<p>This is a paragraph after a code block.</p>"));
}

#[test]
fn test_paragraph_single_word() {
    let norg = "Word";
    let result = parse_and_render_norg(norg).expect("Failed to parse single word paragraph");
    assert!(result.contains("<p>Word</p>"));
}

#[test]
fn test_paragraph_single_character() {
    let norg = "A";
    let result = parse_and_render_norg(norg).expect("Failed to parse single character paragraph");
    assert!(result.contains("<p>A</p>"));
}

#[test]
fn test_paragraph_punctuation_only() {
    let norg = "!@#$%^&*()";
    let result = parse_and_render_norg(norg).expect("Failed to parse punctuation-only paragraph");
    assert!(result.contains("<p>!@#$%^&amp;*()</p>"));
}

#[test]
fn test_paragraph_numbers_only() {
    let norg = "1234567890";
    let result = parse_and_render_norg(norg).expect("Failed to parse numbers-only paragraph");
    assert!(result.contains("<p>1234567890</p>"));
}

#[test]
fn test_paragraph_with_nested_markup() {
    let norg = "This has *bold with /italic/ inside* and more text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with nested markup");
    assert!(result.contains("<p>This has <strong>bold with <em>italic</em> inside</strong> and more text.</p>"));
}

#[test]
fn test_paragraph_with_multiple_markup_types() {
    let norg = "This has *bold*, /italic/, _underline_, -strikethrough-, and `code`.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with multiple markup types");
    assert!(result.contains("<p>This has <strong>bold</strong>, <em>italic</em>, <u>underline</u>, <del>strikethrough</del>, and <code>code</code>.</p>"));
}

#[test]
fn test_paragraph_with_math() {
    let norg = "The equation $E = mc^2$ is famous in physics.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with math");
    assert!(result.contains("<p>The equation"));
    assert!(result.contains("E = mc^2"));
    assert!(result.contains("is famous in physics.</p>"));
}

#[test]
fn test_paragraph_with_variables() {
    let norg = "The value of &variable& is important.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with variables");
    assert!(result.contains("<p>The value of"));
    assert!(result.contains("variable"));
    assert!(result.contains("is important.</p>"));
}

#[test]
fn test_paragraph_with_subscript_superscript() {
    let norg = "Water is H,2,O and energy is E = mc^2^.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with sub/superscript");
    assert!(result.contains("<p>Water is H<sub>2</sub>O and energy is E = mc<sup>2</sup>.</p>"));
}

#[test]
fn test_paragraph_complex_formatting() {
    let norg = r#"This is a *complex* paragraph with /multiple/ formatting _types_ including `code`, 
links to {https://example.com}[websites], and even some -strikethrough- text.
It also has H,2,O and E = mc^2^ for good measure."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex paragraph");
    assert!(result.contains("<p>This is a <strong>complex</strong> paragraph"));
    assert!(result.contains("<em>multiple</em> formatting"));
    assert!(result.contains("<u>types</u> including"));
    assert!(result.contains("<code>code</code>"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains(">websites</a>"));
    assert!(result.contains("<del>strikethrough</del>"));
    assert!(result.contains("H<sub>2</sub>O"));
    assert!(result.contains("E = mc<sup>2</sup>"));
    assert!(result.contains("</p>"));
}