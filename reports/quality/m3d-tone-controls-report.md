# M3D Tone Controls Report

Date: 2026-06-18

Branch: feature/m3d-tone-controls-v2

## Images tested

- `031465694c839f3d8901e9b97dadec39.jpg`
- `O_10152013114829459_1.jpeg`
- `VanGogh_1887_Selbstbildnis.jpg`
- `images.jpeg`
- `Ženski_portret.jpg`

## Variants tested

- `baseline`: contrast `1.0`, gamma `1.0`
- `contrast125`: contrast `1.25`, gamma `1.0`
- `contrast150`: contrast `1.5`, gamma `1.0`
- `contrast125_gamma085`: contrast `1.25`, gamma `0.85`
- `contrast125_gamma115`: contrast `1.25`, gamma `1.15`

## Command examples

```bash
cargo run -p phascii-cli -- /home/axis/codex/local-input/images.jpeg --width 120
cargo run -p phascii-cli -- /home/axis/codex/local-input/images.jpeg --width 120 --contrast 1.25
cargo run -p phascii-cli -- /home/axis/codex/local-input/images.jpeg --width 120 --contrast 1.5
cargo run -p phascii-cli -- /home/axis/codex/local-input/images.jpeg --width 120 --contrast 1.25 --gamma 0.85
cargo run -p phascii-cli -- /home/axis/codex/local-input/images.jpeg --width 120 --contrast 1.25 --gamma 1.15
```

The same command pattern was repeated for the other local images listed above.

## Generated output directory

- `output/m3d-tone-controls/`

Generated `.txt` and `.png` files are local-only and are not committed.

## Evaluation summary

### Logo / text image

- Baseline: sign-like shapes are stable already.
- `contrast125`: slightly clearer strokes without losing ASCII feel.
- `contrast150`: edges become harsher and more posterized.
- `contrast125_gamma085`: highlights open up slightly.
- `contrast125_gamma115`: darker and less airy.

### Single object image

- Baseline: object silhouette is already readable and balanced.
- `contrast125`: gives the object slightly more background separation.
- `contrast150`: starts to look coarse.
- `contrast125_gamma085`: brightens shadows and improves the outline.
- `contrast125_gamma115`: darker than needed and flatter.

### Van Gogh portrait

- Baseline: portrait is recognizable.
- `contrast125`: best general improvement for face structure and hair edges.
- `contrast150`: more recognizable, but increasingly coarse and dense.
- `contrast125_gamma085`: lifts midtones while keeping facial detail visible.
- `contrast125_gamma115`: darker and too heavy for the portrait.

### Low-contrast outdoor image

- Baseline: weakest of the set; low contrast makes it feel washed out.
- `contrast125`: clearer separation, but still restrained.
- `contrast150`: more aggressive, but texture starts to feel harsh.
- `contrast125_gamma085`: best tested variant for this low-contrast scene.
- `contrast125_gamma115`: too dark and loses background separation.

### Ženski_portret.jpg

- Baseline: detailed portrait is readable, but midtones are flat.
- `contrast125`: improves facial outline and hair mass.
- `contrast150`: more recognizable, but compressed and noisy.
- `contrast125_gamma085`: good shadow lift without losing face shape.
- `contrast125_gamma115`: heavier and less balanced.

## Per-image observations

### `031465694c839f3d8901e9b97dadec39.jpg`

- Baseline output is already stable.
- `contrast125` is clearer without losing the ASCII feel.
- `contrast150` is too harsh for comfortable review.

### `O_10152013114829459_1.jpeg`

- Baseline output is balanced.
- `contrast125` improves separation slightly.
- `contrast125_gamma085` is the best of the tested variants here.

### `VanGogh_1887_Selbstbildnis.jpg`

- Baseline output is recognizable.
- `contrast125` gives the best general improvement.
- `contrast150` is more recognizable but more coarse.

### `images.jpeg`

- Baseline output is the weakest of the set.
- `contrast125_gamma085` is the best tested variant.
- `contrast150` feels harsh and less practical to inspect.

### `Ženski_portret.jpg`

- Baseline output is readable but flat in the midtones.
- `contrast125` improves facial outline and hair mass.
- `contrast125_gamma085` keeps detail visible while lifting shadows.

## Overall observations

- Baseline output remains stable and readable.
- `contrast 1.25` is the most generally useful mild adjustment.
- `contrast 1.5` usually becomes too coarse for comfortable review.
- `gamma 0.85` helps the weakest images most.
- `gamma 1.15` tends to darken details away.
- Tone controls are useful for quality testing.
- The default path should stay unchanged.

## User preference

User preference: pending
