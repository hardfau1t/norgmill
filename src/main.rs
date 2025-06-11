#![recursion_limit = "512"]
use std::{collections::HashMap, sync::Arc, time::SystemTime};

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
use tracing::{debug, error, info, instrument, level_filters::LevelFilter, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// parsed html content, if the file is already parsed and no new change is found then serve it immediately
#[derive(Debug, Clone)]
struct ParsedFile {
    content: String,
    last_modified_time: std::time::SystemTime,
}

#[derive(Debug, Clone)]
struct AppState {
    root_dir: std::path::PathBuf,
    parsed_files: dashmap::DashMap<std::path::PathBuf, ParsedFile>,
}

impl AppState {
    fn insert_cache_file(&self, file_path: std::path::PathBuf, content: String) {
        info!(?file_path, "caching rendered file");
        let new_entry = ParsedFile {
            content,
            last_modified_time: SystemTime::now(),
        };
        self.parsed_files.insert(file_path, new_entry);
    }

    async fn get_cached_file(&self, file_path: &std::path::PathBuf) -> Option<String> {
        trace!(?file_path, "checking for cached rendered file");
        let parsed_file = self.parsed_files.get(file_path);
        let metadata = match tokio::fs::metadata(file_path).await {
            Ok(d) => d,
            Err(e) => {
                warn!(
                    error = ?e,
                    "Couldn't get the metadata of the file, skipping cache checking"
                );
                return None;
            }
        };
        let last_modified_time = metadata.modified().ok()?;
        parsed_file
            .filter(|parsed_file| last_modified_time < parsed_file.last_modified_time)
            .map(|parsed_entry| parsed_entry.content.clone())
    }

    async fn get_or_insert_cached_file(
        &self,
        file_path: &mut std::path::PathBuf,
    ) -> Result<Html<String>, http::StatusCode> {
        update_extension(file_path);
        if let Some(s) = self.get_cached_file(&file_path).await {
            info!(?file_path, "returning cached file");
            Ok(Html(s))
        } else {
            info!(?file_path, "rendering fresh copy");
            let rendered_file = read_and_render_file(&file_path).await?;
            self.insert_cache_file(file_path.clone(), rendered_file.0.clone());
            Ok(rendered_file)
        }
    }
}

#[instrument(skip(file_path))]
async fn render_norg_file<'a>(
    file_path: &std::path::PathBuf,
) -> miette::Result<(String, html::text_content::Division)> {
    trace!("rendering norg file");
    let content = tokio::fs::read_to_string(&file_path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| miette!("reading file: {file_path:?}"))?;
    debug!(path = %file_path.display(), "Successfully read file content");

    let title = file_path
        .file_stem()
        .expect("norg file without stem cannot be present")
        .to_string_lossy()
        .to_string();
    let content_div = tokio::task::spawn_blocking(move || {
        renderer::parse_and_render_norg(&content).wrap_err("Couldn't parse the file")
    })
    .await
    .into_diagnostic()
    .wrap_err("Couldn't spawn blocking thread")??;
    debug!(path = %file_path.display(), "Successfully generated HTML page");
    Ok((title, content_div))
}

fn should_it_render_raw(qparams: HashMap<String, String>) -> bool {
    qparams
        .get(constants::ARG_RAW)
        .is_some_and(|val| constants::ARG_RAW_POSSIBLE_VALS.contains(&val.to_lowercase().as_str()))
}

fn update_extension(file_path: &mut std::path::PathBuf) {
    if !file_path.extension().is_some_and(|ext| ext == "norg") {
        debug!(original_path=?file_path,"setting .norg extension");
        file_path.set_extension("norg");
    };
}

async fn read_raw_file(file_path: &std::path::PathBuf) -> Result<Html<String>, http::StatusCode> {
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
}

