use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::parser::parse_sql;

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
        
        let mut style_engine = crate::style::StyleEngine::new();
        style_engine.add_rule(Box::new(crate::style::casing::KeywordCasingRule));
        style_engine.add_rule(Box::new(crate::style::commas::TrailingCommaRule));
        
        // Run style engine to get fixes
        let (_, fixes) = style_engine.format_and_log(&text, "omnisql_fixes.log");
        
        let mut debug_log = String::from("--- Runtime Debug Log ---\n");
        for fix in &fixes {
            debug_log.push_str(&format!("Linted / FIX APPLIED: {}\n", fix));
        }
        debug_log.push_str("-------------------------");
        
        match parse_sql(&text, "postgres") {
            Ok(stmt) => {
                self.client.log_message(MessageType::INFO, debug_log).await;

                let schema_path_guard = self.schema_path.lock().await;
                if let Some(path) = schema_path_guard.as_ref() {
                    let mut registry = crate::semantic::SchemaRegistry::new();
                    match std::fs::read_to_string(path) {
                        Ok(ddl) => {
                            registry.load_from_ddl(&ddl);
                            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("lsp_debug.log") {
                                use std::io::Write;
                                let _ = writeln!(file, "Schema loaded successfully. Statements parsed: {:?}", stmt);
                            }
                            
                            if let crate::parser::Statement::Select(ref ast) = stmt {
                                let mut active_tables = vec![ast.table.clone()];
                                for join in &ast.joins {
                                    active_tables.push(join.table.clone());
                                }
                                
                                let find_col_range = |col_name: &str| -> Range {
                                    let lines: Vec<&str> = text.lines().collect();
                                    for (i, line) in lines.iter().enumerate() {
                                        if let Some(col_idx) = line.find(col_name) {
                                            return Range::new(
                                                Position::new(i as u32, col_idx as u32),
                                                Position::new(i as u32, (col_idx + col_name.len()) as u32)
                                            );
                                        }
                                    }
                                    Range::default()
                                };

                                for col in &ast.columns {
                                    if col.name == "*" { continue; }
                                    
                                    if let Some(ref t) = col.table {
                                        if !active_tables.contains(t) {
                                            diagnostics.push(Diagnostic {
                                                range: find_col_range(t),
                                                severity: Some(DiagnosticSeverity::ERROR),
                                                message: format!("Semantic Error: Table '{}' is not part of the query.", t),
                                                ..Default::default()
                                            });
                                        } else if !registry.validate_column(t, &col.name) {
                                            diagnostics.push(Diagnostic {
                                                range: find_col_range(&col.name),
                                                severity: Some(DiagnosticSeverity::ERROR),
                                                message: format!("Semantic Error: Column '{}.{}' does not exist in schema.", t, col.name),
                                                ..Default::default()
                                            });
                                        }
                                    } else {
                                        let matches = registry.find_tables_with_column(&col.name, &active_tables);
                                        if matches.is_empty() {
                                            diagnostics.push(Diagnostic {
                                                range: find_col_range(&col.name),
                                                severity: Some(DiagnosticSeverity::ERROR),
                                                message: format!("Semantic Error: Column '{}' not found in any queried tables.", col.name),
                                                ..Default::default()
                                            });
                                        } else if matches.len() > 1 {
                                            diagnostics.push(Diagnostic {
                                                range: find_col_range(&col.name),
                                                severity: Some(DiagnosticSeverity::ERROR),
                                                message: format!("Semantic Error: Ambiguous column '{}'.", col.name),
                                                ..Default::default()
                                            });
                                        }
                                    }
                                }
                                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("lsp_debug.log") {
                                    use std::io::Write;
                                    let _ = writeln!(file, "Semantic diagnostics count: {}", diagnostics.len());
                                }
                            }
                        },
                        Err(e) => {
                            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("lsp_debug.log") {
                                use std::io::Write;
                                let _ = writeln!(file, "Failed to read schema file {}: {}", path, e);
                            }
                        }
                    }
                } else {
                    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("lsp_debug.log") {
                        use std::io::Write;
                        let _ = writeln!(file, "No schema path configured.");
                    }
                }
            },
            Err(e) => {
                let l = if e.line > 0 { e.line - 1 } else { 0 };
                let c = if e.col > 0 { e.col - 1 } else { 0 };
                let range = Range::new(
                    Position::new(l as u32, c as u32),
                    Position::new(l as u32, (c + 1) as u32)
                );
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Parse Error: {}", e.message),
                    ..Default::default()
                });
                self.client.log_message(MessageType::ERROR, format!("Parse Error encountered.\n{}", debug_log)).await;
            }
        }
        
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

pub async fn run_server(schema_path: Option<String>) {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("lsp_debug.log") {
        use std::io::Write;
        let _ = writeln!(file, "Starting LSP Server. Schema path received: {:?}", schema_path);
    }

    let (service, socket) = LspService::new(|client| Backend { 
        client, 
        schema_path: Arc::new(Mutex::new(schema_path)) 
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}
