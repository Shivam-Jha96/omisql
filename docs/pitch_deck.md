---
marp: true
theme: default
class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://hero-pattern-url') 
---

# OmniSQL 
## The Next-Generation SQL Linter

**Blazing Fast. Schema-Aware. Extensible.**

---

## The Problem with Legacy SQL Linters

Current tools (like SQLFluff) have critical blind spots:

1. **Syntax-Only:** They check if `SELECT id FROM users` is valid SQL, but they don't know if the `id` column actually exists.
2. **Security & Privacy Risks:** They fail to detect when highly sensitive PII is queried without masking.
3. **Costly Anti-Patterns:** In cloud warehouses (Snowflake, BigQuery), bad queries cost real money (e.g., missing partition filters, cross-joins).
4. **Hard to Extend:** Writing custom business rules requires deep AST knowledge and Python expertise.

---

## The OmniSQL Solution

We built a linter that actually **understands** your data context.

- 🚀 **Blazing Fast:** Written natively in Rust.
- 🧠 **Schema-Aware:** Validates columns and joins using real DDL or `dbt manifest.json`.
- 🛡️ **Multi-Dimensional:** Checks for Style, Semantics, Security, and Cloud Cost.
- 🔌 **Language-Agnostic Plugins:** Write your own rules in TypeScript via WebAssembly (WASM).

---

## Feature Highlight: Semantic Validation

**Catch runtime errors before they reach production.**

- OmniSQL ingests your database schema (or reads your `dbt` project seamlessly).
- It binds every column reference in your query to the schema.
- **Errors Caught:** `InvalidColumnReference`, `TypeMismatchInJoin`, `AmbiguousColumn`.

*Say goodbye to "Column not found" errors in production!*

---

## Feature Highlight: Security & Cost Guardrails

**Protect your data and your wallet.**

- **Cost Control:** Flags `MissingPartitionFilter` and `CartesianJoinWarning` (prevents `$10,000` accidental full-table scans).
- **Security:** Flags `UnmaskedPIISelect` (when developers query `@PII` columns without masking functions).
- **Safe Migrations:** Warns on `DangerousMigration` (e.g., destructive `DROP TABLE`) and `UnsafeGrant`.

---

## Feature Highlight: WebAssembly (WASM) Plugins

**Write rules your way.**

Every company has unique data governance rules. 

With OmniSQL's embedded Wasmtime engine, data teams can write custom linting rules in **TypeScript**, Go, or Rust. The linter passes the AST to your plugin, keeping the core blazingly fast while giving you infinite extensibility.

---

## Seamless Developer Experience

OmniSQL meets developers where they already are.

- **Cross-Platform CLI:** `npm install -g omnisql` or `pip install omnisql`.
- **Zero-Config dbt Integration:** Point it at your dbt project, and it just works.
- **Auto-Fixing Engine:** Run `--fix` to automatically correct casing, trailing commas, and style violations.
- **Real-Time IDE Feedback:** Bundled Language Server Protocol (LSP) powers our native VS Code extension.

---

## Ready to Launch (v0.1.0)

**The MVP is complete and ready for adoption.**

✅ Rust Core Engine
✅ Semantic, Cost, and Security Rules
✅ NPM & PyPI Distribution
✅ VS Code Marketplace Extension
✅ TypeScript Plugin Support

**Try it today:** `npm install -g omnisql`

---
