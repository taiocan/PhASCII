# Task M2 — Render ASCII PNG Preview

## Goal

Add rendered PNG preview output for visual review.

## Required behavior

- CLI saves `.txt` and `.png` in `output/`.
- File names are unique.
- stdout remains clean ASCII only.
- timing and saved paths go to stderr.

## Constraints

- Renderer must not be required by the core transform path unless behind feature/config boundary.
- Keep rendering simple and deterministic.
- Do not add GUI.

## Acceptance criteria

- Running `phascii input.jpg` prints ASCII and creates two files.
- PNG preview visually corresponds to the text output.
- `cargo test` passes.
