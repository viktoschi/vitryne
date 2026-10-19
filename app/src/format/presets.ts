// SPDX-License-Identifier: AGPL-3.0-or-later
// Standard fabric presets (M9, docs/FABRICS.md): 5 textile presets calibrated against
// textile mechanics literature (KES-F / FAST data), and 8 latex sheet presets from CLO's
// measured Radical Rubber profile.

import type { Attribution, Fabric } from "./schema.gen";

export const ID_COTTON_JERSEY = "01a13cdb-cc03-7001-8000-000000000001";
export const ID_DENIM = "01a13cdb-cc03-7001-8000-000000000002";
export const ID_POPLIN = "01a13cdb-cc03-7001-8000-000000000003";
export const ID_FLEECE = "01a13cdb-cc03-7001-8000-000000000004";
export const ID_WOOL_SUITING = "01a13cdb-cc03-7001-8000-000000000005";
export const ID_LATEX_0_25 = "01a13cdb-cc03-7001-8000-000000000006";
export const ID_LATEX_0_4 = "01a13cdb-cc03-7001-8000-000000000007";
export const ID_LATEX_1_0 = "01a13cdb-cc03-7001-8000-000000000008";
export const ID_LATEX_2X_0_25 = "01a13cdb-cc03-7001-8000-000000000009";
export const ID_LATEX_0_4_0_25 = "01a13cdb-cc03-7001-8000-00000000000a";
export const ID_LATEX_2X_0_4 = "01a13cdb-cc03-7001-8000-00000000000b";
export const ID_LATEX_3X_0_4 = "01a13cdb-cc03-7001-8000-00000000000c";
export const ID_LATEX_2X_1_0 = "01a13cdb-cc03-7001-8000-00000000000d";

export const COTTON_JERSEY: Fabric = {
  id: ID_COTTON_JERSEY,
  name: "Cotton jersey",
  category: "knit",
  density: 0.15,
  thickness: 0.45,
  stretch: {
    warp: 14_000,
    weft: 9_000,
    shear: 3_500,
  },
  bending: {
    warp: 6.0e-6,
    weft: 4.0e-6,
  },
  friction: 0.3,
  notes:
    "100% cotton single jersey knit, 150 g/m², typical everyday T-shirt. Soft, compliant drape following body contours. Kawabata (1980) Table 2.2; Miguel et al. (2012).",
};

export const DENIM: Fabric = {
  id: ID_DENIM,
  name: "Denim",
  category: "woven",
  density: 0.42,
  thickness: 0.85,
  stretch: {
    warp: 1_400_000,
    weft: 1_000_000,
    shear: 400_000,
  },
  bending: {
    warp: 1.4e-4,
    weft: 1.0e-4,
  },
  friction: 0.42,
  notes:
    "Heavy 3/1 right-hand twill cotton denim, 420 g/m² (12.5 oz), 5-pocket jeans weight. Stiff, holds shape away from the body in sturdy folds. Kawabata (1980); Breen et al. (1994).",
};

export const POPLIN: Fabric = {
  id: ID_POPLIN,
  name: "Poplin (shirting)",
  category: "woven",
  density: 0.125,
  thickness: 0.22,
  stretch: {
    warp: 1_100_000,
    weft: 800_000,
    shear: 300_000,
  },
  bending: {
    warp: 1.8e-5,
    weft: 1.2e-5,
  },
  friction: 0.25,
  notes:
    "Lightweight combed cotton plain-weave poplin shirting, 125 g/m² (3.7 oz). Crisp hand, fine sharp folds and ripples. Wang et al. (2011) Table 2; FAST-2 standards.",
};

export const FLEECE: Fabric = {
  id: ID_FLEECE,
  name: "Fleece",
  category: "knit",
  density: 0.28,
  thickness: 2.2,
  stretch: {
    warp: 28_000,
    weft: 18_000,
    shear: 7_500,
  },
  bending: {
    warp: 7.5e-5,
    weft: 5.5e-5,
  },
  friction: 0.38,
  notes:
    "Double-faced brushed polyester knit fleece, 280 g/m². Lofty and thick (2.2 mm), with rounded, bulky folds and moderate knit give. Kawabata (1980); FAST-1.",
};

export const WOOL_SUITING: Fabric = {
  id: ID_WOOL_SUITING,
  name: "Wool suiting",
  category: "woven",
  density: 0.25,
  thickness: 0.48,
  stretch: {
    warp: 850_000,
    weft: 650_000,
    shear: 200_000,
  },
  bending: {
    warp: 3.2e-5,
    weft: 2.4e-5,
  },
  friction: 0.32,
  notes:
    "Worsted wool 2/2 twill tailored suiting, 250 g/m² (7.5 oz). Resilient, fluid drape, holding clean creases without limp sagging. Kawabata (1980) Table 2.1; Minami et al. (2002).",
};

/** How every latex preset is credited: the measured CLO profile is CC BY-SA 4.0. */
const LATEX_ATTRIBUTION: Attribution = {
  title: "CLO latex fabric profiles",
  creator: "tabjy",
  license: "CC-BY-SA-4.0",
  source: "https://github.com/tabjy/clo3d-patterns",
  notes:
    "Latex_0.4mm_Radical_Rubber measured with CLO's Fabric Kit; the other thicknesses and laminates are derived from it.",
};

/** Natural rubber on skin, dry. CLO's raw 0.03 (shown as 3) is its default, not a measurement. */
const LATEX_FRICTION = 0.8;

const LATEX_MEASURED =
  "Measured: CLO Fabric Kit profile 'Latex_0.4mm_Radical_Rubber' (Radical Rubber, tabjy/clo3d-patterns).";
