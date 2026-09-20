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

### ✅ Phase 3: Virtual Execution Engine (Dry-Run Analysis)
- Build an in-memory execution module (`src/exec/mod.rs`) that virtually evaluates the AST.
- Implement expression evaluation (constant folding, `WHERE` clause logical checks).
- Enable static "dry-run" capabilities to catch runtime logic errors (e.g., divide by zero, impossible filters) before execution.

### ✅ Phase 4: Security & Cost Analysis
- Implement security rules: `UnmaskedPIISelect`, `DangerousMigration`, `UnsafeGrant`.
- Implement cost rules specific to cloud warehouses: `MissingPartitionFilter`, `CartesianJoinWarning`.

### ✅ Phase 5: Extensibility & Packaging
- Embed a WASM runtime (Wasmtime) into the Rust core.
- Define a stable ABI (Application Binary Interface) for WASM plugins to interact with the SQL AST.
- Build the CLI and setup CI/CD for cross-platform binary distribution.

### ✅ Phase 6: IDE Integration & Advanced Workflow
- Build a native `tower-lsp` Language Server to provide real-time editor squiggles.
- Ingest dbt `manifest.json` metadata natively to remove manual DDL maintenance.
- Introduce an AST/Token-based `--fix` engine to automatically patch style violations safely.

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

## 8. Current Objective: Phase 1 (Core Parsing) [COMPLETED]

This phase finalized the core parsing engine by expanding the AST to support missing clauses and implementing the foundational Style Engine.

### Implemented Changes

#### 1. Expand the AST and Parser
- **[MODIFY] `src/lexer/mod.rs`**: Added tokens for `JOIN`, `ON`, `AND`, `OR`, `GROUP`, and `BY`.
- **[MODIFY] `src/parser/mod.rs`**: 
  - Added `WhereClause` struct and expression tree (Binary expressions: `Left`, `Op`, `Right`).
  - Added `JoinClause` struct (Join Type, Table, Condition).
  - Added `group_by` field to `SelectStatement` to store a list of `ColumnRef`s.
  - Updated `parse_select` to parse these new optional clauses seamlessly.

#### 2. Implement the Style Rule Engine
- **[NEW] `src/style/mod.rs`**: Created a basic trait/interface for Style Rules.
- **[NEW] `src/style/casing.rs`**: Implemented `KeywordCasingRule` (ensures `SELECT`, `FROM`, `WHERE`, `GROUP BY`, etc. are uppercase).
- **[NEW] `src/style/commas.rs`**: Implemented `TrailingCommaRule` (detects extra commas before `FROM`).

#### 3. CLI Integration & Verification
- **[MODIFY] `src/main.rs`**: Integrated the style rules to run over the tokens/AST before (or alongside) the semantic validation.
- **[MODIFY] `src/exec/mod.rs`**: Updated all mock `SelectStatement` initializers in tests to include the new `group_by` field.
- **[NEW] `sample.sql`**: Created a sample SQL file with style violations and a `WHERE`/`GROUP BY` clause. Verified that `cargo run -- lint sample.sql` successfully lexes, parses, and auto-fixes the issues.

---

### 9. Current Objective: Phase 2 (Real Schema Ingestion) [COMPLETED]

This phase successfully moved away from the hardcoded mock schema and built a real semantic engine capable of catching complex structural SQL errors, directly supporting DDL files and dbt manifests.

### Implemented Changes

#### 1. Schema Ingestion
- **[MODIFY] `src/semantic/mod.rs`**: Built `load_from_ddl` to parse basic CREATE TABLE statements and infer partition/PII column metadata from comments.
- **[NEW] `src/semantic/dbt.rs`**: Built a parser to consume standard `dbt manifest.json` files and accurately populate the `SchemaRegistry` with model and source structures.

#### 2. Advanced Semantic Rules
- **[MODIFY] `src/main.rs`**: Implemented `InvalidColumnReference` to check if a requested column exists in the queried table(s).
- **[MODIFY] `src/main.rs`**: Implemented `AmbiguousColumn` logic for queries containing a `JOIN`, ensuring that if a column exists in multiple tables, it must be fully qualified.

