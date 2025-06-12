use tracing::{trace, warn};

pub fn render_unordered_list<'b>(
    level: u16,
    _extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    builder: &'b mut html::text_content::builders::UnorderedListBuilder,
) -> &'b mut html::text_content::builders::UnorderedListBuilder
where
{
    trace!("rendering unordered_list");
    // TODO: replace this and take from root footnote builder
    // but footnote is not allowed in list element
    let mut footnotes = Vec::new();
    let list_item_text = super::render_flat_ast(&text);
    let mut inner_content_rendered = None;
    if !inner_content.is_empty() {
        let mut tokens = inner_content.into_iter().peekable();
        inner_content_rendered = Some(super::render_ast(&mut tokens, &mut footnotes));
    }
    if !footnotes.is_empty() {
        warn!("Footnotes are present in list items which shouldn't be possible");
    }
    builder
        .class(format!("unordered_l{level}"))
        .list_item(|item_builder| {
            item_builder.push(list_item_text);
            inner_content_rendered.map(|inc| item_builder.push(inc));
            item_builder
        });
    builder
}

pub fn render_ordered_list<'b>(
    level: u16,
    _extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    builder: &'b mut html::text_content::builders::OrderedListBuilder,
) -> &'b mut html::text_content::builders::OrderedListBuilder {
    trace!("rendering ordered");
    // TODO: replace this and take from root footnote builder
    // but footnote is not allowed in list element
    let mut footnotes = Vec::new();
    let list_item_text = super::render_flat_ast(&text);
    let mut inner_content_rendered = None;
    if !inner_content.is_empty() {
        let mut tokens = inner_content.into_iter().peekable();
        inner_content_rendered = Some(super::render_ast(&mut tokens, &mut footnotes));
    }
    if !footnotes.is_empty() {
        warn!("Footnotes are present in list items which shouldn't be possible");
    }
    builder
        .class(format!("ordered_l{level}"))
        .list_item(|item_builder| {
            item_builder.push(list_item_text);
            inner_content_rendered.map(|inc| item_builder.push(inc));
            item_builder
        });
    builder
}
