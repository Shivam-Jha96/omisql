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

## Usage Guide (CLI)

While this extension provides real-time linting in the editor, you can also run OmniSQL from the command line for CI/CD or bulk operations:

### Linting Commands

- **Lint a single file:**
  ```bash
  omnisql lint path/to/query.sql
  ```

- **Lint an entire directory:**
  ```bash
  omnisql lint ./sql_queries/
  ```

- **Auto-fix violations:**
  ```bash
  omnisql lint --fix ./sql_queries/
  ```

- **Lint with a specific configuration file:**
  ```bash
  omnisql lint --config path/to/omnisql.yaml ./sql_queries/
  ```

- **Lint with a DDL schema (for semantic validation):**
  ```bash
  omnisql lint --schema path/to/schema.sql ./sql_queries/
  ```
  *To apply style fixes while validating against a schema, add the `--fix` flag:*
  ```bash
  omnisql lint --fix --schema path/to/schema.sql ./sql_queries/
  ```

- **Lint with a dbt manifest (for native dbt project validation):**
  ```bash
  omnisql lint --dbt-manifest target/manifest.json ./sql_queries/
  ```
  *To apply style fixes while validating against a manifest, add the `--fix` flag:*
  ```bash
  omnisql lint --fix --dbt-manifest target/manifest.json ./sql_queries/
  ```

> **⚠️ Disclaimer regarding Auto-Fix (`--fix`)**:
> OmniSQL's auto-fix engine currently only resolves **stylistic violations** (such as SQL keyword casing and trailing commas). It will *not* automatically repair semantic, cost, or security errors (e.g., it will not auto-generate missing JOIN conditions or automatically mask PII columns).

### Upgrading OmniSQL

To stay up to date with the latest features, performance improvements, and rule additions, you should keep the underlying `omnisql` CLI up to date:

- **Via NPM:**
  ```bash
  npm update -g omnisql
  ```

- **Via PyPI:**
  ```bash
  pip install --upgrade omnisql
  ```

## License

MIT License.
