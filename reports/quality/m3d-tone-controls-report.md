# M3D Tone Controls Report

Date: 2026-06-17
Git commit: b3e2d0feea3421e5684b1c24213778405c561381
Branch: feature/m3d-tone-controls

## Images Tested

- `031465694c839f3d8901e9b97dadec39.jpg`
- `O_10152013114829459_1.jpeg`
- `VanGogh_1887_Selbstbildnis.jpg`
- `images.jpeg`
- `Ženski_portret.jpg`

## Variants Tested

- `baseline`: contrast `1.0`, gamma `1.0`
- `contrast125`: contrast `1.25`, gamma `1.0`
- `contrast150`: contrast `1.5`, gamma `1.0`
- `contrast125_gamma085`: contrast `1.25`, gamma `0.85`
- `contrast125_gamma115`: contrast `1.25`, gamma `1.15`

## Commands Used

- `cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width 120`
- `cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width 120 --contrast 1.25`
- `cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width 120 --contrast 1.5`
- `cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width 120 --contrast 1.25 --gamma 0.85`
- `cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width 120 --contrast 1.25 --gamma 1.15`

## Generated Output Directory

- `output/m3d-tone-controls/`

## Results

### 031465694c839f3d8901e9b97dadec39.jpg

- baseline
  - txt: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_baseline.txt`
  - png: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_baseline.png`
  - observation: sign-like shapes are stable already; mild contrast helps edges.
- contrast125
  - txt: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125.txt`
  - png: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125.png`
  - observation: slightly clearer strokes without losing the ASCII feel.
- contrast150
  - txt: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast150.txt`
  - png: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast150.png`
  - observation: edges become harsher and more posterized.
- contrast125_gamma085
  - txt: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125_gamma085.txt`
  - png: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125_gamma085.png`
  - observation: highlights open up a bit; good if the sign feels too dark.
- contrast125_gamma115
  - txt: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125_gamma115.txt`
  - png: `output/m3d-tone-controls/031465694c839f3d8901e9b97dadec39_contrast125_gamma115.png`
  - observation: darker and less airy; usable, but not the clearest.

### O_10152013114829459_1.jpeg

- baseline
  - txt: `output/m3d-tone-controls/O_10152013114829459_1_baseline.txt`
  - png: `output/m3d-tone-controls/O_10152013114829459_1_baseline.png`
  - observation: object silhouette is already readable and balanced.
- contrast125
  - txt: `output/m3d-tone-controls/O_10152013114829459_1_contrast125.txt`
  - png: `output/m3d-tone-controls/O_10152013114829459_1_contrast125.png`
  - observation: gives the object slightly more separation from the background.
- contrast150
  - txt: `output/m3d-tone-controls/O_10152013114829459_1_contrast150.txt`
  - png: `output/m3d-tone-controls/O_10152013114829459_1_contrast150.png`
  - observation: starts to look coarse; detail becomes more blocky.
- contrast125_gamma085
  - txt: `output/m3d-tone-controls/O_10152013114829459_1_contrast125_gamma085.txt`
  - png: `output/m3d-tone-controls/O_10152013114829459_1_contrast125_gamma085.png`
  - observation: brightens shadows and improves the object outline.
- contrast125_gamma115
  - txt: `output/m3d-tone-controls/O_10152013114829459_1_contrast125_gamma115.txt`
  - png: `output/m3d-tone-controls/O_10152013114829459_1_contrast125_gamma115.png`
  - observation: darker than needed and a bit flatter.

### VanGogh_1887_Selbstbildnis.jpg

- baseline
  - txt: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_baseline.txt`
  - png: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_baseline.png`
  - observation: portrait is recognizable; baseline already holds structure.
- contrast125
  - txt: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125.txt`
  - png: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125.png`
  - observation: best general improvement for face structure and hair edges.
- contrast150
  - txt: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast150.txt`
  - png: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast150.png`
  - observation: more recognizable, but increasingly coarse and dense.
- contrast125_gamma085
  - txt: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125_gamma085.txt`
  - png: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125_gamma085.png`
  - observation: lifts midtones and keeps facial detail visible.
- contrast125_gamma115
  - txt: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125_gamma115.txt`
  - png: `output/m3d-tone-controls/VanGogh_1887_Selbstbildnis_contrast125_gamma115.png`
  - observation: darker and a little too heavy for the portrait.

### images.jpeg

- baseline
  - txt: `output/m3d-tone-controls/images_baseline.txt`
  - png: `output/m3d-tone-controls/images_baseline.png`
  - observation: weakest of the set; low contrast makes it feel washed out.
- contrast125
  - txt: `output/m3d-tone-controls/images_contrast125.txt`
  - png: `output/m3d-tone-controls/images_contrast125.png`
  - observation: clearer separation, but still restrained.
- contrast150
  - txt: `output/m3d-tone-controls/images_contrast150.txt`
  - png: `output/m3d-tone-controls/images_contrast150.png`
  - observation: more aggressive, but texture starts to feel harsh.
- contrast125_gamma085
  - txt: `output/m3d-tone-controls/images_contrast125_gamma085.txt`
  - png: `output/m3d-tone-controls/images_contrast125_gamma085.png`
  - observation: best of the tested variants for this low-contrast scene.
- contrast125_gamma115
  - txt: `output/m3d-tone-controls/images_contrast125_gamma115.txt`
  - png: `output/m3d-tone-controls/images_contrast125_gamma115.png`
  - observation: too dark; loses some background separation.

### Ženski_portret.jpg

- baseline
  - txt: `output/m3d-tone-controls/Ženski_portret_baseline.txt`
  - png: `output/m3d-tone-controls/Ženski_portret_baseline.png`
  - observation: detailed portrait is readable, but midtones are a little flat.
- contrast125
  - txt: `output/m3d-tone-controls/Ženski_portret_contrast125.txt`
  - png: `output/m3d-tone-controls/Ženski_portret_contrast125.png`
  - observation: improves the facial outline and the hair mass.
- contrast150
  - txt: `output/m3d-tone-controls/Ženski_portret_contrast150.txt`
  - png: `output/m3d-tone-controls/Ženski_portret_contrast150.png`
  - observation: more recognizable, but also more compressed and noisy.
- contrast125_gamma085
  - txt: `output/m3d-tone-controls/Ženski_portret_contrast125_gamma085.txt`
  - png: `output/m3d-tone-controls/Ženski_portret_contrast125_gamma085.png`
  - observation: good lift in shadows without losing the face shape.
- contrast125_gamma115
  - txt: `output/m3d-tone-controls/Ženski_portret_contrast125_gamma115.txt`
  - png: `output/m3d-tone-controls/Ženski_portret_contrast125_gamma115.png`
  - observation: heavier and less balanced than the brighter variant.

## Summary

- Baseline output remains stable and readable.
- `contrast 1.25` is the most generally useful mild adjustment.
- `contrast 1.5` usually becomes too coarse for comfortable review.
- `gamma 0.85` helps the weakest images the most.
- `gamma 1.15` tends to darken details away.
- Tone controls are useful for quality testing, but the default path should
  stay unchanged.

## User Preference

User preference: pending
