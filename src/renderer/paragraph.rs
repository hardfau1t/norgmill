use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use std::fmt::Write;
use tracing::{debug, warn};

use super::{basic, link};

pub fn render_paragraph(
    para: &norg::ParagraphSegment,
    write_to: &mut String,
    hbr: &Handlebars,
) -> miette::Result<()> {
    // TODO: convert this to miette error
    match para {
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Text(t)) => {
            write!(write_to, "{}", t)
                .into_diagnostic()
                .wrap_err("couldn't write text segment in para")
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Whitespace) => {
            write!(write_to, " ")
                .into_diagnostic()
                .wrap_err("couldn't write whitespace segment in para")
        }
        norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Special(chr))
        | norg::ParagraphSegment::Token(norg::ParagraphSegmentToken::Escape(chr)) => {
            debug!("treating {para:?} as normal, at this point not sure whether norg-parser handles this or this crate, raise a bug if this is an issue");
            write!(write_to, "{}", chr)
                .into_diagnostic()
                .wrap_err("couldn't write escape segment in para")
        }
        norg::ParagraphSegment::AttachedModifier {
            modifier_type,
            content,
        } => {
            let mut attached_string = String::new();
            basic::render_attached(*modifier_type, content, &mut attached_string, hbr);
            write!(write_to, "{}", attached_string)
                .into_diagnostic()
                .wrap_err("couldn't write attached modifier segment in para")
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
            let rendered_link =
                link::render_link(filepath.as_deref(), targets, description.as_deref(), hbr)?;
            write!(write_to, "{}", rendered_link)
                .into_diagnostic()
                .wrap_err("couldn't write link segment in para")
        }
        norg::ParagraphSegment::InlineVerbatim(tokens) => {
            let rendered_code = render_inline_code(tokens, hbr)?;
            write_to.push_str(&rendered_code);
            Ok(())
        }
        _ => {
            warn!("rendering para segment {para:?} is not yet implemented");
            Ok(())
        }
    }
}

#[derive(Debug, Serialize)]
struct InlineCode<'a> {
    code: &'a str,
}

fn render_inline_code(
    tokens: &[norg::ParagraphSegmentToken],
    hbr: &Handlebars,
) -> miette::Result<String> {
    let mut rendered_para = String::new();
    for token in tokens {
        render_paragraph(
            &norg::ParagraphSegment::Token(token.clone()),
            &mut rendered_para,
            hbr,
        )?;
    }
    let inline_code = InlineCode {
        code: &rendered_para,
    };
    let rendered_code = hbr
        .render("inline-code", &inline_code)
        .into_diagnostic()
        .wrap_err("couldn't render inline code block")?;
    Ok(rendered_code)
}
