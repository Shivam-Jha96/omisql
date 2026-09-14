use clap::Parser;
use anyhow::Result;
use std::fs;
use logos::Logos;
use omnisql::cli::{Cli, Commands};
use omnisql::lexer::Token;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lint { path } => {
            println!("Processing path: {}", path);
            
            // Read file content
            let content = fs::read_to_string(path)?;
            
            // 1. Lexer
            let mut lex = Token::lexer(&content);
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
            let ast = match omnisql::parser::parse_select(&tokens) {
                Ok(ast) => ast,
                Err(e) => {
                    println!("Parse Error: {}", e);
                    return Ok(());
                }
            };
            println!("Parsing successful: {:#?}", ast);
            
            // 3. Semantic Engine
            let mut registry = omnisql::semantic::SchemaRegistry::new();
            registry.load_mock_schema();
            
            let mut semantic_errors = 0;
            if ast.columns.len() == 1 && ast.columns[0] == "*" {
                 println!("Semantic OK: Selecting all columns from '{}'", ast.table);
            } else {
                for col in &ast.columns {
                    if registry.validate_column(&ast.table, col) {
                        println!("Semantic OK: Column '{}' found in table '{}'", col, ast.table);
                    } else {
                        println!("Semantic Error: Column '{}' not found in table '{}'", col, ast.table);
                        semantic_errors += 1;
                    }
                }
            }
            
            if semantic_errors == 0 {
                println!("Semantic validation passed.");
            } else {
                println!("Semantic validation failed with {} errors.", semantic_errors);
            }
        }
    }

    Ok(())
}
