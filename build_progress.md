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

*Status: MVP Pipeline Complete. Ready for next phase (e.g., Execution Engine or WHERE clause support).*
