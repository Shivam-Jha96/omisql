# Next-Generation SQL Linter Implementation Plan

This document outlines the current state of advanced SQL linters, the critical gaps they leave unaddressed, and a comprehensive plan to build a next-generation packaged SQL linter that bridges these gaps.

## 1. Market Analysis: The State of SQL Linters

The SQL linting ecosystem has matured significantly, moving from simple regex-based formatting to full AST (Abstract Syntax Tree) parsing. Here are the most advanced tools currently available:

- **SQLFluff**: The industry standard. Written in Python, it supports a massive array of dialects (Snowflake, BigQuery, Postgres, etc.) and integrates seamlessly with dbt. However, it is notoriously slow on large codebases.
- **Sqruff**: A high-performance alternative to SQLFluff, rewritten in Rust. It aims for 10x-100x performance improvements while maintaining SQLFluff's rule compatibility, solving the speed issue but inheriting the same rule limitations.
- **SDF (Data Development Simplified)**: A very fast, Rust-based engine focused on large-scale data engineering. It offers extreme performance but is tightly coupled to the SDF platform.
- **Squawk**: A specialized static analysis tool for PostgreSQL that focuses on migration safety (e.g., catching destructive `ALTER TABLE` commands) rather than style.
- **ZetaSQL**: Google's open-source analyzer. While incredibly powerful at semantic analysis, it is heavy, complex to integrate as a lightweight linter, and primarily focused on Google Standard SQL (BigQuery).

## 2. The Unfulfilled Gaps

Despite the advancements, current linters still fall short in several critical areas:

> [!WARNING]
> **1. Lack of True Semantic Understanding**
> Most linters (like SQLFluff/Sqruff) only understand syntax. They don't know if `SELECT user_id FROM orders` is valid because they don't know if `user_id` exists in the `orders` table. They cannot detect type mismatches (e.g., joining an `INT` to a `VARCHAR`) or invalid column references without complex, brittle database connections.

> [!CAUTION]
> **2. Blind Spots in Security & Privacy**
> Current general-purpose linters do not adequately prevent data leaks or security vulnerabilities. They fail to flag when PII (Personally Identifiable Information) columns are selected without masking, or when overly permissive grants are executed, or when LLM-generated SQL hallucinates unsafe operations.

> [!TIP]
> **3. Cost and Performance Estimation**
> In cloud data warehouses (BigQuery, Snowflake), bad queries cost real money. Existing linters rarely catch anti-patterns like querying a partitioned table without a partition filter, or performing cross-joins that will result in massive data scans.

> [!NOTE]
> **4. Difficult Extensibility**
> Writing custom rules in SQLFluff requires Python knowledge and understanding of its specific AST structure. There is no easy, language-agnostic plugin system for teams to write custom business-logic rules.

## 3. Proposed Solution: "OmniSQL Linter"

We propose building a packaged, next-generation SQL linter that doesn't just check style, but understands the **schema, semantics, cost, and security** of the SQL.

### Core Pillars
1. **Blazing Fast**: Native compiled core.
2. **Schema-Aware by Default**: Seamlessly ingests database schemas, dbt `manifest.json`, or YAML definitions.
3. **Multi-dimensional Rules**: Rules for Style, Semantics, Security, and Cost.
4. **WASM Plugin Ecosystem**: Write custom rules in any language (Rust, Go, TypeScript) compiled to WebAssembly.

---

## 4. Technology Stack

To achieve high performance, safety, and cross-platform distribution, we will use the following stack:

- **Core Engine (Lexer, Parser, AST, Semantic Engine)**: **Rust**. Rust provides the necessary memory safety and C-like performance required to parse thousands of SQL files in milliseconds.
- **Parser Framework**: **Logos** (for fast lexical analysis) + **tree-sitter** (or a custom Pratt parser) for robust, error-tolerant SQL parsing across multiple dialects.
- **Plugin System**: **WebAssembly (WASM)** via **Wasmtime**. This allows users to write custom linting rules in TypeScript, Python, or Go, which are then compiled to WASM and executed securely and rapidly by the Rust core.
- **Distribution**: 
  - **CLI**: Compiled native binaries for Mac, Linux, Windows.
  - **IDE Extensions**: VS Code / IntelliJ plugins utilizing a Rust-based Language Server Protocol (LSP).
  - **NPM / PyPI Wrappers**: For easy integration into JS and Python ecosystems.

---

## 5. Implementation Plan

