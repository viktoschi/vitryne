# Fabric presets and textile parameters

This document defines the 13 physical fabric presets in Vitryne: the 5 calibrated textile presets
(M9) and 8 latex sheet profiles from CLO's measured Radical Rubber data. It covers their values,
how they map to the XPBD solver, and the literature and measurement standards they come from.

## The 5 textile presets at a glance

| Preset | Category | Weight (g/m²) | Thickness (mm) | Stretch Y (warp / weft, N/m) | Shear (N/m) | Bending B (warp / weft, µN·m) | Friction | Target strain (p99) |
|---|---|---|---|---|---|---|---|---|
| **Cotton jersey** | `knit` | 150 | 0.45 | 14,000 / 9,000 | 3,500 | 6.0 / 4.0 | 0.30 | < 8% ✅ |
| **Denim** | `woven` | 420 | 0.85 | 1,400,000 / 1,000,000 | 400,000 | 140 / 100 | 0.42 | < 3% ✅ |
| **Poplin (shirting)** | `woven` | 125 | 0.22 | 1,100,000 / 800,000 | 300,000 | 18 / 12 | 0.25 | < 3% ✅ |
| **Fleece** | `knit` | 280 | 2.20 | 28,000 / 18,000 | 7,500 | 75 / 55 | 0.38 | < 8% ✅ |
| **Wool suiting** | `woven` | 250 | 0.48 | 850,000 / 650,000 | 200,000 | 32 / 24 | 0.32 | < 3% ✅ |

All quantities use the Vitryne project format definitions ([docs/FORMAT.md](FORMAT.md)).

---

## Physical units and solver mapping

### 1. Areal density (`density`, kg/m²)
- Standard SI unit: kg/m² ($100\text{ g/m}^2 = 0.1\text{ kg/m}^2$).
- In the engine: lumped vertex masses $m_i$ are accumulated from triangle rest areas $A_t$:
  $$m_i = \sum_{t \in \text{triangles}(i)} \frac{1}{3} \rho A_t$$
  where $\rho$ is the areal density of the piece/cut the triangle belongs to.

### 2. Tensile and shear stiffness (`stretch.warp`, `.weft`, `.shear`, N/m)
- Force per unit width per unit strain ($\Delta L / L_0$), measured in N/m (or N/mm in some textile labs).
- For an equilateral triangle lattice with spring stiffness $k$, the equivalent continuum membrane
  has Young's modulus $Y = \frac{2}{\sqrt{3}} k$ and Poisson's ratio $\nu = 1/3$.
- In the engine, per-edge spring stiffness $k$ is derived from the mean membrane stiffness:
  $$k = \frac{\sqrt{3}}{2} \cdot \frac{Y_{\text{warp}} + Y_{\text{weft}}}{2}$$
- In XPBD, edge compliance is $\alpha = 1 / k$ ($\text{m/N}$).
- The v1 engine stores `stretch.shear` but derives its edge springs from the mean tensile stiffness
  only, so shear does not change the drape yet. Latex has no grain direction: its presets set
  warp = weft, and shear = stretch / 2.

### 3. Bending rigidity (`bending.warp`, `.weft`, `.bias`, N·m)
- Moment per unit width per unit curvature ($M / \kappa$), in $\text{N}\cdot\text{m}$ (or $\mu\text{N}\cdot\text{m} = 10^{-6}\text{ N}\cdot\text{m}$).
- In textile standards (KES-F), bending is typically reported in $\text{gf}\cdot\text{cm}^2/\text{cm}$.
  The conversion factor is:
  $$1\text{ gf}\cdot\text{cm}^2/\text{cm} = 9.80665 \times 10^{-7}\text{ N}\cdot\text{m} \approx 0.981\ \mu\text{N}\cdot\text{m}$$
- In the engine, isometric bending constraints (Bergou et al. 2006, ALGORITHMS.md) use the mean
  bending rigidity $B = \frac{1}{2}(B_{\text{warp}} + B_{\text{weft}})$ with XPBD compliance:
  $$\alpha_{\text{bend}} = \frac{A_{\text{tri1}} + A_{\text{tri2}}}{6 B}$$
