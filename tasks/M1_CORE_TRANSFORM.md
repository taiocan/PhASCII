# Task M1 — Implement Deterministic JPG to ASCII Text

## Goal

Implement the first real core transformation path from JPG bytes to ASCII text.

## Required behavior

- Accept JPG input.
- Convert to grayscale/luma.
- Resize/downsample to target width, default 120.
- Apply adaptive normalization.
- Map brightness to the default ramp `@%#*+=-:. `.
- Use Unix `\n` line endings.
- Return an `AsciiImage` struct.
- CLI prints ASCII to stdout.

## Constraints

- No Rayon yet.
- No edge detection.
- Dithering may be omitted in this task.
- No Android/JNI code.

## Acceptance criteria

- `cargo test` passes.
- Same input/config produces identical output.
- CLI can process one JPG and print ASCII.
- Add at least one snapshot or expected-output test.