### ✅ Phase 1: The Foundation (Core Parsing)
- Define the universal AST for SQL that can accommodate multiple dialects (Postgres and Snowflake as initial targets).
- Implement a high-performance lexer and parser in Rust.
- Implement a basic formatting and style rule engine (indentation, casing, trailing commas).

### ✅ Phase 2: The Semantic Engine (The Differentiator)
- Build a schema-ingestion module that reads from live databases, standard DDL files, and `dbt` metadata.
- Implement a semantic resolver that traverses the AST and binds column references to the loaded schema.
- Create core semantic rules: `InvalidColumnReference`, `TypeMismatchInJoin`, `AmbiguousColumn`.

### ⚪ Phase 3: Virtual Execution Engine (Dry-Run Analysis)
- Build an in-memory execution module that virtually evaluates the AST against mock schema data.
- Implement expression evaluation (constant folding, `WHERE` clause logical checks).
- Enable static "dry-run" capabilities to catch runtime logic errors (e.g., divide by zero, impossible filters) before execution.

### ⚪ Phase 4: Security & Cost Analysis
- Implement security rules: `UnmaskedPIISelect`, `DangerousMigration`, `UnsafeGrant`.
- Implement cost rules specific to cloud warehouses: `MissingPartitionFilter`, `CartesianJoinWarning`.

### ⚪ Phase 5: Extensibility & Packaging
- Embed a WASM runtime (Wasmtime) into the Rust core.
- Define a stable ABI (Application Binary Interface) for WASM plugins to interact with the SQL AST.
- Build the CLI, setup CI/CD for cross-platform binary distribution, and develop the VS Code extension via LSP.

---

## 6. Strategic Decisions (Opinionated Start)

To begin execution, we will adopt the following opinionated defaults:
1. **Dialect Priority**: Start with **PostgreSQL** (due to its strict standard and widespread use) and **Snowflake** (to target modern cloud data warehouses and demonstrate cost-estimation linting).
2. **Integration Focus**: Provide native **dbt** integration out-of-the-box, as it is the standard for modern data stacks. The linter must be able to read dbt's `manifest.json` for schema awareness.
3. **Plugin Language SDK**: Offer **TypeScript** as the primary SDK for writing WASM plugins. It offers a great developer experience, strong typing, and the widest reach for data engineers and analysts comfortable with JS/TS.

## 7. Go-To-Market & MVP Delivery Strategy

### How We Will Ship It
To ensure maximum adoption, the product must be trivial to install across different ecosystems. Because the core is written in Rust, we can compile a single, blazing-fast native executable. We will distribute it via:
1. **Direct Binaries**: Hosted on GitHub Releases (Windows `.exe`, macOS, Linux).
2. **NPM Wrapper (`npm install -g omnisql`)**: A lightweight Node script that detects the user's OS and downloads the correct pre-compiled Rust binary. Ideal for JS/TS ecosystems.
3. **PyPI Wrapper (`pip install omnisql`)**: A Python wrapper, crucial for data engineers and `dbt` users.
4. **CI/CD Pipelines**: A pre-built Docker image and a native GitHub Action.
5. **IDE Extension**: A VS Code extension that bundles the Language Server Protocol (LSP) version of the linter.

### Local Development vs. Production Packaging
**Yes, we can absolutely develop and ship this from your current Windows machine.**
- **Local Development**: You will write and test the Rust core, the TypeScript SDK, and the parsing logic directly on your Windows machine using `cargo`. It will produce a Windows `.exe` that you can test immediately.
- **Cross-Platform Shipping**: Instead of trying to compile macOS and Linux versions directly on your Windows machine (which is difficult), we will leverage **GitHub Actions**. By setting up a CI/CD pipeline, every time we push a new "Release" tag to GitHub, Microsoft's cloud servers will automatically compile the Windows, Linux, and macOS (Intel and Apple Silicon) binaries and publish them to NPM/PyPI.

### Cost of MVP Development
The financial cost to develop and launch this MVP is **practically $0**, aside from the time invested. 
- **Development Environment**: Free (Local Windows machine + VS Code + Rust toolchain).
- **CI/CD & Cross-Compilation**: Free (GitHub Actions provides 2,000 minutes/month for private repos, and unlimited for public open-source repos).
- **Package Registries**: Free (Publishing to NPM, PyPI, and the VS Code Marketplace costs nothing).
- **Binary Hosting**: Free (GitHub Releases).
- **Domain Name (Optional)**: ~$10-$20/year if you wish to host a documentation website (e.g., `omnisql-linter.dev`), though we can start with free GitHub Pages.
- **The True Cost**: Time and focus. Building a robust SQL parser and a WASM plugin engine is a complex engineering task. The MVP will require focused engineering effort to get the parser, AST, and semantic engine functioning flawlessly for the initial dialects.

