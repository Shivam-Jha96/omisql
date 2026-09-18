# OmniSQL User Guide

Welcome to the OmniSQL User Guide! OmniSQL is a blazing-fast, next-generation SQL linter built in Rust. It goes beyond simple syntax checking by understanding database schemas, semantic meaning, cost estimation, and security vulnerabilities.

This guide will walk you through installing and using OmniSQL across Windows, macOS, and Linux environments.

---

## 1. Installation

OmniSQL provides native, pre-compiled binaries for all major platforms. You can install it using your preferred ecosystem package manager or download the binary directly.

### Option A: Using NPM (Recommended for Frontend/Fullstack)
If you already have Node.js installed, you can install OmniSQL globally. This works seamlessly on Windows, macOS, and Linux.

```bash
npm install -g omnisql
```

### Option B: Using pip (Recommended for Data Engineers / Python)
If you are working in a Python ecosystem (e.g., alongside `dbt`), you can install the wrapper via `pip`:

```bash
pip install omnisql
```

### Option C: Direct Binary Download
You can download the standalone binary directly from our [GitHub Releases](https://github.com/Shivam-Jha96/omnisql/releases) page.

1. Download the archive for your OS:
   - **Windows**: `omnisql-vX.Y.Z-x86_64-windows.zip`
   - **macOS (Apple Silicon)**: `omnisql-vX.Y.Z-aarch64-macos.tar.gz`
   - **macOS (Intel)**: `omnisql-vX.Y.Z-x86_64-macos.tar.gz`
   - **Linux**: `omnisql-vX.Y.Z-x86_64-linux.tar.gz`
2. Extract the archive.
3. Move the `omnisql` (or `omnisql.exe`) binary to a folder in your system's `PATH`.

---

## 2. Getting Started: Linting a Query

By default, OmniSQL can run basic syntax and style checks without any schema knowledge. 

To lint a specific SQL file:
```bash
omnisql lint path/to/your/query.sql
```

To automatically fix basic style violations (like capitalization and trailing commas):
```bash
omnisql lint path/to/your/query.sql --fix
```

---

## 3. Advanced Linting (Schema Awareness)

To unlock OmniSQL's true power (semantic validation, detecting missing columns, ambiguous joins, and type mismatches), you must provide it with your database schema.

### Method 1: Using dbt (Data Build Tool)
If you use dbt, OmniSQL can read your compiled `manifest.json` directly. This requires zero extra configuration!

```bash
omnisql lint models/my_model.sql --dbt-manifest target/manifest.json
```

### Method 2: Raw DDL Files
You can export your database schema as a raw DDL file (e.g., `CREATE TABLE ...`) and pass it to OmniSQL. 

```bash
# Provide the schema file directly
omnisql lint query.sql --schema path/to/schema.sql
```

**Note on Cost and Security Rules:**
If using raw DDL, you can annotate your columns for advanced checks:
- Add `COMMENT '@PII'` to a column to trigger security warnings if unmasked.
- Add `COMMENT '@PARTITION'` to ensure queries filter on that column to reduce cloud warehouse costs.

Example DDL:
```sql
CREATE TABLE users (
    id INT,
    email VARCHAR COMMENT '@PII',
    created_at TIMESTAMP COMMENT '@PARTITION'
);
```

---

## 4. Writing Custom Rules (WASM Plugins)

OmniSQL supports custom linting rules written in any language that compiles to WebAssembly (WASI), such as TypeScript, Go, or Rust. 

To run a query against a custom WASM plugin:
```bash
omnisql lint query.sql --schema schema.sql --plugin my_custom_rule.wasm
```
*Plugins communicate with the OmniSQL engine by reading the JSON-serialized AST from standard input and returning lint violations to standard output.*

---

## 5. IDE Integration (VS Code)

OmniSQL includes a built-in Language Server Protocol (LSP) engine, providing real-time squiggles and diagnostics directly in your editor as you type!

1. Search for **"OmniSQL"** in the VS Code Extensions Marketplace.
2. Install the `omnisql-vscode` extension.
3. Ensure the `omnisql` CLI is installed and available in your system `PATH` (e.g., via `npm install -g omnisql`).
4. The extension will automatically boot the LSP server when you open a `.sql` file.

*To start the LSP server manually from the command line (for other editors like Neovim or IntelliJ), run:*
```bash
omnisql lsp
```
