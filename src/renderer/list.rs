use tracing::{debug, trace, warn};

fn apply_extensions(
    extensions: Vec<norg::DetachedModifierExtension>,
    list_item: &mut html::text_content::ListItem,
) {
    for token in extensions {
        match token {
            norg::DetachedModifierExtension::Todo(todo_status) => {
                list_item.set_class(Some("task-item"));
                let status_span = html::inline_text::Span::builder()
                    .class("status-indicator")
                    .build();
                list_item.children_mut().insert(
                    0,
                    html::text_content::children::ListItemChild::Span(status_span),
                );
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
                list_item.data_map_mut().insert(
                    std::borrow::Cow::from("status"),
                    std::borrow::Cow::from(status),
                );
            }
            norg::DetachedModifierExtension::Priority(_) => todo!(),
            norg::DetachedModifierExtension::Timestamp(_) => todo!(),
            norg::DetachedModifierExtension::DueDate(_) => todo!(),
            norg::DetachedModifierExtension::StartDate(_) => todo!(),
        }
    }
}

/// extract text from div > paragraph
/// this since it is better to keep list item in text instead of div > paragraph
fn render_list_element(
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    extensions: Vec<norg::DetachedModifierExtension>,
) -> Option<html::text_content::ListItem> {
    let text_div = super::render_flat_ast(&text);
    // TODO: replace this and take from root footnote builder
    // but footnote is not allowed in list element
    let mut footnotes = Vec::new();
    let mut inner_content_rendered = None;
    if !inner_content.is_empty() {
        let mut tokens = inner_content.into_iter().peekable();
        inner_content_rendered = Some(super::render_ast(&mut tokens, &mut footnotes));
    }
    if !footnotes.is_empty() {
        warn!("Footnotes are present in list items which shouldn't be possible");
    }
    text_div
        .children()
        .iter()
        .filter_map(|child| {
            if let html::text_content::children::DivisionChild::Paragraph(p) = child {
                Some(
                    p.children()
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<String>(),
                )
            } else {
                warn!(
                    ?child,
                    "Unexpected token found in list element, Raise an issue if it is required"
                );
                None
            }
        })
        .reduce(|mut prev, curr| {
            prev.push_str(curr.as_str());
            prev
        })
        .map(|text| {
            let mut item_builder = html::text_content::ListItem::builder();
            item_builder.text(text);
            if let Some(inner_item) = inner_content_rendered {
                item_builder.push(inner_item).build();
            }
            let mut item = item_builder.build();
            apply_extensions(extensions, &mut item);
            item
        })
}

pub fn render_unordered_list<'b>(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    builder: &'b mut html::text_content::builders::UnorderedListBuilder,
) -> &'b mut html::text_content::builders::UnorderedListBuilder
where
{
    trace!("rendering unordered_list");
    if let Some(list_item) = render_list_element(text, inner_content, extensions) {
        builder.class(format!("unordered_l{level}")).push(list_item);
    }
    builder
}

pub fn render_ordered_list<'b>(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    builder: &'b mut html::text_content::builders::OrderedListBuilder,
) -> &'b mut html::text_content::builders::OrderedListBuilder {
    trace!("rendering ordered list");
    if let Some(list_item) = render_list_element(text, inner_content, extensions) {
        builder.class(format!("unordered_l{level}")).push(list_item);
    }
    builder
}