#### 3. AST Updates
- **[MODIFY] Parser & Lexer**: Updated column parsing to recognize fully qualified names (`table.column`), storing them in a `ColumnRef` struct.

#### 4. CLI Integration
- **[MODIFY] `src/cli/mod.rs`**: Added optional `--schema <PATH>` and `--dbt-manifest <PATH>` flags to the CLI.
- **[MODIFY] `src/main.rs`**: Enabled the loading of schemas from the provided paths, passing the AST through the Semantic Engine rules, and reporting discovered semantic errors.

---

### 10. Current Objective: Phase 3 (Virtual Execution Engine) [COMPLETED]

This phase built an in-memory execution module that virtually evaluates the AST and catches runtime errors via "dry-run" analysis.

### Implemented Changes

#### 1. AST Updates for Math Operators
- **[MODIFY] Lexer (`src/lexer/mod.rs`)**: Updated operator regex to explicitly support `+`, `-`, `*`, `/`.
- **[MODIFY] Parser (`src/parser/mod.rs`)**: Extended `parse_expression` to handle basic arithmetic operations and correctly parse them into `Expression::BinaryOp`.

#### 2. Virtual Execution Engine
- **[NEW] `src/exec/mod.rs`**: Built the execution engine module.
- **[NEW] `src/exec/eval.rs`**: Implemented `evaluate_expression(expr)` that resolves `BinaryOp` nodes to constant values when possible (constant folding).
- **[NEW] `src/exec/rules.rs`**: Implemented rules for:
  - `DivideByZeroError`: Triggered if a division by literal `0` is detected in an expression.
  - `ImpossibleFilterWarning`: Triggered if a `WHERE` clause statically evaluates to `false`.

#### 3. CLI Integration
- **[MODIFY] `src/main.rs`**: Plugged the new execution engine into the pipeline after Semantic Validation, logging dry-run errors/warnings to the console output.


---

## 11. Current Objective: Phase 4 (Security & Cost Analysis) [COMPLETED]

This phase extended the OmniSQL Linter beyond structural and semantic validation to catch security vulnerabilities and costly cloud-warehouse queries.

### Implemented Changes

#### 1. AST and Parser Enhancements
- **[MODIFY] src/lexer/mod.rs**: Added tokens for DROP, ALTER, GRANT, TABLE, ALL, PRIVILEGES.
- **[MODIFY] src/parser/mod.rs**: Added DropStatement, AlterStatement, and GrantStatement to the AST and replaced parse_select with a generic parse_statement entry point.

#### 2. Metadata Ingestion
- **[MODIFY] src/semantic/mod.rs**: Updated SchemaRegistry to use a TableDef struct that tracks standard columns, pii_columns, and partition_column. load_from_ddl now parses COMMENT '@PII' and COMMENT '@PARTITION' to populate these fields.

#### 3. Security Rules
- **[NEW] src/semantic/security.rs**: 
  - UnmaskedPIISelect: Flags an error if a PII column is selected.
  - DangerousMigration: Flags DROP TABLE (error) and ALTER TABLE (warning).
  - UnsafeGrant: Flags GRANT ALL PRIVILEGES as overly permissive.

#### 4. Cost Analysis Rules
- **[NEW] src/semantic/cost.rs**:
  - MissingPartitionFilter: Enforces that queries against partitioned tables filter on the partition column.
  - CartesianJoinWarning: Warns if a JOIN ON condition statically evaluates to a truthy constant (e.g., ON 1=1), potentially causing massive cross joins.

---

## 12. Current Objective: Phase 5 (Extensibility & Packaging) [COMPLETED]

This phase introduced WebAssembly (WASM) plugins for custom rules and prepared the project for cross-platform distribution.

### Implemented Changes

