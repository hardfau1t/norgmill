//! module which does rendering of extensions like todo etc
use std::fmt::Write;
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(output))]
pub fn apply_extension(extension: norg::DetachedModifierExtension, output: &mut String) {
    match extension {
        norg::DetachedModifierExtension::Todo(todo_status) => {
            trace!("applying todo extension");
            match todo_status {
                norg::TodoStatus::Undone => {
                    debug!("Rendering TodoStatus: Undone");
                    write!(output, "<span class=\"status undone\"></span>");
                }
                norg::TodoStatus::Done => {
                    debug!("Rendering TodoStatus: Done");
                    write!(output, "<span class=\"status done\"></span>");
                }
                norg::TodoStatus::NeedsClarification => {
                    debug!("Rendering TodoStatus: NeedsClarification");
                    write!(output, "<span class=\"status question\"></span>");
                }
                norg::TodoStatus::Paused => {
                    debug!("Rendering TodoStatus: Paused");
                    write!(output, "<span class=\"status paused\"></span>");
                }
                norg::TodoStatus::Urgent => {
                    debug!("Rendering TodoStatus: Urgent");
                    write!(output, "<span class=\"status urgent\">󰗖</span>");
                }
                norg::TodoStatus::Recurring(date) => {
                    debug!(?date, "Rendering TodoStatus: Recurring");
                    write!(output, "<span class=\"status recurring\">⟳</span>");
                    if let Some(date) = date {
                        write!(output, "<span class=\"todo-date\">{date}</span>");
                    }
                }
                norg::TodoStatus::Pending => {
                    debug!("Rendering TodoStatus: Pending");
                    write!(output, "<span class=\"status pending\"></span>");
                }
                norg::TodoStatus::Canceled => {
                    debug!("Rendering TodoStatus: Canceled");
                    write!(output, "<span class=\"status canceled\">󰚃</span>");
                }
            }
        }
        _ => {
            warn!("This extensions is not yet supported")
        }
    };
}
