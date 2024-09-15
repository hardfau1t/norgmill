use clap::{Command, Parser, Subcommand};
use miette::{miette, Context, IntoDiagnostic};
use rust_norg::parse;

use axum::{routing, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {}

async fn index() -> Result<String, http::StatusCode> {
    let file_path = shellexpand::full("~/.local/share/notes/index.norg").map_err(|e| {
        error!("failed to expand path: {e}");
        http::StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let content = std::fs::read_to_string(file_path.as_ref())
        .into_diagnostic()
        .wrap_err_with(|| miette!("reading file: {file_path}"))
        .map_err(|e| {
            error!("failed to read {e}");
            http::StatusCode::NOT_FOUND
        })?;
    let tokens = parse(&content)
        .map_err(|e| miette!("failed to parse: {e:?}"))
        .map_err(|e| {
            error!("failed to read {e}");
            http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    debug!("tokens: {tokens:?}");
    Ok(content)
}

#[derive(Debug, Clone, Subcommand)]
enum Functionality {
    Serve,
}

#[derive(Parser, Debug, Clone)]
struct CmdlineArgs {
    #[arg(short, long, global=true, action=clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Functionality,
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/index.html", routing::get(index))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(std::sync::Arc::new(AppState {}));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("Listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdlineArgs::parse();
    dotenv()?;
    #[cfg(debug_assertions)]
    let fmt_event = tracing_subscriber::fmt::format().pretty();
    #[cfg(not(debug_assertions))]
    let fmt_event = tracing_subscriber::fmt::format().compact();
    let log_level = match args.verbose {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        3 => LevelFilter::TRACE,
        _ => {
            eprintln!(concat!(
                "One of the developer of ",
                env!("CARGO_PKG_NAME"),
                " coming to help debug your code"
            ));
            LevelFilter::TRACE
        }
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    "norg_linter=trace,rust_norg=trace,tower_http=trace,axum::rejection=trace"
                } else {
                    "norg_linter=debug,rust_norg=info,tower_http=debug,axum::rejection=info"
                }
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().event_format(fmt_event))
        .with(log_level)
        .init();

    debug!("log level set to {log_level}");
    match args.command {
        Functionality::Serve => serve().await?,
    };
    Ok(())
}
