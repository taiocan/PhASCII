# PhASCII

PhASCII is a Rust-first image-to-ASCII project.

M2 supports JPG-to-ASCII text plus rendered PNG preview.
The 2x2 sample fixture validates the pipeline, but it does not demonstrate
visual recognizability.

Run the CLI:

```bash
cargo run -p phascii-cli -- path/to/input.jpg
```

Example:

```bash
cargo run -p phascii-cli -- test-assets/input/2x2_bw.jpg --width 2
```

The command prints ASCII text to stdout and saves matching `.txt` and `.png`
files in `output/`.

For local-only JPGs, you can run:

```bash
cargo run -p phascii-cli -- /home/axis/codex/local-input/example.jpg --width 120
```

Local preview comparison reports use the ignored `local-input/` and `output/`
directories.

## Text hygiene

Run this before review on Markdown or governance changes:

```bash
python3 scripts/check_text_hygiene.py
```

Before starting a new branch from a recommended PR, verify the exact PR head
and merge state first.
