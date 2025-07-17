use norgmill::renderer::parse_and_render_norg;
use test_log::test;

// Bold Tests
#[test]
fn test_bold_basic() {
    let norg = "*bold text*";
    let result = parse_and_render_norg(norg).expect("Failed to parse bold text");
    assert!(result.contains("<strong>bold text</strong>"));
}

#[test]
fn test_bold_in_sentence() {
    let norg = "This is *bold* text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse bold in sentence");
    assert!(result.contains("This is <strong>bold</strong> text in a sentence."));
}

#[test]
fn test_bold_multiple() {
    let norg = "First *bold* and second *bold* text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple bold");
    assert!(result.contains("First <strong>bold</strong> and second <strong>bold</strong> text."));
}

// Italic Tests
#[test]
fn test_italic_basic() {
    let norg = "/italic text/";
    let result = parse_and_render_norg(norg).expect("Failed to parse italic text");
    assert!(result.contains("<em>italic text</em>"));
}

#[test]
fn test_italic_in_sentence() {
    let norg = "This is /italic/ text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse italic in sentence");
    assert!(result.contains("This is <em>italic</em> text in a sentence."));
}

#[test]
fn test_italic_multiple() {
    let norg = "First /italic/ and second /italic/ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple italic");
    assert!(result.contains("First <em>italic</em> and second <em>italic</em> text."));
}

// Underline Tests
#[test]
fn test_underline_basic() {
    let norg = "_underlined text_";
    let result = parse_and_render_norg(norg).expect("Failed to parse underlined text");
    assert!(result.contains("<u>underlined text</u>"));
}

#[test]
fn test_underline_in_sentence() {
    let norg = "This is _underlined_ text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse underline in sentence");
    assert!(result.contains("This is <u>underlined</u> text in a sentence."));
}

// Strike-through Tests
#[test]
fn test_strikethrough_basic() {
    let norg = "-strikethrough text-";
    let result = parse_and_render_norg(norg).expect("Failed to parse strikethrough text");
    assert!(result.contains("<del>strikethrough text</del>"));
}

#[test]
fn test_strikethrough_in_sentence() {
    let norg = "This is -strikethrough- text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse strikethrough in sentence");
    assert!(result.contains("This is <del>strikethrough</del> text in a sentence."));
}

// Spoiler Tests
#[test]
fn test_spoiler_basic() {
    let norg = "!spoiler text!";
    let result = parse_and_render_norg(norg).expect("Failed to parse spoiler text");
    assert!(
        result.contains("<span class=\"spoiler\">spoiler text</span>")
            || result.contains("<details><summary>spoiler</summary>spoiler text</details>")
    );
}

#[test]
fn test_spoiler_in_sentence() {
    let norg = "This is !spoiler! text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse spoiler in sentence");
    assert!(
        result.contains("This is")
            && result.contains("spoiler")
            && result.contains("text in a sentence.")
    );
}

// Superscript Tests
#[test]
fn test_superscript_basic() {
    let norg = "^superscript^";
    let result = parse_and_render_norg(norg).expect("Failed to parse superscript");
    assert!(result.contains("<sup>superscript</sup>"));
}

#[test]
fn test_superscript_in_sentence() {
    let norg = "This is ^superscript^ text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse superscript in sentence");
    assert!(result.contains("This is <sup>superscript</sup> text in a sentence."));
}

#[test]
fn test_superscript_mathematical() {
    let norg = "E = mc^2^";
    let result = parse_and_render_norg(norg).expect("Failed to parse mathematical superscript");
    assert!(result.contains("E = mc<sup>2</sup>"));
}

// Subscript Tests
#[test]
fn test_subscript_basic() {
    let norg = ",subscript,";
    let result = parse_and_render_norg(norg).expect("Failed to parse subscript");
    assert!(result.contains("<sub>subscript</sub>"));
}

#[test]
fn test_subscript_in_sentence() {
    let norg = "This is ,subscript, text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse subscript in sentence");
    assert!(result.contains("This is <sub>subscript</sub> text in a sentence."));
}

#[test]
fn test_subscript_chemical() {
    let norg = "H,2,O";
    let result = parse_and_render_norg(norg).expect("Failed to parse chemical subscript");
    assert!(result.contains("H<sub>2</sub>O"));
}

// Inline Code Tests
#[test]
fn test_inline_code_basic() {
    let norg = "`inline code`";
    let result = parse_and_render_norg(norg).expect("Failed to parse inline code");
    assert!(result.contains("<code>inline code</code>"));
}

#[test]
fn test_inline_code_in_sentence() {
    let norg = "This is `inline code` in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse inline code in sentence");
    assert!(result.contains("This is <code>inline code</code> in a sentence."));
}

#[test]
fn test_inline_code_with_special_chars() {
    let norg = "`<html>&entities;</html>`";
    let result =
        parse_and_render_norg(norg).expect("Failed to parse inline code with special chars");
    assert!(result.contains("<code>&lt;html&gt;&amp;entities;&lt;/html&gt;</code>"));
}

// Inline Math Tests
#[test]
fn test_inline_math_basic() {
    let norg = "$x = y + z$";
    let result = parse_and_render_norg(norg).expect("Failed to parse inline math");
    assert!(
        result.contains("x = y + z")
            && (result.contains("<math>")
                || result.contains("class=\"math\"")
                || result.contains("\\("))
    );
}

#[test]
fn test_inline_math_in_sentence() {
    let norg = "The equation $E = mc^2$ is famous.";
    let result = parse_and_render_norg(norg).expect("Failed to parse inline math in sentence");
    assert!(
        result.contains("The equation")
            && result.contains("E = mc^2")
            && result.contains("is famous.")
    );
}