const LATEX_DERIVED =
  "Derived from the measured 0.40 mm Radical Rubber profile: density, stretch and shear scale with thickness, bending with thickness³; fully glued laminate, not measured.";

/** A latex sheet preset: isotropic, with the same bending on the bias, and the shared attribution. */
function latexSheet(
  id: string,
  name: string,
  thickness: number,
  density: number,
  stretch: number,
  shear: number,
  bending: number,
  provenance: string,
): Fabric {
  return {
    id,
    name,
    category: "nonwoven",
    density,
    thickness,
    stretch: {
      warp: stretch,
      weft: stretch,
      shear,
    },
    bending: {
      warp: bending,
      weft: bending,
      bias: bending,
    },
    friction: LATEX_FRICTION,
    notes: `${provenance} Natural rubber latex sheet, ${thickness.toFixed(2)} mm, ${Math.round(density * 1000)} g/m². Isotropic: no grain direction, so warp = weft. CLO 5.2.334 loads the measured profile unchanged; CLO's nonlinear curves are all factor 1.0. Friction ${LATEX_FRICTION} is a modelling estimate for dry latex on skin (CLO's 0.03 is its default, not a measurement).`,
    attribution: LATEX_ATTRIBUTION,
  };
}

/** 0.25 mm natural rubber latex sheet, derived from the measured 0.40 mm profile. */
export const LATEX_0_25: Fabric = latexSheet(
  ID_LATEX_0_25,
  "Latex 0.25 mm",
  0.25,
  0.21812,
  217.671,
  108.849,
  6.06e-7,
  LATEX_DERIVED,
);

/** 0.40 mm natural rubber latex sheet: the one measured CLO profile (Radical Rubber). */
export const LATEX_0_4: Fabric = latexSheet(
  ID_LATEX_0_4,
  "Latex 0.4 mm (Radical Rubber)",
  0.4,
  0.34899,
  348.274,
  174.159,
  2.482e-6,
  LATEX_MEASURED,
);

/** 1.0 mm natural rubber latex sheet, derived from the measured 0.40 mm profile. */
export const LATEX_1_0: Fabric = latexSheet(
  ID_LATEX_1_0,
  "Latex 1.0 mm",
  1.0,
  0.87248,
  870.685,
  435.398,
  3.8785e-5,
  LATEX_DERIVED,
);

/** Two fully glued 0.25 mm sheets: 0.50 mm. */
export const LATEX_2X_0_25: Fabric = latexSheet(
  ID_LATEX_2X_0_25,
  "Latex laminate 2 × 0.25 mm",
  0.5,
  0.43624,
  435.343,
  217.699,
  4.848e-6,
  LATEX_DERIVED,
);

/** A fully glued 0.40 mm and 0.25 mm sheet: 0.65 mm. */
export const LATEX_0_4_0_25: Fabric = latexSheet(
  ID_LATEX_0_4_0_25,
  "Latex laminate 0.4 + 0.25 mm",
  0.65,
  0.56711,
  565.946,
  283.008,
  1.0651e-5,
  LATEX_DERIVED,
);

/** Two fully glued 0.40 mm sheets: 0.80 mm. */
export const LATEX_2X_0_4: Fabric = latexSheet(
  ID_LATEX_2X_0_4,
  "Latex laminate 2 × 0.4 mm",
  0.8,
  0.69798,
  696.548,
  348.318,
  1.9858e-5,
  LATEX_DERIVED,
);

/** Three fully glued 0.40 mm sheets: 1.20 mm. */
export const LATEX_3X_0_4: Fabric = latexSheet(
  ID_LATEX_3X_0_4,
  "Latex laminate 3 × 0.4 mm",
  1.2,
  1.04697,
  1044.822,
  522.477,
  6.7021e-5,
  LATEX_DERIVED,
);

/** Two fully glued 1.0 mm sheets: 2.0 mm. */
export const LATEX_2X_1_0: Fabric = latexSheet(
  ID_LATEX_2X_1_0,
  "Latex laminate 2 × 1.0 mm",
  2.0,
  1.74495,
  1741.371,
  870.795,
  3.1028e-4,
  LATEX_DERIVED,
);

export const LATEX_PRESETS: readonly Fabric[] = [
  LATEX_0_25,
  LATEX_0_4,
  LATEX_1_0,
  LATEX_2X_0_25,
  LATEX_0_4_0_25,
  LATEX_2X_0_4,
  LATEX_3X_0_4,
  LATEX_2X_1_0,
] as const;

export const FABRIC_PRESETS: readonly Fabric[] = [
  COTTON_JERSEY,
  DENIM,
  POPLIN,
  FLEECE,
  WOOL_SUITING,
  ...LATEX_PRESETS,
] as const;

export function findPreset(idOrName: string): Fabric | undefined {
  return FABRIC_PRESETS.find(
    (p) => p.id === idOrName || p.name.toLowerCase() === idOrName.toLowerCase(),
  );
}

/** Returns the preset matching the given fabric values closely, if any. */
export function matchingPreset(f: Fabric): Fabric | undefined {
  return FABRIC_PRESETS.find((p) => {
    return (
      p.category === f.category &&
      Math.abs(p.density - f.density) < 1e-4 &&
      Math.abs(p.stretch.warp - f.stretch.warp) < 1e-2 &&
      Math.abs(p.stretch.weft - f.stretch.weft) < 1e-2 &&
      Math.abs(p.bending.warp - f.bending.warp) < 1e-8 &&
      Math.abs(p.bending.weft - f.bending.weft) < 1e-8
    );
  });
}
