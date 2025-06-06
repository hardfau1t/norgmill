use clap::Parser;
use miette::{Context, IntoDiagnostic};

#[derive(Parser, Debug, Clone)]
struct Args {
    // dump flat ast instead of tree
    #[arg(short, long)]
    flat: bool,
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();
    let input = tokio::fs::read_to_string(&args.path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Couldn't read {:?}", args.path))?;
    if args.flat {
        let tokens = norg::parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
        println!("{tokens:#?}");
    } else {
        let tokens =
            norg::parse_tree(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
        println!("{tokens:#?}");
    }
    Ok(())
}