// Variable Tests
#[test]
fn test_variable_basic() {
    let norg = "&variable_name&";
    let result = parse_and_render_norg(norg).expect("Failed to parse variable");
    assert!(
        result.contains("variable_name")
            && (result.contains("<var>") || result.contains("class=\"variable\""))
    );
}

#[test]
fn test_variable_in_sentence() {
    let norg = "The variable &count& is important.";
    let result = parse_and_render_norg(norg).expect("Failed to parse variable in sentence");
    assert!(
        result.contains("The variable")
            && result.contains("count")
            && result.contains("is important.")
    );
}

// Null Modifier Tests
#[test]
fn test_null_modifier_basic() {
    let norg = "%hidden text%";
    let result = parse_and_render_norg(norg).expect("Failed to parse null modifier");
    // Null modifier should either be hidden or rendered as comment
    assert!(!result.contains("hidden text") || result.contains("<!-- hidden text -->"));
}

#[test]
fn test_null_modifier_in_sentence() {
    let norg = "This is %hidden% text in a sentence.";
    let result = parse_and_render_norg(norg).expect("Failed to parse null modifier in sentence");
    assert!(result.contains("This is") && result.contains("text in a sentence."));
}

// Combined Modifiers Tests
#[test]
fn test_bold_italic_combined() {
    let norg = "*/bold and italic/*";
    let result = parse_and_render_norg(norg).expect("Failed to parse bold and italic combined");
    assert!(
        result.contains("<strong><em>bold and italic</em></strong>")
            || result.contains("<em><strong>bold and italic</strong></em>")
    );
}

#[test]
fn test_nested_modifiers() {
    let norg = "*Bold with /italic/ inside*";
    let result = parse_and_render_norg(norg).expect("Failed to parse nested modifiers");
    assert!(result.contains("<strong>Bold with <em>italic</em> inside</strong>"));
}

#[test]
fn test_multiple_modifiers_same_text() {
    let norg = "*Bold* and /italic/ and _underlined_ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple modifiers");
    assert!(
        result.contains("<strong>Bold</strong> and <em>italic</em> and <u>underlined</u> text.")
    );
}

// Edge Cases
#[test]
fn test_modifier_with_punctuation() {
    let norg = "This is *bold!* and /italic?/ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse modifier with punctuation");
    assert!(result.contains("This is <strong>bold!</strong> and <em>italic?</em> text."));
}

#[test]
fn test_modifier_at_line_boundaries() {
    let norg = "*Bold at start* and end is *bold*";
    let result = parse_and_render_norg(norg).expect("Failed to parse modifier at boundaries");
    assert!(result.contains("<strong>Bold at start</strong> and end is <strong>bold</strong>"));
}

#[test]
fn test_modifier_with_numbers() {
    let norg = "*Bold123* and /italic456/ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse modifier with numbers");
    assert!(result.contains("<strong>Bold123</strong> and <em>italic456</em> text."));
}

#[test]
fn test_modifier_with_unicode() {
    let norg = "*Bold émojis 🌟* and /italic café/ text.";
    let result = parse_and_render_norg(norg).expect("Failed to parse modifier with unicode");
    assert!(result.contains("<strong>Bold émojis 🌟</strong> and <em>italic café</em> text."));
}

// Invalid Cases (should not be parsed as modifiers)
#[test]
fn test_modifier_with_space_after_opening() {
    let norg = "* not bold* text";
    let result = parse_and_render_norg(norg).expect("Failed to parse invalid modifier");
    assert!(!result.contains("<strong>not bold</strong>"));
}

#[test]
fn test_modifier_with_space_before_closing() {
    let norg = "*not bold * text";
    let result = parse_and_render_norg(norg).expect("Failed to parse invalid modifier");
    assert!(!result.contains("<strong>not bold</strong>"));
}

#[test]
fn test_modifier_across_paragraph_break() {
    let norg = r#"*not bold

text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse modifier across paragraph");
    assert!(!result.contains("<strong>not bold"));
}

#[test]
fn test_double_modifiers_invalid() {
    let norg = "**not bold** text";
    let result = parse_and_render_norg(norg).expect("Failed to parse double modifiers");
    assert!(!result.contains("<strong>not bold</strong>"));
}

// Free-form Modifiers Tests
#[test]
fn test_freeform_bold_basic() {
    let norg = "*| bold with spaces |*";
    let result = parse_and_render_norg(norg).expect("Failed to parse freeform bold");
    assert!(result.contains("<strong> bold with spaces </strong>"));
}

#[test]
fn test_freeform_italic_basic() {
    let norg = "/| italic with spaces |/";
    let result = parse_and_render_norg(norg).expect("Failed to parse freeform italic");
    assert!(result.contains("<em> italic with spaces </em>"));
}

#[test]
fn test_freeform_code_with_backtick() {
    let norg = "`| code with ` backtick |`";
    let result = parse_and_render_norg(norg).expect("Failed to parse freeform code");
    assert!(result.contains("<code> code with ` backtick </code>"));
}

#[test]
fn test_freeform_underline_basic() {
    let norg = "_| underlined with spaces |_";
    let result = parse_and_render_norg(norg).expect("Failed to parse freeform underline");
    assert!(result.contains("<u> underlined with spaces </u>"));
}

#[test]
fn test_freeform_strikethrough_basic() {
    let norg = "-| strikethrough with spaces |-";
    let result = parse_and_render_norg(norg).expect("Failed to parse freeform strikethrough");
    assert!(result.contains("<del> strikethrough with spaces </del>"));
}