#### 1. Extensibility (WASM)
- **[NEW] src/plugin/mod.rs**: Integrated `wasmtime` and `wasmtime-wasi` to load and execute `.wasm` plugin files.
- **ABI Definition**: Serialized the AST into JSON via `serde` and passed it to WASM guests via standard WASI stdin, parsing `LintIssue` responses from stdout.

#### 2. Packaging
- **[NEW] .github/workflows/release.yml**: Set up automated cross-compilation for Windows, macOS, and Linux using GitHub Actions.

---

## 13. Current Objective: Phase 6 (IDE Integration & Advanced Workflow) [COMPLETED]

This phase brought OmniSQL from a CLI tool to a seamlessly integrated developer experience.

### Implemented Changes

#### 1. Language Server Protocol (LSP) Engine
- **[NEW] src/lsp/mod.rs**: Implemented a tower-lsp server that provides real-time diagnostics as the user types.
- **[MODIFY] src/cli/mod.rs**: Added an lsp subcommand to start the language server.
- **[MODIFY] Cargo.toml**: Added tower-lsp and tokio dependencies.

#### 2. Native dbt Integration
- **[NEW] src/semantic/dbt.rs**: Defined Serde structs to parse the nodes and sources objects inside manifest.json.
- **[MODIFY] src/semantic/mod.rs**: Extended SchemaRegistry with a load_from_dbt_manifest(path) function.
- **[MODIFY] src/cli/mod.rs**: Added a --dbt-manifest <PATH> flag to the lint command.

#### 3. Auto-Fixing Engine
- **[NEW] src/style/fix.rs**: Implemented a token-based string manipulation engine to apply fixes using byte offsets from the lexer.
- **[MODIFY] src/main.rs**: Applied computed fixes to the source string and written back to the file if --fix is passed.

---

## 14. Current Objective: Phase 7 (Wrappers & IDE Extension) [COMPLETED]

To turn Phase 7 green, we need to create wrappers for JS and Python ecosystems to download and run the native binary, and build a VS Code extension that utilizes the Rust language server.

### User Review Required
> [!IMPORTANT]
> 1. **NPM Package Name**: We plan to use `omnisql` on NPM. Is this acceptable?
> 2. **PyPI Package Name**: We plan to use `omnisql` on PyPI. Is this acceptable?
> 3. **VS Code Extension Name**: We will name the extension `omnisql-vscode`. Should it be published under a specific publisher namespace?

### Proposed Changes

#### 1. NPM Wrapper
- **[NEW] `npm/package.json`**: Definition for the NPM package.
- **[NEW] `npm/install.js`**: Script to detect the OS/architecture and download the correct pre-compiled Rust binary from GitHub Releases.
- **[NEW] `npm/index.js`**: Thin wrapper to execute the downloaded binary.

#### 2. PyPI Wrapper
- **[NEW] `python/pyproject.toml`** or `setup.py`: Definition for the Python package.
- **[NEW] `python/omnisql/__init__.py`**: Python wrapper to download and execute the native binary.
- **[NEW] `python/omnisql/install.py`**: OS detection and download script similar to NPM.

#### 3. VS Code Extension
- **[NEW] `vscode-extension/package.json`**: Extension manifest.
- **[NEW] `vscode-extension/src/extension.ts`**: TypeScript code that activates the LSP client.
- **[NEW] `vscode-extension/tsconfig.json`**: TypeScript config for the extension.

### Verification Plan
- **NPM**: Run `npm install` and `npm link` in the `npm` directory locally, and verify the `omnisql` command executes correctly.
- **PyPI**: Create a virtual environment, install the package locally, and verify the `omnisql` CLI is available.
- **VS Code**: Compile the extension (`npm run compile`), and verify it can connect to the locally compiled Rust `omnisql lsp` server.

---

## 15. Current Objective: Phase 8 (Dockerization) [COMPLETED]

This phase enabled CI/CD for dockerization, making it easy to run the linter without any local dependencies.

### Implemented Changes

