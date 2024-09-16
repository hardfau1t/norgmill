use std::collections::HashMap;

use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use rust_norg::parse;
use tracing::{debug, error, info};

pub async fn parse_and_render(input: &str) -> miette::Result<String> {
    let tokens = parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
    debug!("tokens: {tokens:?}");

    let reg = Handlebars::new();

    let mut store = HashMap::new();
    store.insert("name", "hardfau1t");
    // render without register
    let rendered_text = reg
        .render_template("Hello {{name}}", &store)
        .into_diagnostic()
        .wrap_err("couldn't rendered template")?;

    Ok(input.to_string())
}
