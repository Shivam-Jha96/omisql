# OmniSQL for VS Code

OmniSQL is a blazing-fast, next-generation SQL linter built in Rust. It goes beyond simple syntax checking by understanding database schemas, semantic meaning, cost estimation, and security vulnerabilities.

This extension provides seamless integration with the OmniSQL CLI via the Language Server Protocol (LSP), offering real-time diagnostics and auto-fixing directly in your editor.

## Features

- **Real-Time Linting**: Instantly see style, semantic, cost, and security violations as you type.
- **Schema-Aware Validation**: Detects missing columns, ambiguous joins, and type mismatches if a schema is provided.
- **Cost & Security Checks**: Catch unmasked PII and missing partition filters on Snowflake/BigQuery before executing queries.
- **Custom WASM Plugins**: Run your company's custom business logic rules written in TypeScript/Rust/Go.

## Requirements

The `omnisql-vscode` extension requires the `omnisql` CLI to be installed on your system.

### Install via NPM
```bash
npm install -g omnisql
```

### Install via pip
```bash
pip install omnisql
```

## Configuration

You can configure the extension via VS Code settings:
- `omnisql.executablePath`: Path to the `omnisql` binary if it is not in your system `PATH`.
- `omnisql.trace.server`: Set to `messages` or `verbose` to debug the LSP communication.

## Schema Configuration

To enable advanced semantic rules, you should provide a schema to OmniSQL. You can do this by setting up your project workspace with either:
- A `dbt` project (`manifest.json` will be automatically detected).
- Raw DDL files containing `CREATE TABLE` statements.

*(Note: Currently, schema paths should be configured in the global `omnisql` config or run via CLI. Workspace integration for schemas will be expanded in future releases).*

## License

MIT License.
