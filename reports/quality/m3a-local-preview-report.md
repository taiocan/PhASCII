# M3A Local Preview Report

date: 2026-06-16
git commit: 8bac9468d46d63f8ebd3b3400e8f1a54e0b312b2
branch: docs/m3a-local-preview-report
command template used:
  cargo run -p phascii-cli -- /home/axis/codex/local-input/<image>.jpg --width 120
input directory: /home/axis/codex/local-input

## Selected Images

### 031465694c839f3d8901e9b97dadec39.jpg

input file name: 031465694c839f3d8901e9b97dadec39.jpg
input file path: /home/axis/codex/local-input/031465694c839f3d8901e9b97dadec39.jpg
detected file type: JPEG image data, JFIF standard 1.01, resolution (DPI), density
300x300, segment length 16, baseline, precision 8, 800x600, components 3
ASCII width used: 120
generated .txt path: output/phascii_031465694c839f3d8901e9b97dadec39_1781583530585.txt
generated .png path: output/phascii_031465694c839f3d8901e9b97dadec39_1781583530585.png
PNG dimensions: 976x556
Recognizability: pending user review
Observations:
- logo/text silhouette is visible
- contrast appears strong
- text/sign readability appears strong
- preview renderer appears stable

### O_10152013114829459_1.jpeg

input file name: O_10152013114829459_1.jpeg
input file path: /home/axis/codex/local-input/O_10152013114829459_1.jpeg
detected file type: JPEG image data, JFIF standard 1.02, aspect ratio, density
100x100, segment length 16, baseline, precision 8, 600x600, components 3
ASCII width used: 120
generated .txt path: output/phascii_O_10152013114829459_1_1781583530940.txt
generated .png path: output/phascii_O_10152013114829459_1_1781583530940.png
PNG dimensions: 976x736
Recognizability: pending user review
Observations:
- spoon silhouette is clear
- contrast appears strong
- text/sign readability is not applicable
- preview renderer appears stable

### VanGogh_1887_Selbstbildnis.jpg

input file name: VanGogh_1887_Selbstbildnis.jpg
input file path: /home/axis/codex/local-input/VanGogh_1887_Selbstbildnis.jpg
detected file type: JPEG image data, baseline, precision 8, 250x315, components 3
ASCII width used: 120
generated .txt path: output/phascii_VanGogh_1887_Selbstbildnis_1781583531285.txt
generated .png path: output/phascii_VanGogh_1887_Selbstbildnis_1781583531285.png
PNG dimensions: 976x928
Recognizability: pending user review
Observations:
- face silhouette is visible
- contrast appears moderate
- text/sign readability is not applicable
- preview renderer appears stable

### images.jpeg

input file name: images.jpeg
input file path: /home/axis/codex/local-input/images.jpeg
detected file type: JPEG image data, JFIF standard 1.01, aspect ratio, density
1x1, segment length 16, baseline, precision 8, 225x225, components 3
ASCII width used: 120
generated .txt path: output/phascii_images_1781583531583.txt
generated .png path: output/phascii_images_1781583531583.png
PNG dimensions: 976x736
Recognizability: pending user review
Observations:
- tree/outdoor silhouette is visible
- contrast appears weak
- text/sign readability is not applicable
- preview renderer appears stable

### Ženski_portret.jpg

input file name: Ženski_portret.jpg
input file path: /home/axis/codex/local-input/Ženski_portret.jpg
detected file type: JPEG image data, JFIF standard 1.01, aspect ratio, density
1x1, segment length 16, comment: "CREATOR: gd-jpeg v1.0 (using IJG JPEG v80),
quality = 90", baseline, precision 8, 454x704, components 3
ASCII width used: 120
generated .txt path: output/phascii_Ženski_portret_1781583532168.txt
generated .png path: output/phascii_Ženski_portret_1781583532168.png
PNG dimensions: 976x1132
Recognizability: pending user review
Observations:
- face silhouette is visible
- contrast appears moderate
- text/sign readability is not applicable
- preview renderer appears stable

## Final Summary

Which images looked most recognizable:
- O_10152013114829459_1.jpeg
- VanGogh_1887_Selbstbildnis.jpg
- Ženski_portret.jpg
- 031465694c839f3d8901e9b97dadec39.jpg

Which images looked weakest:
- images.jpeg

Whether width 120 seems reasonable:
- Yes. It gives clear results for the logo, spoon, and portraits.
- The low-contrast tree image is still weak, but that looks like an input
  content issue rather than a width issue alone.

Whether a future comparison of width 100 vs 120 vs 160 would be useful:
- Yes. A 3-way comparison would help decide whether slightly narrower or
  wider output improves portrait legibility without making previews too large.

Recommendation for next task:
- Compare a few width settings on the same local images before changing the
  transform algorithm.
