//! module which does rendering of quotes
use tracing::{error, instrument, trace, warn};

#[instrument(skip(extensions, qbuilder))]
pub fn render_quote<'n, 'b>(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_quotes: Vec<norg::NorgAST>,
    qbuilder: &'b mut html::text_content::builders::BlockQuoteBuilder,
) -> &'b mut html::text_content::builders::BlockQuoteBuilder {
    trace!("rendering quote");
    if !extensions.is_empty() {
        warn!("Quote has extensions which is not supposed be, if things have changed, then raise issue to fix this");
    }
    qbuilder.division(|dbuilder| super::render_flat_ast(&text, dbuilder));

    for inner_quote in inner_quotes {
        // only quotes are allowed in quotes,
        // TODO: This has to be fixed in parser
        if let norg::NorgAST::NestableDetachedModifier {
            modifier_type: norg::NestableDetachedModifier::Quote,
            level: inner_level,
            extensions: inner_extensions,
            text: inner_text,
            content: inner_content,
        } = inner_quote
        {
            qbuilder.block_quote(|qb| {
                render_quote(inner_level, inner_extensions, inner_text, inner_content, qb)
            });
        } else {
            error!(tokens=?inner_quote, "Unexpected tokens found in quotes, only quotes are allowed")
        }
    }
    qbuilder
}
