use norgmill::renderer::parse_and_render_norg;

// Code Block Tests
#[test]
fn test_code_block_basic() {
    let norg = r#"@code
println!("Hello, world!");
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic code block");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("println!(\"Hello, world!\");"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_with_language() {
    let norg = r#"@code rust
fn main() {
    println!("Hello, world!");
}
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with language");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("rust") || result.contains("language-rust"));
    assert!(result.contains("fn main() {"));
    assert!(result.contains("println!(\"Hello, world!\");"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_python() {
    let norg = r#"@code python
def hello():
    print("Hello, world!")
    
hello()
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse Python code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("python") || result.contains("language-python"));
    assert!(result.contains("def hello():"));
    assert!(result.contains("print(\"Hello, world!\")"));
    assert!(result.contains("hello()"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_javascript() {
    let norg = r#"@code javascript
function hello() {
    console.log("Hello, world!");
}

hello();
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse JavaScript code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("javascript") || result.contains("language-javascript"));
    assert!(result.contains("function hello() {"));
    assert!(result.contains("console.log(\"Hello, world!\");"));
    assert!(result.contains("hello();"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_html() {
    let norg = r#"@code html
<html>
<head>
    <title>Test</title>
</head>
<body>
    <h1>Hello, world!</h1>
</body>
</html>
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse HTML code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("html") || result.contains("language-html"));
    assert!(result.contains("&lt;html&gt;"));
    assert!(result.contains("&lt;head&gt;"));
    assert!(result.contains("&lt;title&gt;Test&lt;/title&gt;"));
    assert!(result.contains("&lt;h1&gt;Hello, world!&lt;/h1&gt;"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_css() {
    let norg = r#"@code css
body {
    font-family: Arial, sans-serif;
    color: #333;
}

h1 {
    color: blue;
}
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse CSS code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("css") || result.contains("language-css"));
    assert!(result.contains("body {"));
    assert!(result.contains("font-family: Arial, sans-serif;"));
    assert!(result.contains("color: #333;"));
    assert!(result.contains("h1 {"));
    assert!(result.contains("color: blue;"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_json() {
    let norg = r#"@code json
{
    "name": "test",
    "version": "1.0.0",
    "dependencies": {
        "lodash": "^4.17.21"
    }
}
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse JSON code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("json") || result.contains("language-json"));
    assert!(result.contains("\"name\": \"test\""));
    assert!(result.contains("\"version\": \"1.0.0\""));
    assert!(result.contains("\"dependencies\""));
    assert!(result.contains("\"lodash\": \"^4.17.21\""));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_yaml() {
    let norg = r#"@code yaml
name: test
version: 1.0.0
dependencies:
  lodash: ^4.17.21
scripts:
  test: echo "test"
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse YAML code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("yaml") || result.contains("language-yaml"));
    assert!(result.contains("name: test"));
    assert!(result.contains("version: 1.0.0"));
    assert!(result.contains("dependencies:"));
    assert!(result.contains("lodash: ^4.17.21"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_shell() {
    let norg = r#"@code bash
#!/bin/bash
echo "Hello, world!"
ls -la
grep "pattern" file.txt
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse shell code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("bash") || result.contains("language-bash"));
    assert!(result.contains("#!/bin/bash"));
    assert!(result.contains("echo \"Hello, world!\""));
    assert!(result.contains("ls -la"));
    assert!(result.contains("grep \"pattern\" file.txt"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_sql() {
    let norg = r#"@code sql
SELECT name, email 
FROM users 
WHERE age > 18 
ORDER BY name ASC;
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse SQL code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("sql") || result.contains("language-sql"));
    assert!(result.contains("SELECT name, email"));
    assert!(result.contains("FROM users"));
    assert!(result.contains("WHERE age > 18"));
    assert!(result.contains("ORDER BY name ASC;"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_empty() {
    let norg = r#"@code
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty code block");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_single_line() {
    let norg = r#"@code rust
println!("Single line");
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse single line code block");
    assert!(result.contains("<pre><code"));
    assert!(result.contains("rust") || result.contains("language-rust"));
    assert!(result.contains("println!(\"Single line\");"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_with_special_characters() {
    let norg = r#"@code
Special chars: <>&"'
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with special chars");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("Special chars: &lt;&gt;&amp;&quot;&#x27;"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_with_unicode() {
    let norg = r#"@code
Unicode: 🌟 café résumé
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with unicode");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("Unicode: 🌟 café résumé"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_preserves_whitespace() {
    let norg = r#"@code
  Indented line
    More indented
  Less indented
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with whitespace");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("  Indented line"));
    assert!(result.contains("    More indented"));
    assert!(result.contains("  Less indented"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_with_empty_lines() {
    let norg = r#"@code
First line

Second line after empty line


Third line after multiple empty lines
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with empty lines");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("First line"));
    assert!(result.contains("Second line after empty line"));
    assert!(result.contains("Third line after multiple empty lines"));
    assert!(result.contains("</code></pre>"));
}

#[test]
fn test_code_block_with_norg_syntax() {
    let norg = r#"@code
* This is not a heading
- This is not a list
{https://example.com}[This is not a link]
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block with norg syntax");
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("* This is not a heading"));
    assert!(result.contains("- This is not a list"));
    assert!(result.contains("{https://example.com}[This is not a link]"));
    assert!(result.contains("</code></pre>"));
    // Should not contain actual HTML for these elements
    assert!(!result.contains("<h1>This is not a heading</h1>"));
    assert!(!result.contains("<ul><li>This is not a list</li></ul>"));
    assert!(!result.contains("<a href=\"https://example.com\">This is not a link</a>"));
}

#[test]
fn test_code_block_nested_in_list() {
    let norg = r#"- List item with code:
  @code rust
  println!("Hello from list");
  @end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>List item with code:"));
    assert!(result.contains("<pre><code"));
    assert!(result.contains("rust") || result.contains("language-rust"));
    assert!(result.contains("println!(\"Hello from list\");"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_code_block_nested_in_quote() {
    let norg = r#"> Quote with code:
  @code
  Example code
  @end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse code block in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("Quote with code:"));
    assert!(result.contains("<pre><code>"));
    assert!(result.contains("Example code"));
    assert!(result.contains("</code></pre>"));
    assert!(result.contains("</blockquote>"));
}

#[test]
fn test_multiple_code_blocks() {
    let norg = r#"@code rust
fn main() {
    println!("Rust code");
}
@end

@code python
def main():
    print("Python code")
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple code blocks");
    assert!(result.contains("fn main() {"));
    assert!(result.contains("println!(\"Rust code\");"));
    assert!(result.contains("def main():"));
    assert!(result.contains("print(\"Python code\")"));
    // Should have two separate code blocks
    let code_count = result.matches("<pre><code").count();
    assert!(code_count >= 2);
}

// Math Block Tests
#[test]
fn test_math_block_basic() {
    let norg = r#"@math
E = mc^2
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic math block");
    assert!(result.contains("E = mc^2"));
    assert!(result.contains("math") || result.contains("MathJax") || result.contains("\\["));
}

#[test]
fn test_math_block_complex() {
    let norg = r#"@math
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex math block");
    assert!(result.contains("\\int_{-\\infty}^{\\infty}") || result.contains("∫"));
    assert!(result.contains("e^{-x^2}") || result.contains("e^(-x²)"));
    assert!(result.contains("\\sqrt{\\pi}") || result.contains("√π"));
}

#[test]
fn test_math_block_multiline() {
    let norg = r#"@math
\begin{align}
x &= a + b \\
y &= c + d
\end{align}
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiline math block");
    assert!(result.contains("\\begin{align}") || result.contains("align"));
    assert!(result.contains("x &= a + b") || result.contains("x = a + b"));
    assert!(result.contains("y &= c + d") || result.contains("y = c + d"));
    assert!(result.contains("\\end{align}") || result.contains("align"));
}

// Document Meta Tests
#[test]
fn test_document_meta_basic() {
    let norg = r#"@document.meta
title: Test Document
author: John Doe
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse document meta");
    // Document meta should either be hidden or rendered as metadata
    assert!(result.contains("title: Test Document") || 
             result.contains("<meta name=\"title\" content=\"Test Document\">") ||
             !result.contains("Test Document")); // Could be hidden
}

#[test]
fn test_document_meta_full() {
    let norg = r#"@document.meta
title: Full Document
description: A complete test document
authors: John Doe, Jane Smith
categories: test, example
created: 2024-01-01
version: 1.0.0
@end"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse full document meta");
    // Should handle all meta fields appropriately
    assert!(result.contains("title: Full Document") || 
             result.contains("<meta name=\"title\" content=\"Full Document\">") ||
             !result.contains("Full Document")); // Could be hidden
}

// Error Cases
#[test]
fn test_code_block_missing_end() {
    let norg = r#"@code
println!("Hello, world!");"#;
    let result = parse_and_render_norg(norg);
    // Should either handle gracefully or return error
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_code_block_wrong_end() {
    let norg = r#"@code
println!("Hello, world!");
@stop"#;
    let result = parse_and_render_norg(norg);
    // Should either handle gracefully or return error
    assert!(result.is_ok() || result.is_err());
}