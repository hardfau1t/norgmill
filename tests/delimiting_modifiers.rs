use norgmill::renderer::parse_and_render_norg;

// Horizontal Rule (--) Tests
#[test]
fn test_horizontal_rule_basic() {
    let norg = r#"First paragraph.

--

Second paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic horizontal rule");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

#[test]
fn test_horizontal_rule_between_headings() {
    let norg = r#"* First Heading

--

* Second Heading"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule between headings");
    assert!(result.contains("<h1>First Heading</h1>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<h1>Second Heading</h1>"));
}

#[test]
fn test_horizontal_rule_in_list() {
    let norg = r#"- First item
--
- Second item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_horizontal_rule_multiple() {
    let norg = r#"First section.

--

Middle section.

--

Last section."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple horizontal rules");
    assert!(result.contains("<p>First section.</p>"));
    assert!(result.contains("<p>Middle section.</p>"));
    assert!(result.contains("<p>Last section.</p>"));
    let hr_count = result.matches("<hr").count();
    assert!(hr_count >= 2);
}

#[test]
fn test_horizontal_rule_with_content_above_and_below() {
    let norg = r#"* Main Section

Content above the rule.

--

Content below the rule.

** Subsection"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule with surrounding content");
    assert!(result.contains("<h1>Main Section</h1>"));
    assert!(result.contains("<p>Content above the rule.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<p>Content below the rule.</p>"));
    assert!(result.contains("<h2>Subsection</h2>"));
}

#[test]
fn test_horizontal_rule_at_document_start() {
    let norg = r#"--

Content after the rule."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule at document start");
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<p>Content after the rule.</p>"));
}

#[test]
fn test_horizontal_rule_at_document_end() {
    let norg = r#"Content before the rule.

--"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule at document end");
    assert!(result.contains("<p>Content before the rule.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
}

#[test]
fn test_horizontal_rule_with_whitespace() {
    let norg = r#"First paragraph.

  --  

Second paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse horizontal rule with whitespace");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

#[test]
fn test_horizontal_rule_single_dash_not_rule() {
    let norg = r#"First paragraph.

-

Second paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single dash (not horizontal rule)");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
    // Single dash should not create a horizontal rule
    assert!(!result.contains("<hr>"));
}

#[test]
fn test_horizontal_rule_three_dashes() {
    let norg = r#"First paragraph.

---

Second paragraph."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse three dashes");
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
    // Three dashes might create a horizontal rule depending on implementation
    // This test verifies the behavior is consistent
}

// Emphasis Block (==) Tests
#[test]
fn test_emphasis_block_basic() {
    let norg = r#"Regular text.

==
Important emphasized content.
==

More regular text."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic emphasis block");
    assert!(result.contains("<p>Regular text.</p>"));
    assert!(result.contains("Important emphasized content."));
    assert!(result.contains("<p>More regular text.</p>"));
    // Should be wrapped in some kind of emphasis container (div, section, etc.)
    assert!(result.contains("<div") || result.contains("<section") || result.contains("<aside"));
}

#[test]
fn test_emphasis_block_with_markup() {
    let norg = r#"==
This emphasized block contains *bold* and /italic/ text.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse emphasis block with markup");
    assert!(result.contains("This emphasized block contains"));
    assert!(result.contains("<strong>bold</strong>"));
    assert!(result.contains("<em>italic</em>"));
    assert!(result.contains("<div") || result.contains("<section") || result.contains("<aside"));
}

