use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use rust_norg::{parse, NorgASTFlat, ParagraphSegment, ParagraphSegmentToken};
use serde::Serialize;
use std::fmt::Write;

pub async fn parse_and_render_body<'a>(
    input: &str,
    hbr: &Handlebars<'a>,
) -> miette::Result<String> {
    let tokens = parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
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
                .map(|segment| render_paragraph(segment, &mut para))
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

fn render_paragraph(para: ParagraphSegment, write_to: &mut String) -> std::fmt::Result {
    match para {
        ParagraphSegment::Token(ParagraphSegmentToken::Text(t)) => write!(write_to, "{}", t),
        ParagraphSegment::Token(ParagraphSegmentToken::Whitespace) => write!(write_to, " "),
        _ => Ok(()),
    }
}
