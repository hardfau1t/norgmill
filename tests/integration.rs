#![recursion_limit = "512"]
use html::text_content::Division;
use norgmill::renderer::parse_and_render_norg;

#[test]
fn test_all_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let norg_file = std::fs::read_to_string("./tests/fixtures/all_tokens.norg")
        .expect("couldn't read all tokens file");
    let html_file = std::fs::read_to_string("./tests/fixtures/all_tokens.html")
        .expect("fixtures/all_tokens.html file is part of test, which is not found");

    let mut builder = Division::builder();

    let rendered = parse_and_render_norg(&norg_file, &mut builder)
        .expect("Couldn't render all_tokens")
        .build()
        .to_string();

    assert_eq!(rendered, html_file);

    Ok(())
}
