# BENCHMARK_PLAN.md — PhASCII Benchmark Plan

## Environment

Initial target: Ubuntu WSL.

Record:

- OS details
- CPU model
- Rust version
- build profile
- input image dimensions
- input image file size
- command used

## First benchmark image

Use one 1920×1080 JPG.

## Required timing breakdown

Report these stages if possible:

- file read
- JPG decode
- resize/downsample
- luma conversion
- adaptive normalization
- character mapping
- text assembly
- `.txt` write
- PNG render
- `.png` write
- total CLI time
- core transform time

## Initial performance policy

Do not fail the build on timing in M0/M1. First collect reliable baseline data. Add thresholds only after the first working implementation.
