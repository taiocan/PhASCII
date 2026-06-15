# SPEC.md — PhASCII MVP Specification

## Objective

Create a simple, high-performance Rust-based image-to-ASCII transformation tool.

The MVP converts a JPG image into:

1. ASCII text printed to stdout.
2. A saved `.txt` file.
3. A rendered `.png` preview.

The output must clearly preserve the visible relationship to the original image. The first user is a developer using the CLI. Later users may be Android app users.

## Product definition

PhASCII is not a destructive image editor. It does not modify the original image. It generates ASCII-derived representations of the original image.

## MVP scope

### In scope

- Rust workspace.
- `phascii-core` library crate.
- `phascii-cli` binary crate.
- JPG input.
- Grayscale ASCII output.
- Rendered PNG preview.
- Configurable output width.
- Default width: 120 characters.
- Default dark-to-light ramp: `@%#*+=-:. `.
- Unix `\n` line endings.
- Adaptive normalization in default stylized mode.
- Deterministic output for same input/config.
- Test and benchmark report structure.

### Out of scope for MVP

- Android application.
- JNI integration.
- Real-time camera preview.
- ASCII video.
- Color ASCII.
- Batch processing.
- GPU acceleration.
- Web/WASM frontend.
- Advanced UI.
- Edge detection as default behavior.

## Visual quality requirement

The ASCII output must be recognizable as derived from the original image. The first quality target is general recognizability across faces, objects, landscapes, and text/logo-like inputs.

Default mode is `stylized`, which may use adaptive contrast normalization to improve recognizability.

Dithering is experimental until proven useful by rendered output comparison.

## Performance target

Target environment: Ubuntu WSL.

Initial benchmark image size: 1920×1080 JPG.

First performance target:

- total CLI time reported, including file read, decode, transform, text write, PNG render;
- core transform time reported separately, excluding file read and PNG render;
- no hard pass/fail threshold in M0;
- propose threshold after first benchmark report.

## Acceptance criteria

MVP is accepted when:

1. `cargo test` passes.
2. `cargo run -p phascii-cli -- input.jpg` prints ASCII to stdout.
3. The CLI creates unique `.txt` and `.png` outputs in `output/`.
4. Same input/config produces byte-identical `.txt` output.
5. A benchmark report exists for one 1080p JPG.
6. A quality report compares at least one rendered output against the original.
7. Core crate contains no Android/JNI/UI dependency.
