# PhASCII

PhASCII is a Rust-first image-to-ASCII project.

M1 supports JPG-to-ASCII text only.
PNG preview is planned for M2.
The 2x2 sample fixture validates the pipeline, but it does not demonstrate visual recognizability.

Run the CLI:

```bash
cargo run -p phascii-cli -- path/to/input.jpg
```

## Text hygiene

Run this before review on Markdown or governance changes:

```bash
python3 scripts/check_text_hygiene.py
```

Before starting a new branch from a recommended PR, verify the exact PR head
and merge state first.
