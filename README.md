# OmniSQL Linter

OmniSQL is a blazing-fast, next-generation SQL linter built in Rust. It goes beyond simple syntax checking by understanding database schemas, semantic meaning, cost estimation, and security vulnerabilities.

## Core Features
- **Schema-Aware by Default**: Integrates with live databases or dbt's `manifest.json` to ensure columns and tables actually exist.
- **Blazing Fast**: Written in Rust for parallelized, high-performance linting.
- **Extensible Plugin Ecosystem**: Write custom business rules in TypeScript via embedded WebAssembly (WASM).
- **Multi-dimensional Linting**: Checks for Code Style, Semantic Accuracy, Cost Optimization (e.g., missing partition filters in Snowflake/BigQuery), and Security (e.g., unmasked PII).

## Installation
Pre-compiled binaries for Windows, macOS (Intel & Apple Silicon), and Linux are automatically built via GitHub Actions and attached to each release. 

## Development Setup

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (cargo, rustc)
- Visual Studio C++ Build Tools (Windows only)
- WebAssembly targets for custom plugins (e.g., `rustup target add wasm32-wasip1`)

### Build and Run
```bash
# Compile the project
cargo build --release

# Run the linter against a sample SQL file with a DDL schema and a custom WASM plugin
cargo run -- lint path/to/query.sql --schema path/to/schema.sql --plugin path/to/plugin.wasm
```

## Architecture
The repository is modularized into several components:
- `src/lexer`: Tokenizes SQL strings into manageable `logos` tokens.
- `src/parser`: Converts tokens into an Abstract Syntax Tree (AST).
- `src/semantic`: Schema resolution, semantic validation, cost analysis, and security checks.
- `src/exec`: Virtual execution engine for static expression evaluation (constant folding).
- `src/style`: Regex-based pre-processing engine for auto-fixing style violations.
- `src/plugin`: `wasmtime`-based plugin engine to execute WASI WebAssembly modules.
- `src/cli`: Command-line interface handling using `clap`.
