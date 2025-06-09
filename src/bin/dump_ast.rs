use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::{debug, info, instrument};

#[derive(Parser, Debug, Clone)]
struct Args {
    // dump flat ast instead of tree
    #[arg(short, long)]
    flat: bool,
    path: std::path::PathBuf,
}

#[tokio::main]
#[instrument]
async fn main() -> miette::Result<()> {
    let args = Args::parse();
    info!(args = ?args, "Starting AST dump");
    let input = tokio::fs::read_to_string(&args.path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Couldn't read {:?}", args.path))?;
    debug!(path = %args.path.display(), bytes = input.len(), "Successfully read file content");
    if args.flat {
        info!("Dumping flat AST");
        let tokens = norg::parse(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
        println!("{tokens:#?}");
    } else {
        info!("Dumping tree AST");
        let tokens =
            norg::parse_tree(&input).map_err(|e| miette::miette!("failed to parse: {e:?}"))?;
        println!("{tokens:#?}");
    }
    info!(path = %args.path.display(), "Successfully dumped AST");
    Ok(())
}
