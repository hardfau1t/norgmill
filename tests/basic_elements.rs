use norgmill::renderer::parse_and_render_norg;

// Basic Character Tests
#[test]
fn test_single_character() {
    let norg = "a";
    let result = parse_and_render_norg(norg).expect("Failed to parse single character");
    assert!(result.contains("<p>a</p>"));
}

#[test]
fn test_multiple_characters() {
    let norg = "hello";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple characters");
    assert!(result.contains("<p>hello</p>"));
}

#[test]
fn test_special_characters() {
    let norg = "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
    let result = parse_and_render_norg(norg).expect("Failed to parse special characters");
    assert!(result.contains("<p>Special chars: !@#$%^&amp;*()_+-=[]{}|;':&quot;,./_&lt;&gt;?</p>"));
}

#[test]
fn test_html_special_characters() {
    let norg = "HTML chars: & < > \" '";
    let result = parse_and_render_norg(norg).expect("Failed to parse HTML special characters");
    assert!(result.contains("<p>HTML chars: &amp; &lt; &gt; &quot; &#x27;</p>"));
}

#[test]
fn test_numeric_characters() {
    let norg = "Numbers: 0123456789";
    let result = parse_and_render_norg(norg).expect("Failed to parse numeric characters");
    assert!(result.contains("<p>Numbers: 0123456789</p>"));
}

#[test]
fn test_mixed_case_characters() {
    let norg = "Mixed Case: AbCdEfGhIjKlMnOpQrStUvWxYz";
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed case characters");
    assert!(result.contains("<p>Mixed Case: AbCdEfGhIjKlMnOpQrStUvWxYz</p>"));
}

#[test]
fn test_unicode_characters() {
    let norg = "Unicode: café, naïve, résumé, 你好, 🌟, 🎉";
    let result = parse_and_render_norg(norg).expect("Failed to parse unicode characters");
    assert!(result.contains("<p>Unicode: café, naïve, résumé, 你好, 🌟, 🎉</p>"));
}

#[test]
fn test_emoji_characters() {
    let norg = "Emojis: 😀😃😄😁😆😅😂🤣😊😇🙂🙃😉😌😍🥰😘😗😙😚😋😛😝😜🤪";
    let result = parse_and_render_norg(norg).expect("Failed to parse emoji characters");
    assert!(result.contains("<p>Emojis: 😀😃😄😁😆😅😂🤣😊😇🙂🙃😉😌😍🥰😘😗😙😚😋😛😝😜🤪</p>"));
}

#[test]
fn test_mathematical_symbols() {
    let norg = "Math: ∑∏∫∞∂∆∇∀∃∈∉∪∩⊂⊃⊆⊇≤≥≠≈≡±×÷√∝∴∵∠∥⊥⟂";
    let result = parse_and_render_norg(norg).expect("Failed to parse mathematical symbols");
    assert!(result.contains("<p>Math: ∑∏∫∞∂∆∇∀∃∈∉∪∩⊂⊃⊆⊇≤≥≠≈≡±×÷√∝∴∵∠∥⊥⟂</p>"));
}

#[test]
fn test_currency_symbols() {
    let norg = "Currency: $¢£¥€₹₽₿";
    let result = parse_and_render_norg(norg).expect("Failed to parse currency symbols");
    assert!(result.contains("<p>Currency: $¢£¥€₹₽₿</p>"));
}

// Basic Word Tests
#[test]
fn test_single_word() {
    let norg = "word";
    let result = parse_and_render_norg(norg).expect("Failed to parse single word");
    assert!(result.contains("<p>word</p>"));
}

#[test]
fn test_multiple_words() {
    let norg = "multiple words here";
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple words");
    assert!(result.contains("<p>multiple words here</p>"));
}

#[test]
fn test_words_with_spaces() {
    let norg = "words   with   multiple   spaces";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with multiple spaces");
    assert!(result.contains("<p>words   with   multiple   spaces</p>"));
}

#[test]
fn test_words_with_tabs() {
    let norg = "words\twith\ttabs";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with tabs");
    assert!(result.contains("<p>words\twith\ttabs</p>"));
}

#[test]
fn test_words_with_mixed_whitespace() {
    let norg = "words \t with \t mixed \t whitespace";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with mixed whitespace");
    assert!(result.contains("<p>words \t with \t mixed \t whitespace</p>"));
}

#[test]
fn test_words_with_punctuation() {
    let norg = "Hello, world! How are you? I'm fine.";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with punctuation");
    assert!(result.contains("<p>Hello, world! How are you? I'm fine.</p>"));
}

#[test]
fn test_words_with_contractions() {
    let norg = "I'm, you're, he's, she's, it's, we're, they're, can't, won't, don't";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with contractions");
    assert!(result.contains("<p>I'm, you're, he's, she's, it's, we're, they're, can't, won't, don't</p>"));
}

