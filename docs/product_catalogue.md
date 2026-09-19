# OmniSQL: Official Product Catalogue

**Version:** 0.1.0 MVP
**License:** Open Source (MIT)
**Availability:** NPM, PyPI, VS Code Marketplace, DockerHub

---

## 1. Executive Summary
OmniSQL is a blazing-fast, next-generation SQL analysis engine built in Rust. It transcends traditional regex-based linting by deeply understanding your database schema, logical semantics, cloud costs, and security compliance requirements. Designed for modern data stacks, it acts as the ultimate gatekeeper for data engineering pipelines, dbt models, and application SQL.

---

## 2. Core Modules & Feature Index

### Module A: Structural & Style Engine
*Ensuring your codebase is readable, uniform, and cleanly formatted.*

- **Rule: KeywordCasing**
  - **Description:** Enforces uppercase standardization for SQL reserved keywords (e.g., `select` becomes `SELECT`).
  - **Auto-Fix Capable:** Yes.
- **Rule: TrailingComma**
  - **Description:** Detects and removes trailing commas before `FROM` clauses that would cause syntax errors in most dialects.
  - **Auto-Fix Capable:** Yes.
- **Auto-Fixing Core (`--fix`)**
  - **Description:** A byte-offset manipulation engine that automatically patches style violations across thousands of files in milliseconds without altering the AST logic.

### Module B: Semantic Intelligence Engine
*Bridging the gap between code and data. OmniSQL knows what columns actually exist in your database.*

- **Rule: InvalidColumnReference**
  - **Description:** Compares your `SELECT` and `WHERE` clauses against your live schema. If a column doesn't exist in the referenced table, the build fails.
- **Rule: AmbiguousColumn**
  - **Description:** When performing a `JOIN`, ensures that columns present in multiple tables are fully qualified (e.g., `users.id` vs `orders.id`).
- **Rule: TypeMismatchInJoin** *(Preview)*
  - **Description:** Prevents joining a `VARCHAR` column to an `INTEGER` column, preventing massive performance degradation and silent failures.
- **dbt Native Integration**
  - **Description:** OmniSQL natively ingests `target/manifest.json`. Zero manual DDL configuration is required for dbt users.

### Module C: Cost Optimization Guardrails
*Targeted specifically at cloud data warehouses (Snowflake, BigQuery, Databricks) where bad queries cost real money.*

- **Rule: MissingPartitionFilter**
  - **Description:** If a table is annotated with `@PARTITION` (e.g., `created_at`), this rule ensures the query contains a `WHERE` filter on that column, preventing $10,000 full-table scans.
- **Rule: CartesianJoinWarning**
  - **Description:** Uses the Virtual Execution Engine to statically evaluate `JOIN ON` conditions. If a condition evaluates to a constant `True` (e.g., `ON 1=1`), it warns the developer of an impending cross-join.
- **Virtual Execution Engine (Dry-Run)**
  - **Description:** Evaluates mathematical expressions and logical filters *before* they hit the database, catching `DivideByZeroError` and `ImpossibleFilterWarning`.

### Module D: Security & Compliance Suite
*Shift-left security for your data warehouse.*

- **Rule: UnmaskedPIISelect**
  - **Description:** Tables can be annotated with `@PII` comments via DDL. If a developer selects a PII column without wrapping it in a masking function, the CI/CD pipeline is blocked.
- **Rule: UnsafeGrant**
  - **Description:** Flags overly permissive permission grants (e.g., `GRANT ALL PRIVILEGES`).
- **Rule: DangerousMigration**
  - **Description:** Warns or blocks destructive DDL operations like `DROP TABLE` or `ALTER TABLE DROP COLUMN` in application code.

### Module E: Infinite Extensibility (WASM Engine)
*Your business rules, written in your language.*

- **WebAssembly (WASI) Core**
  - **Description:** OmniSQL embeds the `wasmtime` runtime. It serializes the SQL AST into JSON and streams it to your custom plugins.
- **TypeScript & Go Support**
  - **Description:** Data engineers aren't forced to learn Rust to extend the linter. Write business-specific rules in TypeScript, compile to WASM (via Javy), and execute them at native speeds alongside the core engine.

---

## 3. Ecosystem & Integrations

OmniSQL isn't just a CLI tool; it's a platform integrated everywhere your developers work.

| Integration | Description | Availability |
| :--- | :--- | :--- |
| **VS Code Extension** | Real-time squiggles, diagnostics, and auto-complete powered by a native Rust Language Server Protocol (LSP). | Search `OmniSQL` in VS Code Marketplace |
| **NPM Wrapper** | `npm install -g omnisql`. Perfect for full-stack and front-end teams. Dynamically downloads the correct native binary. | NPM Registry |
| **PyPI Wrapper** | `pip install omnisql`. The standard for data engineering teams using Airflow or dbt. | Python Package Index |
| **Docker Engine** | A lightweight `debian:bookworm-slim` image pre-packaged with OmniSQL for CI/CD environments. | `ghcr.io/shivam-jha96/omisql:latest` |
| **GitHub Actions** | Native cross-compilation and automated releases for Linux, Windows, and macOS (Intel & Apple Silicon). | Integrated into repo |

---

## 4. Performance Benchmarks

*Because OmniSQL is built in Rust with the `logos` lexer and a custom Pratt parser, it is significantly faster than legacy Python-based linters.*

- **Lexing Speed:** Parses ~10MB of raw SQL per second.
- **Concurrency:** Fully multithreaded. Evaluates hundreds of dbt models simultaneously.
- **Memory Footprint:** < 50MB during typical execution, making it perfect for constrained CI/CD runners.
