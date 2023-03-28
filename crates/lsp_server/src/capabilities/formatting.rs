use lsp_types::{
    request::{Formatting, RangeFormatting, Request},
    DocumentFormattingParams, DocumentRangeFormattingParams, Position, TextEdit,
};
use syn::spanned::Spanned;

use crate::{core::rsx_document::RsxDocument, LanguageServer, RequestHandler, Result};

type FormattingResult = Result<<Formatting as Request>::Result>;

impl RequestHandler<Formatting> for LanguageServer {
    fn handle(&self, params: DocumentFormattingParams) -> FormattingResult {
        let document = self
            .documents
            .get(&params.text_document.uri.path().to_string())
            .unwrap();

        // TODO do not always re-parse whole document, track 'dirty' macro callsites and only reformat them instead
        let rsx_doc = RsxDocument::parse(&document.get_text()).unwrap();

        let text_edits: Vec<_> = rsx_doc
            .macro_callsites
            .iter()
            .map(|view_macro| {
                // let ViewMacro { start, end, .. } = view_macro;
                let span = view_macro.span();
                let start = span.start();
                let end = span.end();

                let start = Position {
                    line: start.line as u32 - 1,
                    character: start.column as u32,
                };
                let end = Position {
                    line: end.line as u32 - 1,
                    character: end.column as u32,
                };
                let formatted = leptosfmt_formatter::format_macro(view_macro, Default::default());
                TextEdit {
                    range: lsp_types::Range { start, end },
                    new_text: formatted,
                }
            })
            .collect();

        Ok(Some(text_edits))
    }
}

impl RequestHandler<RangeFormatting> for LanguageServer {
    fn handle(&self, _: DocumentRangeFormattingParams) -> FormattingResult {
        Ok(None)
    }
}
