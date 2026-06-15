# AGENTS.md — PhASCII Subagent Operating Rules

## Project identity

Project name: PhASCII.

PhASCII is a Rust-first image-to-ASCII transformation project. The first product is a developer-facing Rust library and CLI that converts JPG images into deterministic grayscale ASCII art and a rendered PNG preview. The original image is never modified.

The long-term target is an Android application capable of real-time camera-to-ASCII preview and ASCII video capture. That target must influence architecture, but it must not expand the MVP implementation scope.

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

Implement a clear single-threaded baseline first. Do not introduce Rayon, SIMD-specific code, edge detection, or complex dithering until the baseline is tested and visually reviewed.

### 4. Fast by measurement, not assumption

Performance claims must be backed by benchmark reports. Separate total CLI time from core transform time.

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

## Repository structure target

```text
phascii/
├── Cargo.toml
├── AGENTS.md
├── README.md
├── SPEC.md
├── DECISIONS.md
├── PLAN.md
├── QUALITY_RUBRIC.md
├── BENCHMARK_PLAN.md
├── crates/
│   ├── phascii-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs
│   │       ├── error.rs
│   │       ├── decode.rs
│   │       ├── transform.rs
│   │       ├── render.rs
│   │       └── metrics.rs
│   └── phascii-cli/
│       ├── Cargo.toml
│       └── src/main.rs
├── test-assets/
│   ├── input/
│   ├── expected/
│   └── README.md
├── reports/
│   ├── benchmarks/
│   └── quality/
└── tasks/
    ├── M0_BOOTSTRAP.md
    ├── M1_CORE_TRANSFORM.md
    ├── M2_RENDER_PREVIEW.md
    └── M3_QUALITY_COMPARISON.md
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

Every subagent response must include:

```text
Summary:
Files changed:
Tests run:
Benchmark impact:
Decisions needed:
Risks:
Next recommended task:
```

Do not hide failures. If something was not completed, say so directly.

## Forbidden behavior

- Do not add Android implementation during MVP.
- Do not add camera/video code during MVP.
- Do not add GPU code during MVP.
- Do not replace the project objective with a generic image editor.
- Do not optimize before there is a quality baseline.
- Do not silently change default width, ramp, line endings, or output behavior.
- Do not claim performance without benchmark output.
