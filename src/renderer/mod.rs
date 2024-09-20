use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, trace, warn};

mod basic;
mod heading;
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

pub async fn dump_ast(path: std::path::PathBuf) -> miette::Result<()> {
    let input = tokio::fs::read_to_string(&path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Couldn't read {path:?}"))?;
    let tokens = norg::parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    println!("{tokens:#?}");
    Ok(())
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
        //norg::NorgASTFlat::NestableDetachedModifier { modifier_type, level, extensions, content } => todo!(),
        //norg::NorgASTFlat::RangeableDetachedModifier { modifier_type, title, extensions, content } => todo!(),
        norg::NorgASTFlat::Heading {
            level,
            title,
            extensions,
        } => {
            let mut heading_string = String::new();
            heading::render_heading(level, title, extensions, &mut heading_string, hbr)
                .into_diagnostic()
                .wrap_err("Failed to construct paragraph")?;
        }
        //norg::NorgASTFlat::CarryoverTag { tag_type, name, parameters, next_object } => todo!(),
        //norg::NorgASTFlat::VerbatimRangedTag { name, parameters, content } => todo!(),
        //norg::NorgASTFlat::RangedTag { name, parameters, content } => todo!(),
        //norg::NorgASTFlat::InfirmTag { name, parameters } => todo!(),
        _ => {
            warn!("rendering ast {ast:?} is not yet implemented");
        }
    };
    Ok(rendered_string)
}
