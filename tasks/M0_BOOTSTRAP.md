# Task M0 — Bootstrap PhASCII Workspace

## Goal

Create a compiling Rust workspace for PhASCII with a library crate and CLI crate.

## Inputs

Read:

- `AGENTS.md`
- `SPEC.md`
- `DECISIONS.md`
- `PLAN.md`

## Required changes

Create or update:

```text
Cargo.toml
crates/phascii-core/Cargo.toml
crates/phascii-core/src/lib.rs
crates/phascii-core/src/config.rs
crates/phascii-core/src/error.rs
crates/phascii-core/src/metrics.rs
crates/phascii-cli/Cargo.toml
crates/phascii-cli/src/main.rs
```

## Implementation requirements

- Workspace must compile.
- `phascii-core` must define `AsciiConfig`, `AsciiRamp`, `TransformMode`, `AsciiImage`, `TransformMetrics`, and `PhasciiError` placeholders.
- CLI may print a placeholder message in M0.
- Include at least one unit test for default config values.
- Do not implement image decoding yet unless trivial.

## Acceptance criteria

- `cargo fmt --check` passes.
- `cargo test` passes.
- `cargo run -p phascii-cli -- --help` runs or the CLI prints a clear placeholder.

## Report format

Use `prompts/SUBAGENT_REPORT_TEMPLATE.md`.
