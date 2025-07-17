use norgmill::renderer::parse_and_render_norg;

// Language Extensions Tests
#[test]
fn test_language_extension_basic() {
    let norg = r#"This is *lang:en bold English text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic language extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("bold English text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include language attribute
    assert!(result.contains("lang=\"en\"") || result.contains("lang='en'"));
}

#[test]
fn test_language_extension_different_languages() {
    let norg = r#"French: *lang:fr bonjour*
Spanish: *lang:es hola*
German: *lang:de hallo*
Japanese: *lang:ja こんにちは*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse different language extensions");
    assert!(result.contains("French:"));
    assert!(result.contains("lang=\"fr\"") || result.contains("lang='fr'"));
    assert!(result.contains("bonjour"));
    assert!(result.contains("Spanish:"));
    assert!(result.contains("lang=\"es\"") || result.contains("lang='es'"));
    assert!(result.contains("hola"));
    assert!(result.contains("German:"));
    assert!(result.contains("lang=\"de\"") || result.contains("lang='de'"));
    assert!(result.contains("hallo"));
    assert!(result.contains("Japanese:"));
    assert!(result.contains("lang=\"ja\"") || result.contains("lang='ja'"));
    assert!(result.contains("こんにちは"));
}

#[test]
fn test_language_extension_with_italic() {
    let norg = r#"This is /lang:fr texte en italique français/."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse language extension with italic");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<em"));
    assert!(result.contains("texte en italique français"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("lang=\"fr\"") || result.contains("lang='fr'"));
}

#[test]
fn test_language_extension_with_code() {
    let norg = r#"Code example: `lang:python print("Hello, world!")`"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse language extension with code");
    assert!(result.contains("<p>Code example:"));
    assert!(result.contains("<code"));
    assert!(result.contains("print(\"Hello, world!\")"));
    assert!(result.contains("</code>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("lang=\"python\"") || result.contains("lang='python'"));
}

#[test]
fn test_language_extension_with_underline() {
    let norg = r#"Underlined German: _lang:de unterstrichener Text_"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse language extension with underline");
    assert!(result.contains("<p>Underlined German:"));
    assert!(result.contains("<u"));
    assert!(result.contains("unterstrichener Text"));
    assert!(result.contains("</u>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("lang=\"de\"") || result.contains("lang='de'"));
}

#[test]
fn test_language_extension_regional_codes() {
    let norg = r#"Regional codes: *lang:en-US American English*, *lang:en-GB British English*, *lang:fr-CA Canadian French*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse regional language codes");
    assert!(result.contains("Regional codes:"));
    assert!(result.contains("lang=\"en-US\"") || result.contains("lang='en-US'"));
    assert!(result.contains("American English"));
    assert!(result.contains("lang=\"en-GB\"") || result.contains("lang='en-GB'"));
    assert!(result.contains("British English"));
    assert!(result.contains("lang=\"fr-CA\"") || result.contains("lang='fr-CA'"));
    assert!(result.contains("Canadian French"));
}

// Color Extensions Tests
#[test]
fn test_color_extension_basic() {
    let norg = r#"This is *color:red red text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic color extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("red text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include color styling
    assert!(result.contains("color:red") || result.contains("color=\"red\"") || result.contains("style"));
}

#[test]
fn test_color_extension_named_colors() {
    let norg = r#"Colors: *color:red red*, *color:blue blue*, *color:green green*, *color:yellow yellow*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse named color extensions");
    assert!(result.contains("Colors:"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("color:blue") || result.contains("color=\"blue\""));
    assert!(result.contains("color:green") || result.contains("color=\"green\""));
    assert!(result.contains("color:yellow") || result.contains("color=\"yellow\""));
}

#[test]
fn test_color_extension_hex_colors() {
    let norg = r#"Hex colors: *color:#FF0000 red*, *color:#00FF00 green*, *color:#0000FF blue*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse hex color extensions");
    assert!(result.contains("Hex colors:"));
    assert!(result.contains("#FF0000") || result.contains("color=\"#FF0000\""));
    assert!(result.contains("#00FF00") || result.contains("color=\"#00FF00\""));
    assert!(result.contains("#0000FF") || result.contains("color=\"#0000FF\""));
}

#[test]
fn test_color_extension_rgb_colors() {
    let norg = r#"RGB colors: *color:rgb(255,0,0) red*, *color:rgb(0,255,0) green*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse RGB color extensions");
    assert!(result.contains("RGB colors:"));
    assert!(result.contains("rgb(255,0,0)"));
    assert!(result.contains("rgb(0,255,0)"));
}

#[test]
fn test_color_extension_with_italic() {
    let norg = r#"Colored italic: /color:purple purple italic text/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse color extension with italic");
    assert!(result.contains("<p>Colored italic:"));
    assert!(result.contains("<em"));
    assert!(result.contains("purple italic text"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("color:purple") || result.contains("color=\"purple\""));
}

#[test]
fn test_color_extension_with_code() {
    let norg = r#"Colored code: `color:darkgreen console.log("Hello")`"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse color extension with code");
    assert!(result.contains("<p>Colored code:"));
    assert!(result.contains("<code"));
    assert!(result.contains("console.log(\"Hello\")"));
    assert!(result.contains("</code>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("color:darkgreen") || result.contains("color=\"darkgreen\""));
}

// Class Extensions Tests
#[test]
fn test_class_extension_basic() {
    let norg = r#"This is *class:highlight highlighted text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic class extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("highlighted text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include class attribute
    assert!(result.contains("class=\"highlight\"") || result.contains("class='highlight'"));
}

#[test]
fn test_class_extension_multiple_classes() {
    let norg = r#"Multiple classes: *class:primary important emphasized text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple class extensions");
    assert!(result.contains("Multiple classes:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("emphasized text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("class=\"primary important\"") || result.contains("class='primary important'"));
}

#[test]
fn test_class_extension_with_italic() {
    let norg = r#"Classed italic: /class:italic-style italic text with class/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse class extension with italic");
    assert!(result.contains("<p>Classed italic:"));
    assert!(result.contains("<em"));
    assert!(result.contains("italic text with class"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("class=\"italic-style\"") || result.contains("class='italic-style'"));
}

#[test]
fn test_class_extension_with_underline() {
    let norg = r#"Classed underline: _class:underline-style underlined text with class_"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse class extension with underline");
    assert!(result.contains("<p>Classed underline:"));
    assert!(result.contains("<u"));
    assert!(result.contains("underlined text with class"));
    assert!(result.contains("</u>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("class=\"underline-style\"") || result.contains("class='underline-style'"));
}

// ID Extensions Tests
#[test]
fn test_id_extension_basic() {
    let norg = r#"This is *id:important-text important text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic id extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("important text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include id attribute
    assert!(result.contains("id=\"important-text\"") || result.contains("id='important-text'"));
}

#[test]
fn test_id_extension_with_italic() {
    let norg = r#"ID italic: /id:italic-element italic text with id/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse id extension with italic");
    assert!(result.contains("<p>ID italic:"));
    assert!(result.contains("<em"));
    assert!(result.contains("italic text with id"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("id=\"italic-element\"") || result.contains("id='italic-element'"));
}

#[test]
fn test_id_extension_with_code() {
    let norg = r#"ID code: `id:code-snippet function()`"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse id extension with code");
    assert!(result.contains("<p>ID code:"));
    assert!(result.contains("<code"));
    assert!(result.contains("function()"));
    assert!(result.contains("</code>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("id=\"code-snippet\"") || result.contains("id='code-snippet'"));
}

// Style Extensions Tests
#[test]
fn test_style_extension_basic() {
    let norg = r#"This is *style:font-weight:bold;color:red styled text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic style extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("styled text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include style attribute
    assert!(result.contains("style=\"font-weight:bold;color:red\"") || result.contains("style='font-weight:bold;color:red'"));
}

#[test]
fn test_style_extension_background_color() {
    let norg = r#"Background: *style:background-color:yellow highlighted text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse style extension with background");
    assert!(result.contains("Background:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("highlighted text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("background-color:yellow"));
}

#[test]
fn test_style_extension_font_properties() {
    let norg = r#"Font styling: *style:font-size:18px;font-family:Arial large Arial text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse style extension with font properties");
    assert!(result.contains("Font styling:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("large Arial text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("font-size:18px"));
    assert!(result.contains("font-family:Arial"));
}

#[test]
fn test_style_extension_with_italic() {
    let norg = r#"Styled italic: /style:color:blue;text-decoration:underline blue underlined italic/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse style extension with italic");
    assert!(result.contains("<p>Styled italic:"));
    assert!(result.contains("<em"));
    assert!(result.contains("blue underlined italic"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("color:blue"));
    assert!(result.contains("text-decoration:underline"));
}

// Data Extensions Tests
#[test]
fn test_data_extension_basic() {
    let norg = r#"This is *data-type:important important data*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic data extension");
    assert!(result.contains("<p>This is"));
    assert!(result.contains("<strong"));
    assert!(result.contains("important data"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</p>"));
    // Should include data attribute
    assert!(result.contains("data-type=\"important\"") || result.contains("data-type='important'"));
}

#[test]
fn test_data_extension_multiple_attributes() {
    let norg = r#"Multiple data: *data-id:123 data-category:primary text with data*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple data extensions");
    assert!(result.contains("Multiple data:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("text with data"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("data-id=\"123\"") || result.contains("data-id='123'"));
    assert!(result.contains("data-category=\"primary\"") || result.contains("data-category='primary'"));
}

#[test]
fn test_data_extension_with_underline() {
    let norg = r#"Data underline: _data-highlight:true underlined with data_"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse data extension with underline");
    assert!(result.contains("<p>Data underline:"));
    assert!(result.contains("<u"));
    assert!(result.contains("underlined with data"));
    assert!(result.contains("</u>"));
    assert!(result.contains("</p>"));
    assert!(result.contains("data-highlight=\"true\"") || result.contains("data-highlight='true'"));
}

// Combined Extensions Tests
#[test]
fn test_combined_extensions() {
    let norg = r#"Combined: *lang:en color:red class:highlight important text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse combined extensions");
    assert!(result.contains("Combined:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("important text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("lang=\"en\"") || result.contains("lang='en'"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"highlight\"") || result.contains("class='highlight'"));
}

#[test]
fn test_combined_extensions_with_italic() {
    let norg = r#"Combined italic: /lang:fr color:blue class:french-text texte français/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse combined extensions with italic");
    assert!(result.contains("Combined italic:"));
    assert!(result.contains("<em"));
    assert!(result.contains("texte français"));
    assert!(result.contains("</em>"));
    assert!(result.contains("lang=\"fr\"") || result.contains("lang='fr'"));
    assert!(result.contains("color:blue") || result.contains("color=\"blue\""));
    assert!(result.contains("class=\"french-text\"") || result.contains("class='french-text'"));
}

#[test]
fn test_combined_extensions_with_code() {
    let norg = r#"Combined code: `lang:javascript color:darkgreen class:code-snippet console.log("Hello")`"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse combined extensions with code");
    assert!(result.contains("Combined code:"));
    assert!(result.contains("<code"));
    assert!(result.contains("console.log(\"Hello\")"));
    assert!(result.contains("</code>"));
    assert!(result.contains("lang=\"javascript\"") || result.contains("lang='javascript'"));
    assert!(result.contains("color:darkgreen") || result.contains("color=\"darkgreen\""));
    assert!(result.contains("class=\"code-snippet\"") || result.contains("class='code-snippet'"));
}

#[test]
fn test_all_extensions_combined() {
    let norg = r#"All extensions: *lang:en color:red class:highlight id:special-text style:font-weight:bold;text-decoration:underline data-type:important ultimate text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse all extensions combined");
    assert!(result.contains("All extensions:"));
    assert!(result.contains("<strong"));
    assert!(result.contains("ultimate text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("lang=\"en\"") || result.contains("lang='en'"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"highlight\"") || result.contains("class='highlight'"));
    assert!(result.contains("id=\"special-text\"") || result.contains("id='special-text'"));
    assert!(result.contains("style=\"font-weight:bold;text-decoration:underline\"") || result.contains("style='font-weight:bold;text-decoration:underline'"));
    assert!(result.contains("data-type=\"important\"") || result.contains("data-type='important'"));
}

// Extensions with Different Modifiers Tests
#[test]
fn test_extensions_with_strikethrough() {
    let norg = r#"Strikethrough: -color:gray class:deleted deleted text-"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions with strikethrough");
    assert!(result.contains("Strikethrough:"));
    assert!(result.contains("<del"));
    assert!(result.contains("deleted text"));
    assert!(result.contains("</del>"));
    assert!(result.contains("color:gray") || result.contains("color=\"gray\""));
    assert!(result.contains("class=\"deleted\"") || result.contains("class='deleted'"));
}

#[test]
fn test_extensions_with_spoiler() {
    let norg = r#"Spoiler: !color:red class:spoiler hidden content!"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions with spoiler");
    assert!(result.contains("Spoiler:"));
    assert!(result.contains("hidden content"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"spoiler\"") || result.contains("class='spoiler'"));
}

#[test]
fn test_extensions_with_superscript() {
    let norg = r#"Superscript: ^color:blue class:super superscript text^"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions with superscript");
    assert!(result.contains("Superscript:"));
    assert!(result.contains("<sup"));
    assert!(result.contains("superscript text"));
    assert!(result.contains("</sup>"));
    assert!(result.contains("color:blue") || result.contains("color=\"blue\""));
    assert!(result.contains("class=\"super\"") || result.contains("class='super'"));
}

#[test]
fn test_extensions_with_subscript() {
    let norg = r#"Subscript: ,color:green class:sub subscript text,"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions with subscript");
    assert!(result.contains("Subscript:"));
    assert!(result.contains("<sub"));
    assert!(result.contains("subscript text"));
    assert!(result.contains("</sub>"));
    assert!(result.contains("color:green") || result.contains("color=\"green\""));
    assert!(result.contains("class=\"sub\"") || result.contains("class='sub'"));
}

#[test]
fn test_extensions_with_variable() {
    let norg = r#"Variable: &color:purple class:var variable_name&"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions with variable");
    assert!(result.contains("Variable:"));
    assert!(result.contains("<var"));
    assert!(result.contains("variable_name"));
    assert!(result.contains("</var>"));
    assert!(result.contains("color:purple") || result.contains("color=\"purple\""));
    assert!(result.contains("class=\"var\"") || result.contains("class='var'"));
}

// Extensions in Different Contexts Tests
#[test]
fn test_extensions_in_headings() {
    let norg = r#"* Heading with *color:red class:important important text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions in headings");
    assert!(result.contains("<h1>Heading with"));
    assert!(result.contains("<strong"));
    assert!(result.contains("important text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</h1>"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"important\"") || result.contains("class='important'"));
}

#[test]
fn test_extensions_in_lists() {
    let norg = r#"- Item with *color:blue class:highlight blue text*
- Another item with /lang:fr class:french texte français/"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions in lists");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Item with"));
    assert!(result.contains("<strong"));
    assert!(result.contains("blue text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</li>"));
    assert!(result.contains("<li>Another item with"));
    assert!(result.contains("<em"));
    assert!(result.contains("texte français"));
    assert!(result.contains("</em>"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("color:blue") || result.contains("color=\"blue\""));
    assert!(result.contains("class=\"highlight\"") || result.contains("class='highlight'"));
    assert!(result.contains("lang=\"fr\"") || result.contains("lang='fr'"));
    assert!(result.contains("class=\"french\"") || result.contains("class='french'"));
}

#[test]
fn test_extensions_in_quotes() {
    let norg = r#"> This quote contains *color:red class:emphasis emphasized text*."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extensions in quotes");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains"));
    assert!(result.contains("<strong"));
    assert!(result.contains("emphasized text"));
    assert!(result.contains("</strong>"));
    assert!(result.contains("</blockquote>"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"emphasis\"") || result.contains("class='emphasis'"));
}

// Error Cases and Edge Cases Tests
#[test]
fn test_malformed_extension_syntax() {
    let norg = r#"Malformed: *color red text* and *lang en text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed extension syntax");
    assert!(result.contains("Malformed:"));
    assert!(result.contains("red text"));
    assert!(result.contains("en text"));
    // Should handle gracefully, either as regular bold or ignore malformed extensions
}

#[test]
fn test_empty_extension_values() {
    let norg = r#"Empty values: *color: text* and *lang: text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty extension values");
    assert!(result.contains("Empty values:"));
    assert!(result.contains("text"));
    // Should handle gracefully
}

#[test]
fn test_extension_with_special_characters() {
    let norg = r#"Special chars: *color:red class:special-chars text with & < > " ' chars*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with special characters");
    assert!(result.contains("Special chars:"));
    assert!(result.contains("text with &amp; &lt; &gt; &quot; &#x27; chars"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"special-chars\"") || result.contains("class='special-chars'"));
}

#[test]
fn test_extension_with_unicode() {
    let norg = r#"Unicode: *color:blue class:unicode text with émojis 🌟*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with unicode");
    assert!(result.contains("Unicode:"));
    assert!(result.contains("text with émojis 🌟"));
    assert!(result.contains("color:blue") || result.contains("color=\"blue\""));
    assert!(result.contains("class=\"unicode\"") || result.contains("class='unicode'"));
}

#[test]
fn test_extension_with_whitespace() {
    let norg = r#"Whitespace: *color:red class:test    text with spaces   *"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with whitespace");
    assert!(result.contains("Whitespace:"));
    assert!(result.contains("text with spaces"));
    assert!(result.contains("color:red") || result.contains("color=\"red\""));
    assert!(result.contains("class=\"test\"") || result.contains("class='test'"));
}

#[test]
fn test_extension_case_sensitivity() {
    let norg = r#"Case: *COLOR:RED CLASS:TEST text* and *Color:Blue Class:Test text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension case sensitivity");
    assert!(result.contains("Case:"));
    assert!(result.contains("text"));
    // Should handle case sensitivity appropriately
}

#[test]
fn test_extension_with_numbers() {
    let norg = r#"Numbers: *color:#FF0000 class:item-123 data-id:456 numbered text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with numbers");
    assert!(result.contains("Numbers:"));
    assert!(result.contains("numbered text"));
    assert!(result.contains("#FF0000"));
    assert!(result.contains("item-123"));
    assert!(result.contains("456"));
}

#[test]
fn test_extension_with_hyphens_and_underscores() {
    let norg = r#"Separators: *class:my-class_name data-custom-attr:value_123 text*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with separators");
    assert!(result.contains("Separators:"));
    assert!(result.contains("text"));
    assert!(result.contains("my-class_name"));
    assert!(result.contains("data-custom-attr"));
    assert!(result.contains("value_123"));
}

#[test]
fn test_nested_extensions() {
    let norg = r#"Nested: *color:red outer *class:inner inner text* back to outer*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested extensions");
    assert!(result.contains("Nested:"));
    assert!(result.contains("outer"));
    assert!(result.contains("inner text"));
    assert!(result.contains("back to outer"));
    // Should handle nesting appropriately
}

#[test]
fn test_extension_with_urls() {
    let norg = r#"URL style: *style:background-image:url(https://example.com/image.jpg) background image*"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse extension with URLs");
    assert!(result.contains("URL style:"));
    assert!(result.contains("background image"));
    assert!(result.contains("background-image:url(https://example.com/image.jpg)"));
}