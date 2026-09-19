# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Initial Public Release

This is the very first Minimum Viable Product (MVP) release of OmniSQL, a next-generation SQL linter built natively in Rust.

### Added
- **High-Performance Parsing Engine**: Lexes and parses SQL instantly, with initial strong support for PostgreSQL and Snowflake dialects.
- **Style Rule Engine**: Detects and auto-fixes basic formatting issues (e.g., keyword casing, trailing commas). Run with `--fix` to auto-resolve.
- **Semantic Validation Engine**: Understands schema context. Flags `InvalidColumnReference` and `AmbiguousColumn` (for un-qualified joins) by reading your actual DDL or `dbt manifest.json`.
- **Cost Estimation Rules**: 
  - `MissingPartitionFilter`: Enforces querying with partition keys to avoid expensive cloud warehouse full-table scans.
  - `CartesianJoinWarning`: Statically evaluates `JOIN` conditions to warn about `1=1` cross-joins.
- **Security Analysis**:
  - `UnmaskedPIISelect`: Automatically flags when columns annotated with `@PII` are selected without masking functions.
  - `UnsafeGrant` & `DangerousMigration`: Warns on highly privileged grants or dropping tables.
- **WASM Plugin Ecosystem**: Write your own company-specific linting rules in TypeScript, Rust, or Go, compile them to WebAssembly, and run them blazingly fast in the OmniSQL core.
- **Native dbt Integration**: Reads `target/manifest.json` automatically to ingest your `models` and `sources` schemas without any configuration.
- **VS Code Extension & LSP**: Bundled Language Server Protocol provides real-time squiggles and diagnostics in your IDE.
- **Cross-Platform Distribution**: Pre-compiled binaries available directly via NPM (`npm install -g omnisql`) and PyPI (`pip install omnisql`).