#### 1. Docker Build
- **[NEW] `Dockerfile`**: Added a multi-stage Dockerfile that builds the rust binary and packages it into a lightweight `debian:bookworm-slim` image.
- **[NEW] `.dockerignore`**: Created an ignore file to prevent large directories from entering the Docker build context.

#### 2. CI/CD Integration
- **[NEW] `.github/workflows/docker.yml`**: Added a GitHub Action workflow to automatically build and publish the Docker image to GitHub Container Registry (ghcr.io) on tags and pushes to main.

#### 3. Documentation
- **[MODIFY] `README.md`**: Updated with instructions on how to use the Docker image to run the linter.

---

## 16. Current Objective: Phase 9 (Go-To-Market & Polish) [COMPLETED]

This phase finalized the Go-To-Market (GTM) materials and polished the wrapper installation scripts to ensure real functionality for the v0.1.0 MVP launch.

### Implemented Changes

#### 1. Go-To-Market Documentation
- **[NEW] CHANGELOG.md**: Created a detailed changelog for the v0.1.0 MVP release.
- **[NEW] docs/pitch_deck.md**: Created the official pitch deck highlighting OmniSQL's value proposition.
- **[NEW] docs/product_catalogue.md**: Created the product catalogue detailing the core modules and rules.
- **[NEW] scode-extension/README.md**: Added a user-facing README for the VS Code marketplace.
- **[NEW] scode-extension/icon.png**: Added the extension logo.

#### 2. Wrapper Polish
- **[MODIFY] 
pm/install.js**: Replaced mock download logic with actual download and extraction from GitHub Releases.
- **[MODIFY] python/omnisql/install.py**: Added real download logic for PyPI distribution using urllib.
- **[MODIFY] python/omnisql/__init__.py**: Cleaned up the wrapper script.
- **[MODIFY] python/setup.py**: Ensured the install script runs during the build process so the binary is included.
- **[MODIFY] scode-extension/package.json**: Added repository URL, license, and icon configuration.

