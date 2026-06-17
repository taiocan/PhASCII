# DECISIONS.md — PhASCII Decision Log

## Accepted decisions

### D001 — Project name

Decision: Use `PhASCII`.

### D002 — First user

Decision: Developer first, end user later.

### D003 — MVP output

Decision: Produce both ASCII text and rendered PNG preview.

Rationale: Text is the true core output. PNG preview makes visual review stable across fonts, terminals, and editors.

### D004 — Input format

Decision: JPG only for MVP.

Rationale: Keeps decode and test scope narrow. PNG/WebP may be added later.

### D005 — Default width

Decision: Default output width is 120 characters.

Rationale: More recognizable than 80, less unwieldy than 160.

### D006 — Line endings

Decision: Use Unix `\n` internally and in files.

Rationale: Deterministic snapshots and cross-platform diffs.

### D007 — CLI output behavior

Decision: CLI prints ASCII to stdout and stores `.txt` plus `.png` in `output/` using unique names.

Rationale: Supports both piping and persistent artifacts.

### D008 — Default ramp

Decision: Use dark-to-light ramp `@%#*+=-:. `.

Rationale: Short, readable, stable, includes space for highlights.

### D009 — Default visual mode

Decision: Default mode is stylized with adaptive normalization.

Rationale: Recognizability matters more than strict luminance fidelity for MVP.

### D010 — Edge detection

Decision: Defer edge detection.

Rationale: It can over-stylize and should not be added before baseline quality is established.

### D011 — Parallelism

Decision: Start single-threaded. Add Rayon only after visual baseline and benchmark evidence.

### D012 — SIMD

Decision: No hand-written SIMD in MVP. SIMD-capable dependencies may be considered later if measured bottlenecks justify them.

### D013 — Android

Decision: Android is a likely future implementation target, but not part of MVP.

### D014 — Future real-time goal

Decision: Long-term architecture must support raw frame input for future camera preview and ASCII video.

## Open decisions

### O001 — JPG decode crate

Candidates:

- `image` crate with JPEG support.
- Dedicated JPEG decoder crate.

Decision needed after initial prototype.

### D015 — JPG decode crate selection

Decision: Use the `image` crate with JPEG-only features for M1.

Rationale: It keeps the first decode path small, portable, and easy to wire into the workspace.

### D016 — PNG preview renderer dependency

Decision: Use `font8x8` for deterministic bitmap glyphs and the `image` crate
PNG encoder for preview output.

Rationale: A tiny bitmap font keeps the renderer pure Rust and reproducible
without system font discovery. Reusing the existing `image` crate keeps PNG
encoding simple and portable.

Dependency details:

- dependency name: `font8x8`
- purpose: built-in bitmap glyphs for ASCII preview rendering
- license, if known: not checked in this work order
- why it is needed: the preview must render text into stable PNG pixels
- why simpler alternatives were not used: hand-writing a complete font table
  would be larger and harder to maintain
- Android-future compatibility risk, if any: low; it is pure Rust and does not
  rely on system fonts

### D017 — Experimental tone controls

Decision: Add optional contrast and gamma controls for quality testing, but
keep the default output unchanged.

Rationale: Low-contrast images may benefit from small tone adjustments during
evaluation, while the default path must remain stable for users and tests.

Dependency details:

- dependency name: none
- purpose: optional tone adjustment in the existing ASCII transform pipeline
- license, if known: not applicable
- why it is needed: evaluate whether weak images become more recognizable
- why simpler alternatives were not used: the existing pipeline needs a small,
  explicit tone hook rather than a separate mode
- Android-future compatibility risk, if any: low; the change is pure Rust and
  keeps the default path unchanged

### O002 — PNG preview text rendering method

Candidates:

- simple bitmap/font crate;
- image-only minimal renderer;
- external font dependency.

Resolved by D016.

### O003 — Snapshot framework

Candidates:

- `insta`;
- custom expected `.txt` files.

Decision needed during M1.

### O004 — Android bridge later

Candidates:

- handwritten JNI;
- UniFFI;
- C ABI plus wrapper.

Decision deferred until Android prototype phase.
