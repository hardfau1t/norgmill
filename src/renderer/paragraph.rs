use tracing::{trace, warn};

use super::{basic, link};

pub fn render_paragraph(para_segments: &[norg::ParagraphSegment]) -> html::text_content::Paragraph {
    let mut para_builder = html::text_content::Paragraph::builder();
    para_segments
        .iter()
        .for_each(|segment| render_paragraph_segment(segment, &mut para_builder));
    para_builder.build()
}

pub fn render_paragraph_segment<'b>(
    para: &norg::ParagraphSegment,
    builder: &'b mut html::text_content::builders::ParagraphBuilder,
) {
    trace!(para=?para,"rendering paragraph");
    match para {
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(t)) => {
            builder.text(t.clone());
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Whitespace) => {
            builder.text(" ");
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Special(chr))
        | norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Escape(chr)) => {
            // TODO: this is not correct, norg special characters are not special for html
            builder.text(
                chr.to_string()
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;"),
            );
        }
        norg::ParagraphSegment::AttachedModifier {
            modifier_type,
            content,
        } => {
            basic::render_attached(*modifier_type, content, builder);
        }
        //ParagraphSegment::AttachedModifierOpener(_) => todo!(),
        //ParagraphSegment::AttachedModifierOpenerFail(_) => todo!(),
        //ParagraphSegment::AttachedModifierCloserCandidate(_) => todo!(),
        //ParagraphSegment::AttachedModifierCloser(_) => todo!(),
        //ParagraphSegment::AttachedModifierCandidate { modifier_type, content, closer } => todo!(),
        //ParagraphSegment::AnchorDefinition { content, target } => todo!(),
        //ParagraphSegment::Anchor { content, description } => todo!(),
        //ParagraphSegment::InlineLinkTarget(_) => todo!(),
        norg::ParagraphSegment::Link {
            filepath,
            targets,
            description,
        } => {
            link::render_link(
                filepath.as_deref(),
                targets,
                description.as_deref(),
                builder,
            );
        }
        norg::ParagraphSegment::InlineVerbatim(tokens) => {
            let mut rendered_code = String::with_capacity(tokens.len());
            for token in tokens {
                match token {
                    norg::ParagraphSegmentToken::Text(text) => rendered_code.push_str(&text),
                    norg::ParagraphSegmentToken::Whitespace => rendered_code.push(' '),
                    norg::ParagraphSegmentToken::Special(c) => rendered_code.push(*c),
                    norg::ParagraphSegmentToken::Escape(c) => rendered_code.push(*c),
                }
            }
            builder.code(|cb| cb.text(rendered_code));
        }
        _ => {
            warn!("rendering para segment {para:?} is not yet implemented");
        }
    };
}

/// Sometime All you want is text in given paragraph segments(ex. in link targets or description)
pub fn render_paragraph_to_string(segments: &[norg::ParagraphSegment]) -> String {
    trace!(?segments, "rendering paragraph segments to string");
    render_paragraph(segments)
        .children()
        .iter()
        .map(|i| i.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use norg::ParagraphSegment;
    use norg::ParagraphSegmentToken;

    #[test]
    fn test_render_paragraph_text() {
        let segment = [ParagraphSegment::Token(ParagraphSegmentToken::Text(
            "hello world".to_string(),
        ))];
        let result = render_paragraph(&segment).to_string();
        assert_eq!(result, "<p>hello world</p>");
    }

    #[test]
    fn test_render_paragraph_whitespace() {
        let mut builder = html::text_content::Paragraph::builder();
        let segment = ParagraphSegment::Token(ParagraphSegmentToken::Whitespace);
        render_paragraph_segment(&segment, &mut builder);
        let result = builder.build().to_string();
        assert_eq!(result, "<p> </p>");
    }

    #[test]
    fn test_render_paragraph_special() {
        let mut builder = html::text_content::Paragraph::builder();
        let segment = ParagraphSegment::Token(ParagraphSegmentToken::Special('&'));
        render_paragraph_segment(&segment, &mut builder);
        let result = builder.build().to_string();
        assert_eq!(result, "<p>&amp;</p>");
    }

    #[test]
    fn test_render_paragraph_escape() {
        let mut builder = html::text_content::Paragraph::builder();
        let segment = ParagraphSegment::Token(ParagraphSegmentToken::Escape('<'));
        render_paragraph_segment(&segment, &mut builder);
        let result = builder.build().to_string();
        assert_eq!(result, "<p>&lt;</p>");
    }

    #[test]
    fn test_render_paragraph_to_string_multiple_segments() {
        let segments = vec![
            ParagraphSegment::Token(ParagraphSegmentToken::Text("hello".to_string())),
            ParagraphSegment::Token(ParagraphSegmentToken::Whitespace),
            ParagraphSegment::Token(ParagraphSegmentToken::Text("world".to_string())),
        ];
        let result = render_paragraph_to_string(&segments);
        assert_eq!(result, "hello world");
    }
}
