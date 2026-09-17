# Build Progress Log

This document tracks the progress of the OmniSQL project throughout its development lifecycle.

## [2026-09-14] - Foundation & Lexer MVP
- **Initialized**: Created Rust workspace at `D:\Dev\repos\omnisql`.
- **Configured Dependencies**: Added `clap` (CLI), `logos` (Lexer), and `anyhow` (Error handling) to `Cargo.toml`.
- **Environment Setup**: Validated Rust toolchain and MSVC Build Tools installation.
- **Implemented Lexer Placeholder**: Created a foundational Lexer using `logos` capable of parsing `SELECT`, `FROM`, `WHERE`, identifiers, strings, numbers, and basic operators.
- **Modularization**: Refactored the codebase to follow standard Rust best practices (`src/lexer/`, `src/cli/`, `src/parser/`, `src/semantic/`).
- **Documentation**: Generated initial `README.md` and `build_progress.md`.

## [2026-09-14] - Semantic Engine MVP (Parallel Execution)
- **Schema Registry**: Implemented `SchemaRegistry` to manage table-column mappings using a `HashMap`.
- **Validation Engine**: Built `validate_column` function to verify if columns exist in the registered schema.
- **Mock Schema**: Added `load_mock_schema` to populate a dummy `users` table with `user_id`, `email`, and `age` for immediate testing.
- **Testing**: Added inline unit tests for the schema validation logic.

## [2026-09-14] - AST Parser MVP (Parallel Execution)
- **Parser Engine**: Created the foundational `parse_select` function in `src/parser/mod.rs` to iterate over raw Lexer tokens.
- **AST Generation**: Implemented the `SelectStatement` struct to capture requested columns and the target table.
- **Error Handling**: Implemented graceful failure states for unexpected tokens and syntax errors.
- **Testing**: Added inline unit tests to verify `SELECT *` and explicit column parsing logic.

## [2026-09-14] - Subagent Integration & Verification
- **Test Suite Pass**: Ran `cargo test`, successfully executing 4/4 parallel-developed unit tests across the Parser and Semantic engines without errors.
- **Pipeline Integration**: Connected the Lexer, Parser, and Semantic Engine sequentially in `main.rs`. Updated the Lexer and Parser to dynamically extract and store `Token::Identifier(String)` instances.
- **E2E Validation**: Validated the full pipeline by parsing `sample.sql` and confirming that the extracted table columns successfully pass semantic validation against the mock schema.

## [2026-09-17] - Phase 1 Completion (AST & Style Engine)
- **AST Expansion**: Rewrote the parser to support `WHERE` clauses (via binary expressions) and `JOIN` clauses.
- **Style Engine**: Implemented a regex-based pre-processing style engine in `src/style/` that auto-fixes casing and removes trailing commas.
- **Auto-Fix Logging**: Configured `main.rs` to write applied style fixes directly to `omnisql_fixes.log`.

*Status: Phase 1 (Foundation) Complete. Moving towards Phase 2 (Real Schema Ingestion) and Phase 3 (Virtual Execution Engine).*

## [2026-09-17] - Phase 2 Completion (Schema Ingestion & Semantic Rules)
- **Schema Ingestion**: Replaced mock schema with `load_from_ddl` which parses raw `CREATE TABLE` DDL queries to extract tables and columns.
- **AST Updates**: Updated Lexer and Parser to support compound identifiers (`table.column`) by adding `Token::Dot`.
- **Advanced Semantic Rules**: Implemented `AmbiguousColumn` logic to flag errors when unqualified columns exist in multiple joined tables.
- **Validation**: Verified against `test_dbs/sample_db1/1. pagila-schema.sql` via `--schema` argument.

*Status: Phase 2 Complete. Ready for Phase 3 (Virtual Execution Engine).*

## [2026-09-17] - Phase 3 Completion (Virtual Execution Engine)
- **Math Operators**: Extended Lexer and Parser to support arithmetic operators (+, -, *, /).
- **Constant Folding**: Implemented evaluate_expression in src/exec/eval.rs to statically resolve simple mathematical and logical binary operations.
- **Dry-Run Analysis**: Added rules in src/exec/rules.rs to catch Divide by Zero (as a hard error) and Impossible Filter (e.g., WHERE 0 = 1 as a warning).
- **Validation**: CLI now performs Virtual Execution Engine checks after semantic validation. Wrote unit tests and verified manually on test queries.

*Status: Phase 3 Complete. Ready for Phase 4 (Security & Cost Analysis).*


## [2026-09-17] - Phase 4 Completion (Security & Cost Analysis)
- **Lexer & Parser Enhancements**: Added support for non-SELECT statements (DROP, ALTER, GRANT) into the Lexer and Statement enum.
- **Metadata Ingestion**: Updated SchemaRegistry to parse DDL comments (@PII and @PARTITION) to assign security/cost metadata to columns.
- **Security Rules**: Implemented UnmaskedPIISelect, DangerousMigration, and UnsafeGrant within the semantic::security module.
- **Cost Rules**: Implemented MissingPartitionFilter and CartesianJoinWarning (leveraging Virtual Execution) within the semantic::cost module.
- **Validation**: Hooked all rules up to the CLI and manually verified against test SQL covering PII leakage, drop operations, cartesian joins, and missing partition filters.

*Status: Phase 4 Complete. Ready for Phase 5 (Extensibility & Packaging).*

