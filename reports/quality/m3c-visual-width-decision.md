# M3C Visual Width Decision

date: 2026-06-16
git commit: 5a9d91c6a25b0d9a540abf3e9e0e4aa1d57f7d2b
branch: docs/m3c-record-width-decision
input image set:
- 031465694c839f3d8901e9b97dadec39.jpg
- O_10152013114829459_1.jpeg
- VanGogh_1887_Selbstbildnis.jpg
- images.jpeg
- Ženski_portret.jpg
widths compared: 100, 120, 160
local review index: output/m3c-width-review/index.html
user decision summary:
  Width 160 is the most recognizable compared with the original image, but it
  reduces the visible ASCII-art character of the output.
  Width 120 is less maximally recognizable, but preserves the ASCII-art feel
  better.
  Therefore, keep 120 as the default for now and document 160 as the
  detail/recognizability width.
decision status: width 120 kept as default; 160 is detail mode; 100 is not the
main preview width

| Image | Best width | Notes | User confirmed |
|---|---:|---|---|
| 031465694c839f3d8901e9b97dadec39.jpg | 120 | User selected 120 |
| O_10152013114829459_1.jpeg | 120 | User selected 120 |
| VanGogh_1887_Selbstbildnis.jpg | 160 | User selected 160 |
| images.jpeg | 160 | User selected 160 |
| Ženski_portret.jpg | pending / not explicitly selected | User did not explicitly provide fifth selection |

Engineering conclusion:
- Keep width 120 as the default for now.
- Treat width 160 as the detail/recognizability option.
- Do not recommend width 100 as the main preview width.

Next recommendation:
- Use width 160 only when the user wants maximum recognizability and accepts
  a less ASCII-like preview.
