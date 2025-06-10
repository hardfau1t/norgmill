//! module which does rendering of quotes
use tracing::{error, instrument, trace, warn};

#[instrument(skip(inner_quotes))]
pub fn render_quote(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_quotes: Vec<norg::NorgAST>,
) -> html::text_content::BlockQuote {
    trace!("rendering quote");
    if !extensions.is_empty() {
        warn!("Quote has extensions which is not supposed be, if things have changed, then raise issue to fix this");
    }
    let mut qbuilder = html::text_content::BlockQuote::builder();
    qbuilder.push(super::render_flat_ast(&text));

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
            qbuilder.push(render_quote(
                inner_level,
                inner_extensions,
                inner_text,
                inner_content,
            ));
        } else {
            error!(tokens=?inner_quote, "Unexpected tokens found in quotes, only quotes are allowed")
        }
    }
    qbuilder.build()
}