#[test]
fn test_words_with_hyphens() {
    let norg = "well-known, state-of-the-art, twenty-one, self-explanatory";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with hyphens");
    assert!(result.contains("<p>well-known, state-of-the-art, twenty-one, self-explanatory</p>"));
}

#[test]
fn test_words_with_apostrophes() {
    let norg = "It's a beautiful day. John's car is red. The cats' toys are everywhere.";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with apostrophes");
    assert!(result.contains("<p>It's a beautiful day. John's car is red. The cats' toys are everywhere.</p>"));
}

#[test]
fn test_words_with_underscores() {
    let norg = "variable_name, another_variable, CONSTANT_VALUE";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with underscores");
    assert!(result.contains("<p>variable_name, another_variable, CONSTANT_VALUE</p>"));
}

#[test]
fn test_words_with_numbers() {
    let norg = "version2.0, page123, item1st, 2nd-place, 3rd-party";
    let result = parse_and_render_norg(norg).expect("Failed to parse words with numbers");
    assert!(result.contains("<p>version2.0, page123, item1st, 2nd-place, 3rd-party</p>"));
}

#[test]
fn test_mixed_language_words() {
    let norg = "English français español 日本語 한국어 中文 العربية русский";
    let result = parse_and_render_norg(norg).expect("Failed to parse mixed language words");
    assert!(result.contains("<p>English français español 日本語 한국어 中文 العربية русский</p>"));
}

// Basic Paragraph Tests
#[test]
fn test_single_paragraph() {
    let norg = "This is a single paragraph with multiple sentences. It contains various words and punctuation. The paragraph should be wrapped in HTML paragraph tags.";
    let result = parse_and_render_norg(norg).expect("Failed to parse single paragraph");
    assert!(result.contains("<p>This is a single paragraph with multiple sentences. It contains various words and punctuation. The paragraph should be wrapped in HTML paragraph tags.</p>"));
}

#[test]
fn test_multiple_paragraphs() {
    let norg = r#"This is the first paragraph. It has some content.

This is the second paragraph. It has different content.

This is the third paragraph. It completes the set."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple paragraphs");
    assert!(result.contains("<p>This is the first paragraph. It has some content.</p>"));
    assert!(result.contains("<p>This is the second paragraph. It has different content.</p>"));
    assert!(result.contains("<p>This is the third paragraph. It completes the set.</p>"));
}

#[test]
fn test_paragraph_with_line_breaks() {
    let norg = r#"This is a paragraph
with line breaks
that should be preserved
in the output."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with line breaks");
    assert!(result.contains("<p>This is a paragraph"));
    assert!(result.contains("with line breaks"));
    assert!(result.contains("that should be preserved"));
    assert!(result.contains("in the output.</p>"));
}

#[test]
fn test_paragraph_with_multiple_spaces() {
    let norg = "This    paragraph    has    multiple    spaces    between    words.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with multiple spaces");
    assert!(result.contains("<p>This    paragraph    has    multiple    spaces    between    words.</p>"));
}

#[test]
fn test_paragraph_with_leading_spaces() {
    let norg = "    This paragraph has leading spaces.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with leading spaces");
    assert!(result.contains("<p>    This paragraph has leading spaces.</p>"));
}

#[test]
fn test_paragraph_with_trailing_spaces() {
    let norg = "This paragraph has trailing spaces.    ";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with trailing spaces");
    assert!(result.contains("<p>This paragraph has trailing spaces.    </p>"));
}

#[test]
fn test_paragraph_with_mixed_whitespace() {
    let norg = "  This\tparagraph\thas\tmixed\twhitespace  ";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with mixed whitespace");
    assert!(result.contains("<p>  This\tparagraph\thas\tmixed\twhitespace  </p>"));
}

#[test]
fn test_empty_paragraph() {
    let norg = "";
    let result = parse_and_render_norg(norg).expect("Failed to parse empty paragraph");
    // Empty input should result in empty or minimal output
    assert!(result.is_empty() || result.trim().is_empty() || result == "<p></p>");
}

#[test]
fn test_paragraph_with_only_spaces() {
    let norg = "   ";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with only spaces");
    // Should handle gracefully
    assert!(result.contains("<p>   </p>") || result.trim().is_empty());
}

#[test]
fn test_paragraph_with_only_newlines() {
    let norg = "\n\n\n";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with only newlines");
    // Should handle gracefully
    assert!(result.is_empty() || result.trim().is_empty() || result.contains("<p>"));
}

