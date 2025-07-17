use std::fmt::Write;
use tracing::{trace, warn};

use super::{basic, link};

pub fn render_segments(para_segments: &[norg::ParagraphSegment]) -> Result<String, std::fmt::Error> {
    let mut output = String::new();
    for segment in para_segments {
        render_segment(segment, &mut output)?;
    }
    Ok(output)
}

pub fn render_paragraph(para_segments: &[norg::ParagraphSegment], output: &mut String)->std::fmt::Result {
    output.push_str("<p>");
    for segment in para_segments {
        render_segment(segment, output)?;
    }
    output.push_str("</p>");
    Ok(())

}

pub fn render_segment(para: &norg::ParagraphSegment, output: &mut String)->std::fmt::Result {
    trace!(para=?para,"rendering paragraph");
    match para {
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(t)) => {
            let sanitized = crate::html::sanitize_html(t);
            write!(output, "{}", sanitized)?;
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Whitespace) => {
            write!(output, " ")?;
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Special(chr))
        | norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Escape(chr)) => {
            crate::html::write_char_sanitized(*chr, output)
        }
        norg::ParagraphSegment::AttachedModifier {
            modifier_type,
            content,
        } => {
            write!(output, "<span>")?;
            basic::render_attached(*modifier_type, content, output)?;
            write!(output, "</span>")?;
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
            write!(output, "<span>")?;
            link::render_link(filepath.as_deref(), targets, description.as_deref(), output)?;
            write!(output, "</span>")?;
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
            let sanitized = crate::html::sanitize_html(&rendered_code);
            write!(output, "<code>{}</code>", sanitized)?;
        }
        _ => {
            warn!("rendering para segment {para:?} is not yet implemented");
        }
    };
    Ok(())
}
