//! HTML utility functions for sanitization and escaping

use std::borrow::Cow;

/// Sanitizes HTML by escaping special characters that could be used for XSS attacks
/// or to break HTML structure.
///
/// This function escapes the following characters:
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `&` → `&amp;`
/// - `"` → `&quot;`
/// - `'` → `&#x27;`
///
/// # Examples
///
/// ```
/// use norgmill::html::sanitize_html;
///
/// assert_eq!(sanitize_html("<script>alert('xss')</script>"),
///            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
/// assert_eq!(sanitize_html("Hello & goodbye"), "Hello &amp; goodbye");
/// assert_eq!(sanitize_html("\"quoted\""), "&quot;quoted&quot;");
/// ```
pub fn sanitize_html(input: &str) -> Cow<str> {
    // First pass: check if any special characters exist
    let mut first_special_char_pos = None;
    for (i, ch) in input.char_indices() {
        match ch {
            '<' | '>' | '&' | '"' | '\'' => {
                first_special_char_pos = Some(i);
                break;
            }
            _ => {}
        }
    }

    // If no special characters found, return the original string as borrowed
    let first_pos = match first_special_char_pos {
        Some(pos) => pos,
        None => return Cow::Borrowed(input),
    };

    // Special characters found, build escaped string
    let mut output = String::with_capacity(input.len()); // Pre-allocate for efficiency

    // Add the safe prefix
    output.push_str(&input[..first_pos]);

    // Process the rest with escaping
    for ch in input[first_pos..].chars() {
        write_char_sanitized(ch, &mut output)
    }

    Cow::Owned(output)
}

/// Writes a character to the output string, sanitizing it if it's a special HTML character
pub fn write_char_sanitized(ch: char, output: &mut String) {
    match ch {
        '<' => output.push_str("&lt;"),
        '>' => output.push_str("&gt;"),
        '&' => output.push_str("&amp;"),
        '"' => output.push_str("&quot;"),
        '\'' => output.push_str("&#x27;"),
        _ => output.push(ch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_html_basic_escaping() {
        assert_eq!(sanitize_html("<"), "&lt;");
        assert_eq!(sanitize_html(">"), "&gt;");
        assert_eq!(sanitize_html("&"), "&amp;");
        assert_eq!(sanitize_html("\""), "&quot;");
        assert_eq!(sanitize_html("'"), "&#x27;");
    }

    #[test]
    fn test_sanitize_html_script_tag() {
        let input = "<script>alert('xss')</script>";
        let expected = "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_img_tag_with_onerror() {
        let input = "<img src=\"x\" onerror=\"alert('xss')\">";
        let expected = "&lt;img src=&quot;x&quot; onerror=&quot;alert(&#x27;xss&#x27;)&quot;&gt;";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_normal_text() {
        let input = "Hello world! This is normal text.";
        assert_eq!(sanitize_html(input), input);
    }

    #[test]
    fn test_sanitize_html_mixed_content() {
        let input = "Hello & goodbye <world> \"quoted\" text";
        let expected = "Hello &amp; goodbye &lt;world&gt; &quot;quoted&quot; text";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_empty_string() {
        assert_eq!(sanitize_html(""), "");
    }

    #[test]
    fn test_sanitize_html_unicode_characters() {
        let input = "Hello 世界 & <test> 'quoted'";
        let expected = "Hello 世界 &amp; &lt;test&gt; &#x27;quoted&#x27;";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_multiple_ampersands() {
        let input = "A & B & C";
        let expected = "A &amp; B &amp; C";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_already_escaped() {
        let input = "&lt;script&gt;";
        let expected = "&amp;lt;script&amp;gt;";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_javascript_protocol() {
        let input = "<a href=\"javascript:alert('xss')\">Click me</a>";
        let expected =
            "&lt;a href=&quot;javascript:alert(&#x27;xss&#x27;)&quot;&gt;Click me&lt;/a&gt;";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_data_uri() {
        let input = "<img src=\"data:text/html,<script>alert('xss')</script>\">";
        let expected = "&lt;img src=&quot;data:text/html,&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;&quot;&gt;";
        assert_eq!(sanitize_html(input), expected);
    }
}
