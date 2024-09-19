use handlebars::Handlebars;
use std::fmt::Write;
use tracing::{debug, warn};

use super::{link, basic};

pub fn render_paragraph(
    para: norg::ParagraphSegment,
    write_to: &mut String,
    hbr: &Handlebars,
) -> std::fmt::Result {
    // TODO: convert this to miette error
    match para {
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(t)) => {
            write!(write_to, "{}", t)
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Whitespace) => {
            write!(write_to, " ")
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Special(chr))
        | norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Escape(chr)) => {
            debug!("treating {para:?} as normal, at this point not sure whether norg-parser handles this or this crate, raise a bug if this is an issue");
            write!(write_to, "{}", chr)
        }
        norg::ParagraphSegment::AttachedModifier {
            modifier_type,
            content,
        } => {
            let mut attached_string = String::new();
            basic::render_attached(modifier_type, content, &mut attached_string, hbr);
            write!(write_to, "{}", attached_string)
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
            let rendered_link = link::render_link(filepath, targets, description, hbr);
            write!(write_to, "{}", rendered_link)
        }
        _ => {
            warn!("rendering para segment {para:?} is not yet implemented");
            Ok(())
        }
    }
}
