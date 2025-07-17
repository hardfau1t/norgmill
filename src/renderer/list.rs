use std::fmt::Write;
use tracing::{debug, trace, warn};

fn apply_extensions(extensions: Vec<norg::DetachedModifierExtension>, output: &mut String) -> Result<bool, std::fmt::Error> {
    let mut add_todo = false;
    for token in extensions {
        match token {
            norg::DetachedModifierExtension::Todo(todo_status) => {
                trace!("applying todo extension");
                let status = match todo_status {
                    norg::TodoStatus::Undone => "undone",
                    norg::TodoStatus::Done => "done",
                    norg::TodoStatus::NeedsClarification => "needs-clarification",
                    norg::TodoStatus::Paused => "paused",
                    norg::TodoStatus::Urgent => "urgent",
                    norg::TodoStatus::Recurring(_date) => {
                        "recurring"
                        // debug!(?date, "Rendering TodoStatus: Recurring");
                        // if let Some(date) = date {
                        //     spb.span(|sspb| sspb.class("todo-date").text(date));
                        // }
                    }
                    norg::TodoStatus::Pending => "pending",
                    norg::TodoStatus::Canceled => "canceled",
                };
                debug!(status, "Rendering TodoStatus");
                write!(output, " data-status=\"{}\"", status)?;
                add_todo = true;
            }
            norg::DetachedModifierExtension::Priority(_) => todo!(),
            norg::DetachedModifierExtension::Timestamp(_) => todo!(),
            norg::DetachedModifierExtension::DueDate(_) => todo!(),
            norg::DetachedModifierExtension::StartDate(_) => todo!(),
        }
    }
    Ok(add_todo)
}

#[derive(Debug)]
pub enum ListKind {
    Ordered,
    Unordered,
}

impl std::fmt::Display for ListKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ordered => write!(f, "ordered"),
            Self::Unordered => write!(f, "unordered"),
        }
    }
}

/// extract text from div > paragraph
/// this since it is better to keep list item in text instead of div > paragraph
pub fn render_list_element(
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    level: u16,
    kind: ListKind,
    extensions: Vec<norg::DetachedModifierExtension>,
    output: &mut String,
) -> std::fmt::Result {
    let mut text_content = String::new();
    super::render_flat_ast(&text, &mut text_content)?;

    // TODO: replace this and take from root footnote builder
    // but footnote is not allowed in list element
    let mut footnotes = Vec::new();
    let mut inner_content_rendered = String::new();
    if !inner_content.is_empty() {
        let mut tokens = inner_content.into_iter().peekable();
        super::render_ast(&mut tokens, &mut footnotes, &mut inner_content_rendered)?;
    }
    if !footnotes.is_empty() {
        warn!("Footnotes are present in list items which shouldn't be possible");
    }

    // Start list item with extensions as attributes
    write!(output, "<li class={kind}_l{level}")?;
    let add_todo = apply_extensions(extensions, output)?;
    output.push('>');

    // Add status indicator span for todo items
    if add_todo {
        output.push_str("<span class=\"status-indicator\"></span>");
    }

    // Add the text content
    output.push_str(&text_content);

    // Add inner content if present
    if !inner_content_rendered.is_empty() {
        output.push_str(&inner_content_rendered);
    }

    output.push_str("</li>");
    Ok(())
}
