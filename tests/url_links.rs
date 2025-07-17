use norgmill::renderer::parse_and_render_norg;

#[test]
fn test_url_link_basic() {
    let norg = "{https://example.com}";
    let result = parse_and_render_norg(norg).expect("Failed to parse basic URL link");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("https://example.com</a>"));
}

#[test]
fn test_url_link_with_description() {
    let norg = "{https://example.com}[Example Website]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with description");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Example Website</a>"));
}

#[test]
fn test_url_link_http() {
    let norg = "{http://example.com}";
    let result = parse_and_render_norg(norg).expect("Failed to parse HTTP URL link");
    assert!(result.contains("<a href=\"http://example.com\">"));
    assert!(result.contains("http://example.com</a>"));
}

#[test]
fn test_url_link_https() {
    let norg = "{https://secure.example.com}";
    let result = parse_and_render_norg(norg).expect("Failed to parse HTTPS URL link");
    assert!(result.contains("<a href=\"https://secure.example.com\">"));
    assert!(result.contains("https://secure.example.com</a>"));
}

#[test]
fn test_url_link_with_port() {
    let norg = "{https://example.com:8080}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with port");
    assert!(result.contains("<a href=\"https://example.com:8080\">"));
    assert!(result.contains("https://example.com:8080</a>"));
}

#[test]
fn test_url_link_with_path() {
    let norg = "{https://example.com/path/to/page}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with path");
    assert!(result.contains("<a href=\"https://example.com/path/to/page\">"));
    assert!(result.contains("https://example.com/path/to/page</a>"));
}

#[test]
fn test_url_link_with_query_params() {
    let norg = "{https://example.com/search?q=test&lang=en}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with query params");
    assert!(result.contains("<a href=\"https://example.com/search?q=test&amp;lang=en\">"));
    assert!(result.contains("https://example.com/search?q=test&amp;lang=en</a>"));
}

#[test]
fn test_url_link_with_fragment() {
    let norg = "{https://example.com/page#section}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with fragment");
    assert!(result.contains("<a href=\"https://example.com/page#section\">"));
    assert!(result.contains("https://example.com/page#section</a>"));
}

#[test]
fn test_url_link_with_query_and_fragment() {
    let norg = "{https://example.com/page?q=test#section}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with query and fragment");
    assert!(result.contains("<a href=\"https://example.com/page?q=test#section\">"));
    assert!(result.contains("https://example.com/page?q=test#section</a>"));
}

#[test]
fn test_url_link_github() {
    let norg = "{https://github.com/user/repo}";
    let result = parse_and_render_norg(norg).expect("Failed to parse GitHub URL link");
    assert!(result.contains("<a href=\"https://github.com/user/repo\">"));
    assert!(result.contains("https://github.com/user/repo</a>"));
}

#[test]
fn test_url_link_github_with_description() {
    let norg = "{https://github.com/user/repo}[My Repository]";
    let result = parse_and_render_norg(norg).expect("Failed to parse GitHub URL link with description");
    assert!(result.contains("<a href=\"https://github.com/user/repo\">"));
    assert!(result.contains("My Repository</a>"));
}

#[test]
fn test_url_link_subdomain() {
    let norg = "{https://api.example.com/v1/users}";
    let result = parse_and_render_norg(norg).expect("Failed to parse subdomain URL link");
    assert!(result.contains("<a href=\"https://api.example.com/v1/users\">"));
    assert!(result.contains("https://api.example.com/v1/users</a>"));
}

#[test]
fn test_url_link_localhost() {
    let norg = "{http://localhost:3000}";
    let result = parse_and_render_norg(norg).expect("Failed to parse localhost URL link");
    assert!(result.contains("<a href=\"http://localhost:3000\">"));
    assert!(result.contains("http://localhost:3000</a>"));
}

#[test]
fn test_url_link_ip_address() {
    let norg = "{http://192.168.1.1}";
    let result = parse_and_render_norg(norg).expect("Failed to parse IP address URL link");
    assert!(result.contains("<a href=\"http://192.168.1.1\">"));
    assert!(result.contains("http://192.168.1.1</a>"));
}

#[test]
fn test_url_link_ftp() {
    let norg = "{ftp://ftp.example.com/file.txt}";
    let result = parse_and_render_norg(norg).expect("Failed to parse FTP URL link");
    assert!(result.contains("<a href=\"ftp://ftp.example.com/file.txt\">"));
    assert!(result.contains("ftp://ftp.example.com/file.txt</a>"));
}

#[test]
fn test_url_link_mailto() {
    let norg = "{mailto:test@example.com}";
    let result = parse_and_render_norg(norg).expect("Failed to parse mailto URL link");
    assert!(result.contains("<a href=\"mailto:test@example.com\">"));
    assert!(result.contains("mailto:test@example.com</a>"));
}

#[test]
fn test_url_link_file_scheme() {
    let norg = "{file:///path/to/file.txt}";
    let result = parse_and_render_norg(norg).expect("Failed to parse file scheme URL link");
    assert!(result.contains("<a href=\"file:///path/to/file.txt\">"));
    assert!(result.contains("file:///path/to/file.txt</a>"));
}

#[test]
fn test_url_link_in_paragraph() {
    let norg = "Visit {https://example.com} for more information.";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link in paragraph");
    assert!(result.contains("<p>Visit <a href=\"https://example.com\">"));
    assert!(result.contains("https://example.com</a> for more information.</p>"));
}

