use norgmill::renderer::parse_and_render_norg;

// Single Definition Tests
#[test]
fn test_definition_basic() {
    let norg = r#"$ Term
This is the definition of the term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Term</dt>"));
    assert!(result.contains("<dd>This is the definition of the term.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_single_word_term() {
    let norg = r#"$ API
Application Programming Interface."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single word definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>API</dt>"));
    assert!(result.contains("<dd>Application Programming Interface.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_multi_word_term() {
    let norg = r#"$ Machine Learning
A subset of artificial intelligence that uses algorithms to learn from data."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multi-word definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Machine Learning</dt>"));
    assert!(result.contains("<dd>A subset of artificial intelligence that uses algorithms to learn from data.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_markup_in_term() {
    let norg = r#"$ *Important* Term
This is a definition with markup in the term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with markup in term");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt><strong>Important</strong> Term</dt>"));
    assert!(result.contains("<dd>This is a definition with markup in the term.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_markup_in_content() {
    let norg = r#"$ Technical Term
This definition has *bold* and /italic/ text in the content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with markup in content");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Technical Term</dt>"));
    assert!(result.contains("<dd>This definition has <strong>bold</strong> and <em>italic</em> text in the content.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_code_in_content() {
    let norg = r#"$ Function
A `function` is a block of code that performs a specific task."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with code in content");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Function</dt>"));
    assert!(result.contains("<dd>A <code>function</code> is a block of code that performs a specific task.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_link_in_content() {
    let norg = r#"$ Documentation
See {https://example.com}[the official docs] for more information."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with link in content");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Documentation</dt>"));
    assert!(result.contains("<dd>See"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("the official docs</a> for more information.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_multiline_content() {
    let norg = r#"$ Complex Term
This is a definition that spans multiple lines
and contains detailed information
about the term being defined."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Complex Term</dt>"));
    assert!(result.contains("<dd>This is a definition that spans multiple lines"));
    assert!(result.contains("and contains detailed information"));
    assert!(result.contains("about the term being defined.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_special_characters() {
    let norg = r#"$ C++ Programming
C++ is a programming language with & < > " ' special characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with special characters");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>C++ Programming</dt>"));
    assert!(result.contains("<dd>C++ is a programming language with &amp; &lt; &gt; &quot; &#x27; special characters.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_unicode() {
    let norg = r#"$ Café
A café is a place where people drink coffee ☕ and socialize."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with unicode");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Café</dt>"));
    assert!(result.contains("<dd>A café is a place where people drink coffee ☕ and socialize.</dd>"));
    assert!(result.contains("</dl>"));
}

// Multiple Definitions (Grouping) Tests
#[test]
fn test_multiple_definitions_grouped() {
    let norg = r#"$ First Term
Definition of the first term.
$ Second Term
Definition of the second term.
$ Third Term
Definition of the third term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple grouped definitions");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>First Term</dt>"));
    assert!(result.contains("<dd>Definition of the first term.</dd>"));
    assert!(result.contains("<dt>Second Term</dt>"));
    assert!(result.contains("<dd>Definition of the second term.</dd>"));
    assert!(result.contains("<dt>Third Term</dt>"));
    assert!(result.contains("<dd>Definition of the third term.</dd>"));
    assert!(result.contains("</dl>"));
    // Should be grouped in one definition list
    assert_eq!(result.matches("<dl>").count(), 1);
    assert_eq!(result.matches("</dl>").count(), 1);
}

#[test]
fn test_definitions_separated_by_paragraph_break() {
    let norg = r#"$ First Term
Definition of the first term.

$ Second Term
Definition of the second term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definitions separated by paragraph break");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>First Term</dt>"));
    assert!(result.contains("<dd>Definition of the first term.</dd>"));
    assert!(result.contains("<dt>Second Term</dt>"));
    assert!(result.contains("<dd>Definition of the second term.</dd>"));
    assert!(result.contains("</dl>"));
    // Should be in separate definition lists due to paragraph break
    assert!(result.matches("<dl>").count() >= 2);
    assert!(result.matches("</dl>").count() >= 2);
}

#[test]
fn test_definitions_mixed_with_other_elements() {
    let norg = r#"$ Technical Term
This is a definition.

This is a regular paragraph.

$ Another Term
Another definition here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definitions mixed with other elements");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Technical Term</dt>"));
    assert!(result.contains("<dd>This is a definition.</dd>"));
    assert!(result.contains("</dl>"));
    assert!(result.contains("<p>This is a regular paragraph.</p>"));
    assert!(result.contains("<dt>Another Term</dt>"));
    assert!(result.contains("<dd>Another definition here.</dd>"));
}

// Ranged Definition Tests
#[test]
fn test_ranged_definition_basic() {
    let norg = r#"$$ Complex Term
This is a ranged definition
that can span multiple paragraphs.

It can contain more complex content.
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic ranged definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Complex Term</dt>"));
    assert!(result.contains("<dd>This is a ranged definition"));
    assert!(result.contains("that can span multiple paragraphs."));
    assert!(result.contains("It can contain more complex content.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_with_lists() {
    let norg = r#"$$ Term with List
This definition contains:
- First item
- Second item
- Third item
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged definition with lists");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Term with List</dt>"));
    assert!(result.contains("<dd>This definition contains:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_with_code_block() {
    let norg = r#"$$ Function Example
Here's how to use this function:

@code python
def hello():
    print("Hello, world!")
@end
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged definition with code block");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Function Example</dt>"));
    assert!(result.contains("<dd>Here's how to use this function:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def hello():"));
    assert!(result.contains("print(\"Hello, world!\")"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_with_quotes() {
    let norg = r#"$$ Important Concept
According to the documentation:

> This is a quote within the definition
  that provides additional context.
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged definition with quotes");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Important Concept</dt>"));
    assert!(result.contains("<dd>According to the documentation:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote within the definition"));
    assert!(result.contains("that provides additional context."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_empty_content() {
    let norg = r#"$$ Empty Term
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged definition with empty content");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Empty Term</dt>"));
    assert!(result.contains("<dd>"));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_with_headings() {
    let norg = r#"$$ Comprehensive Topic
* Overview
This section provides an overview.

** Details
More detailed information here.
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged definition with headings");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Comprehensive Topic</dt>"));
    assert!(result.contains("<dd>"));
    assert!(result.contains("<h1>Overview</h1>"));
    assert!(result.contains("This section provides an overview."));
    assert!(result.contains("<h2>Details</h2>"));
    assert!(result.contains("More detailed information here."));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_multiple_ranged_definitions() {
    let norg = r#"$$ First Complex Term
This is the first ranged definition.

It has multiple paragraphs.
$$

$$ Second Complex Term
This is the second ranged definition.

It also has multiple paragraphs.
$$"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple ranged definitions");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>First Complex Term</dt>"));
    assert!(result.contains("<dd>This is the first ranged definition."));
    assert!(result.contains("It has multiple paragraphs.</dd>"));
    assert!(result.contains("<dt>Second Complex Term</dt>"));
    assert!(result.contains("<dd>This is the second ranged definition."));
    assert!(result.contains("It also has multiple paragraphs.</dd>"));
    assert!(result.contains("</dl>"));
}

// Mixed Single and Ranged Definitions
#[test]
fn test_mixed_single_and_ranged_definitions() {
    let norg = r#"$ Simple Term
Simple definition.

$$ Complex Term
Complex definition with multiple paragraphs.

More content here.
$$

$ Another Simple Term
Another simple definition."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed single and ranged definitions");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Simple Term</dt>"));
    assert!(result.contains("<dd>Simple definition.</dd>"));
    assert!(result.contains("<dt>Complex Term</dt>"));
    assert!(result.contains("<dd>Complex definition with multiple paragraphs."));
    assert!(result.contains("More content here.</dd>"));
    assert!(result.contains("<dt>Another Simple Term</dt>"));
    assert!(result.contains("<dd>Another simple definition.</dd>"));
    assert!(result.contains("</dl>"));
}

// Definitions in Other Contexts
#[test]
fn test_definition_in_list() {
    let norg = r#"- First item
- Item with definition:
  $ Term
  Definition content.
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Item with definition:"));
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Term</dt>"));
    assert!(result.contains("<dd>Definition content.</dd>"));
    assert!(result.contains("</dl>"));
    assert!(result.contains("</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_definition_in_quote() {
    let norg = r#"> This quote contains a definition:
  $ Quoted Term
  Definition within a quote."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains a definition:"));
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Quoted Term</dt>"));
    assert!(result.contains("<dd>Definition within a quote.</dd>"));
    assert!(result.contains("</dl>"));
    assert!(result.contains("</blockquote>"));
}

// Links to Definitions
#[test]
fn test_link_to_definition() {
    let norg = r#"$ Important Term
This is an important definition.

See {$ Important Term}[the definition] for details."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse link to definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Important Term</dt>"));
    assert!(result.contains("<dd>This is an important definition.</dd>"));
    assert!(result.contains("</dl>"));
    assert!(result.contains("See"));
    assert!(result.contains("<a href=\"#important-term\">") || result.contains("<a href=\"#Important_Term\">"));
    assert!(result.contains("the definition</a> for details."));
}

