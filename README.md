# PhASCII OAP Starter

This repository control layer defines the initial operating state for the
PhASCII project.

PhASCII is a Rust-first image-to-ASCII transformation tool. The first
milestone is a developer-facing Rust library plus CLI that converts JPG input
into deterministic ASCII text and a rendered PNG preview. Android,
real-time camera preview, and ASCII video are future targets, not MVP
implementation scope.

## Immediate use

1. Give `AGENTS.md` to the coding subagent as the governing instruction file.
2. Give `SPEC.md`, `DECISIONS.md`, and `tasks/M0_BOOTSTRAP.md` as the first
   task context.
3. Require the subagent to return the report format defined in
   `prompts/SUBAGENT_REPORT_TEMPLATE.md`.
4. Do not allow scope expansion unless `DECISIONS.md` is updated.

## First milestone

M0 creates the repository skeleton, Rust workspace, core crate, CLI crate,
placeholder tests, and documentation gates. It does not need to implement the
full ASCII algorithm yet.

## Text hygiene

Run this before review on Markdown or governance changes:

```bash
python3 scripts/check_text_hygiene.py
```