---

## 8. Current Objective: Finishing Phase 1

To completely finish Phase 1 and turn the indicator green, we need to flesh out the AST and implement the foundational Style Engine. 

### User Review Required
> [!IMPORTANT]
> The style engine will run independently of the semantic engine. Should the linter automatically fix style issues (e.g., auto-capitalize keywords) or just report them as errors for now?

### Proposed Changes

#### 1. Expand the AST and Parser
- **[MODIFY] `src/lexer/mod.rs`**: Add tokens for `JOIN`, `ON`, `AND`, `OR`, `GROUP BY`, etc.
- **[MODIFY] `src/parser/mod.rs`**: 
  - Add `WhereClause` struct and expression tree (Binary expressions: `Left`, `Op`, `Right`).
  - Add `JoinClause` struct (Join Type, Table, Condition).
  - Update `parse_select` to parse these new optional clauses.

#### 2. Implement the Style Rule Engine
- **[NEW] `src/style/mod.rs`**: Create a basic trait/interface for Style Rules.
- **[NEW] `src/style/casing.rs`**: Implement `KeywordCasingRule` (ensures `SELECT`, `FROM`, `WHERE` are uppercase).
- **[NEW] `src/style/commas.rs`**: Implement `TrailingCommaRule` (detects extra commas before `FROM`).

#### 3. CLI Integration
- **[MODIFY] `src/main.rs`**: Run the style rules over the tokens/AST before (or alongside) the semantic validation.

### Verification Plan
- Write unit tests for parsing `WHERE` and `JOIN` clauses.
- Write unit tests for the Casing and Comma style rules.
- Update `sample.sql` to include style violations (e.g., lowercase `select`) and a `WHERE` clause, and verify the CLI catches them.

---

## 9. Current Objective: Phase 2 (Real Schema Ingestion)

To turn Phase 2 green, we need to move away from our hardcoded mock schema and build a real semantic engine capable of catching complex structural SQL errors.

### User Review Required
> [!IMPORTANT]
> 1. **Schema Format**: For the MVP, should we define a simple, custom `schema.json` format to represent our tables and columns, or attempt to parse a standard `dbt manifest.json` right away? (Recommendation: Start with a simple custom `schema.json` to prove the logic).
> 2. **Missing Schemas**: If a user runs the linter without providing a schema file, should the linter fail entirely, or just run the Style Engine and skip semantic validation?

### Proposed Changes

#### 1. Schema Ingestion (`src/semantic/schema.rs`)
- **[NEW] `SchemaConfig`**: Define a Serde-compatible struct to parse a `schema.json` file. The schema should define tables, columns, and column data types (e.g., INT, VARCHAR).
- **[MODIFY] `src/semantic/mod.rs`**: Remove `load_mock_schema()` and replace it with `load_from_file(path)`.

#### 2. Advanced Semantic Rules (`src/semantic/rules.rs`)
- **[NEW] `InvalidColumnReference`**: Check if a requested column exists in the queried table(s).
- **[NEW] `AmbiguousColumn`**: If a query contains a `JOIN`, and a column exists in *both* tables (e.g., `id`), flag an error if the user doesn't fully qualify it (e.g., `users.id`).

#### 3. AST Updates (`src/parser/mod.rs` & `src/lexer/mod.rs`)
- **[MODIFY] Parser**: Update column parsing to recognize fully qualified names (`table.column`). 
  - Add support for the `.` token in the Lexer.
  - Update AST `SelectStatement` to store `CompoundIdentifier`s or split them into `(Option<String>, String)` representing `(Table, Column)`.

#### 4. CLI Integration (`src/main.rs`)
- **[MODIFY] `src/cli/mod.rs`**: Add an optional `--schema <PATH>` flag to the CLI.
- **[MODIFY] `src/main.rs`**: Load the schema from the provided path, pass the AST through the new Semantic Engine rules, and report all semantic errors discovered.

### Verification Plan
- Create a `schema.json` file with multiple tables (`users`, `orders`).
- Create `test_queries.sql` with a `JOIN` that contains an ambiguous column reference.
- Verify the linter catches the ambiguous column and correctly validates columns across multiple joined tables.
