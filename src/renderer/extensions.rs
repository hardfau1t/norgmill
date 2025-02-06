//! module which does rendering of extensions like todo etc
use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{instrument, trace, warn};

#[derive(Debug, Serialize)]
struct TodoExtension<'a> {
    content: &'a str,
    date: Option<&'a str>,
}

#[instrument(skip(content))]
pub fn apply_extension(
    extension: &norg::DetachedModifierExtension,
    content: &str,
    hbr: &Handlebars,
) -> miette::Result<String> {
    trace!("applying extension");
    match extension {
        norg::DetachedModifierExtension::Todo(todo_status) => {
            let mut todo_extension = TodoExtension {
                content: &content,
                date: None,
            };
            match todo_status {
                norg::TodoStatus::Undone => hbr.render("todo-undone", &todo_extension),
                norg::TodoStatus::Done => hbr.render("todo-done", &todo_extension),
                norg::TodoStatus::NeedsClarification => hbr.render("todo-clarify", &todo_extension),
                norg::TodoStatus::Paused => hbr.render("todo-paused", &todo_extension),
                norg::TodoStatus::Urgent => hbr.render("todo-urgent", &todo_extension),
                norg::TodoStatus::Recurring(date) => {
                    if let Some(date) = date {
                        todo_extension.date = Some(date);
                        hbr.render("todo-recurring-date", &todo_extension)
                    } else {
                        hbr.render("todo-recurring", &todo_extension)
                    }
                }
                norg::TodoStatus::Pending => hbr.render("todo-pending", &todo_extension),
                norg::TodoStatus::Canceled => hbr.render("todo-cancelled", &todo_extension),
            }
        }
        _ => {
            warn!(?extension, "This extensions is not yet supported");
            miette::bail!("Unsupported feature")
        }
    }
    .into_diagnostic()
    .wrap_err("Couldn't render extension")
}
