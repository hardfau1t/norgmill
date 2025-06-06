//! module which does rendering of extensions like todo etc
use tracing::{instrument, trace, warn};

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
                    spb.class("status undone").text("");
                }
                norg::TodoStatus::Done => {
                    spb.class("status done").text("");
                }
                norg::TodoStatus::NeedsClarification => {
                    spb.class("status question").text("");
                }
                norg::TodoStatus::Paused => {
                    spb.class("status paused").text("");
                }
                norg::TodoStatus::Urgent => {
                    spb.class("status urgent").text("󰗖");
                }
                norg::TodoStatus::Recurring(date) => {
                    spb.class("status recurring").text("⟳");
                    if let Some(date) = date {
                        spb.span(|sspb| sspb.class("todo-date").text(date));
                    }
                }
                norg::TodoStatus::Pending => {
                    spb.class("status pending").text("");
                }
                norg::TodoStatus::Canceled => {
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
