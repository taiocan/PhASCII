# AGENTS.md - PhASCII Subagent Operating Rules

## Project identity

Project name: PhASCII.

PhASCII is a Rust-first image-to-ASCII transformation project.
The first product is a developer-facing Rust library and CLI that converts JPG
images into deterministic grayscale ASCII art and a rendered PNG preview.
The original image is never modified.

The long-term target is an Android application capable of real-time
camera-to-ASCII preview and ASCII video capture.
That target must influence architecture, but it must not expand the MVP
implementation scope.

## Continuous workflow

Every work order follows this sequence:

```text
work order
-> feature/docs/chore branch
-> bounded implementation
-> local checks
-> PR or PR-ready diff
-> CI
-> diff-based report
-> strategic review
-> user merge decision
```

The implementation subagent must keep the task bounded, report honestly, and stop short of merge approval.

## Authority model

User:
- Owns final product decisions.
- Owns final merge decisions.
- Only the user may merge PRs.

Strategic agent:
- Reviews PR summary, diff, CI report, tests, risks, and sample output.
- May recommend: `Recommend merge`, `Request changes`, or `Reject PR`.
- Does not replace the user final decision.

Implementation subagent:
- Implements one bounded work order.
- Creates a branch and PR or PR-ready diff.
- Reports honestly.
- Must not merge.
- Must not say `merge approved`.

## Branch naming

Use a focused branch name that matches the task type:

```text
feature/m2-png-preview
fix/m1-output-newline
docs/agents-workflow
chore/ci-checkout-v6
```

Prefer `feature/`, `fix/`, `docs/`, or `chore/` prefixes. Keep the name short and specific.

## Pull request requirements

Every PR description should include:

```text
Summary
Scope
Diff stat
Tests run
CI status
Known limitations
Risks
Sample output, if relevant
Strategic review requested
```

Do not merge the PR from the implementation subagent.

## Diff-based reporting

Required commands for every non-trivial work order:

```bash
git diff --stat main...HEAD
git diff --name-status main...HEAD
```

For strategic review, provide either a PR link or a patch:

```bash
git diff main...HEAD
```

For large diffs, provide focused diffs:

```bash
git diff main...HEAD -- .github/workflows/ci.yml
git diff main...HEAD -- AGENTS.md
git diff main...HEAD -- crates/phascii-core/src
git diff main...HEAD -- crates/phascii-cli/src
git diff main...HEAD -- README.md
git diff main...HEAD -- PLAN.md
git diff main...HEAD -- DECISIONS.md
```

## Required final subagent report format

Every final response must use exactly this structure:

```text
Summary:
Branch:
PR title:
PR link or local PR status:
Diff stat:
Name-status diff:
Files changed:
Tests run:
CI status:
Sample output:
Benchmark impact:
Decisions needed:
Risks:
Merge recommendation from subagent:
Strategic review requested:
Next recommended task:
```

For `Merge recommendation from subagent`, allowed values are only:

```text
Ready for strategic review
Needs more work before review
Blocked
```

## Current milestone

Current milestone: M0/M1 foundation.

Build the project in small, verifiable steps:

1. Rust workspace skeleton.
2. `phascii-core` library crate.
3. `phascii-cli` binary crate.
4. Config and output data structures.
5. JPG input path.
6. Deterministic ASCII text output.
7. Rendered PNG preview output.
8. Basic tests.
9. Benchmark/report structure.

## Non-negotiable MVP constraints

- JPG input only.
- ASCII text output and rendered PNG preview.
- Developer-first CLI.
- Library-first core.
- Rust core must not depend on Android, Kotlin, JNI, UI frameworks, or file-system assumptions.
- Default width is 120 characters.
- Internal and file line endings use Unix `\n`.
- Default ramp is dark-to-light: `@%#*+=-:. `.
- Adaptive normalization is enabled in the default stylized mode.
- Dithering is experimental and must be compared before becoming default.
- Edge detection is deferred unless explicitly authorized.
- No real-time camera, video, Android UI, JNI, GPU, color ASCII, or batch processing in MVP.

## Design principles

### 1. Library-first

The core transformation API must be reusable from CLI, future Android bindings, future WASM, and future real-time frame processing.

Do not put CLI-specific or Android-specific logic inside `phascii-core`.

### 2. Deterministic output

The same input bytes and same config must produce byte-identical ASCII text.

Snapshot tests are allowed and encouraged.

### 3. Quality before parallelism

Implement a clear single-threaded baseline first.
Do not introduce Rayon, SIMD-specific code, edge detection, or complex
dithering until the baseline is tested and visually reviewed.

### 4. Fast by measurement, not assumption

Performance claims must be backed by benchmark reports.
Separate total CLI time from core transform time.

Minimum timing breakdown:

- file read
- JPG decode
- resize/downsample
- luma conversion
- normalization
- ASCII mapping
- text assembly
- PNG rendering

### 5. Small task packets

Assume the coding subagent is capable but should receive bounded tasks. Prefer changes that touch fewer files and have explicit acceptance criteria.

Do not perform large rewrites without first updating `PLAN.md` and `DECISIONS.md`.

## Decision logging

Update `PLAN.md` when milestone scope or sequencing changes.
Update `DECISIONS.md` when a material implementation choice is made.
Do not silently change defaults, output behavior, or target architecture.

## Quality gates

Before a PR is presented for strategic review:

- The work must remain within the work order scope.
- Tests must pass locally for the touched area.
- CI must be defined or updated when the task requires it.
- Sample output must be shown when the task produces user-visible text or artifacts.
- Risks and limitations must be stated directly.
- The branch must be ready for review without additional hidden work.

