#![recursion_limit = "512"]
use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::{Html, Redirect},
    routing, Router,
};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use miette::{miette, Context, IntoDiagnostic};
use norgmill::{constants, renderer};
use tokio::net::TcpListener;
use tracing::{debug, error, info, instrument, level_filters::LevelFilter, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
struct AppState {
    root_dir: std::path::PathBuf,
}

#[instrument(skip(file_path))]
async fn render_norg_file<'a>(
    file_path: std::path::PathBuf,
) -> Result<Html<String>, http::StatusCode> {
    trace!("rendering norg file");
    let content = tokio::fs::read_to_string(&file_path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| miette!("reading file: {file_path:?}"))
        .map_err(|e| {
            error!("failed to read {e}");
            http::StatusCode::NOT_FOUND
        })?;
    debug!(path = %file_path.display(), "Successfully read file content");

    let title = file_path
        .file_stem()
        .expect("norg file without stem cannot be present")
        .to_string_lossy()
        .to_string();
    let body = tokio::task::spawn_blocking(move || {
        let mut builder = html::root::Body::builder();
        renderer::parse_and_render_body(&content, &mut builder)
            .expect("Couldn't parse the file")
            .build()
    })
    .await
    .expect("Couldn't generate html body");
    let page = html::root::Html::builder().title(title).push(body).build();

    debug!(path = %file_path.display(), "Successfully generated HTML page");
    Ok(Html(page.to_string()))
}

#[instrument(skip(_state))]
async fn system_files(
    State(_state): State<Arc<AppState>>,
    Path(system_path): Path<std::path::PathBuf>,
) -> Result<Html<String>, http::StatusCode> {
    trace!("service system files: {system_path:?}");
    // this is required because `system_path` will not contain / at the beginning
    let mut file_path = std::path::PathBuf::from("/");
    file_path.push(system_path);
    debug!(full_path = %file_path.display(), "Constructed full path for system file");
    if file_path.extension().is_some_and(|e| e == "norg") {
        debug!("Identified as .norg file, proceeding to render.");
        render_norg_file(file_path).await
    } else {
        error!("File not a .norg or not implemented, returning error.");
        Err(http::StatusCode::NOT_IMPLEMENTED)
    }
}

