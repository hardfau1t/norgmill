//! module which does rendering of quotes
use std::fmt::Write;
use tracing::{error, instrument, trace, warn};

#[instrument(skip(inner_quotes, output))]
pub fn render_quote(
    level: u16,
    extensions: Vec<norg::DetachedModifierExtension>,
    text: Box<norg::NorgASTFlat>,
    inner_quotes: Vec<norg::NorgAST>,
    output: &mut String,
) -> std::fmt::Result {
    trace!("rendering quote");
    if !extensions.is_empty() {
        warn!("Quote has extensions which is not supposed be, if things have changed, then raise issue to fix this");
    }

    write!(output, "<blockquote>").unwrap();
    super::render_flat_ast(&text, output)?;

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
            render_quote(
                inner_level,
                inner_extensions,
                inner_text,
                inner_content,
                output,
            )?;
        } else {
            error!(tokens=?inner_quote, "Unexpected tokens found in quotes, only quotes are allowed")
        }
    }

    write!(output, "</blockquote>")
}