## Local checks

Run the smallest relevant checks for the task.

Required for docs-only work:

```bash
git diff --check
```

If code or workflow files were touched, also run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

If a check cannot run locally, say why and report the limitation clearly.

## CI requirements

When a work order touches code or workflow files, CI should verify formatting, workspace build, tests, and Clippy unless the work order explicitly narrows the scope.
Prefer minimal, least-privilege workflows.
Keep the workflow aligned with the current branch and PR expectations.

## Text hygiene

Governance files must remain reviewable as plain text.

AGENTS.md must be ASCII-only, LF-only, free of bidi Unicode control
characters, and free of collapsed long structural lines.

Before requesting strategic review for Markdown or governance changes, run:

```bash
python3 scripts/check_text_hygiene.py
git diff --check
```

When GitHub UI warnings conflict with byte-level checks, use the exact PR head
SHA and raw GitHub payload as the source of truth.

## Benchmark reporting

Benchmark claims require measurement.
Do not claim speed improvements without timing output.
If a work order is not about performance, state `Benchmark impact: not measured`.
When benchmarks are relevant, separate CLI time from core transform time and record the environment.

## Handling blocked work

If progress is blocked by missing inputs, tool failures, or repo state:

- Say exactly what is blocked.
- Do not invent completion.
- Do not merge.
- Do not claim strategic approval.
- Stop only after the blockage is real and repeatable, or after you have a clear user-visible question.

## Merge discipline

The implementation subagent must never merge into `main`.
The strategic agent may recommend one of `Recommend merge`, `Request changes`, or `Reject PR`.
Only the user may make the final merge decision.
Do not use the phrase `merge approved`.

## Repository structure target

```text
phascii/
|- Cargo.toml
|- AGENTS.md
|- README.md
|- SPEC.md
|- DECISIONS.md
|- PLAN.md
|- QUALITY_RUBRIC.md
|- BENCHMARK_PLAN.md
|- crates/
|  |- phascii-core/
|  |  |- Cargo.toml
|  |  `- src/
|  |     |- lib.rs
|  |     |- config.rs
|  |     |- error.rs
|  |     |- decode.rs
|  |     |- transform.rs
|  |     |- render.rs
|  |     `- metrics.rs
|  `- phascii-cli/
|     |- Cargo.toml
|     `- src/main.rs
|- test-assets/
|  |- input/
|  |- expected/
|  `- README.md
|- reports/
|  |- benchmarks/
|  `- quality/
`- tasks/
   |- M0_BOOTSTRAP.md
   |- M1_CORE_TRANSFORM.md
   |- M2_RENDER_PREVIEW.md
   `- M3_QUALITY_COMPARISON.md
```

## Core API target

The exact implementation may evolve, but the design should move toward this shape:

```rust
pub struct AsciiConfig {
    pub width: u32,
    pub aspect_correction: f32,
    pub invert: bool,
    pub mode: TransformMode,
    pub ramp: AsciiRamp,
}

pub enum TransformMode {
    Stylized,
    Faithful,
    DitheredExperimental,
}

pub struct AsciiImage {
    pub width_chars: u32,
    pub height_chars: u32,
    pub cells: Vec<u8>,
    pub text: String,
    pub metrics: TransformMetrics,
}

pub fn jpg_bytes_to_ascii(bytes: &[u8], config: &AsciiConfig) -> Result<AsciiImage, PhasciiError>;

pub fn rgba_to_ascii(
    rgba: &[u8],
    width: u32,
    height: u32,
    config: &AsciiConfig,
) -> Result<AsciiImage, PhasciiError>;

pub fn luma_to_ascii(
    luma: &[u8],
    width: u32,
    height: u32,
    config: &AsciiConfig,
) -> Result<AsciiImage, PhasciiError>;
```

## CLI behavior target

Command:

```bash
phascii input.jpg
```

Required behavior:

1. Print ASCII text to stdout.
2. Save a unique `.txt` file in `output/`.
3. Save a unique `.png` rendered preview in `output/`.
4. Print timing and file paths to stderr.
5. Exit non-zero on error.

Output naming pattern:

```text
output/phascii_<timestamp>_<short_hash>.txt
output/phascii_<timestamp>_<short_hash>.png
```

## Feature flags

Use feature flags from the beginning, but keep them simple.

Initial proposal:

```toml
[features]
default = ["std", "jpg", "png-preview"]
std = []
jpg = []
png-preview = []
parallel = []
android-jni = []
wasm = []
```

Rules:

- `android-jni` must remain empty or placeholder until Android work starts.
- Do not implement WASM in MVP.
- Do not enable `parallel` by default until benchmarks justify it.

## Dependency policy

Prefer pure Rust dependencies with permissive licenses.

A non-pure-Rust dependency is allowed only if:

1. it provides a measured performance benefit,
2. it is portable to Ubuntu WSL and future Android,
3. it does not complicate the build substantially,
4. its license is compatible,
5. the decision is recorded in `DECISIONS.md`.

## Output format required from subagent

Use the required final subagent report format above. Do not hide failures. If something was not completed, say so directly.

## Forbidden behavior

- Do not add Android implementation during MVP.
- Do not add camera/video code during MVP.
- Do not add GPU code during MVP.
- Do not replace the project objective with a generic image editor.
- Do not optimize before there is a quality baseline.
- Do not silently change default width, ramp, line endings, or output behavior.
- Do not claim performance without benchmark output.