#[test]
fn test_long_paragraph() {
    let norg = "This is a very long paragraph that contains many words and sentences. It is designed to test how the parser handles longer text content. The paragraph should be properly wrapped in HTML paragraph tags regardless of its length. It includes various punctuation marks, numbers like 123 and 456, and should maintain all formatting. The goal is to ensure that longer paragraphs are handled correctly by the parsing and rendering system.";
    let result = parse_and_render_norg(norg).expect("Failed to parse long paragraph");
    assert!(result.contains("<p>This is a very long paragraph"));
    assert!(result.contains("parsing and rendering system.</p>"));
}

// Complex Paragraph Tests
#[test]
fn test_paragraph_with_nested_punctuation() {
    let norg = r#"This paragraph contains "quoted text" and 'single quotes' as well as (parentheses) and [square brackets] and {curly braces}."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with nested punctuation");
    assert!(result.contains("<p>This paragraph contains &quot;quoted text&quot; and 'single quotes' as well as (parentheses) and [square brackets] and {curly braces}.</p>"));
}

#[test]
fn test_paragraph_with_email_addresses() {
    let norg = "Contact us at support@example.com or admin@test.org for assistance.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with email addresses");
    assert!(result.contains("<p>Contact us at support@example.com or admin@test.org for assistance.</p>"));
}

#[test]
fn test_paragraph_with_urls() {
    let norg = "Visit https://example.com or http://test.org for more information.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with URLs");
    assert!(result.contains("<p>Visit https://example.com or http://test.org for more information.</p>"));
}

#[test]
fn test_paragraph_with_file_paths() {
    let norg = "The file is located at /path/to/file.txt or C:\\Windows\\System32\\file.exe.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with file paths");
    assert!(result.contains("<p>The file is located at /path/to/file.txt or C:\\Windows\\System32\\file.exe.</p>"));
}

#[test]
fn test_paragraph_with_dates() {
    let norg = "The meeting is scheduled for 2023-12-15 or December 15, 2023.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with dates");
    assert!(result.contains("<p>The meeting is scheduled for 2023-12-15 or December 15, 2023.</p>"));
}

#[test]
fn test_paragraph_with_times() {
    let norg = "The event starts at 14:30 or 2:30 PM.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with times");
    assert!(result.contains("<p>The event starts at 14:30 or 2:30 PM.</p>"));
}

#[test]
fn test_paragraph_with_code_like_text() {
    let norg = "The variable name is user_id and the function is get_user_data().";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with code-like text");
    assert!(result.contains("<p>The variable name is user_id and the function is get_user_data().</p>"));
}

#[test]
fn test_paragraph_with_version_numbers() {
    let norg = "We are using version 1.2.3 or v2.0.0-beta.1 of the software.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with version numbers");
    assert!(result.contains("<p>We are using version 1.2.3 or v2.0.0-beta.1 of the software.</p>"));
}

// Whitespace and Formatting Tests
#[test]
fn test_paragraph_with_various_whitespace() {
    let norg = r#"Text\twith\ttabs\nand\nnewlines\rand\rcarriage\freturns\vand\vvertical\ttabs."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with various whitespace");
    assert!(result.contains(r#"<p>Text\twith\ttabs"#));
    assert!(result.contains(r#"and\nnewlines"#));
    assert!(result.contains(r#"carriage\freturns"#));
    assert!(result.contains(r#"vertical\ttabs.</p>"#));
}

#[test]
fn test_paragraph_with_zero_width_characters() {
    let norg = "Text\u{200B}with\u{200C}zero\u{200D}width\u{FEFF}characters.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with zero-width characters");
    assert!(result.contains("<p>Text\u{200B}with\u{200C}zero\u{200D}width\u{FEFF}characters.</p>"));
}

#[test]
fn test_paragraph_with_combining_characters() {
    let norg = "Text with combining characters: a\u{0301}e\u{0301}i\u{0301}o\u{0301}u\u{0301}";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with combining characters");
    assert!(result.contains("<p>Text with combining characters: a\u{0301}e\u{0301}i\u{0301}o\u{0301}u\u{0301}</p>"));
}

#[test]
fn test_paragraph_with_rtl_text() {
    let norg = "This paragraph contains RTL text: مرحبا بالعالم and back to LTR.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with RTL text");
    assert!(result.contains("<p>This paragraph contains RTL text: مرحبا بالعالم and back to LTR.</p>"));
}

#[test]
fn test_paragraph_with_mixed_scripts() {
    let norg = "Mixed scripts: English, 中文, العربية, русский, ελληνικά, हिन्दी, 한국어, 日本語.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with mixed scripts");
    assert!(result.contains("<p>Mixed scripts: English, 中文, العربية, русский, ελληνικά, हिन्दी, 한국어, 日本語.</p>"));
}

// Paragraph Separation Tests
#[test]
fn test_paragraphs_single_newline() {
    let norg = "First paragraph.\nSecond paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with single newline");
    // Single newline should not create separate paragraphs
    assert!(result.contains("<p>First paragraph.\nSecond paragraph.</p>"));
}

#[test]
fn test_paragraphs_double_newline() {
    let norg = "First paragraph.\n\nSecond paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with double newline");
    // Double newline should create separate paragraphs
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

#[test]
fn test_paragraphs_multiple_newlines() {
    let norg = "First paragraph.\n\n\n\nSecond paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with multiple newlines");
    // Multiple newlines should still create separate paragraphs
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

#[test]
fn test_paragraphs_with_spaces_between() {
    let norg = "First paragraph.\n   \nSecond paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with spaces between");
    // Spaces between newlines should still separate paragraphs
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

#[test]
fn test_paragraphs_with_mixed_whitespace_between() {
    let norg = "First paragraph.\n \t \nSecond paragraph.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraphs with mixed whitespace between");
    // Mixed whitespace between newlines should still separate paragraphs
    assert!(result.contains("<p>First paragraph.</p>"));
    assert!(result.contains("<p>Second paragraph.</p>"));
}

// Edge Cases and Error Handling
#[test]
fn test_paragraph_with_null_character() {
    let norg = "Text with null\0character.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with null character");
    assert!(result.contains("<p>Text with null\0character.</p>"));
}

#[test]
fn test_paragraph_with_control_characters() {
    let norg = "Text with control\x01characters\x02and\x03more.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with control characters");
    assert!(result.contains("<p>Text with control\x01characters\x02and\x03more.</p>"));
}

#[test]
fn test_paragraph_with_very_long_word() {
    let norg = "This paragraph contains a verylongwordthatdoesnotcontainanyspacesorbreaksandmightcauseproblemswithwordwrappinginsomelayoutsystems.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with very long word");
    assert!(result.contains("<p>This paragraph contains a verylongwordthatdoesnotcontainanyspacesorbreaksandmightcauseproblemswithwordwrappinginsomelayoutsystems.</p>"));
}

