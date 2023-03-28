use lsp_types::{
    notification::{DidChangeTextDocument, DidOpenTextDocument},
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, TextDocumentContentChangeEvent,
    TextDocumentItem, Url,
};

use crate::{core::text_document::TextDocument, LanguageServer, NotificationHandler};

impl NotificationHandler<DidChangeTextDocument> for LanguageServer {
    fn handle(&self, params: DidChangeTextDocumentParams) -> crate::Result<()> {
        update_text_document(self, &params.text_document.uri, params.content_changes);
        Ok(())
    }
}

impl NotificationHandler<DidOpenTextDocument> for LanguageServer {
    fn handle(&self, params: DidOpenTextDocumentParams) -> crate::Result<()> {
        self.documents.insert(
            params.text_document.uri.path().to_owned(),
            params.text_document.into(),
        );

        Ok(())
    }
}

pub fn update_text_document(
    language_server: &LanguageServer,
    url: &Url,
    changes: Vec<TextDocumentContentChangeEvent>,
) {
    // TODO: What to do if document is not found?
    language_server
        .documents
        .try_get_mut(url.path())
        .try_unwrap()
        .map(|mut document| {
            changes.iter().for_each(|change| {
                document.apply_change(change);
            });
            eprintln!("{}", document.get_text());
        })
        .unwrap();
}

impl From<TextDocumentItem> for TextDocument {
    fn from(value: TextDocumentItem) -> Self {
        Self::new(
            value.version,
            value.uri.to_string(),
            value.language_id,
            &value.text,
        )
    }
}
