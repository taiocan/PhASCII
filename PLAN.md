# PLAN.md — PhASCII Implementation Plan

## M0 — Bootstrap

Goal: create the repo skeleton and compile empty crates.

Deliverables:

- Rust workspace.
- `phascii-core` crate.
- `phascii-cli` crate.
- Initial config structs and error types.
- Placeholder tests.
- `cargo test` passes.

## M1 — Core text transform

Goal: convert JPG input into deterministic ASCII text.

Deliverables:

- JPG decode path.
- Resize/downsample path.
- Luma conversion.
- Adaptive normalization.
- Ramp mapping.
- `AsciiImage` output struct.
- CLI prints ASCII to stdout.
- Golden text snapshot for one test image.

## M2 — Rendered PNG preview

Goal: save rendered ASCII preview PNG.

Deliverables:

- renderer module.
- CLI saves `.txt` and `.png` in `output/`.
- unique file naming.
- visual report template.

## M3 — Quality comparison

Goal: compare baseline stylized output with experimental alternatives.

Deliverables:

- `stylized` mode.
- `dithered-experimental` mode.
- rendered comparison outputs.
- manual score report.

## M4 — Benchmark baseline

Goal: measure speed on Ubuntu WSL for one 1080p JPG.

Deliverables:

- benchmark command.
- timing breakdown.
- benchmark report committed under `reports/benchmarks/`.