async fn read_and_render_file(
    file_path: &std::path::PathBuf,
) -> Result<Html<String>, http::StatusCode> {
    // if the extension is not .norg then set it and load the norg file
    debug!(path = %file_path.display(), "Constructed full path for index route");
    match render_norg_file(file_path).await {
        Ok((title, body)) => Ok(generate_html_page(title, body)),
        Err(e) => {
            error!("Failed to render norg file: {e}");
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn generate_html_page(title: String, content: html::text_content::Division) -> Html<String> {
    let navigation_buttons = html::content::Header::builder()
        .class("site-header")
        .division(|div_b| {
            div_b
                .class("header-content")
                .heading_1(|hb| hb.class("site-title").text(title.clone()))
                .navigation(|nav_b| {
                    nav_b
                        .anchor(|anchor_b| {
                            anchor_b
                                .href(constants::CURRENT_WORKSPACE_PATH)
                                .text("Home")
                        })
                        .anchor(|anchor_b| anchor_b.href("#").text("Up"))
                        .anchor(|anchor_b| anchor_b.href("#").text("Next"))
                        .anchor(|anchor_b| anchor_b.href("#").text("Prev"))
                        .button(|button_b| {
                            button_b
                                .class("theme-toggle")
                                .aria_label("Toggel dark/light mode")
                                .span(|span_b| span_b.class("icon").text("☀️"))
                        })
                })
        })
        .build();
    let body = html::root::Body::builder()
        .push(navigation_buttons)
        .main(|main_b| {
            main_b
                .class("norg_content")
                .article(|art_b| art_b.push(content))
        })
        .build();

    Html(
        html::root::Html::builder()
            .head(|hb| {hb.title(|tb|
                  tb.text(title))
                  .lang("en")
                  .meta(|mb| mb.charset("UTF-8"))
                  .meta(|mb| mb.name("viewport").content("width=device-width, initial-scale=1.0"))
                  .link(|link_builder| link_builder.rel("stylesheet").href("/static/style.css")) // TODO: replace href with reading while compile time
                  .script(|script_b| script_b.src("/static/scripts.js"))
                  .link(|lb| lb.rel("preconnect").href("https://fonts.googleapis.com"))
                  .link(|lb| lb.rel("preconnect").href("https://fonts.gstatic.com"))
                  .link(|lb| lb.rel("stylesheet").href("https://fonts.googleapis.com/css2?family=Source+Sans+Pro:wght@600;700&family=Source+Serif+Pro:wght@400;700&display=swap"))
            })
            .push(body)
            .build()
            .to_string(),
    )
}

#[instrument]
async fn render_home_file(
    State(state): State<Arc<AppState>>,
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
    if should_it_render_raw(qparams) {
        read_raw_file(&file_path).await
    } else {
        state.get_or_insert_cached_file(&mut file_path).await
    }
}

#[instrument]
async fn render_root_system_file(
    State(state): State<Arc<AppState>>,
    Path(norg_file_path): Path<std::path::PathBuf>,
    Query(qparams): Query<HashMap<String, String>>,
) -> Result<Html<String>, http::StatusCode> {
    trace!("rendering from system files");
    let mut file_path = std::path::PathBuf::from("/");
    file_path.push(&norg_file_path);
    if should_it_render_raw(qparams) {
        read_raw_file(&file_path).await
    } else {
        state.get_or_insert_cached_file(&mut file_path).await
    }
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
    if should_it_render_raw(qparams) {
        read_raw_file(&file_path).await
    } else {
        update_extension(&mut file_path);
        state.get_or_insert_cached_file(&mut file_path).await
    }
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

#[instrument]
async fn serve(root_dir: std::path::PathBuf) -> miette::Result<()> {
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
        .with_state(std::sync::Arc::new(AppState {
            root_dir,
            parsed_files: dashmap::DashMap::new(),
        }));

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
        Functionality::Serve { root_dir } => serve(root_dir)
            .await
            .wrap_err("Couldn't run the http server")?,
    };
    Ok(())
}