#### 3. CI/CD & Cleanup
- **[MODIFY] .github/workflows/release.yml**: Configured full cross-platform release workflow using GitHub Actions.
- **[DELETE] test_plugin/**: Removed the dummy test plugin project as it is no longer needed.

---

## 17. Current Objective: Phase 10 (Day 1 Readiness & v0.1.3 Release) [COMPLETED]

This phase resolved critical deployment bugs and refactored wrapper architectures to ensure a flawless public launch of `v0.1.3`.

### Implemented Changes

#### 1. VS Code Marketplace Fixes
- **[MODIFY] vscode-extension/icon.png**: Converted the auto-generated JPEG to a true PNG format to pass VS Code's strict marketplace validation.
- **[MODIFY] vscode-extension/package.json**: Restored the `icon` field and bumped version to `0.1.3`.
- **[MODIFY] .vscodeignore**: Configured ignore rules to prevent warnings and keep the VSIX package small while preserving `node_modules` dependencies.

#### 2. Python Architecture Refactor (JIT Downloads)
- **[MODIFY] python/setup.py**: Removed custom build hooks and `package_data` inclusion. PyPI packages are now pure-Python and lightweight.
- **[MODIFY] python/omnisql/__init__.py**: Refactored to download the native binary *at runtime* (JIT) if it does not exist, rather than at `pip install` time.
- **[MODIFY] python/omnisql/install.py**: Changed the download destination to `~/.omnisql/bin/omnisql` to resolve global `sudo` permissions errors across all operating systems.

#### 3. CI/CD Pipeline Fixes
- **[MODIFY] .github/workflows/release.yml**: 
  - Upgraded node versions in GitHub Actions from `18.x` to `20.x` to resolve a known bug with `vsce publish` and `undici`.
  - Fixed a critical `actions/download-artifact@v4` glob resolution bug by defining explicit patterns and using recursive `artifacts/**` for `action-gh-release`.
- **[NEW] LICENSE**: Added a standard MIT License to the repository root to prevent interactive prompt hangs during automated CI/CD builds.

---

## 18. Current Objective: Phase 11 (Runtime Debug Logs & Repository Cleanup) [COMPLETED]

This phase introduced detailed runtime debug logs directly to the user (via CLI and the VS Code Output panel) and strictly sanitized the root repository.

### Implemented Changes

#### 1. Runtime Debug Logs & Verbose Flag
- **[MODIFY] `src/cli/mod.rs`**: Added a `--verbose` flag to the `lint` command to allow users to opt into runtime logs.
- **[MODIFY] `src/style/mod.rs`**: Upgraded `format_and_log` to return a `Vec<String>` of applied stylistic fixes alongside the modified SQL. Also strictly enforced `Send + Sync` bounds for the `StyleRule` trait to ensure thread safety across the LSP async executor.
- **[MODIFY] `src/main.rs`**: Modified the core CLI engine to print the `--- Runtime Debug Log ---` when `--verbose` is passed, or automatically when a hard parse error occurs, preventing users from needing to manually check hidden `.log` files.

#### 2. LSP Integration
- **[MODIFY] `src/lsp/mod.rs`**: Integrated the `StyleEngine` directly into the LSP's `on_change` event stream. It now pipes the active debug logs directly to the VS Code client via `window/logMessage` as an `INFO` stream, and attaches them to `ERROR` payloads if a parsing failure occurs.

#### 3. Repository Cleanup & Ignored Artifacts
- **[MODIFY] `src/parser/mod.rs`**: Removed completely unused and redundant SQL dialect imports.
- **[MODIFY] `.gitignore`**: Re-encoded the garbled trailing lines, properly ignored multiple temporary `target*/` directories, and added rules for `lsp_debug.log` and scratch `tests*.rs` files to keep source control clean.
- **[MODIFY] `sample.sql`**: Relocated the root-level scratch demo file into the `examples/sample.sql` directory.

---

## 19. Current Objective: Phase 12 (Usage Guides & Documentation) [COMPLETED]

This phase focused on improving the documentation across all package distributions by adding explicit usage guides and upgrade commands for the CLI and extensions.

### Implemented Changes

#### 1. NPM and General README Update
- **[MODIFY] `README.md`**: Added a new Usage Guide section detailing single-file linting, directory linting, auto-fix, and configuration flags. Added upgrade commands for NPM and PyPI.

#### 2. PyPI Package README Update
- **[MODIFY] `python/README.md`**: Added the new Usage Guide section and PyPI-specific upgrade instructions for users operating within Python/dbt ecosystems.

#### 3. VS Code Extension README Update
- **[MODIFY] `vscode-extension/README.md`**: Added the CLI Usage Guide to the extension marketplace details page to inform users how to run OmniSQL outside of the editor environment, alongside upgrade instructions.

---

## 20. Current Objective: Phase 13 (Dynamic Wrapper Versions) [COMPLETED]

This phase resolved a caching bug where the wrappers (NPM and PyPI) would download older versions of the pre-compiled binary or show outdated versions when running `omnisql -V`, by switching to fully dynamic version resolution based on package metadata.

### Implemented Changes

#### 1. Dynamic Installer Resolution
- **[MODIFY] `npm/install.js`**: Removed the hardcoded version string and configured the installer to dynamically read the target version from `package.json`.
- **[MODIFY] `python/omnisql/install.py`**: Removed the hardcoded version string and leveraged Python's `importlib.metadata` to automatically resolve the installed PyPI package version.

#### 2. Instant Version Reporting (CLI Interception)
- **[MODIFY] `npm/index.js`**: Added interception logic for the `-V` and `--version` flags to instantaneously print the `package.json` version without spawning the underlying Rust binary.
- **[MODIFY] `python/omnisql/__init__.py`**: Added interception logic for the `-V` and `--version` flags to instantaneously print the dynamic package version without triggering a binary download or execution.

#### 3. Dynamic README Badges
- **[MODIFY] `README.md`**: Removed static `v1.0.0` strings across the document and added dynamic live-updating version badges for NPM, PyPI, and the VS Code Marketplace.