#[test]
fn test_paragraph_with_many_short_words() {
    let norg = "a b c d e f g h i j k l m n o p q r s t u v w x y z";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with many short words");
    assert!(result.contains("<p>a b c d e f g h i j k l m n o p q r s t u v w x y z</p>"));
}

#[test]
fn test_paragraph_with_repeated_punctuation() {
    let norg = "What??? Really!!! Yes... Maybe??? No!!!";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with repeated punctuation");
    assert!(result.contains("<p>What??? Really!!! Yes... Maybe??? No!!!</p>"));
}

#[test]
fn test_paragraph_with_mathematical_expressions() {
    let norg = "The equation is: x² + y² = r² and the formula is: E = mc².";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with mathematical expressions");
    assert!(result.contains("<p>The equation is: x² + y² = r² and the formula is: E = mc².</p>"));
}

#[test]
fn test_paragraph_with_chemical_formulas() {
    let norg = "Water is H₂O and carbon dioxide is CO₂.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with chemical formulas");
    assert!(result.contains("<p>Water is H₂O and carbon dioxide is CO₂.</p>"));
}

#[test]
fn test_paragraph_with_fractions() {
    let norg = "The fractions are: ½, ¼, ¾, ⅓, ⅔, ⅛, ⅜, ⅝, ⅞.";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with fractions");
    assert!(result.contains("<p>The fractions are: ½, ¼, ¾, ⅓, ⅔, ⅛, ⅜, ⅝, ⅞.</p>"));
}

#[test]
fn test_paragraph_with_arrows() {
    let norg = "Arrows: → ← ↑ ↓ ↔ ↕ ↖ ↗ ↘ ↙ ⇒ ⇐ ⇑ ⇓ ⇔ ⇕";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with arrows");
    assert!(result.contains("<p>Arrows: → ← ↑ ↓ ↔ ↕ ↖ ↗ ↘ ↙ ⇒ ⇐ ⇑ ⇓ ⇔ ⇕</p>"));
}

#[test]
fn test_paragraph_with_geometric_shapes() {
    let norg = "Shapes: ■ □ ▲ △ ● ○ ◆ ◇ ★ ☆ ♠ ♣ ♥ ♦";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with geometric shapes");
    assert!(result.contains("<p>Shapes: ■ □ ▲ △ ● ○ ◆ ◇ ★ ☆ ♠ ♣ ♥ ♦</p>"));
}

#[test]
fn test_paragraph_with_weather_symbols() {
    let norg = "Weather: ☀️ ⛅ ☁️ 🌧️ ⛈️ 🌩️ ❄️ 🌨️ 🌪️ 🌈";
    let result = parse_and_render_norg(norg).expect("Failed to parse paragraph with weather symbols");
    assert!(result.contains("<p>Weather: ☀️ ⛅ ☁️ 🌧️ ⛈️ 🌩️ ❄️ 🌨️ 🌪️ 🌈</p>"));
}
