#![recursion_limit = "512"]

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use norgmill::renderer;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tracing::{debug, info, instrument};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input .norg file to convert (reads from stdin if not provided).
    #[clap(value_parser)]
    input_file: Option<PathBuf>,

    /// Output .html file (writes to stdout if not provided).
    #[clap(value_parser)]
    output_file: Option<PathBuf>,
}

#[instrument]
fn main() -> miette::Result<()> {
    // Initialize tracing_subscriber if you want to see logs
    // tracing_subscriber::fmt::init();

    let args = Args::parse();

    let norg_content = match &args.input_file {
        Some(path) => {
            info!(input = %path.display(), "Reading from input file");
            fs::read_to_string(path)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to read Norg file: {}", path.display()))?
        }
        None => {
            info!("Reading from stdin");
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .into_diagnostic()
                .wrap_err("Failed to read from stdin")?;
            buffer
        }
    };

    debug!(bytes = norg_content.len(), "Read Norg content successfully");

    let content = renderer::parse_and_render_norg(&norg_content)
        .wrap_err("Failed to parse and render Norg content")?;

    let title = args
        .input_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Norg Document".to_string());

    let escaped_title = norgmill::html::sanitize_html(&title);
    let full_html_output = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
    {}
</body>
</html>"#,
        escaped_title, content
    );

    debug!(
        bytes = full_html_output.len(),
        "Generated HTML successfully"
    );

    match &args.output_file {
        Some(path) => {
            info!(output = %path.display(), "Writing to output file");
            fs::write(path, full_html_output)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to write HTML to file: {}", path.display()))?;
        }
        None => {
            info!("Writing to stdout");
            io::stdout()
                .write_all(full_html_output.as_bytes())
                .into_diagnostic()
                .wrap_err("Failed to write to stdout")?;
        }
    }

    Ok(())
}