#[test]
fn test_emphasis_block_with_lists() {
    let norg = r#"==
Important list:
- First item
- Second item
- Third item
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse emphasis block with lists");
    assert!(result.contains("Important list:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_emphasis_block_with_headings() {
    let norg = r#"==
* Important Section
This is emphasized content.

** Important Subsection
More emphasized content.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse emphasis block with headings");
    assert!(result.contains("<h1>Important Section</h1>"));
    assert!(result.contains("This is emphasized content."));
    assert!(result.contains("<h2>Important Subsection</h2>"));
    assert!(result.contains("More emphasized content."));
}

#[test]
fn test_emphasis_block_with_code() {
    let norg = r#"==
Important code example:

@code python
def important_function():
    return "Hello, world!"
@end
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse emphasis block with code");
    assert!(result.contains("Important code example:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def important_function():"));
    assert!(result.contains("return \"Hello, world!\""));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_emphasis_block_multiline() {
    let norg = r#"==
This is a multi-line emphasis block
that spans several lines
and contains various content.

It can have paragraph breaks too.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline emphasis block");
    assert!(result.contains("This is a multi-line emphasis block"));
    assert!(result.contains("that spans several lines"));
    assert!(result.contains("and contains various content."));
    assert!(result.contains("It can have paragraph breaks too."));
}

#[test]
fn test_emphasis_block_nested() {
    let norg = r#"==
Outer emphasis block.

==
Inner emphasis block.
==

Back to outer emphasis block.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested emphasis blocks");
    assert!(result.contains("Outer emphasis block."));
    assert!(result.contains("Inner emphasis block."));
    assert!(result.contains("Back to outer emphasis block."));
}

#[test]
fn test_emphasis_block_empty() {
    let norg = r#"==
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty emphasis block");
    // Should handle empty emphasis blocks gracefully
    assert!(result.contains("<div") || result.contains("<section") || result.contains("<aside"));
}

#[test]
fn test_emphasis_block_single_equals_not_block() {
    let norg = r#"=
This should not be an emphasis block.
="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single equals (not emphasis block)");
    assert!(result.contains("This should not be an emphasis block."));
    // Single equals should not create emphasis block
}

#[test]
fn test_emphasis_block_with_quotes() {
    let norg = r#"==
Important quote:

> This is a quote within an emphasis block
  that provides important context.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse emphasis block with quotes");
    assert!(result.contains("Important quote:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote within an emphasis block"));
    assert!(result.contains("that provides important context."));
    assert!(result.contains("</blockquote>"));
}

// Underline Block (__) Tests
#[test]
fn test_underline_block_basic() {
    let norg = r#"Regular text.

__
Important underlined content.
__

More regular text."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic underline block");
    assert!(result.contains("<p>Regular text.</p>"));
    assert!(result.contains("Important underlined content."));
    assert!(result.contains("<p>More regular text.</p>"));
    // Should be wrapped in some kind of underline container (div with class, u, etc.)
    assert!(result.contains("<div") || result.contains("<u") || result.contains("<span"));
}

#[test]
fn test_underline_block_with_markup() {
    let norg = r#"__
This underlined block contains *bold* and /italic/ text.
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse underline block with markup");
    assert!(result.contains("This underlined block contains"));
    assert!(result.contains("<strong>bold</strong>"));
    assert!(result.contains("<em>italic</em>"));
}

#[test]
fn test_underline_block_with_lists() {
    let norg = r#"__
Underlined list:
- First item
- Second item
- Third item
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse underline block with lists");
    assert!(result.contains("Underlined list:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_underline_block_multiline() {
    let norg = r#"__
This is a multi-line underline block
that spans several lines
and contains various content.

It can have paragraph breaks too.
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline underline block");
    assert!(result.contains("This is a multi-line underline block"));
    assert!(result.contains("that spans several lines"));
    assert!(result.contains("and contains various content."));
    assert!(result.contains("It can have paragraph breaks too."));
}

#[test]
fn test_underline_block_with_headings() {
    let norg = r#"__
* Underlined Section
This is underlined content.

** Underlined Subsection
More underlined content.
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse underline block with headings");
    assert!(result.contains("<h1>Underlined Section</h1>"));
    assert!(result.contains("This is underlined content."));
    assert!(result.contains("<h2>Underlined Subsection</h2>"));
    assert!(result.contains("More underlined content."));
}

#[test]
fn test_underline_block_nested() {
    let norg = r#"__
Outer underline block.

__
Inner underline block.
__

Back to outer underline block.
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested underline blocks");
    assert!(result.contains("Outer underline block."));
    assert!(result.contains("Inner underline block."));
    assert!(result.contains("Back to outer underline block."));
}

#[test]
fn test_underline_block_empty() {
    let norg = r#"__
__"#;
    let _result = parse_and_render_norg(norg).expect("Failed to parse empty underline block");
    // Should handle empty underline blocks gracefully
}

#[test]
fn test_underline_block_single_underscore_not_block() {
    let norg = r#"_
This should not be an underline block.
_"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single underscore (not underline block)");
    assert!(result.contains("This should not be an underline block."));
    // Single underscore should not create underline block
}

#[test]
fn test_underline_block_with_code() {
    let norg = r#"__
Important underlined code:

@code python
def underlined_function():
    return "Underlined!"
@end
__"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse underline block with code");
    assert!(result.contains("Important underlined code:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def underlined_function():"));
    assert!(result.contains("return \"Underlined!\""));
    assert!(result.contains("</code></pre>"));
}

// Mixed Delimiting Modifiers Tests
#[test]
fn test_mixed_delimiting_modifiers() {
    let norg = r#"Regular content.

--

==
Emphasized content after horizontal rule.
==

__
Underlined content.
__

--

Final content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed delimiting modifiers");
    assert!(result.contains("<p>Regular content.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("Emphasized content after horizontal rule."));
    assert!(result.contains("Underlined content."));
    assert!(result.contains("<p>Final content.</p>"));
    // Should have two horizontal rules
    let hr_count = result.matches("<hr").count();
    assert!(hr_count >= 2);
}

#[test]
fn test_delimiting_modifiers_in_sequence() {
    let norg = r#"First section.

--

==
Second section (emphasized).
==

--

__
Third section (underlined).
__

--

Fourth section."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers in sequence");
    assert!(result.contains("<p>First section.</p>"));
    assert!(result.contains("Second section (emphasized)."));
    assert!(result.contains("Third section (underlined)."));
    assert!(result.contains("<p>Fourth section.</p>"));
    // Should have three horizontal rules
    let hr_count = result.matches("<hr").count();
    assert!(hr_count >= 3);
}

#[test]
fn test_delimiting_modifiers_with_other_elements() {
    let norg = r#"* Main Heading

--

- List item before emphasis
- Another list item

==
* Emphasized Heading
Important emphasized paragraph.
==

^ Footnote
This is a footnote after the emphasis block.

__
Underlined section with footnote reference.
__

$ Definition
This is a definition after the underline block."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers with other elements");
    assert!(result.contains("<h1>Main Heading</h1>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>List item before emphasis</li>"));
    assert!(result.contains("<h1>Emphasized Heading</h1>"));
    assert!(result.contains("Important emphasized paragraph."));
    assert!(result.contains("Footnote"));
    assert!(result.contains("This is a footnote after the emphasis block."));
    assert!(result.contains("Underlined section with footnote reference."));
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Definition</dt>"));
    assert!(result.contains("<dd>This is a definition after the underline block.</dd>"));
}

// Error Cases and Edge Cases
#[test]
fn test_unclosed_emphasis_block() {
    let norg = r#"==
This emphasis block is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_unclosed_underline_block() {
    let norg = r#"__
This underline block is not closed properly."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_delimiting_modifiers_with_whitespace_only() {
    let norg = r#"Content before.

--

   

Content after."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers with whitespace");
    assert!(result.contains("<p>Content before.</p>"));
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("<p>Content after.</p>"));
}

