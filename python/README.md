# OmniSQL Python Wrapper

This is the Python wrapper for OmniSQL, a blazing-fast SQL linter built in Rust.

For full documentation, visit the [main repository](https://github.com/Shivam-Jha96/omnisql).

## Usage Guide

### Linting Commands
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

### Upgrading OmniSQL
To stay up to date with the latest features, performance improvements, and rule additions, you can upgrade OmniSQL via PyPI:

```bash
pip install --upgrade omnisql
```
