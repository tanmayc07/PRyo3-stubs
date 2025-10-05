# Copilot Instructions for PRyo3-Stubs

## Project Overview
- **PRyo3-Stubs** is a VS Code extension and Rust CLI tool for generating Python type stubs (`.pyi`) from single-file PyO3 Rust projects.
- The repo is split into two main components:
  - `/cli/`: Rust CLI for parsing `src/lib.rs` and generating `.pyi` output.
  - `/extension/`: VS Code extension (TypeScript) that invokes the CLI and manages file events.

## Key Architectural Patterns
- **Single File Constraint**: Only supports single-crate, single-file (`src/lib.rs`) Rust projects for MVP.
- **Type Mapping**: Rust types are mapped to Python types using explicit rules (see `PLAN.md` Section 3.2).
- **Output Location**: The generated `.pyi` file is written to the project root, named after the module.
- **No Cross-Crate Support**: Types from dependencies or other crates are not supported in MVP.

## Developer Workflows
- **Build CLI**: `cd cli && cargo build --release` (produces binary for your OS)
- **Extension Packaging**: Place built binaries in `/extension/bin/` with OS-specific names (`pryo3-stubs-cli-macos`, etc.)
- **Run Generator**: Save `src/lib.rs` in a supported project, or run the CLI directly: `./pryo3-stubs-cli-macos path/to/lib.rs`
- **Extension Activation**: Triggered on Rust file save or via the `pryo3-stubs.runGenerator` command in VS Code.

## Project-Specific Conventions
- **Rust CLI Entrypoint**: `cli/src/main.rs` (handles file I/O, error logging, and calls parser)
- **AST Parsing**: `cli/src/parser.rs` (finds `#[pyfunction]` and `#[pyclass]`)
- **Type Translation**: `cli/src/translator.rs` (maps Rust types to Python types)
- **Extension Entrypoint**: `extension/src/extension.ts` (handles OS detection, binary execution, and file events)
- **Binary Naming**: Binaries must be named per-OS and placed in `/extension/bin/` for the extension to find them.

## Integration Points
- **Rust <-> Node.js**: Extension calls the CLI binary using Node's `child_process.exec`, passing the path to `lib.rs`.
- **Error Handling**: CLI errors are surfaced in VS Code notifications.

## Examples
- To add a new Rust type mapping, update `cli/src/translator.rs` and extend the match logic.
- To support multi-file or cross-crate projects, major changes to the parser and CLI input logic are required (see `PLAN.md`).

## References
- See `PLAN.md` for detailed architecture, type mapping table, and extension orchestration pseudocode.
- See `README.md` for a high-level project summary.

---
For questions about project structure or workflows, see `PLAN.md` or ask in project discussions.
