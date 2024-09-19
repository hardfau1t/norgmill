use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use rust_norg::{parse, LinkTarget, NorgASTFlat, ParagraphSegment, ParagraphSegmentToken};
use serde::Serialize;
use std::fmt::Write;
use tracing::{debug, instrument, warn};

pub async fn parse_and_render_body<'h>(
    input: &str,
    hbr: &Handlebars<'h>,
) -> miette::Result<String> {
    let tokens = parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("found tokens: {tokens:#?}");
    tokens.into_iter().map(|ast| render_ast(ast, hbr)).collect()
}

#[derive(Serialize, Debug)]
struct Para {
    para: String,
}

fn render_ast(ast: NorgASTFlat, hbr: &Handlebars) -> miette::Result<String> {
    let mut rendered_string = String::new();
    match ast {
        NorgASTFlat::Paragraph(p) => {
            let mut para = String::new();
            p.into_iter()
                .map(|segment| render_paragraph(segment, &mut para, hbr))
                .collect::<Result<(), _>>()
                .into_diagnostic()
                .wrap_err("Failed to construct paragraph")?;
            let para = Para { para };
            let rendered_para = hbr
                .render("paragraph", &para)
                .into_diagnostic()
                .wrap_err("Failed to render paragraph")?;
            rendered_string.push_str(&rendered_para);
        }
        _ => (),
    };
    Ok(rendered_string)
}

#[instrument()]
fn generate_link<'a>(
    file_path: Option<String>,
    targets: Vec<LinkTarget>,
    description: Option<Vec<ParagraphSegment>>,
) -> Link {
    if let Some(norgfile_path) = file_path {
        debug!("link to another norg file");
        let target = if norgfile_path.ends_with(".norg") {
            norgfile_path
        } else {
            format!("{}.norg", norgfile_path)
        };
        let description = target.clone();
        Link {
            target,
            description,
        }
    } else {
        debug!("not yet parsed link");
        todo!()
    }
}

#[derive(Debug, Serialize)]
struct Link {
    target: String,
    description: String,
}
fn render_paragraph(
    para: ParagraphSegment,
    write_to: &mut String,
    hbr: &Handlebars,
) -> std::fmt::Result {
    // TODO: convert this to miette error
    match para {
        ParagraphSegment::Token(ParagraphSegmentToken::Text(t)) => write!(write_to, "{}", t),
        ParagraphSegment::Token(ParagraphSegmentToken::Whitespace) => write!(write_to, " "),
        ParagraphSegment::Token(ParagraphSegmentToken::Special(chr))
        | ParagraphSegment::Token(ParagraphSegmentToken::Escape(chr)) => {
            debug!("treating {para:?} as normal, at this point not sure whether norg-parser handles this or this crate, raise a bug if this is an issue");
            write!(write_to, "{}", chr)
        }
        //ParagraphSegment::AttachedModifierOpener(_) => todo!(),
        //ParagraphSegment::AttachedModifierOpenerFail(_) => todo!(),
        //ParagraphSegment::AttachedModifierCloserCandidate(_) => todo!(),
        //ParagraphSegment::AttachedModifierCloser(_) => todo!(),
        //ParagraphSegment::AttachedModifierCandidate { modifier_type, content, closer } => todo!(),
        //ParagraphSegment::AttachedModifier { modifier_type, content } => todo!(),
        //ParagraphSegment::AnchorDefinition { content, target } => todo!(),
        //ParagraphSegment::Anchor { content, description } => todo!(),
        //ParagraphSegment::InlineLinkTarget(_) => todo!(),
        ParagraphSegment::Link {
            filepath,
            targets,
            description,
        } => {
            let link = generate_link(filepath, targets, description);
            let rendered_link = hbr
                .render("link", &link)
                .expect("change this to error type");
            write!(write_to, "{}", rendered_link)
        }
        _ => {
            warn!("rendering {para:?} is not yet implemented");
            Ok(())
        }
    }
}
