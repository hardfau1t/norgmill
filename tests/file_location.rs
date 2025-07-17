use norgmill::renderer::parse_and_render_norg;

// File Location Tests
#[test]
fn test_file_location_basic() {
    let norg = r#"Open {:file.txt} to see the content."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic file location");
    assert!(result.contains("<p>Open"));
    assert!(result.contains("<a href=\"file.txt\">"));
    assert!(result.contains("file.txt</a>"));
    assert!(result.contains("to see the content.</p>"));
}

#[test]
fn test_file_location_with_path() {
    let norg = r#"Check {:docs/readme.md} for documentation."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location with path");
    assert!(result.contains("<p>Check"));
    assert!(result.contains("<a href=\"docs/readme.md\">"));
    assert!(result.contains("docs/readme.md</a>"));
    assert!(result.contains("for documentation.</p>"));
}

#[test]
fn test_file_location_absolute_path() {
    let norg = r#"File at {:/home/user/document.txt} contains data."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse absolute file location");
    assert!(result.contains("<p>File at"));
    assert!(result.contains("<a href=\"/home/user/document.txt\">"));
    assert!(result.contains("/home/user/document.txt</a>"));
    assert!(result.contains("contains data.</p>"));
}

#[test]
fn test_file_location_windows_path() {
    let norg = r#"Windows file {:C:\Users\name\file.doc} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse Windows file location");
    assert!(result.contains("<p>Windows file"));
    assert!(result.contains("<a href=\"C:\\Users\\name\\file.doc\">"));
    assert!(result.contains("C:\\Users\\name\\file.doc</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_file_location_with_extension() {
    let norg = r#"Files: {:config.json}, {:script.py}, {:style.css}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file locations with extensions");
    assert!(result.contains("Files:"));
    assert!(result.contains("<a href=\"config.json\">config.json</a>"));
    assert!(result.contains("<a href=\"script.py\">script.py</a>"));
    assert!(result.contains("<a href=\"style.css\">style.css</a>"));
}

#[test]
fn test_file_location_with_spaces() {
    let norg = r#"File with spaces {:my document.txt} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location with spaces");
    assert!(result.contains("<p>File with spaces"));
    assert!(result.contains("<a href=\"my document.txt\">"));
    assert!(result.contains("my document.txt</a>"));
    assert!(result.contains("here.</p>"));
}

#[test]
fn test_file_location_relative_path() {
    let norg = r#"Relative file {:../parent/file.txt} reference."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative file location");
    assert!(result.contains("<p>Relative file"));
    assert!(result.contains("<a href=\"../parent/file.txt\">"));
    assert!(result.contains("../parent/file.txt</a>"));
    assert!(result.contains("reference.</p>"));
}

#[test]
fn test_file_location_with_description() {
    let norg = r#"See {:config.json}[configuration file] for settings."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location with description");
    assert!(result.contains("<p>See"));
    assert!(result.contains("<a href=\"config.json\">"));
    assert!(result.contains("configuration file</a>"));
    assert!(result.contains("for settings.</p>"));
}

#[test]
fn test_file_location_in_list() {
    let norg = r#"Files to check:
- {:readme.md}
- {:package.json}
- {:src/main.rs}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file locations in list");
    assert!(result.contains("Files to check:"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li><a href=\"readme.md\">readme.md</a></li>"));
    assert!(result.contains("<li><a href=\"package.json\">package.json</a></li>"));
    assert!(result.contains("<li><a href=\"src/main.rs\">src/main.rs</a></li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_file_location_multiple_in_paragraph() {
    let norg = r#"Compare {:old.txt} with {:new.txt} to see changes."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple file locations");
    assert!(result.contains("<p>Compare"));
    assert!(result.contains("<a href=\"old.txt\">old.txt</a>"));
    assert!(result.contains("with"));
    assert!(result.contains("<a href=\"new.txt\">new.txt</a>"));
    assert!(result.contains("to see changes.</p>"));
}

#[test]
fn test_file_location_without_extension() {
    let norg = r#"File {:makefile} contains build instructions."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location without extension");
    assert!(result.contains("<p>File"));
    assert!(result.contains("<a href=\"makefile\">"));
    assert!(result.contains("makefile</a>"));
    assert!(result.contains("contains build instructions.</p>"));
}

#[test]
fn test_file_location_with_special_chars() {
    let norg = r#"File {:file-name_v2.0.txt} with special characters."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location with special chars");
    assert!(result.contains("<p>File"));
    assert!(result.contains("<a href=\"file-name_v2.0.txt\">"));
    assert!(result.contains("file-name_v2.0.txt</a>"));
    assert!(result.contains("with special characters.</p>"));
}

#[test]
fn test_file_location_in_heading() {
    let norg = r#"* Documentation in {:docs/guide.md}"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse file location in heading");
    assert!(result.contains("<h1>Documentation in"));
    assert!(result.contains("<a href=\"docs/guide.md\">"));
    assert!(result.contains("docs/guide.md</a>"));
    assert!(result.contains("</h1>"));
}

#[test]
fn test_file_location_malformed() {
    let norg = r#"Malformed {:file.txt without closing brace"#;
    let result = parse_and_render_norg(norg);
    assert!(result.is_ok());
    // Should handle gracefully
}

#[test]
fn test_file_location_empty() {
    let norg = r#"Empty file location {:} here."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty file location");
    assert!(result.contains("Empty file location"));
    assert!(result.contains("here."));
}