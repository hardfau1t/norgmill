use tracing::trace;

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
    builder
        .class(format!("unordered_l{level}"))
        .list_item(|item_builder| {
            item_builder.push(super::render_flat_ast(&text));
            if !inner_content.is_empty() {
                let mut tokens = inner_content.into_iter().peekable();
                item_builder.push(super::render_ast(&mut tokens, &mut footnotes));
            }
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
    builder
        .class(format!("ordered_l{level}"))
        .list_item(|item_builder| {
            item_builder.push(super::render_flat_ast(&text));
            if !inner_content.is_empty() {
                let mut tokens = inner_content.into_iter().peekable();
                item_builder.push(super::render_ast(&mut tokens, &mut footnotes));
            }
            item_builder
        });
    builder
}
