# M3C Visual Width Decision

date: 2026-06-16
git commit: 24a5ac8c4c435122a48920f1d1dbbe9d7bec1cd6
branch: docs/m3c-visual-width-decision
input image set:
- 031465694c839f3d8901e9b97dadec39.jpg
- O_10152013114829459_1.jpeg
- VanGogh_1887_Selbstbildnis.jpg
- images.jpeg
- Ženski_portret.jpg
widths compared: 100, 120, 160
local review index: output/m3c-width-review/index.html
decision status: pending user visual review

| Image | Best width | Notes | User confirmed |
|---|---:|---|---|
| 031465694c839f3d8901e9b97dadec39.jpg | pending | pending user review | no |
| O_10152013114829459_1.jpeg | pending | pending user review | no |
| VanGogh_1887_Selbstbildnis.jpg | pending | pending user review | no |
| images.jpeg | pending | pending user review | no |
| Ženski_portret.jpg | pending | pending user review | no |

Preliminary engineering recommendation:
- Keep width 120 as the default until user visual review confirms otherwise.
- Treat width 160 as a likely detail mode for portraits and high-detail images.
- Treat width 100 as a compact mode only if the user accepts detail loss.
- Do not change the algorithm based on width comparison alone.
