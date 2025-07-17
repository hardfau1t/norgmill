use norgmill::renderer::parse_and_render_norg;

// Unordered Lists Tests
#[test]
fn test_unordered_list_level_1() {
    let norg = "- First item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 1");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_level_2() {
    let norg = "-- Second level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 2");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Second level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_level_3() {
    let norg = "--- Third level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 3");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Third level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_level_4() {
    let norg = "---- Fourth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 4");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Fourth level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_level_5() {
    let norg = "----- Fifth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 5");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Fifth level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_level_6() {
    let norg = "------ Sixth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list level 6");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Sixth level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_multiple_items() {
    let norg = r#"- First item
- Second item
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple unordered list items");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_nested() {
    let norg = r#"- Top level item
-- Nested item
--- Deeply nested item
-- Another nested item
- Another top level item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested unordered list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Top level item"));
    assert!(result.contains("<li>Nested item</li>"));
    assert!(result.contains("<li>Deeply nested item</li>"));
    assert!(result.contains("<li>Another nested item</li>"));
    assert!(result.contains("<li>Another top level item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_with_markup() {
    let norg = "- Item with *bold* and /italic/ text";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list with markup");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Item with <strong>bold</strong> and <em>italic</em> text</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_with_links() {
    let norg = "- Item with {https://example.com}[link]";
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list with links");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Item with"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains(">link</a>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_unordered_list_multiline_content() {
    let norg = r#"- First item
  This is continuation of the first item.
- Second item
  This is continuation of the second item."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse unordered list with multiline content");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item"));
    assert!(result.contains("continuation of the first item"));
    assert!(result.contains("<li>Second item"));
    assert!(result.contains("continuation of the second item"));
    assert!(result.contains("</ul>"));
}

// Ordered Lists Tests
#[test]
fn test_ordered_list_level_1() {
    let norg = "~ First item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 1");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_level_2() {
    let norg = "~~ Second level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 2");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Second level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_level_3() {
    let norg = "~~~ Third level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 3");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Third level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_level_4() {
    let norg = "~~~~ Fourth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 4");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Fourth level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_level_5() {
    let norg = "~~~~~ Fifth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 5");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Fifth level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_level_6() {
    let norg = "~~~~~~ Sixth level item";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list level 6");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Sixth level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_multiple_items() {
    let norg = r#"~ First item
~ Second item
~ Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple ordered list items");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_nested() {
    let norg = r#"~ Top level item
~~ Nested item
~~~ Deeply nested item
~~ Another nested item
~ Another top level item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested ordered list");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Top level item"));
    assert!(result.contains("<li>Nested item</li>"));
    assert!(result.contains("<li>Deeply nested item</li>"));
    assert!(result.contains("<li>Another nested item</li>"));
    assert!(result.contains("<li>Another top level item</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_with_markup() {
    let norg = "~ Item with *bold* and /italic/ text";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list with markup");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Item with <strong>bold</strong> and <em>italic</em> text</li>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_with_links() {
    let norg = "~ Item with {https://example.com}[link]";
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list with links");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Item with"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains(">link</a>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_ordered_list_multiline_content() {
    let norg = r#"~ First item
  This is continuation of the first item.
~ Second item
  This is continuation of the second item."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ordered list with multiline content");
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>First item"));
    assert!(result.contains("continuation of the first item"));
    assert!(result.contains("<li>Second item"));
    assert!(result.contains("continuation of the second item"));
    assert!(result.contains("</ol>"));
}

// Mixed Lists Tests
#[test]
fn test_mixed_list_types() {
    let norg = r#"- Unordered item
~ Ordered item
- Another unordered item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed list types");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Unordered item</li>"));
    assert!(result.contains("<li>Ordered item</li>"));
    assert!(result.contains("<li>Another unordered item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_mixed_nested_lists() {
    let norg = r#"- Unordered item
~~ Nested ordered item
--- Deeply nested unordered item
~ Another ordered item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed nested lists");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<ol>"));
    assert!(result.contains("<li>Unordered item"));
    assert!(result.contains("<li>Nested ordered item</li>"));
    assert!(result.contains("<li>Deeply nested unordered item</li>"));
    assert!(result.contains("<li>Another ordered item</li>"));
}

#[test]
fn test_list_with_empty_items() {
    let norg = r#"- First item
- 
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse list with empty items");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_list_with_special_characters() {
    let norg = "- Item with & < > \" ' characters";
    let result = parse_and_render_norg(norg).expect("Failed to parse list with special characters");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Item with &amp; &lt; &gt; &quot; &#x27; characters</li>"));
    assert!(result.contains("</ul>"));
}