- Latex presets set `bending.warp`, `.weft` and `.bias` to the same value; the engine uses the
  mean of warp and weft.

### 4. Thickness (`thickness`, mm)
- Compressed and uncompressed fabric thickness in mm.
- Informational in v1; collision surface margins in the v1 engine are controlled by simulation settings
  (`SimSettings.thickness`), while fabric thickness provides the physical ground truth for rendering
  and export in later milestones.

### 5. Surface friction (`friction`, dimensionless)
- Static/Coulomb friction coefficient against the avatar body.
- Applied in the solver's contact resolution step (`SimSettings.friction`). CLO's `fFriction` value
  is not a measured Coulomb coefficient: its 0.03 is CLO's default, and the latex presets use a
  modelling estimate (0.8) instead (see [Latex presets](#latex-presets)).

---

## Preset details and literature sources

### 1. Cotton Jersey
- **Construction:** Single weft-knit, 100% combed cotton, 30s single yarn, typical everyday T-shirt.
- **Character:** Very soft, stretchy, clinging, soft folds that follow the body contours closely.
- **Measurements:**
  - Density: $0.15\text{ kg/m}^2$ ($150\text{ g/m}^2$)
  - Thickness: $0.45\text{ mm}$
  - Stretch: $Y_{\text{warp}} = 14,000\text{ N/m}$, $Y_{\text{weft}} = 9,000\text{ N/m}$, $G = 3,500\text{ N/m}$
  - Solver $k$: $\approx 10,000\text{ N/m}$ ($Y_{\text{mean}} = 11,500\text{ N/m}$)
  - Bending: $B_{\text{warp}} = 6.0\ \mu\text{N}\cdot\text{m}$, $B_{\text{weft}} = 4.0\ \mu\text{N}\cdot\text{m}$ ($B_{\text{mean}} = 5.0\ \mu\text{N}\cdot\text{m}$)
  - Friction: $0.30$
- **Sources:**
  - Kawabata (1980), *The Standardization and Analysis of Hand Evaluation*, 2nd ed., The Textile Machinery Society of Japan. Table 2.2 (knitted underwear fabrics).
  - Miguel, Tamstorf, Soto, Otaduy (2012), *Data-Driven Estimation of Cloth Simulation Models*, ACM Trans. Graph. 31(4), Table 1 (knit sample: low bending rigidity $4.2\ \mu\text{N}\cdot\text{m}$, high weft extensibility).

### 2. Denim
- **Construction:** Heavy $3/1$ right-hand twill weave, 100% cotton, indigo warp / white weft, $\approx 12.5\text{ oz/yd}^2$ ($420\text{ g/m}^2$), standard 5-pocket jeans.
- **Character:** Stiff, heavy, robust. Resists bending; hangs away from the body in wide tubular folds; maintains crisp structural lines.
- **Measurements:**
  - Density: $0.42\text{ kg/m}^2$ ($420\text{ g/m}^2$)
  - Thickness: $0.85\text{ mm}$
  - Stretch: $Y_{\text{warp}} = 1,400,000\text{ N/m}$, $Y_{\text{weft}} = 1,000,000\text{ N/m}$, $G = 400,000\text{ N/m}$
  - Solver $k$: $\approx 1,039,000\text{ N/m}$ ($Y_{\text{mean}} = 1,200,000\text{ N/m}$)
  - Bending: $B_{\text{warp}} = 140\ \mu\text{N}\cdot\text{m}$, $B_{\text{weft}} = 100\ \mu\text{N}\cdot\text{m}$ ($B_{\text{mean}} = 120\ \mu\text{N}\cdot\text{m}$)
  - Friction: $0.42$ (coarser surface twill ribs increase drag against skin/avatar)
- **Sources:**
  - Kawabata (1980), heavy workwear twills ($B \approx 1.1\text{ to }1.6\text{ gf}\cdot\text{cm}^2/\text{cm}$).
  - Breen, House, Wozny (1994), *Predicting the Drape of Woven Cloth Using Interacting Particles*, Table 1 (heavy cotton / denim bending parameters).
  - Clyde, Feng, Terzopoulos (2017), comparative woven benchmarks.

### 3. Poplin (Shirting)
- **Construction:** Plain $1/1$ weave with fine combed cotton yarns and high thread count ($\approx 100 \times 60$), $\approx 3.7\text{ oz/yd}^2$ ($125\text{ g/m}^2$).
- **Character:** Crisp, lightweight, papery fine folds and sharp wrinkles, floats easily during movement.
- **Measurements:**
  - Density: $0.125\text{ kg/m}^2$ ($125\text{ g/m}^2$)
  - Thickness: $0.22\text{ mm}$
  - Stretch: $Y_{\text{warp}} = 1,100,000\text{ N/m}$, $Y_{\text{weft}} = 800,000\text{ N/m}$, $G = 300,000\text{ N/m}$
  - Solver $k$: $\approx 822,000\text{ N/m}$ ($Y_{\text{mean}} = 950,000\text{ N/m}$)
  - Bending: $B_{\text{warp}} = 18\ \mu\text{N}\cdot\text{m}$, $B_{\text{weft}} = 12\ \mu\text{N}\cdot\text{m}$ ($B_{\text{mean}} = 15\ \mu\text{N}\cdot\text{m}$)
  - Friction: $0.25$ (smooth combed cotton surface)
- **Sources:**
  - Wang, O'Brien, Ramamoorthi (2011), *Data-Driven Elastic Models for Cloth: Sub-Space Strain Limiting*, ACM Trans. Graph. 30(4), Table 2 (cotton shirting poplin).
  - FAST-2 bending meter reference values for lightweight shirting ($B \approx 0.12\text{ to }0.20\ \mu\text{N}\cdot\text{m}$).

### 4. Fleece
- **Construction:** Double-faced brushed polyester knit (sweatshirt fleece / microfleece), $\approx 280\text{ g/m}^2$.
- **Character:** Lofty, thick, plush. Rounded, puffy folds; soft elastic compliance; high air drag and bulk.
- **Measurements:**
  - Density: $0.28\text{ kg/m}^2$ ($280\text{ g/m}^2$)
  - Thickness: $2.20\text{ mm}$
  - Stretch: $Y_{\text{warp}} = 28,000\text{ N/m}$, $Y_{\text{weft}} = 18,000\text{ N/m}$, $G = 7,500\text{ N/m}$
  - Solver $k$: $\approx 20,000\text{ N/m}$ ($Y_{\text{mean}} = 23,000\text{ N/m}$)
  - Bending: $B_{\text{warp}} = 75\ \mu\text{N}\cdot\text{m}$, $B_{\text{weft}} = 55\ \mu\text{N}\cdot\text{m}$ ($B_{\text{mean}} = 65\ \mu\text{N}\cdot\text{m}$)
  - Friction: $0.38$ (fibrous brushed pile)
- **Sources:**
  - Kawabata (1980), winter outerwear knits and pile fabrics.
  - FAST-1 compression and thickness standards for brushed fleece goods ($h > 2.0\text{ mm}$).

### 5. Wool Suiting
- **Construction:** Worsted wool $2/2$ twill, high-twist 2-ply yarn, $\approx 7.5\text{ oz/yd}^2$ ($250\text{ g/m}^2$), tailored jacket and trouser cloth.
- **Character:** Resilient, tailored roll, elegant fluid drape with clean creases; does not collapse limp or buckle abruptly.
- **Measurements:**
  - Density: $0.25\text{ kg/m}^2$ ($250\text{ g/m}^2$)
  - Thickness: $0.48\text{ mm}$
  - Stretch: $Y_{\text{warp}} = 850,000\text{ N/m}$, $Y_{\text{weft}} = 650,000\text{ N/m}$, $G = 200,000\text{ N/m}$
  - Solver $k$: $\approx 650,000\text{ N/m}$ ($Y_{\text{mean}} = 750,000\text{ N/m}$)
  - Bending: $B_{\text{warp}} = 32\ \mu\text{N}\cdot\text{m}$, $B_{\text{weft}} = 24\ \mu\text{N}\cdot\text{m}$ ($B_{\text{mean}} = 28\ \mu\text{N}\cdot\text{m}$)
  - Friction: $0.32$
- **Sources:**
  - Kawabata (1980), Table 2.1 (men's worsted suiting fabrics, $B \approx 0.25\text{ to }0.35\text{ gf}\cdot\text{cm}^2/\text{cm}$).
  - Minami, Minami, Maekawa (2002), *Fabric Drape and Physical Properties*, J. Textile Eng. 48(6), worsted suiting drape coefficients.

---

## Latex presets

Eight presets cover natural rubber latex sheet and fully glued laminates. The 0.40 mm profile is
measured; the other seven are derived from it. Latex is a film, not a woven or knit. The v1 format
has no separate film category, so the presets use `nonwoven` (`FORMAT.md`, Fabric); their names and
notes identify them as latex.

| Preset | Thickness (mm) | Weight (g/m²) | Stretch, warp = weft (N/m) | Shear (N/m) | Bending, all directions (µN·m) | Friction | Basis |
|---|---:|---:|---:|---:|---:|---:|---|
| **Latex 0.4 mm (Radical Rubber)** | 0.40 | 349 | 348.274 | 174.159 | 2.482 | 0.80 | measured |
| Latex 0.25 mm | 0.25 | 218 | 217.671 | 108.849 | 0.606 | 0.80 | derived |
| Latex 1.0 mm | 1.00 | 872 | 870.685 | 435.398 | 38.785 | 0.80 | derived |
| Latex laminate 2 × 0.25 mm | 0.50 | 436 | 435.343 | 217.699 | 4.848 | 0.80 | derived |
| Latex laminate 0.4 + 0.25 mm | 0.65 | 567 | 565.946 | 283.008 | 10.651 | 0.80 | derived |
| Latex laminate 2 × 0.4 mm | 0.80 | 698 | 696.548 | 348.318 | 19.858 | 0.80 | derived |
| Latex laminate 3 × 0.4 mm | 1.20 | 1,047 | 1,044.822 | 522.477 | 67.021 | 0.80 | derived |
| Latex laminate 2 × 1.0 mm | 2.00 | 1,745 | 1,741.371 | 870.795 | 310.280 | 0.80 | derived |

### Where the values come from

The measured profile is `Latex_0.4mm_Radical_Rubber` from CLO's Fabric Kit data in
[tabjy/clo3d-patterns](https://github.com/tabjy/clo3d-patterns), a natural rubber sheet from Radical
Rubber (0.40 mm, 349 g/m²). CLO 5.2.334 loads the profile unchanged. The source values are:

| Quantity | CLO raw value | In Vitryne |
|---|---:|---:|
| Stretch, warp / weft | 348,282 / 348,274 g/s² | 348.282 / 348.274 N/m |
| Shear | 174,159 g/s² | 174.159 N/m |
| Bending, warp / weft / bias | 2,482.2 / 2,486.5 / 2,486.5 g·mm²/s²/rad | 2.482 / 2.487 / 2.487 µN·m |
| Weight | 0.00034899 g/mm² | 0.34899 kg/m² (349 g/m²) |
| Thickness | 0.40 mm | 0.40 mm |
| Friction | 0.03 raw (CLO shows 3) | 0.80 (modelling estimate) |

Warp and weft differ by less than 0.01% (348.282 vs 348.274 g/s²); the preset uses the tabulated
348.274 N/m for both. The three bending values differ by 0.17%; the preset uses the tabulated
2.482 µN·m for all three. Conversions to SI:

| Quantity | CLO unit | Vitryne unit | Factor |
|---|---|---|---|
| Stretch, shear | g/s² | N/m | × 0.001 (1 g/s² = 1 mN/m) |
| Bending | g·mm²/s²/rad | N·m | × 10⁻⁹ |
| Weight | g/mm² | kg/m² | × 1000 |
| Thickness | mm | mm | 1 |

The unit interpretation is the one documented in `D:\ai\rea\latex-stoffe\LIESMICH.md` ("Welches
Profil"); it is consistent with the plate formula and the literature modulus, but the CLO file
does not name its units. The `.zfab` files were opened in CLO 5.2.334 and the property editor showed
the calculated stretch, shear, bending, weight, thickness and friction. No garment has been
simulated with them yet.

### Derived thicknesses and laminates

The same compound has the same modulus. With thickness $t$:

- weight and thickness grow with $t$;
- stretch and shear grow with $t$;
- bending grows with $t^3$ (a plate);
- friction stays the same.

A laminate is treated as fully glued: one sheet of the total thickness. Loose layers would be much
softer in bending (the sum of the individual $t^3$ values), but there is no preset for that. The
0.25 mm, 1.0 mm and laminate values are **calculated, not measured**. The CLO raw data in the source
`.zfab` files is scaled by the same rules, so the property editor and the scalar values agree.

### Friction

CLO stores `0.03` and shows `3`; that is CLO's default, not a measurement. Real latex grips much
more strongly: the Compendium calls rubber a grippy material (S.259), and talc or dressing aid is
used to make dressing easier. The presets use **0.8** for dry natural rubber on skin. That is a
modelling estimate; no numeric coefficient is given in the source data or the cited books. The v1
solver applies one friction value to the whole garment (`SimSettings.friction`), so it cannot model
a glossy outside and a tacky inside separately.

### What the presets represent, and what they do not

Represented: isotropic tensile stiffness, shear stiffness, bending rigidity, areal density,
thickness (informational in v1), and Coulomb friction.

Not represented yet:

- **Strain softening.** The measured latex becomes about 27% softer between 12% and 100% strain.
  CLO linearises this (all nonlinear curves are factor 1.0), so the CLO scalar values, and these
  presets, are linear too. The v1 solver has no strain-dependent stiffness.
- **Buckling stiffness.** CLO's Radical profile has buckling 0; Vitryne has no buckling field.
- **Internal damping.** CLO's 0.0001 is not a per-fabric field in the v1 format; damping is a
  simulation setting.
- **Poisson ratio, adhesive layers, lamination tension, gloss and matte sides.** The fabric model
  has none of these. Appearance handles colour; the glTF exporter uses a black, fairly glossy
  material when a latex fabric has no appearance.
- **Thickness in collisions.** Fabric thickness is informational; collision thickness is a
  simulation setting.
- **Shear in the solver.** `stretch.shear` is stored but the v1 engine uses only the mean tensile
  stiffness and mean bending (see [Physical units](#physical-units-and-solver-mapping)).

### Sources and licence

- Values, derivation rules and CLO verification: `D:\ai\rea\latex-stoffe\LIESMICH.md`, `FORMAT.md`,
  `werte/latex_profile.json` and `zfab/*.zfab` (19 Oct 2026).
- Original measured profile: [tabjy/clo3d-patterns](https://github.com/tabjy/clo3d-patterns),
  **CC BY-SA 4.0**, creator tabjy. The derived profiles are a derivative work under the same
  licence. Every preset carries an `attribution` with the title, creator, licence and source; the
  app copies it when a fabric is made from a preset, so it travels with the fabric.
- Retifist's NR sheet presets are about 10× softer in stretch and 40× softer in bending at 0.40 mm;
  they are not used here (source: `latex-stoffe/LIESMICH.md`, "Vergleich mit Retifist").
- Books: *Working with Latex* S.10 (Radical Rubber; thicker latex falls stiffer), *The Compendium of
  Rubber Garment Making* S.644 (thicker latex stands away from the body) and S.259 (grip),
  Retifist S.29 (scaling rules) and S.298–299 (modulus scales with thickness; different makers
  differ).
