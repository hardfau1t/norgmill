//! module which does rendering of attached modifiers, like lists etc
use handlebars::Handlebars;
use miette::{Context, IntoDiagnostic};
use serde::Serialize;
use tracing::{debug, error, instrument, trace, warn};


#[derive(Debug, Serialize)]
pub struct HtmlList {
    level: u16,
    kind: norg::NestableDetachedModifier,
    items: Vec<String>,
}

impl HtmlList {
    /// finish the list and render content to page
    pub fn render(self, write_to: &mut String, hbr: &Handlebars) -> miette::Result<()> {
        debug!("rendering html list: {:?} ", self.items);
        match self.kind {
            norg::NestableDetachedModifier::UnorderedList => {
                trace!("rendering unorder list");
                let rendered_list = hbr
                    .render("unordered_list", &self)
                    .into_diagnostic()
                    .wrap_err("Couldn't render unordered list")?;
                write_to.push_str(&rendered_list);
                Ok(())
            }
            norg::NestableDetachedModifier::OrderedList => {
                trace!("rendering order list");
                let rendered_list = hbr
                    .render("ordered_list", &self)
                    .into_diagnostic()
                    .wrap_err("Couldn't render ordered list")?;
                write_to.push_str(&rendered_list);
                Ok(())
            }
            norg::NestableDetachedModifier::Quote => {
                error!("Quotes are not stored in Html list, unexpected item");
                miette::bail!("Unexpected quote found in html list")
            }
        }
    }

    /// create a new html list
    pub fn new(level: u16, kind: norg::NestableDetachedModifier) -> Self {
        Self {
            level,
            kind,
            items: Vec::new(),
        }
    }

    /// checks if this item/ast tree can extend current list
    /// if so user needs to call render to complete the current list
    pub fn expected(&self, ast: &norg::NorgAST) -> bool {
        match ast {
            norg::NorgAST::NestableDetachedModifier {
                modifier_type,
                level,
                ..
            } => *level == self.level && (modifier_type == &self.kind),
            _ => false,
        }
    }

    /// append an item html list item
    #[instrument(skip(hbr, nested_content, extensions))]
    pub fn push(
        &mut self,
        text: Box<norg::NorgASTFlat>,
        nested_content: Vec<norg::NorgAST>,
        extensions: Vec<norg::DetachedModifierExtension>,
        hbr: &Handlebars,
    ) -> miette::Result<()> {
        trace!("Adding row to html list");
        let list_item = super::render_flat_ast(&text, hbr)?;
        let mut inner_context = super::NorgContext::default();
        let list_item = nested_content.into_iter().try_fold(
            list_item,
            |mut acc, ast| -> miette::Result<String> {
                let part = super::render_ast(ast, &mut inner_context, hbr)?;
                acc.push_str(&part);
                miette::Result::Ok(acc)
            },
        )?;
        // this needs to be applied first since modifiers which are applied at the end should not be applied to inner lists
        let mut formatted_list_item = extensions
            .into_iter()
            .try_fold(list_item.clone(), |acc, extension| {
                super::extensions::apply_extension(&extension, &acc, hbr)
            })
            .unwrap_or_else(|e| {
                error!(?e, "Couldn't render the extension");
                list_item
            });
        debug!(formatted_list_item, "list item after applying extensions");
        // apply all extensions
        inner_context.flush(&mut formatted_list_item, hbr)?;
        self.items.push(formatted_list_item);
        Ok(())
    }
}
