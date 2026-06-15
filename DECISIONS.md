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

### O002 — PNG preview text rendering method

Candidates:

- simple bitmap/font crate;
- image-only minimal renderer;
- external font dependency.

Decision needed during M2.

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