async fn read_and_render_file(
    qparams: HashMap<String, String>,
    mut file_path: std::path::PathBuf,
) -> Result<Html<String>, http::StatusCode> {
    if qparams
        .get(constants::ARG_RAW)
        .is_some_and(|val| constants::ARG_RAW_POSSIBLE_VALS.contains(&val.to_lowercase().as_str()))
    {
        let content = match tokio::fs::read(&file_path).await {
            Ok(v) => v,
            Err(e) => {
                error!("Couldn't load raw file {file_path:?} : {e}");
                return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        match String::from_utf8(content) {
            Ok(raw_string) => {
                let page = html::root::Html::builder()
                    .body(|body_builder| {
                        body_builder
                            .division(|div_builder| {
                                div_builder
                                    .preformatted_text(|text_builder| text_builder.text(raw_string))
                            })
                            .class("raw_div")
                    })
                    .build()
                    .to_string();
                Ok(Html(page))
            }
            Err(e) => {
                error!("Possible raw file, raw binary files are not yet supported: {e}");
                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        // if the extension is not .norg then set it and load the norg file
        if !file_path.extension().is_some_and(|ext| ext == "norg") {
            debug!(original_path=?file_path,"setting .norg extension");
            file_path.set_extension("norg");
        };
        debug!(path = %file_path.display(), "Constructed full path for index route");
        render_norg_file(file_path).await
    }
}

#[instrument]
async fn render_home_file(
    Query(qparams): Query<HashMap<String, String>>,
    Path(norg_file_path): Path<std::path::PathBuf>,
) -> Result<Html<String>, http::StatusCode> {
    trace!("rendering from system files");
    let Ok(home_path) = std::env::var("HOME") else {
        error!("Couldn't get user home directory, this is really odd");
        return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
    };
    let mut file_path = std::path::PathBuf::from(home_path);
    file_path.push(&norg_file_path);
    read_and_render_file(qparams, file_path).await
}

#[instrument]
async fn render_root_system_file(
    Path(norg_file_path): Path<std::path::PathBuf>,
    Query(qparams): Query<HashMap<String, String>>,
) -> Result<Html<String>, http::StatusCode> {
    trace!("rendering from system files");
    let mut file_path = std::path::PathBuf::from("/");
    file_path.push(&norg_file_path);
    read_and_render_file(qparams, file_path).await
}

#[instrument(skip(state))]
async fn render_current_workspace_file(
    State(state): State<Arc<AppState>>,
    Query(qparams): Query<HashMap<String, String>>,
    Path(norg_file_path): Path<std::path::PathBuf>,
) -> Result<Html<String>, http::StatusCode> {
    trace!("rendering index file");
    let mut file_path = state.root_dir.clone();
    file_path.push(&norg_file_path);
    read_and_render_file(qparams, file_path).await
}

#[derive(Debug, Clone, Subcommand)]
enum Functionality {
    Serve {
        #[arg(short, long)]
        root_dir: std::path::PathBuf,
    },
}

#[derive(Parser, Debug, Clone)]
struct CmdlineArgs {
    #[arg(short, long, global=true, action=clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Functionality,
}

#[instrument(skip(dev_mode))]
async fn serve(root_dir: std::path::PathBuf, dev_mode: bool) -> miette::Result<()> {
    debug!(root_dir = %root_dir.display(), dev_mode = %dev_mode, "Serve function called");
    info!("starting server");

    let app = Router::new()
        .route(
            "/",
            routing::get(|| async { Redirect::to(constants::paths::CURRENT_WORKSPACE_ROOT) }),
        )
        .route(
            constants::CURRENT_WORKSPACE_PATH,
            routing::get(|| async { Redirect::to(constants::paths::CURRENT_WORKSPACE_ROOT) }),
        )
        .route(
            const_format::concatcp!(constants::CURRENT_WORKSPACE_PATH, "/"),
            routing::get(|| async { Redirect::to(constants::paths::CURRENT_WORKSPACE_ROOT) }),
        )
        .route(
            constants::paths::CURRENT_WORKSPACE_FILE,
            routing::get(render_current_workspace_file),
        )
        .route(constants::paths::HOME_FILES, routing::get(render_home_file))
        .route(
            constants::paths::SYSTEM_FILES,
            routing::get(render_root_system_file),
        )
        .nest_service("/static", tower_http::services::ServeDir::new("assets"))
        .nest_service(
            constants::paths::DIRECTORY_SERVE,
            tower_http::services::ServeDir::new("/"),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(std::sync::Arc::new(AppState { root_dir }));

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .into_diagnostic()
        .wrap_err("Unable to start a tcp listener")?;
    info!(
        "Listening on {:?}",
        listener
            .local_addr()
            .into_diagnostic()
            .wrap_err("Couldn't get the local address")?
    );
    axum::serve(listener, app.into_make_service())
        .await
        .into_diagnostic()
        .wrap_err("Failed to start axum server")?;
    Ok(())
}

#[tokio::main]
#[instrument]
async fn main() -> miette::Result<()> {
    let args = CmdlineArgs::parse();
    //#[cfg(debug_assertions)]
    if let Err(e) = dotenv().into_diagnostic() {
        eprintln!("Couldn't load dotenv file: {e}");
    }
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
                    "norgmill=trace,rust_norg=trace,tower_http=trace,axum::rejection=trace"
                } else {
                    "norgmill=debug,rust_norg=info,tower_http=debug,axum::rejection=info"
                }
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().event_format(fmt_event))
        .with(log_level)
        .init();

    debug!("log level set to {log_level}");
    info!(command = ?args.command, "Executing command");
    match args.command {
        Functionality::Serve { dev_mode, root_dir } => serve(root_dir, dev_mode)
            .await
            .wrap_err("Couldn't run the http server")?,
    };
    Ok(())
}
