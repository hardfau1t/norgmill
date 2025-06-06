use tracing::warn;

use super::{basic, link};

pub fn render_paragraph<'b>(
    para: &norg::ParagraphSegment,
    builder: &'b mut html::text_content::builders::ParagraphBuilder,
) -> &'b mut html::text_content::builders::ParagraphBuilder {
    match para {
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(t)) => {
            builder.text(t.clone());
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Whitespace) => {
            builder.text(" ");
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Special(chr))
        | norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Escape(chr)) => {
            builder.text(chr.to_string());
        }
        norg::ParagraphSegment::AttachedModifier {
            modifier_type,
            content,
        } => basic::render_attached(*modifier_type, content, builder),
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
    builder
}

/// Sometime All you want is text in given paragraph segments(ex. in link targets or description)
pub fn render_paragraph_to_string(segments: &[norg::ParagraphSegment]) -> String {
    let mut para_builder = html::text_content::Paragraph::builder();
    for segment in segments {
        render_paragraph(segment, &mut para_builder);
    }
    para_builder
        .build()
        .children()
        .iter()
        .map(|c| c.to_string())
        .collect::<String>()
}