#[test]
fn test_definition_with_numbers() {
    let norg = r#"$ Version 2.0
The second major version of the software."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with numbers");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Version 2.0</dt>"));
    assert!(result.contains("<dd>The second major version of the software.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_with_punctuation() {
    let norg = r#"$ Q&A Session
A question and answer session where participants can ask questions."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with punctuation");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Q&amp;A Session</dt>"));
    assert!(result.contains("<dd>A question and answer session where participants can ask questions.</dd>"));
    assert!(result.contains("</dl>"));
}

// Error Cases
#[test]
fn test_definition_missing_content() {
    let norg = "$ Term Without Content";
    let result = parse_and_render_norg(norg).expect("Failed to parse definition without content");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Term Without Content</dt>"));
    assert!(result.contains("<dd>"));
    assert!(result.contains("</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_ranged_definition_missing_closing() {
    let norg = r#"$$ Unclosed Term
This ranged definition has no closing marker."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_definition_empty_term() {
    let norg = r#"$ 
This definition has an empty term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with empty term");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>"));
    assert!(result.contains("</dt>"));
    assert!(result.contains("<dd>This definition has an empty term.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_definition_whitespace_only_term() {
    let norg = r#"$    
This definition has a whitespace-only term."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse definition with whitespace-only term");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>"));
    assert!(result.contains("</dt>"));
    assert!(result.contains("<dd>This definition has a whitespace-only term.</dd>"));
    assert!(result.contains("</dl>"));
}