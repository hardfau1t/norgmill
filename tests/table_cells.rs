use norgmill::renderer::parse_and_render_norg;

// Basic Table Cell Tests
#[test]
fn test_table_cell_basic() {
    let norg = r#": A1
Content of cell A1."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<tr>"));
    assert!(result.contains("<td>Content of cell A1.</td>"));
    assert!(result.contains("</tr>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_simple_positions() {
    let norg = r#": A1
First cell.
: B1
Second cell.
: A2
Third cell."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse simple table cell positions");
    assert!(result.contains("<table>"));
    assert!(result.contains("<tr>"));
    assert!(result.contains("<td>First cell.</td>"));
    assert!(result.contains("<td>Second cell.</td>"));
    assert!(result.contains("<td>Third cell.</td>"));
    assert!(result.contains("</tr>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_markup() {
    let norg = r#": A1
Cell with *bold* and /italic/ text."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with markup");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with <strong>bold</strong> and <em>italic</em> text.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_code() {
    let norg = r#": A1
Cell with `inline code`."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with code");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with <code>inline code</code>.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_links() {
    let norg = r#": A1
Cell with {https://example.com}[link]."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with links");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("link</a>.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_multiline_content() {
    let norg = r#": A1
This is a cell with
multiple lines of content
that should be handled properly."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with multiline content");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This is a cell with"));
    assert!(result.contains("multiple lines of content"));
    assert!(result.contains("that should be handled properly.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_special_characters() {
    let norg = r#": A1
Cell with & < > " ' special characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with special characters");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with &amp; &lt; &gt; &quot; &#x27; special characters.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_unicode() {
    let norg = r#": A1
Cell with émojis 🌟 and Unicode."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with unicode");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with émojis 🌟 and Unicode.</td>"));
    assert!(result.contains("</table>"));
}

// Table Cell Positioning Tests
#[test]
fn test_table_cell_alphabetic_positions() {
    let norg = r#": A1
Cell A1.
: B1
Cell B1.
: C1
Cell C1."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse alphabetic table cell positions");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell A1.</td>"));
    assert!(result.contains("<td>Cell B1.</td>"));
    assert!(result.contains("<td>Cell C1.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_numeric_positions() {
    let norg = r#": A1
Row 1 Col 1.
: A2
Row 2 Col 1.
: A3
Row 3 Col 1."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse numeric table cell positions");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Row 1 Col 1.</td>"));
    assert!(result.contains("<td>Row 2 Col 1.</td>"));
    assert!(result.contains("<td>Row 3 Col 1.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_double_letter_positions() {
    let norg = r#": AA1
Cell AA1.
: AB1
Cell AB1.
: AC1
Cell AC1."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse double letter table cell positions");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell AA1.</td>"));
    assert!(result.contains("<td>Cell AB1.</td>"));
    assert!(result.contains("<td>Cell AC1.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_large_numbers() {
    let norg = r#": A100
Cell A100.
: A200
Cell A200.
: A300
Cell A300."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse large number table cell positions");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell A100.</td>"));
    assert!(result.contains("<td>Cell A200.</td>"));
    assert!(result.contains("<td>Cell A300.</td>"));
    assert!(result.contains("</table>"));
}

// Table Cell Motions Tests
#[test]
fn test_table_cell_root_motion() {
    let norg = r#": .
Root cell content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse root motion table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Root cell content.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_directional_motions() {
    let norg = r#": .
Root cell.
: >
Right cell.
: v
Down cell.
: <
Left cell.
: ^
Up cell."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse directional motion table cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Root cell.</td>"));
    assert!(result.contains("<td>Right cell.</td>"));
    assert!(result.contains("<td>Down cell.</td>"));
    assert!(result.contains("<td>Left cell.</td>"));
    assert!(result.contains("<td>Up cell.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_floor_motion() {
    let norg = r#": .
Root cell.
: >
Right cell.
: _
Floor motion cell."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse floor motion table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Root cell.</td>"));
    assert!(result.contains("<td>Right cell.</td>"));
    assert!(result.contains("<td>Floor motion cell.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_ceiling_motion() {
    let norg = r#": .
Root cell.
: v
Down cell.
: /
Ceiling motion cell."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ceiling motion table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Root cell.</td>"));
    assert!(result.contains("<td>Down cell.</td>"));
    assert!(result.contains("<td>Ceiling motion cell.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_motion_repetition() {
    let norg = r#": .
Root cell.
: 3>
Three right motion.
: 2v
Two down motion.
: 2>v
Two right then down."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse motion repetition table cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Root cell.</td>"));
    assert!(result.contains("<td>Three right motion.</td>"));
    assert!(result.contains("<td>Two down motion.</td>"));
    assert!(result.contains("<td>Two right then down.</td>"));
    assert!(result.contains("</table>"));
}

// Intersecting Modifier Tests
#[test]
fn test_table_cell_intersecting_modifier() {
    let norg = r#": A1 : Content of cell A1
: B1 : Content of cell B1
: C1 : Content of cell C1"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with intersecting modifier");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Content of cell A1</td>"));
    assert!(result.contains("<td>Content of cell B1</td>"));
    assert!(result.contains("<td>Content of cell C1</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_intersecting_modifier_with_markup() {
    let norg = r#": A1 : Content with *bold* text
: B1 : Content with /italic/ text"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell intersecting with markup");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Content with <strong>bold</strong> text</td>"));
    assert!(result.contains("<td>Content with <em>italic</em> text</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_intersecting_modifier_with_links() {
    let norg = r#": A1 : Cell with {https://example.com}[link]
: B1 : Another cell with {https://test.com}[another link]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell intersecting with links");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell with"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("link</a></td>"));
    assert!(result.contains("href=\"https://test.com\""));
    assert!(result.contains("another link</a></td>"));
    assert!(result.contains("</table>"));
}

// Ranged Table Cell Tests
#[test]
fn test_ranged_table_cell_basic() {
    let norg = r#":: A1
This is a ranged table cell
with multiple lines of content.

It can contain complex structures.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic ranged table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This is a ranged table cell"));
    assert!(result.contains("with multiple lines of content."));
    assert!(result.contains("It can contain complex structures.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_ranged_table_cell_with_lists() {
    let norg = r#":: A1
This cell contains a list:
- First item
- Second item
- Third item
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged table cell with lists");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell contains a list:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item</li>"));
    assert!(result.contains("<li>Second item</li>"));
    assert!(result.contains("<li>Third item</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_ranged_table_cell_with_code_block() {
    let norg = r#":: A1
This cell contains code:

@code python
def hello():
    print("Hello, world!")
@end
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged table cell with code block");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell contains code:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("def hello():"));
    assert!(result.contains("print(\"Hello, world!\")"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_ranged_table_cell_with_quotes() {
    let norg = r#":: A1
This cell contains a quote:

> This is a quote within the table cell
  that spans multiple lines.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged table cell with quotes");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell contains a quote:"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This is a quote within the table cell"));
    assert!(result.contains("that spans multiple lines."));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_ranged_table_cell_with_headings() {
    let norg = r#":: A1
* Cell Heading
Content under the heading.

** Sub Heading
More content here.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ranged table cell with headings");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>"));
    assert!(result.contains("<h1>Cell Heading</h1>"));
    assert!(result.contains("Content under the heading."));
    assert!(result.contains("<h2>Sub Heading</h2>"));
    assert!(result.contains("More content here."));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_multiple_ranged_table_cells() {
    let norg = r#":: A1
First ranged cell with
multiple paragraphs.

Second paragraph.
::

:: B1
Second ranged cell with
different content.

Another paragraph.
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple ranged table cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>First ranged cell with"));
    assert!(result.contains("multiple paragraphs."));
    assert!(result.contains("Second paragraph.</td>"));
    assert!(result.contains("<td>Second ranged cell with"));
    assert!(result.contains("different content."));
    assert!(result.contains("Another paragraph.</td>"));
    assert!(result.contains("</table>"));
}

// Mixed Single and Ranged Table Cells
#[test]
fn test_mixed_single_and_ranged_table_cells() {
    let norg = r#": A1
Simple cell.

:: B1
Ranged cell with
multiple paragraphs.

More content.
::

: C1
Another simple cell."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed single and ranged table cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Simple cell.</td>"));
    assert!(result.contains("<td>Ranged cell with"));
    assert!(result.contains("multiple paragraphs."));
    assert!(result.contains("More content.</td>"));
    assert!(result.contains("<td>Another simple cell.</td>"));
    assert!(result.contains("</table>"));
}

// Complex Table Tests
#[test]
fn test_complex_table_structure() {
    let norg = r#": A1 : Header 1
: B1 : Header 2
: C1 : Header 3
: A2 : Row 1 Col 1
: B2 : Row 1 Col 2
: C2 : Row 1 Col 3
: A3 : Row 2 Col 1
: B3 : Row 2 Col 2
: C3 : Row 2 Col 3"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex table structure");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Header 1</td>"));
    assert!(result.contains("<td>Header 2</td>"));
    assert!(result.contains("<td>Header 3</td>"));
    assert!(result.contains("<td>Row 1 Col 1</td>"));
    assert!(result.contains("<td>Row 1 Col 2</td>"));
    assert!(result.contains("<td>Row 1 Col 3</td>"));
    assert!(result.contains("<td>Row 2 Col 1</td>"));
    assert!(result.contains("<td>Row 2 Col 2</td>"));
    assert!(result.contains("<td>Row 2 Col 3</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_with_empty_cells() {
    let norg = r#": A1 : Content A1
: B1 : 
: C1 : Content C1"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table with empty cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Content A1</td>"));
    assert!(result.contains("<td></td>") || result.contains("<td> </td>"));
    assert!(result.contains("<td>Content C1</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_with_sparse_cells() {
    let norg = r#": A1 : Cell A1
: C1 : Cell C1 (skipped B1)
: A3 : Cell A3 (skipped row 2)"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table with sparse cells");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Cell A1</td>"));
    assert!(result.contains("<td>Cell C1 (skipped B1)</td>"));
    assert!(result.contains("<td>Cell A3 (skipped row 2)</td>"));
    assert!(result.contains("</table>"));
}

// Error Cases
#[test]
fn test_table_cell_missing_content() {
    let norg = ": A1";
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell without content");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_ranged_table_cell_missing_closing() {
    let norg = r#":: A1
This ranged cell has no closing marker."#;
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_table_cell_empty_position() {
    let norg = r#": 
This cell has an empty position."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with empty position");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell has an empty position.</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_invalid_position() {
    let norg = r#": 123
This cell has an invalid position (numbers only)."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with invalid position");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell has an invalid position (numbers only).</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_table_cell_with_nested_tables() {
    let norg = r#":: A1
This cell contains a nested table:

: X1 : Nested A1
: X2 : Nested A2
::"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse table cell with nested table");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>This cell contains a nested table:"));
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Nested A1</td>"));
    assert!(result.contains("<td>Nested A2</td>"));
    assert!(result.contains("</table>"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
    // Should have two tables
    assert!(result.matches("<table>").count() >= 2);
    assert!(result.matches("</table>").count() >= 2);
}