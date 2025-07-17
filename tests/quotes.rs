use norgmill::renderer::parse_and_render_norg;

#[test]
fn test_quote_level_1() {
    let norg = "> This is a quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 1");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_2() {
    let norg = ">> This is a nested quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 2");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a nested quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_3() {
    let norg = ">>> This is a deeply nested quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 3");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a deeply nested quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_4() {
    let norg = ">>>> This is a very deeply nested quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 4");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a very deeply nested quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_5() {
    let norg = ">>>>> This is an extremely nested quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 5");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is an extremely nested quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_6() {
    let norg = ">>>>>> This is the maximum nested quote";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 6");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is the maximum nested quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_level_7_fallback() {
    let norg = ">>>>>>> This should fall back to level 6";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote level 7");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This should fall back to level 6"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_markup() {
    let norg = "> This quote has *bold* and /italic/ text";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with markup");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote has <strong>bold</strong> and <em>italic</em> text"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_links() {
    let norg = "> This quote has a {https://example.com}[link]";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with links");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote has a"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains(">link</a>"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_multiline() {
    let norg = r#"> This is a multiline quote
  that continues on the next line
  and even more content here"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a multiline quote"));
    assert!(result.contains("that continues on the next line"));
    assert!(result.contains("and even more content here"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_multiple_quotes() {
    let norg = r#"> First quote
> Second quote
> Third quote"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple quotes");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("First quote"));
    assert!(result.contains("Second quote"));
    assert!(result.contains("Third quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_nested_quotes() {
    let norg = r#"> Outer quote
>> Inner quote
>>> Deeply nested quote
>> Back to inner quote
> Back to outer quote"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested quotes");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Outer quote"));
    assert!(result.contains("Inner quote"));
    assert!(result.contains("Deeply nested quote"));
    assert!(result.contains("Back to inner quote"));
    assert!(result.contains("Back to outer quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_code() {
    let norg = "> This quote contains `inline code`";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with code");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains <code>inline code</code>"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_special_characters() {
    let norg = "> Quote with & < > \" ' special characters";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with special characters");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Quote with &amp; &lt; &gt; &quot; &#x27; special characters"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_unicode() {
    let norg = "> Quote with émojis 🌟 and Unicode characters";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with unicode");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Quote with émojis 🌟 and Unicode characters"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_empty() {
    let norg = ">";
    let result = parse_and_render_norg(norg).expect("Failed to parse empty quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_whitespace_handling() {
    let norg = ">   Quote with extra spaces   ";
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with whitespace");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Quote with extra spaces"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_paragraph_break() {
    let norg = r#"> First paragraph of quote

> Second paragraph of quote"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with paragraph break");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("First paragraph of quote"));
    assert!(result.contains("Second paragraph of quote"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_mixed_with_other_elements() {
    let norg = r#"This is regular text.

> This is a quote
  with multiple lines.

This is more regular text.

>> This is a nested quote.

And back to regular text."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse quote mixed with other elements");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is regular text."));
    assert!(result.contains("This is a quote"));
    assert!(result.contains("with multiple lines."));
    assert!(result.contains("This is more regular text."));
    assert!(result.contains("This is a nested quote."));
    assert!(result.contains("And back to regular text."));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_with_lists() {
    let norg = r#"> This quote contains a list:
  - Item 1
  - Item 2
  - Item 3"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with lists");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains a list:"));
    assert!(result.contains("Item 1"));
    assert!(result.contains("Item 2"));
    assert!(result.contains("Item 3"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_quote_author_citation() {
    let norg = r#"> To be or not to be, that is the question.
  — William Shakespeare"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse quote with citation");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("To be or not to be, that is the question."));
    assert!(result.contains("— William Shakespeare"));
    assert!(result.contains("</blockquote>"));
}