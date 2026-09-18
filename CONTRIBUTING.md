# Contribution & Repository Structure Guidelines

To maintain a scalable and clean codebase, please adhere to the following directory structure and guidelines when adding new features, scripts, or files to the `omnisql` project.

## Guiding Principle: Keep the Root Clean
Do not dump temporary files, logs, scripts, or standalone markdown documents in the root directory. The root should be reserved strictly for critical configuration files (e.g., `Cargo.toml`, `.gitignore`, `README.md`, `CONTRIBUTING.md`).

## Directory Structure Breakdown

### 1. `src/` (Rust Source Code)
All application logic, features, and source code must go inside the `src/` directory.
- **`src/main.rs` & `src/lib.rs`**: Entry points for the application. Keep them as minimal as possible.
- **Submodules (e.g., `cli/`, `exec/`, `lexer/`, `parser/`, `semantic/`, `style/`)**: Group related functionalities into their own directories. If you are adding a completely new subsystem, create a new directory here and expose it via `mod` in `lib.rs`.

### 2. `docs/` (Documentation & Planning)
All project documentation, design plans, and progress trackers belong here.
- Add architectural notes, implementation plans, and meeting notes into this folder.
- Use Markdown (`.md`) format.

### 3. `scripts/` (SQL, DDL, and Utility Scripts)
Any standalone scripts used for database initialization, migrations, or testing belong here.
- **`.sql` and `.ddl` files**: Keep test queries and data definition scripts in this folder instead of the root.
- **Bash/PowerShell scripts**: If you write automation scripts (e.g., for building, deploying, or testing), place them here.

### 4. `test_results/` & `logs/` (Transient Output)
These directories are strictly for generated output and are ignored by `.gitignore`.
- **`test_results/`**: Output from automated testing, test reports (XML/HTML), or coverage logs.
- **`logs/`**: Standard execution logs or debug output files (`.log`).
- **DO NOT** commit files in these directories to version control.

### 5. `test_dbs/` (Test Databases)
Stores SQLite/OmniSQL test database files. Do not commit these files unless they are explicitly meant to act as seeded fixtures for test suites.

## Adding New File Types
If you introduce a entirely new type of file or workflow (e.g., Dockerfiles, CI/CD pipelines, or benchmark reports):
1. Evaluate if it fits into an existing folder.
2. If it requires a new top-level folder (like `.github/` for CI or `benchmarks/`), create the folder and update this `CONTRIBUTING.md` file to reflect the new structure.

## Committing & Pushing Guidelines (Preventing File Size Issues)
When adding new sub-projects (like WASM plugins) or committing code, you must ensure that massive build artifacts and data files are excluded from version control to prevent GitHub file size errors (100MB limit).

1. **Global Git Ignore Rules**: Ensure `.gitignore` rules apply globally where appropriate. Use `target/` instead of `/target/` to ignore compilation artifacts across *all* sub-directories (e.g., `test_plugin/target/`).
2. **Compiled Binaries**: Never commit `.exe`, `.rlib`, `.wasm`, or any other compiled binary artifacts. Ensure these file extensions are tracked in `.gitignore`.
3. **Data Files**: Do not commit large database dumps, `manifest.json` files, or CSVs into `test_dbs/` unless they are explicitly small fixtures necessary for the test suite. If they are large, mock them or generate them programmatically.
4. **Pre-push Check**: Before running `git commit -a` or `git add .`, always run `git status` to verify that untracked generated files (like `omnisql_fixes.log` or nested `target/` directories) are not accidentally being staged.
