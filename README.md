### **🎉 OmniSQL: The Next-Generation SQL Linter**

[![npm version](https://badge.fury.io/js/omnisql.svg)](https://www.npmjs.com/package/omnisql)
[![PyPI version](https://badge.fury.io/py/omnisql.svg)](https://pypi.org/project/omnisql/)
[![VS Marketplace](https://vsmarketplacebadge.apphb.com/version/omnisql.omnisql-vscode.svg)](https://marketplace.visualstudio.com/items?itemName=omnisql.omnisql-vscode)

We are incredibly excited to announce the official launch of OmniSQL!

After extensive development and stabilization, OmniSQL is ready to completely change how data engineers, analysts, and developers write SQL. Built natively in Rust, OmniSQL is not just a style checker—it is a blazingly fast, context-aware semantic engine that actually understands your database schemas, catches security risks, and estimates query costs before they ever reach production.
### 
### **🚀 Key Highlights**
**⚡ Blazing-Fast Rust Core**
Forget waiting minutes for your linter to run across your dbt project. OmniSQL's lexer and parser are written entirely in Rust, analyzing thousands of files in milliseconds.

**🧠 Semantic & Schema Awareness**
Unlike legacy linters that only look at syntax, OmniSQL natively ingests your DDL files or dbt manifest.json.

Invalid Column Detection: Catches typos and missing columns by verifying against your actual table schemas.
Ambiguity Checks: Ensures columns are properly qualified when performing complex JOINs.

**💸 Cost & Security Analysis**
In cloud data warehouses, a bad query is expensive.

**Cost Rules:** OmniSQL statically analyzes WHERE clauses and JOIN conditions, warning you about accidental Cartesian (1=1) cross-joins and enforcing filters on partitioned tables.
**Security Rules:** Automatically flags the selection of unmasked @PII columns, prevents overly permissive GRANT ALL statements, and catches dangerous DROP TABLE migrations.

**🔌 WebAssembly (WASM) Plugin Ecosystem**
Need to enforce a company-specific naming convention or business rule? You don't need to learn Rust. Write your custom rules in TypeScript, Python, or Go, compile them to WebAssembly, and run them securely and natively within OmniSQL's core.

**🛠️ Flawless Developer Experience**
We believe tooling should fade into the background. OmniSQL ships with:

**VS Code Extension (omnisql-vscode):** A native Language Server Protocol (LSP) implementation that provides real-time squiggles and diagnostics as you type.
Auto-Fixer: Run omnisql --fix to instantly resolve style violations (casing, commas, whitespace) across your entire project.
### 
### **📦 Installation**
OmniSQL distributes native binaries seamlessly. You don't need a Rust toolchain to install it—we've built pure, JIT-downloading wrappers for your favorite ecosystems.

**Via NPM (JavaScript/TypeScript Ecosystems):**

```
bash
npm install -g omnisql
```
**Via PyPI (Python/dbt Ecosystems):**

```
bash

pip install omnisql
```
**Via VS Code:**
 Search for OmniSQL in the Extensions Marketplace and click Install.

### **📖 Usage Guide**

#### **Linting Commands**
You can run OmniSQL from the command line to lint your SQL files:

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

- **Lint with a dbt manifest (for native dbt project validation):**
  ```bash
  omnisql lint --dbt-manifest target/manifest.json ./sql_queries/
  ```

#### **Upgrading OmniSQL**
To stay up to date with the latest features, performance improvements, and rule additions, upgrade OmniSQL using your package manager.

- **Via NPM:**
  ```bash
  npm update -g omnisql
  ```

- **Via PyPI:**
  ```bash
  pip install --upgrade omnisql
  ```
