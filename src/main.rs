use clap::Parser;
use anyhow::Result;
use std::fs;
use std::io::{BufReader, Write};
use omnisql::cli::{Cli, Commands};
use omnisql::stream::StreamingSqlReader;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lint { path, schema, dbt_manifest, plugin, fix, verbose, dialect } => {
            if *verbose {
                println!("Starting linting process for path: {}", path);
            } else {
                println!("Processing path: {}", path);
            }
            
            // File size heuristic
            let metadata = fs::metadata(path)?;
            let is_large_file = metadata.len() > 100 * 1024 * 1024; // 100MB
            let mut fix_enabled = *fix;
            
            if is_large_file && fix_enabled {
                println!("Warning: File is larger than 100MB. Style Auto-Fixer is disabled to prevent excessive memory usage.");
                fix_enabled = false;
            }

            // Prepare schema registry
            let mut registry = omnisql::semantic::SchemaRegistry::new();
            let mut schema_loaded = false;

            if let Some(schema_path) = schema {
                let ddl = fs::read_to_string(schema_path)?;
                registry.load_from_ddl(&ddl);
                schema_loaded = true;
            }

            if let Some(manifest_path) = dbt_manifest {
                match registry.load_from_dbt_manifest(manifest_path) {
                    Ok(_) => {
                        println!("Successfully loaded dbt manifest.");
                        schema_loaded = true;
                    }
                    Err(e) => {
                        println!("Failed to parse dbt manifest: {}", e);
                        return Ok(());
                    }
                }
            }

            // Setup streaming reader
            let file = fs::File::open(path)?;
            let mut reader = StreamingSqlReader::new(BufReader::new(file));

            let log_path = "omnisql_fixes.log";
            
            // Clean up previous log if exists
            let _ = fs::remove_file(log_path);

            let mut style_engine = omnisql::style::StyleEngine::new();
            style_engine.add_rule(Box::new(omnisql::style::casing::KeywordCasingRule));
            style_engine.add_rule(Box::new(omnisql::style::commas::TrailingCommaRule));

            let print_debug_log = |fixes: &Vec<String>| {
                println!("--- Runtime Debug Log ---");
                for fix in fixes {
                    println!("Linted / FIX APPLIED: {}", fix);
                }
                println!("-------------------------");
            };
            
            let temp_path = format!("{}.tmp", path);
            let mut temp_file = if fix_enabled {
                Some(std::fs::File::create(&temp_path)?)
            } else {
                None
            };
            
            let mut any_fixes_applied = false;

            while let Some(stmt_str) = reader.next_statement()? {
                let mut current_stmt_str = stmt_str.clone();
                let mut fixes = Vec::new();
                
                // 0. Style Engine (only run if we aren't bypassing due to size constraints)
                if !is_large_file {
                    let (fixed_content, mut current_fixes) = style_engine.format_and_log(&current_stmt_str, log_path);
                    current_stmt_str = omnisql::style::fix::TokenFixer::fix_casing(&fixed_content);
                    fixes.append(&mut current_fixes);
                    
                    if current_stmt_str != stmt_str {
                        any_fixes_applied = true;
                    }
                }
                
                if let Some(tf) = &mut temp_file {
                    tf.write_all(current_stmt_str.as_bytes())?;
                }

                // Skip parsing for empty statements
                if current_stmt_str.trim().is_empty() {
                    continue;
                }

                // 1. & 2. Lexer and Parser
                let stmt = match omnisql::parser::parse_sql(&current_stmt_str, dialect) {
                    Ok(stmt) => stmt,
                    Err(e) => {
                        println!("Parse Error: {}", e.message);
                        println!("Failed at parsing. Displaying runtime debug log:");
                        print_debug_log(&fixes);
                        // Continue to next statement
                        continue;
                    }
                };

                if *verbose && !fixes.is_empty() {
                    print_debug_log(&fixes);
                }

                // 2.5. Plugin Engine
                if let Some(plugin_path) = plugin {
                    match omnisql::plugin::PluginEngine::new() {
                        Ok(engine) => {
                            if *verbose {
                                println!("Running WASM plugin: {}", plugin_path);
                            }
                            match engine.run_plugin(std::path::Path::new(plugin_path), &stmt) {
                                Ok(issues) => {
                                    for issue in issues {
                                        println!("Plugin {}: {} - {}", issue.severity, issue.rule_name, issue.message);
                                    }
                                }
                                Err(e) => {
                                    println!("Plugin Execution Error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to initialize plugin engine: {}", e);
                        }
                    }
                }

                // 3. Semantic Engine
                if schema_loaded {
                    let mut semantic_errors = 0;
                    
                    if let omnisql::parser::Statement::Select(ref ast) = stmt {
                        let mut active_tables = vec![ast.table.clone()];
                        for join in &ast.joins {
                            active_tables.push(join.table.clone());
                        }
                        
                        for col in &ast.columns {
                            if col.name == "*" || col.name == "function_or_expr" {
                                continue;
                            }
                            
                            if let Some(ref t) = col.table {
                                if !active_tables.contains(t) {
                                    println!("Semantic Error: Table '{}' is not part of the query.", t);
                                    semantic_errors += 1;
                                } else if !registry.validate_column(t, &col.name) {
                                    println!("Semantic Error: Column '{}.{}' does not exist in schema.", t, col.name);
                                    semantic_errors += 1;
                                } else if *verbose {
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
                                } else if *verbose {
                                    println!("Semantic OK: '{}' validated (belongs to '{}').", col.name, matches[0]);
                                }
                            }
                        }
                    }
                    
                    if semantic_errors == 0 {
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
                        
                        if (all_errors > 0 || all_warnings > 0) && *verbose {
                            println!("Dry-run completed with {} errors and {} warnings.", all_errors, all_warnings);
                        }
                    } else if *verbose {
                        println!("Semantic validation failed with {} errors.", semantic_errors);
                    }
                }
                
                // End of statement processing, `stmt` is dropped here.
            }
            
            // Explicitly drop temp_file so it can be renamed on Windows
            drop(temp_file);

            if !schema_loaded {
                println!("Warning: No schema provided. Skipping semantic validation and dry-run execution.");
                if let Ok(mut log_file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = writeln!(log_file, "WARNING: Semantic engine skipped due to missing schema.");
                }
            }

            if fix_enabled && any_fixes_applied {
                if *verbose {
                    println!("Style auto-fixes applied. Check {} for details.", log_path);
                }
                println!("Applying fixes to source file...");
                fs::rename(&temp_path, path)?;
            } else if fix_enabled {
                let _ = fs::remove_file(&temp_path);
            }
            
            if *verbose {
                println!("Parsing and Linting successful.");
            } else {
                println!("Parsing successful.");
            }
        }
        Commands::Lsp { schema, dialect: _ } => {
            let mut schema = schema.clone();
            eprintln!("Starting LSP server...");
            if schema.is_none() {
                if let Ok(settings) = std::fs::read_to_string(".vscode/settings.json") {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&settings) {
                        if let Some(path) = json.get("omnisql.schemaPath").and_then(|v| v.as_str()) {
                            if !path.trim().is_empty() {
                                schema = Some(path.to_string());
                            }
                        }
                    }
                }
            }
            tokio::runtime::Runtime::new().unwrap().block_on(omnisql::lsp::run_server(schema));
        }
    }

    Ok(())
}
