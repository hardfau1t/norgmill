//! module which does rendering of extensions like todo etc
use tracing::{debug, instrument, trace, warn};

#[instrument(skip(spb))]
pub fn apply_extension(
    extension: norg::DetachedModifierExtension,
    spb: &mut html::inline_text::builders::SpanBuilder,
) -> &mut html::inline_text::builders::SpanBuilder {
    match extension {
        norg::DetachedModifierExtension::Todo(todo_status) => {
            trace!("applying todo extension");
            match todo_status {
                norg::TodoStatus::Undone => {
                    debug!("Rendering TodoStatus: Undone");
                    spb.class("status undone").text("");
                }
                norg::TodoStatus::Done => {
                    debug!("Rendering TodoStatus: Done");
                    spb.class("status done").text("");
                }
                norg::TodoStatus::NeedsClarification => {
                    debug!("Rendering TodoStatus: NeedsClarification");
                    spb.class("status question").text("");
                }
                norg::TodoStatus::Paused => {
                    debug!("Rendering TodoStatus: Paused");
                    spb.class("status paused").text("");
                }
                norg::TodoStatus::Urgent => {
                    debug!("Rendering TodoStatus: Urgent");
                    spb.class("status urgent").text("󰗖");
                }
                norg::TodoStatus::Recurring(date) => {
                    debug!(?date, "Rendering TodoStatus: Recurring");
                    spb.class("status recurring").text("⟳");
                    if let Some(date) = date {
                        spb.span(|sspb| sspb.class("todo-date").text(date));
                    }
                }
                norg::TodoStatus::Pending => {
                    debug!("Rendering TodoStatus: Pending");
                    spb.class("status pending").text("");
                }
                norg::TodoStatus::Canceled => {
                    debug!("Rendering TodoStatus: Canceled");
                    spb.class("status canceled").text("󰚃");
                }
            }
        }
        _ => {
            warn!("This extensions is not yet supported")
        }
    };
    spb
}
