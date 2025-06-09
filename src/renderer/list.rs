use tracing::trace;

pub fn render_unordered_list<'b>(
    _level: u16,
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
    builder.list_item(|item_builder| {
        item_builder.division(|dbuilder| super::render_flat_ast(&text, dbuilder));
        if !inner_content.is_empty() {
            item_builder.division(|dbuilder| {
                let mut tokens = inner_content.into_iter().peekable();
                super::render_ast(
                    &mut tokens,
                    &mut footnotes,
                    dbuilder,
                )
            });
        }
        item_builder
    });
    builder
}

pub fn render_ordered_list<'b>(
    _level: u16,
    _extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_content: Vec<norg::NorgAST>,
    builder: &'b mut html::text_content::builders::OrderedListBuilder,
) -> &'b mut html::text_content::builders::OrderedListBuilder {
    trace!("rendering ordered");
    // TODO: replace this and take from root footnote builder
    // but footnote is not allowed in list element
    let mut footnotes = Vec::new();
    builder.list_item(|item_builder| {
        item_builder.division(|dbuilder| super::render_flat_ast(&text, dbuilder));
        if !inner_content.is_empty() {
            item_builder.division(|dbuilder| {
                let mut tokens = inner_content.into_iter().peekable();
                super::render_ast(
                    &mut tokens,
                    &mut footnotes,
                    dbuilder,
                )
            });
        }
        item_builder
    });
    builder
}
