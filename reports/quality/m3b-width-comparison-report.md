# M3B Width Comparison Report

date: 2026-06-16
git commit: c32a389232f6e39f0d8b6793c7436cea88176728
branch: docs/m3b-width-comparison
input directory: /home/axis/codex/local-input

## Image list

- `031465694c839f3d8901e9b97dadec39.jpg`
- `O_10152013114829459_1.jpeg`
- `VanGogh_1887_Selbstbildnis.jpg`
- `images.jpeg`
- `Ženski_portret.jpg`

## Widths compared

- `100`
- `120`
- `160`

## Command template

```bash
cargo run -p phascii-cli -- /home/axis/codex/local-input/<image> --width <width>
```

## Output directory

- `output/m3b-width-comparison`

## 031465694c839f3d8901e9b97dadec39.jpg

- Width `100`
  - TXT: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w100.txt`
  - PNG: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w100.png`
  - Dimensions: `816x472`
  - Observations: Recognizable. 100 is compact and loses a bit of interior logo
    detail, but the wordmark remains clear.
- Width `120`
  - TXT: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w120.txt`
  - PNG: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w120.png`
  - Dimensions: `976x556`
  - Observations: Recognizable. 120 looks balanced and keeps the logo structure
    readable without feeling cramped.
- Width `160`
  - TXT: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w160.txt`
  - PNG: `output/m3b-width-comparison/031465694c839f3d8901e9b97dadec39_w160.png`
  - Dimensions: `1296x736`
  - Observations: Recognizable. 160 adds some detail, but the larger canvas does
    not improve the result much for this simple logo.

User preference: pending

## O_10152013114829459_1.jpeg

- Width `100`
  - TXT: `output/m3b-width-comparison/O_10152013114829459_1_w100.txt`
  - PNG: `output/m3b-width-comparison/O_10152013114829459_1_w100.png`
  - Dimensions: `816x616`
  - Observations: Recognizable. 100 keeps the spoon silhouette clear, but the
    bowl and handle details are a little compressed.
- Width `120`
  - TXT: `output/m3b-width-comparison/O_10152013114829459_1_w120.txt`
  - PNG: `output/m3b-width-comparison/O_10152013114829459_1_w120.png`
  - Dimensions: `976x736`
  - Observations: Recognizable. 120 looks balanced and preserves both the spoon
    shape and the fine handle edge.
- Width `160`
  - TXT: `output/m3b-width-comparison/O_10152013114829459_1_w160.txt`
  - PNG: `output/m3b-width-comparison/O_10152013114829459_1_w160.png`
  - Dimensions: `1296x976`
  - Observations: Recognizable. 160 gives a little more edge separation, but the
    larger output is not clearly better for a single object.

User preference: pending

## VanGogh_1887_Selbstbildnis.jpg

- Width `100`
  - TXT: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w100.txt`
  - PNG: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w100.png`
  - Dimensions: `816x772`
  - Observations: Recognizable. 100 preserves the face silhouette, but some
    portrait texture is compressed.
- Width `120`
  - TXT: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w120.txt`
  - PNG: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w120.png`
  - Dimensions: `976x928`
  - Observations: Recognizable. 120 is the best balance here and keeps the face
    structure readable.
- Width `160`
  - TXT: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w160.txt`
  - PNG: `output/m3b-width-comparison/VanGogh_1887_Selbstbildnis_w160.png`
  - Dimensions: `1296x1228`
  - Observations: Recognizable. 160 adds nuance and texture, but it also becomes
    denser and heavier to inspect.

User preference: pending

## images.jpeg

- Width `100`
  - TXT: `output/m3b-width-comparison/images_w100.txt`
  - PNG: `output/m3b-width-comparison/images_w100.png`
  - Dimensions: `816x616`
  - Observations: Weakest of the three. The tree/outdoor silhouette is visible,
    but contrast and background separation are limited.
- Width `120`
  - TXT: `output/m3b-width-comparison/images_w120.txt`
  - PNG: `output/m3b-width-comparison/images_w120.png`
  - Dimensions: `976x736`
  - Observations: Still weak, but 120 makes the tree and horizon easier to read
    than 100.
- Width `160`
  - TXT: `output/m3b-width-comparison/images_w160.txt`
  - PNG: `output/m3b-width-comparison/images_w160.png`
  - Dimensions: `1296x976`
  - Observations: Best of the three for this low-contrast scene, but the preview
    is still noisy and large relative to the gain.

User preference: pending

## Ženski_portret.jpg

- Width `100`
  - TXT: `output/m3b-width-comparison/Ženski_portret_w100.txt`
  - PNG: `output/m3b-width-comparison/Ženski_portret_w100.png`
  - Dimensions: `816x952`
  - Observations: Recognizable. 100 is compact, but some facial shading and
    contour nuance are compressed.
- Width `120`
  - TXT: `output/m3b-width-comparison/Ženski_portret_w120.txt`
  - PNG: `output/m3b-width-comparison/Ženski_portret_w120.png`
  - Dimensions: `976x1132`
  - Observations: Recognizable. 120 looks balanced and preserves the face
    structure well.
- Width `160`
  - TXT: `output/m3b-width-comparison/Ženski_portret_w160.txt`
  - PNG: `output/m3b-width-comparison/Ženski_portret_w160.png`
  - Dimensions: `1296x1504`
  - Observations: Recognizable. 160 gives the most nuance, but the larger output
    is noticeably heavy for quick review.

User preference: pending

## Final summary

Which images looked most recognizable:

- The portraits and the spoon looked strong at all three widths.
- `VanGogh_1887_Selbstbildnis.jpg` and `Ženski_portret.jpg` benefited most
  from `120` or `160`.

Which images looked weakest:

- `images.jpeg` was the weakest across all widths because the source image is
  low contrast.

Whether width `100` loses too much detail:

- Sometimes. It is fine for compact terminal output, but it compresses detail
  more than is ideal for portraits and the low-contrast scene.

Whether width `120` looks balanced:

- Yes. It stayed the most balanced across the set and matched the current
  default well.

Whether width `160` improves recognizability or becomes too large/noisy:

- It improves detail for portraits and the outdoor scene, but it also makes the
  previews heavier to inspect and is not a clear win for simpler images.

Whether the preview remains practical to inspect:

- Yes at `120`.
- `160` is still practical, but it is better as an opt-in width than a default.

Preliminary engineering recommendation:

- Keep width `120` as default if it remains balanced across most images.
- Consider width `160` for portrait/detail-heavy images if it clearly improves
  recognizability.
- Consider width `100` for compact terminal use if detail loss is acceptable.
