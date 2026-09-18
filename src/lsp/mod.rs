use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::sync::Arc;
use tokio::sync::Mutex;
use logos::Logos;

use crate::lexer::Token;
use crate::parser::parse_statement;

pub struct Backend {
    client: Client,
    #[allow(dead_code)]
    schema_path: Arc<Mutex<Option<String>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "OmniSQL Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.pop() {
            self.on_change(params.text_document.uri, change.text).await;
        }
    }
}

impl Backend {
    async fn on_change(&self, uri: Url, text: String) {
        let mut diagnostics = Vec::new();
        
        let mut lex = Token::lexer(&text);
        let mut tokens = Vec::new();
        let mut lex_error = false;
        
        while let Some(res) = lex.next() {
            match res {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    lex_error = true;
                    // Generic diagnostic for lex error
                    diagnostics.push(Diagnostic {
                        range: Range::default(),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!("Lexing Error at '{}'", lex.slice()),
                        ..Default::default()
                    });
                    break;
                }
            }
        }
        
        if !lex_error {
            if let Err(e) = parse_statement(&tokens) {
                diagnostics.push(Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Parse Error: {}", e),
                    ..Default::default()
                });
            }
        }
        
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { 
        client, 
        schema_path: Arc::new(Mutex::new(None)) 
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}
