use norgmill::renderer::parse_and_render_norg;

// File Links Tests
#[test]
fn test_file_link_basic() {
    let norg = r#"See {/ readme.md} for documentation."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic file link");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"readme.md\">"));
    assert!(result.contains("readme.md</a>"));
    assert!(result.contains("for documentation.</p>"));
}

#[test]
fn test_file_link_with_path() {
    let norg = r#"Check {/ docs/guide.md} for the guide."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link with path");
    assert!(result.contains("<p>Check"));
    assert!(result.contains("<a href=\"docs/guide.md\">"));
    assert!(result.contains("docs/guide.md</a>"));
    assert!(result.contains("for the guide.</p>"));
}

#[test]
fn test_file_link_absolute_path() {
    let norg = r#"File at {/ /usr/share/doc/manual.txt} contains info."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse absolute file link");
    assert!(result.contains("<p>File at"));
    assert!(result.contains("<a href=\"/usr/share/doc/manual.txt\">"));
    assert!(result.contains("/usr/share/doc/manual.txt</a>"));
    assert!(result.contains("contains info.</p>"));
}

#[test]
fn test_file_link_with_extension() {
    let norg = r#"Files: {/ config.json}, {/ script.py}, {/ style.css}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file links with extensions");
    assert!(result.contains("Files:"));
    assert!(result.contains("<a href=\"config.json\">config.json</a>"));
    assert!(result.contains("<a href=\"script.py\">script.py</a>"));
    assert!(result.contains("<a href=\"style.css\">style.css</a>"));
}

#[test]
fn test_file_link_with_description() {
    let norg = r#"Read {/ manual.pdf}[the manual] for instructions."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link with description");
    assert!(result.contains("<p>Read"));
    assert!(result.contains("<a href=\"manual.pdf\">"));
    assert!(result.contains("the manual</a>"));
    assert!(result.contains("for instructions.</p>"));
}

#[test]
fn test_file_link_relative_path() {
    let norg = r#"Parent file {/ ../parent/file.txt} reference."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative file link");
    assert!(result.contains("<p>Parent file"));
    assert!(result.contains("<a href=\"../parent/file.txt\">"));
    assert!(result.contains("../parent/file.txt</a>"));
    assert!(result.contains("reference.</p>"));
}

#[test]
fn test_file_link_with_spaces() {
    let norg = r#"File {/ my document.txt} with spaces."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link with spaces");
    assert!(result.contains("<p>File"));
    assert!(result.contains("<a href=\"my document.txt\">"));
    assert!(result.contains("my document.txt</a>"));
    assert!(result.contains("with spaces.</p>"));
}

#[test]
fn test_file_link_in_list() {
    let norg = r#"Important files:
- {/ package.json}
- {/ src/main.rs}
- {/ tests/test.rs}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file links in list");
    assert!(result.contains("Important files:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li><a href=\"package.json\">package.json</a></li>"));
    assert!(result.contains("<li><a href=\"src/main.rs\">src/main.rs</a></li>"));
    assert!(result.contains("<li><a href=\"tests/test.rs\">tests/test.rs</a></li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_file_link_windows_path() {
    let norg = r#"Windows file {/ C:\Users\name\document.docx} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse Windows file link");
    assert!(result.contains("<p>Windows file"));
    assert!(result.contains("<a href=\"C:\\Users\\name\\document.docx\">"));
    assert!(result.contains("C:\\Users\\name\\document.docx</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_file_link_without_extension() {
    let norg = r#"File {/ makefile} contains build rules."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link without extension");
    assert!(result.contains("<p>File"));
    assert!(result.contains("<a href=\"makefile\">"));
    assert!(result.contains("makefile</a>"));
    assert!(result.contains("contains build rules.</p>"));
}

#[test]
fn test_file_link_with_special_chars() {
    let norg = r#"File {/ file-name_v2.0.txt} with special characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link with special chars");
    assert!(result.contains("<p>File"));
    assert!(result.contains("<a href=\"file-name_v2.0.txt\">"));
    assert!(result.contains("file-name_v2.0.txt</a>"));
    assert!(result.contains("with special characters.</p>"));
}

#[test]
fn test_file_link_in_heading() {
    let norg = r#"* Documentation in {/ docs/api.md}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file link in heading");
    assert!(result.contains("<h1>Documentation in"));
    assert!(result.contains("<a href=\"docs/api.md\">"));
    assert!(result.contains("docs/api.md</a>"));
    assert!(result.contains("</h1>"));
}

#[test]
fn test_file_link_multiple_in_paragraph() {
    let norg = r#"Compare {/ old.txt} with {/ new.txt} to see differences."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple file links");
    assert!(result.contains("<p>Compare"));
    assert!(result.contains("<a href=\"old.txt\">old.txt</a>"));
    assert!(result.contains("with"));
    assert!(result.contains("<a href=\"new.txt\">new.txt</a>"));
    assert!(result.contains("to see differences.</p>"));
}

#[test]
fn test_file_link_malformed() {
    let norg = r#"Malformed {/ file.txt without closing brace"#;
    let result = parse_and_render_norg(norg);
    assert!(result.is_ok());
    // Should handle gracefully
}

#[test]
fn test_file_link_empty() {
    let norg = r#"Empty file link {/} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty file link");
    assert!(result.contains("Empty file link"));
    assert!(result.contains("here."));
}