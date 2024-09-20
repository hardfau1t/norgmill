use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing, Router,
};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use miette::{miette, Context, IntoDiagnostic};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod renderer;

#[derive(Debug, Clone)]
struct AppState {
    hbr: handlebars::Handlebars<'static>,
    root_dir: std::path::PathBuf,
}

#[derive(Debug, Serialize)]
struct NorgPage {
    title: String,
    body: String,
}

async fn index(
    State(state): State<Arc<AppState>>,
    Path(norg_file_path): Path<std::path::PathBuf>,
) -> Result<Html<String>, http::StatusCode> {
    let mut file_path = state.root_dir.clone();
    file_path.push(&norg_file_path);
    let content = tokio::fs::read_to_string(&file_path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| miette!("reading file: {file_path:?}"))
        .map_err(|e| {
            error!("failed to read {e}");
            http::StatusCode::NOT_FOUND
        })?;
    let body = renderer::parse_and_render_body(&content, &state.hbr)
        .await
        .map_err(|e| {
            error!("failed to read {e}");
            http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    debug!("norg page: {body}");
    let norg_page = NorgPage {
        title: file_path
            .file_stem()
            .expect("norg file without stem cannot be present")
            .to_string_lossy()
            .to_string(),
        body,
    };
    let page = state
        .hbr
        .render("base", &norg_page)
        .into_diagnostic()
        .wrap_err("Couldn't render base template")
        .map_err(|e| {
            error!("failed to render: {e}");
            http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Html(page))
}

#[derive(Debug, Clone, Subcommand)]
enum Functionality {
    Serve {
        #[arg(short, long)]
        /// automatically refresh templates without restarting
        dev_mode: bool,
    },
    DumpAst {
        path: std::path::PathBuf,
    },
}

#[derive(Parser, Debug, Clone)]
struct CmdlineArgs {
    #[arg(short, long, global=true, action=clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long)]
    root_dir: std::path::PathBuf,
    #[command(subcommand)]
    command: Functionality,
}

async fn serve(
    root_dir: std::path::PathBuf,
    dev_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("serving content of {root_dir:?}");
    let mut handlebars_registry = handlebars::Handlebars::new();
    handlebars_registry.set_dev_mode(dev_mode);
    let load_options = handlebars::DirectorySourceOptions::default();
    handlebars_registry.register_templates_directory("./templates", load_options)?;
    let app = Router::new()
        .route(
            "/",
            routing::get(|| async { Redirect::to("/read/index.norg") }),
        )
        .route(
            "/read/",
            routing::get(|| async { Redirect::to("/read/index.norg") }),
        )
        .route(
            "/read",
            routing::get(|| async { Redirect::to("/read/index.norg") }),
        )
        .route("/read/*file_path", routing::get(index))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(std::sync::Arc::new(AppState {
            root_dir,
            hbr: handlebars_registry,
        }));

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
                    "norg_viewer=trace,rust_norg=trace,tower_http=trace,axum::rejection=trace"
                } else {
                    "norg_viewer=debug,rust_norg=info,tower_http=debug,axum::rejection=info"
                }
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().event_format(fmt_event))
        .with(log_level)
        .init();

    debug!("log level set to {log_level}");
    match args.command {
        Functionality::Serve { dev_mode } => serve(args.root_dir, dev_mode).await?,
        Functionality::DumpAst { path } => {
            let mut absolute_path = args.root_dir.clone();
            absolute_path.push(path);
            renderer::dump_ast(absolute_path).await?
        }
    };
    Ok(())
}
