pub mod capabilities;
mod core;

use crate::core::rsx_document::RsxDocument;

use crate::core::text_document::TextDocument;

use dashmap::DashMap;
use lsp_server::ResponseError;
use lsp_types::{notification::Notification, request::Request, Url};

pub type Result<T> = std::result::Result<T, ResponseError>;

pub trait RequestHandler<R: Request> {
    fn handle(&self, params: R::Params) -> Result<R::Result>;
}

pub trait NotificationHandler<N: Notification> {
    fn handle(&self, params: N::Params) -> Result<()>;
}

#[allow(unused)]
#[derive(Default)]
pub struct LanguageServer {
    documents: DashMap<String, TextDocument>,
    rsx_documents: DashMap<String, RsxDocument>,
}

impl LanguageServer {
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum DocumentError {
    DocumentNotFound { path: String },
}

impl LanguageServer {
    pub fn get_text_document(&self, url: &Url) -> std::result::Result<TextDocument, DocumentError> {
        self.documents
            .try_get(url.path())
            .try_unwrap()
            .ok_or_else(|| DocumentError::DocumentNotFound {
                path: url.path().to_string(),
            })
            .map(|document| document.clone())
    }
}
