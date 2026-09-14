# OmniSQL Linter

OmniSQL is a blazing-fast, next-generation SQL linter built in Rust. It goes beyond simple syntax checking by understanding database schemas, semantic meaning, cost estimation, and security vulnerabilities.

## Core Features
- **Schema-Aware by Default**: Integrates with live databases or dbt's `manifest.json` to ensure columns and tables actually exist.
- **Blazing Fast**: Written in Rust for parallelized, high-performance linting.
- **Extensible Plugin Ecosystem**: Write custom business rules in TypeScript via embedded WebAssembly (WASM).
- **Multi-dimensional Linting**: Checks for Code Style, Semantic Accuracy, Cost Optimization (e.g., missing partition filters in Snowflake/BigQuery), and Security (e.g., unmasked PII).

## Installation
*(Coming Soon)*

## Development Setup

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (cargo, rustc)
- Visual Studio C++ Build Tools (Windows only)

### Build and Run
```bash
# Compile the project
cargo build

# Run the linter against a sample SQL file
cargo run -- lint sample.sql
```

## Architecture
The repository is modularized into several components:
- `src/lexer`: Tokenizes SQL strings into manageable `logos` tokens.
- `src/parser`: Converts tokens into an Abstract Syntax Tree (AST).
- `src/semantic`: Schema resolution and binding logic.
- `src/cli`: Command-line interface handling using `clap`.