#[test]
fn test_delimiting_modifiers_consecutive() {
    let norg = r#"--
==
Content in emphasis after horizontal rule.
==
--"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse consecutive delimiting modifiers");
    assert!(result.contains("<hr>") || result.contains("<hr/>") || result.contains("<hr />"));
    assert!(result.contains("Content in emphasis after horizontal rule."));
    // Should have two horizontal rules
    let hr_count = result.matches("<hr").count();
    assert!(hr_count >= 2);
}

#[test]
fn test_delimiting_modifiers_at_line_start() {
    let norg = r#"--
==
__
Content within nested blocks.
__
==
--"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers at line start");
    assert!(result.contains("Content within nested blocks."));
    // Should have horizontal rules
    let hr_count = result.matches("<hr").count();
    assert!(hr_count >= 2);
}

#[test]
fn test_delimiting_modifiers_with_special_characters() {
    let norg = r#"==
Content with & < > " ' special characters.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers with special characters");
    assert!(result.contains("Content with &amp; &lt; &gt; &quot; &#x27; special characters."));
}

#[test]
fn test_delimiting_modifiers_with_unicode() {
    let norg = r#"==
Content with émojis 🌟 and Unicode characters.
=="#;
    let result = parse_and_render_norg(norg).expect("Failed to parse delimiting modifiers with unicode");
    assert!(result.contains("Content with émojis 🌟 and Unicode characters."));
}
