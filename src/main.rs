use miette::{miette, Context, IntoDiagnostic};
use rust_norg::parse;

fn main() -> miette::Result<()> {
    let file_path = "/home/gireesh/.local/share/notes/projects/saankhya/ddaas/ddas.norg";
    let content = std::fs::read_to_string(file_path)
        .into_diagnostic()
        .wrap_err_with(|| miette!("reading file: {file_path}"))?;
    let tokens = parse(&content).map_err(|e| miette!("failed to parse: {e:?}"))?;
    println!("tokens: {tokens:#?}");
    Ok(())
}
