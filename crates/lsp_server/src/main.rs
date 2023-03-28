use std::error::Error;

use leptos_language_server::{LanguageServer, NotificationHandler, RequestHandler};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument, Notification};
use lsp_types::request::{Formatting, Request};
use lsp_types::{InitializeParams, ServerCapabilities};
use lsp_types::{OneOf, TextDocumentSyncCapability, TextDocumentSyncKind};

use lsp_server::{Connection, ExtractError, Message, RequestId, Response};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting Leptos Language Server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        document_formatting_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    let language_server = LanguageServer::new();
    eprintln!("starting main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                match req.method.as_str() {
                    Formatting::METHOD => {
                        handle_request::<Formatting>(&language_server, &connection, req)?;
                    }
                    _ => {}
                };
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(notif) => {
                eprintln!("got notification: {notif:?}");
                match notif.method.as_str() {
                    DidChangeTextDocument::METHOD => {
                        handle_notification::<DidChangeTextDocument>(
                            &language_server,
                            &connection,
                            notif,
                        )?;
                    }
                    DidOpenTextDocument::METHOD => {
                        handle_notification::<DidOpenTextDocument>(
                            &language_server,
                            &connection,
                            notif,
                        )?;
                    }
                    _ => {}
                };
            }
        }
    }
    Ok(())
}

// TODO this is a very naive implementation that handles everything on the main thread -> should create a dispatcher that can dispatch requests on a thread pool
// or maybe look into tower-lsp?
fn handle_request<R>(
    language_server: &LanguageServer,
    connection: &Connection,
    req: lsp_server::Request,
) -> Result<(), Box<dyn Error + Sync + Send>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
    LanguageServer: RequestHandler<R>,
{
    let (id, params) = cast::<R>(req)?;
    let result = RequestHandler::<R>::handle(language_server, params);

    let resp = match result {
        Ok(result) => {
            let result = serde_json::to_value(&result).unwrap();
            Response {
                id,
                result: Some(result),
                error: None,
            }
        }
        Err(error) => Response {
            id,
            result: None,
            error: Some(error),
        },
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_notification<N>(
    language_server: &LanguageServer,
    connection: &Connection,
    notif: lsp_server::Notification,
) -> Result<(), Box<dyn Error + Sync + Send>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
    LanguageServer: NotificationHandler<N>,
{
    // TODO better error handling?
    let params = cast_notif::<N>(notif).unwrap();
    NotificationHandler::<N>::handle(language_server, params).unwrap();
    Ok(())
}

fn cast_notif<N>(
    notif: lsp_server::Notification,
) -> Result<N::Params, ExtractError<lsp_server::Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    notif.extract(N::METHOD)
}

fn cast<R>(
    req: lsp_server::Request,
) -> Result<(RequestId, R::Params), ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
