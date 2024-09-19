use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, warn};

mod basic;
mod link;
mod paragraph;

pub async fn parse_and_render_body<'h>(
    input: &str,
    hbr: &Handlebars<'h>,
) -> miette::Result<String> {
    let tokens = norg::parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("found tokens: {tokens:#?}");
    tokens.into_iter().map(|ast| render_ast(ast, hbr)).collect()
}

#[derive(Serialize, Debug)]
struct Para {
    para: String,
}

fn render_ast(ast: norg::NorgASTFlat, hbr: &Handlebars) -> miette::Result<String> {
    let mut rendered_string = String::new();
    match ast {
        norg::NorgASTFlat::Paragraph(p) => {
            let mut para = String::new();
            p.into_iter()
                .map(|segment| paragraph::render_paragraph(segment, &mut para, hbr))
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
        _ => {
            warn!("rendering ast {ast:?} is not yet implemented");
        }
    };
    Ok(rendered_string)
}