#[test]
fn test_url_link_with_markup_in_description() {
    let norg = "{https://example.com}[*Bold* and /italic/ description]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with markup in description");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("<strong>Bold</strong> and <em>italic</em> description</a>"));
}

#[test]
fn test_url_link_multiple_in_paragraph() {
    let norg = "Visit {https://example.com} and {https://another.com}[Another Site] for info.";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple URL links in paragraph");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("https://example.com</a>"));
    assert!(result.contains("<a href=\"https://another.com\">"));
    assert!(result.contains("Another Site</a>"));
}

#[test]
fn test_url_link_in_list() {
    let norg = r#"- First item with {https://example.com}[link]
- Second item with {https://another.com}
- Third item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse URL links in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>First item with <a href=\"https://example.com\">"));
    assert!(result.contains("link</a></li>"));
    assert!(result.contains("<li>Second item with <a href=\"https://another.com\">"));
    assert!(result.contains("https://another.com</a></li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_url_link_in_quote() {
    let norg = "> This quote has a {https://example.com}[link] in it.";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote has a <a href=\"https://example.com\">"));
    assert!(result.contains("link</a> in it."));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_url_link_in_heading() {
    let norg = "* Heading with {https://example.com}[link]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link in heading");
    assert!(result.contains("<h1>Heading with <a href=\"https://example.com\">"));
    assert!(result.contains("link</a></h1>"));
}

#[test]
fn test_url_link_long_url() {
    let norg = "{https://very-long-domain-name.example.com/very/long/path/to/resource?with=many&query=parameters&and=values#long-fragment-identifier}";
    let result = parse_and_render_norg(norg).expect("Failed to parse long URL link");
    assert!(result.contains("<a href=\"https://very-long-domain-name.example.com/very/long/path/to/resource?with=many&amp;query=parameters&amp;and=values#long-fragment-identifier\">"));
}

#[test]
fn test_url_link_with_special_characters() {
    let norg = "{https://example.com/path with spaces}[Description with & special chars]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with special characters");
    assert!(result.contains("<a href=\"https://example.com/path with spaces\">"));
    assert!(result.contains("Description with &amp; special chars</a>"));
}

#[test]
fn test_url_link_with_unicode() {
    let norg = "{https://example.com/café}[Café Website]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with unicode");
    assert!(result.contains("<a href=\"https://example.com/café\">"));
    assert!(result.contains("Café Website</a>"));
}

#[test]
fn test_url_link_with_encoded_characters() {
    let norg = "{https://example.com/search?q=hello%20world}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with encoded characters");
    assert!(result.contains("<a href=\"https://example.com/search?q=hello%20world\">"));
    assert!(result.contains("https://example.com/search?q=hello%20world</a>"));
}

#[test]
fn test_url_link_complex_query_params() {
    let norg = "{https://example.com/api?sort=name&order=asc&limit=10&offset=20}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with complex query params");
    assert!(result.contains("<a href=\"https://example.com/api?sort=name&amp;order=asc&amp;limit=10&amp;offset=20\">"));
}

#[test]
fn test_url_link_with_authentication() {
    let norg = "{https://user:pass@example.com/secure}";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with authentication");
    assert!(result.contains("<a href=\"https://user:pass@example.com/secure\">"));
}

#[test]
fn test_url_link_case_insensitive_scheme() {
    let norg = "{HTTPS://EXAMPLE.COM}";
    let result = parse_and_render_norg(norg).expect("Failed to parse case insensitive URL link");
    assert!(result.contains("<a href=\"HTTPS://EXAMPLE.COM\">"));
    assert!(result.contains("HTTPS://EXAMPLE.COM</a>"));
}

#[test]
fn test_url_link_empty_description() {
    let norg = "{https://example.com}[]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with empty description");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("</a>"));
}

#[test]
fn test_url_link_description_with_spaces() {
    let norg = "{https://example.com}[A description with multiple spaces]";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with spaced description");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("A description with multiple spaces</a>"));
}

#[test]
fn test_url_link_nested_in_bold() {
    let norg = "*Bold text with {https://example.com}[link] inside*";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link nested in bold");
    assert!(result.contains("<strong>Bold text with <a href=\"https://example.com\">"));
    assert!(result.contains("link</a> inside</strong>"));
}

#[test]
fn test_url_link_nested_in_italic() {
    let norg = "/Italic text with {https://example.com}[link] inside/";
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link nested in italic");
    assert!(result.contains("<em>Italic text with <a href=\"https://example.com\">"));
    assert!(result.contains("link</a> inside</em>"));
}

#[test]
fn test_url_link_with_line_breaks_in_description() {
    let norg = r#"{https://example.com}[Description
with line breaks]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse URL link with line breaks in description");
    assert!(result.contains("<a href=\"https://example.com\">"));
    assert!(result.contains("Description"));
    assert!(result.contains("with line breaks</a>"));
}

// Edge cases and error handling
#[test]
fn test_url_link_malformed_missing_closing_brace() {
    let norg = "{https://example.com";
    let result = parse_and_render_norg(norg);
    // Should handle gracefully - either parse as text or handle the error
    assert!(result.is_ok());
}

#[test]
fn test_url_link_malformed_missing_opening_brace() {
    let norg = "https://example.com}";
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed URL link");
    // Should be treated as regular text
    assert!(result.contains("https://example.com}"));
}

#[test]
fn test_url_link_malformed_missing_description_closing() {
    let norg = "{https://example.com}[description";
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_url_link_empty_url() {
    let norg = "{}[Empty URL]";
    let result = parse_and_render_norg(norg);
    // Should handle gracefully
    assert!(result.is_ok());
}