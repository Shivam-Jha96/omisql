use clap::Parser;
use anyhow::Result;
use std::fs;
use logos::Logos;
use omnisql::cli::{Cli, Commands};
use omnisql::lexer::Token;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lint { path, schema } => {
            println!("Processing path: {}", path);
            
            // Read file content
            let content = fs::read_to_string(path)?;
            
            // 0. Style Engine
            let mut style_engine = omnisql::style::StyleEngine::new();
            style_engine.add_rule(Box::new(omnisql::style::casing::KeywordCasingRule));
            style_engine.add_rule(Box::new(omnisql::style::commas::TrailingCommaRule));
            
            let log_path = "omnisql_fixes.log";
            let fixed_content = style_engine.format_and_log(&content, log_path);
            
            if fixed_content != content {
                println!("Style auto-fixes applied. Check {} for details.", log_path);
            }
            
            // 1. Lexer
            let mut lex = Token::lexer(&fixed_content);
            let mut tokens = Vec::new();
            
            while let Some(res) = lex.next() {
                match res {
                    Ok(token) => tokens.push(token),
                    Err(_) => {
                        println!("Lexing Error at '{}'", lex.slice());
                        return Ok(());
                    }
                }
            }
            println!("Lexing successful: {} tokens", tokens.len());
            
            // 2. Parser
            let stmt = match omnisql::parser::parse_statement(&tokens) {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Parse Error: {}", e);
                    return Ok(());
                }
            };
            println!("Parsing successful.");
            
            // 3. Semantic Engine
            if let Some(schema_path) = schema {
                let ddl = match fs::read_to_string(schema_path) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Failed to read schema file: {}", e);
                        return Ok(());
                    }
                };
                
                let mut registry = omnisql::semantic::SchemaRegistry::new();
                registry.load_from_ddl(&ddl);
                
                let mut semantic_errors = 0;
                
                if let omnisql::parser::Statement::Select(ref ast) = stmt {
                    let mut active_tables = vec![ast.table.clone()];
                    for join in &ast.joins {
                        active_tables.push(join.table.clone());
                    }
                    
                    for col in &ast.columns {
                        if col.name == "*" {
                            continue;
                        }
                        
                        if let Some(ref t) = col.table {
                            if !active_tables.contains(t) {
                                println!("Semantic Error: Table '{}' is not part of the query.", t);
                                semantic_errors += 1;
                            } else if !registry.validate_column(t, &col.name) {
                                println!("Semantic Error: Column '{}.{}' does not exist in schema.", t, col.name);
                                semantic_errors += 1;
                            } else {
                                println!("Semantic OK: '{}.{}' validated.", t, col.name);
                            }
                        } else {
                            // Unqualified column
                            let matches = registry.find_tables_with_column(&col.name, &active_tables);
                            if matches.is_empty() {
                                println!("Semantic Error: Column '{}' not found in any queried tables ({:?}).", col.name, active_tables);
                                semantic_errors += 1;
                            } else if matches.len() > 1 {
                                println!("Semantic Error: Ambiguous column '{}'. Found in tables: {:?}", col.name, matches);
                                semantic_errors += 1;
                            } else {
                                println!("Semantic OK: '{}' validated (belongs to '{}').", col.name, matches[0]);
                            }
                        }
                    }
                }
                
                if semantic_errors == 0 {
                    println!("Semantic validation passed.");
                    
                    // 4. Rule Engine (Exec, Security, Cost)
                    let mut all_errors = 0;
                    let mut all_warnings = 0;
                    
                    if let omnisql::parser::Statement::Select(ref ast) = stmt {
                        let exec_issues = omnisql::exec::rules::check_execution_rules(ast);
                        for issue in &exec_issues {
                            match issue {
                                omnisql::exec::rules::ExecIssue::Error(msg) => {
                                    println!("Exec Error: {}", msg);
                                    all_errors += 1;
                                }
                                omnisql::exec::rules::ExecIssue::Warning(msg) => {
                                    println!("Exec Warning: {}", msg);
                                    all_warnings += 1;
                                }
                            }
                        }
                    }

                    let sec_issues = omnisql::semantic::security::check_security_rules(&stmt, &registry);
                    for issue in &sec_issues {
                        match issue {
                            omnisql::semantic::security::SecurityIssue::Error(msg) => {
                                println!("Security Error: {}", msg);
                                all_errors += 1;
                            }
                            omnisql::semantic::security::SecurityIssue::Warning(msg) => {
                                println!("Security Warning: {}", msg);
                                all_warnings += 1;
                            }
                        }
                    }

                    let cost_issues = omnisql::semantic::cost::check_cost_rules(&stmt, &registry);
                    for issue in &cost_issues {
                        match issue {
                            omnisql::semantic::cost::CostIssue::Error(msg) => {
                                println!("Cost Error: {}", msg);
                                all_errors += 1;
                            }
                            omnisql::semantic::cost::CostIssue::Warning(msg) => {
                                println!("Cost Warning: {}", msg);
                                all_warnings += 1;
                            }
                        }
                    }
                    
                    if all_errors == 0 && all_warnings == 0 {
                        println!("Dry-run execution passed without issues.");
                    } else {
                        println!("Dry-run completed with {} errors and {} warnings.", all_errors, all_warnings);
                    }
                } else {
                    println!("Semantic validation failed with {} errors.", semantic_errors);
                }
            } else {
                println!("Warning: No --schema provided. Skipping semantic validation and dry-run execution.");
                use std::io::Write;
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = writeln!(file, "WARNING: Semantic engine skipped due to missing schema.");
                }
            }
        }
    }

    Ok(())
}
