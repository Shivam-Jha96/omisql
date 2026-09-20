# Changelog

All notable changes to this project will be documented in this file.

# Release 1.0.6

*Date: 2026-09-20*

OmniSQL 1.0.6 introduces critical documentation enhancements for our auto-fix engine and schema integrations, ensuring users understand the current boundaries of the semantic checking engine.

## 📝 Documentation
- **Schema & dbt Manifest Examples**: Added explicit command-line examples across all READMEs demonstrating how to lint SQL files using local DDL schemas (`--schema`) or dbt configurations (`--dbt-manifest`).
- **Auto-Fix Disclaimer**: Added explicit `--fix` examples alongside schema validation, supplemented with an important disclaimer clarifying that OmniSQL's auto-fix engine currently handles *stylistic* violations (casing, commas) but does *not* automatically modify semantic or security errors.

## 🛠️ Maintenance & Chores
- **Version Bumps**: Incremented versions to `1.0.6` across `Cargo.toml`, Node environments, and PyPI scripts to trigger a fresh CI/CD rollout.

---
*Thank you to everyone who contributed to this release!*

## Release 1.0.5

*Date: 2026-09-20*

OmniSQL 1.0.5 introduces dynamic versioning for our NPM and PyPI wrappers, live-updating repository badges, and a new verbose runtime logging system for the CLI.

### 🚀 Features
- **Verbose Runtime Logs**: Implemented detailed runtime debug logs directly in the CLI engine. Users can now opt-in for deeper insights during linting.
- **Instant Version Reporting**: The `-V` and `--version` flags now intercept commands instantaneously inside the JS and Python wrappers to print correct versions without invoking the underlying Rust binary.

### 🐛 Bug Fixes
- **Dynamic Installer Resolution**: Fixed an issue where Node and Python package installations could cache or point to outdated `omnisql` native binaries. Installation scripts now correctly read their dynamic package versions directly from `package.json` and `importlib.metadata`.

### 📝 Documentation
- **Usage Guides & Upgrade Commands**: Overhauled all README files across the repository, VS Code marketplace, and PyPI to include explicit CLI linting guides, auto-fix instructions, and package update commands.
- **Dynamic Version Badges**: Replaced statically hardcoded `v1.0.0` references in the documentation with live-updating version badges for NPM, PyPI, and the VS Code Marketplace.
- **Implementation Plan Sync**: Kept living documentation completely up to date with recent structural completions (Phases 11-13).

### 🛠️ Maintenance & Chores
- **Repository Sanitization**: Cleaned up dummy files, corrected `.gitignore` rules for temporary target directories, and relocated scratch files to keep the core repository pristine.
- **Version Bumps**: Bumped across `Cargo.toml`, `npm/package.json`, `python/setup.py`, and `vscode-extension/package.json` to prepare for `1.0.4` deployment.
