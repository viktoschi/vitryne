// SPDX-License-Identifier: AGPL-3.0-or-later
//! Standard fabric presets (M9, docs/FABRICS.md): 5 physical textile presets covering knit and
//! woven garments, calibrated against textile literature (KES-F / FAST data), and 8 latex sheet
//! presets from CLO's measured Radical Rubber profile.

use serde_json::Map;

use crate::{
    id::Id,
    model::{Attribution, Bending, Category, Fabric, Stretch},
};

pub const ID_COTTON_JERSEY: &str = "01a13cdb-cc03-7001-8000-000000000001";
pub const ID_DENIM: &str = "01a13cdb-cc03-7001-8000-000000000002";
pub const ID_POPLIN: &str = "01a13cdb-cc03-7001-8000-000000000003";
pub const ID_FLEECE: &str = "01a13cdb-cc03-7001-8000-000000000004";
pub const ID_WOOL_SUITING: &str = "01a13cdb-cc03-7001-8000-000000000005";
pub const ID_LATEX_0_25: &str = "01a13cdb-cc03-7001-8000-000000000006";
pub const ID_LATEX_0_4: &str = "01a13cdb-cc03-7001-8000-000000000007";
pub const ID_LATEX_1_0: &str = "01a13cdb-cc03-7001-8000-000000000008";
pub const ID_LATEX_2X_0_25: &str = "01a13cdb-cc03-7001-8000-000000000009";
pub const ID_LATEX_0_4_0_25: &str = "01a13cdb-cc03-7001-8000-00000000000a";
pub const ID_LATEX_2X_0_4: &str = "01a13cdb-cc03-7001-8000-00000000000b";
pub const ID_LATEX_3X_0_4: &str = "01a13cdb-cc03-7001-8000-00000000000c";
pub const ID_LATEX_2X_1_0: &str = "01a13cdb-cc03-7001-8000-00000000000d";

/// All latex presets use the same friction: dry natural rubber on skin. CLO's raw 0.03 (shown as
/// 3) is its default, not a measurement; this is a modelling estimate, not a measured value.
const LATEX_FRICTION: f64 = 0.8;

const LATEX_MEASURED: &str = "Measured: CLO Fabric Kit profile 'Latex_0.4mm_Radical_Rubber' \
                              (Radical Rubber, tabjy/clo3d-patterns).";
const LATEX_DERIVED: &str = "Derived from the measured 0.40 mm Radical Rubber profile: density, \
                             stretch and shear scale with thickness, bending with thickness³; fully \
                             glued laminate, not measured.";

/// 100% cotton single jersey knit (T-shirt fabric). Soft, stretchy, clinging.
pub fn cotton_jersey() -> Fabric {
    Fabric {
        id: Id::parse(ID_COTTON_JERSEY).unwrap(),
        name: "Cotton jersey".into(),
        category: Category::Knit,
        density: 0.15,
        thickness: Some(0.45),
        stretch: Stretch {
            warp: 14_000.0,
            weft: 9_000.0,
            shear: 3_500.0,
            other: Map::new(),
        },
        bending: Bending {
            warp: 6.0e-6,
            weft: 4.0e-6,
            bias: None,
            other: Map::new(),
        },
        friction: 0.30,
        appearance: None,
        notes: Some(
            "100% cotton single jersey knit, 150 g/m², typical everyday T-shirt. Soft, compliant \
             drape following body contours. Kawabata (1980) Table 2.2; Miguel et al. (2012)."
                .into(),
        ),
        attribution: None,
        library_ref: None,
        other: Map::new(),
    }
}

/// Heavy 3/1 cotton twill denim (12.5 oz/yd²). Stiff, robust, wide tubular folds.
pub fn denim() -> Fabric {
    Fabric {
        id: Id::parse(ID_DENIM).unwrap(),
        name: "Denim".into(),
        category: Category::Woven,
        density: 0.42,
        thickness: Some(0.85),
        stretch: Stretch {
            warp: 1_400_000.0,
            weft: 1_000_000.0,
            shear: 400_000.0,
            other: Map::new(),
        },
        bending: Bending {
            warp: 1.4e-4,
            weft: 1.0e-4,
            bias: None,
            other: Map::new(),
        },
        friction: 0.42,
        appearance: None,
        notes: Some(
            "Heavy 3/1 right-hand twill cotton denim, 420 g/m² (12.5 oz), 5-pocket jeans weight. \
             Stiff, holds shape away from the body in sturdy folds. Kawabata (1980); Breen et al. (1994)."
                .into(),
        ),
        attribution: None,
        library_ref: None,
        other: Map::new(),
    }
}

/// Lightweight plain-weave combed cotton shirting. Crisp, papery fine wrinkles.
pub fn poplin() -> Fabric {
    Fabric {
        id: Id::parse(ID_POPLIN).unwrap(),
        name: "Poplin (shirting)".into(),
        category: Category::Woven,
        density: 0.125,
        thickness: Some(0.22),
        stretch: Stretch {
            warp: 1_100_000.0,
            weft: 800_000.0,
            shear: 300_000.0,
            other: Map::new(),
        },
        bending: Bending {
            warp: 1.8e-5,
            weft: 1.2e-5,
            bias: None,
            other: Map::new(),
        },
        friction: 0.25,
        appearance: None,
        notes: Some(
            "Lightweight combed cotton plain-weave poplin shirting, 125 g/m² (3.7 oz). Crisp hand, \
             fine sharp folds and ripples. Wang et al. (2011) Table 2; FAST-2 standards."
                .into(),
        ),
        attribution: None,
        library_ref: None,
        other: Map::new(),
    }
}

/// Double-faced brushed polyester sweatshirt fleece. Thick, warm, lofty rounded folds.
pub fn fleece() -> Fabric {
    Fabric {
        id: Id::parse(ID_FLEECE).unwrap(),
        name: "Fleece".into(),
        category: Category::Knit,
        density: 0.28,
        thickness: Some(2.20),
        stretch: Stretch {
            warp: 28_000.0,
            weft: 18_000.0,
            shear: 7_500.0,
            other: Map::new(),
        },
        bending: Bending {
            warp: 7.5e-5,
            weft: 5.5e-5,
            bias: None,
            other: Map::new(),
        },
        friction: 0.38,
        appearance: None,
        notes: Some(
            "Double-faced brushed polyester knit fleece, 280 g/m². Lofty and thick (2.2 mm), with \
             rounded, bulky folds and moderate knit give. Kawabata (1980); FAST-1."
                .into(),
        ),
        attribution: None,
        library_ref: None,
        other: Map::new(),
    }
}

/// Worsted wool 2/2 twill suiting cloth. Tailored weight, resilient smooth rolls.
pub fn wool_suiting() -> Fabric {
    Fabric {
        id: Id::parse(ID_WOOL_SUITING).unwrap(),
        name: "Wool suiting".into(),
        category: Category::Woven,
        density: 0.25,
        thickness: Some(0.48),
        stretch: Stretch {
            warp: 850_000.0,
            weft: 650_000.0,
            shear: 200_000.0,
            other: Map::new(),
        },
        bending: Bending {
            warp: 3.2e-5,
            weft: 2.4e-5,
            bias: None,
            other: Map::new(),
        },
        friction: 0.32,
        appearance: None,
        notes: Some(
            "Worsted wool 2/2 twill tailored suiting, 250 g/m² (7.5 oz). Resilient, fluid drape, \
             holding clean creases without limp sagging. Kawabata (1980) Table 2.1; Minami et al. (2002)."
                .into(),
        ),
        attribution: None,
        library_ref: None,
        other: Map::new(),
    }
}

/// The values of one latex sheet preset.
struct LatexSpec {
    id: &'static str,
    name: &'static str,
    thickness: f64,
    density: f64,
    stretch: f64,
    shear: f64,
    bending: f64,
    provenance: &'static str,
}

/// A latex sheet preset: isotropic, with the same bending on the bias, and the shared attribution.
fn latex_sheet(spec: LatexSpec) -> Fabric {
    let LatexSpec {
        id,
        name,
        thickness,
        density,
        stretch,
        shear,
        bending,
        provenance,
    } = spec;
    Fabric {
        id: Id::parse(id).unwrap(),
        name: name.into(),
        category: Category::Nonwoven,
        density,
        thickness: Some(thickness),
        stretch: Stretch {
            warp: stretch,
            weft: stretch,
            shear,
            other: Map::new(),
        },
        bending: Bending {
            warp: bending,
            weft: bending,
            bias: Some(bending),
            other: Map::new(),
        },
        friction: LATEX_FRICTION,
        appearance: None,
        notes: Some(format!(
            "{provenance} Natural rubber latex sheet, {thickness:.2} mm, {:.0} g/m². Isotropic: no \
             grain direction, so warp = weft. CLO 5.2.334 loads the measured profile unchanged; \
             CLO's nonlinear curves are all factor 1.0. Friction {LATEX_FRICTION} is a modelling \
             estimate for dry latex on skin (CLO's 0.03 is its default, not a measurement).",
            density * 1000.0
        )),
        attribution: Some(Attribution {
            title: Some("CLO latex fabric profiles".into()),
            creator: Some("tabjy".into()),
            license: "CC-BY-SA-4.0".into(),
            source: Some("https://github.com/tabjy/clo3d-patterns".into()),
            notes: Some(
                "Latex_0.4mm_Radical_Rubber measured with CLO's Fabric Kit; the other thicknesses \
                 and laminates are derived from it."
                    .into(),
            ),
            other: Map::new(),
        }),
        library_ref: None,
        other: Map::new(),
    }
}

/// 0.25 mm natural rubber latex sheet, derived from the measured 0.40 mm profile.
pub fn latex_0_25() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_0_25,
        name: "Latex 0.25 mm",
        thickness: 0.25,
        density: 0.21812,
        stretch: 217.671,
        shear: 108.849,
        bending: 6.06e-7,
        provenance: LATEX_DERIVED,
    })
}

/// 0.40 mm natural rubber latex sheet: the one measured CLO profile (Radical Rubber).
pub fn latex_0_4() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_0_4,
        name: "Latex 0.4 mm (Radical Rubber)",
        thickness: 0.4,
        density: 0.34899,
        stretch: 348.274,
        shear: 174.159,
        bending: 2.482e-6,
        provenance: LATEX_MEASURED,
    })
}

/// 1.0 mm natural rubber latex sheet, derived from the measured 0.40 mm profile.
pub fn latex_1_0() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_1_0,
        name: "Latex 1.0 mm",
        thickness: 1.0,
        density: 0.87248,
        stretch: 870.685,
        shear: 435.398,
        bending: 3.8785e-5,
        provenance: LATEX_DERIVED,
    })
}

/// Two fully glued 0.25 mm sheets: 0.50 mm.
pub fn latex_2x_0_25() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_2X_0_25,
        name: "Latex laminate 2 × 0.25 mm",
        thickness: 0.5,
        density: 0.43624,
        stretch: 435.343,
        shear: 217.699,
        bending: 4.848e-6,
        provenance: LATEX_DERIVED,
    })
}

/// A fully glued 0.40 mm and 0.25 mm sheet: 0.65 mm.
pub fn latex_0_4_0_25() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_0_4_0_25,
        name: "Latex laminate 0.4 + 0.25 mm",
        thickness: 0.65,
        density: 0.56711,
        stretch: 565.946,
        shear: 283.008,
        bending: 1.0651e-5,
        provenance: LATEX_DERIVED,
    })
}

/// Two fully glued 0.40 mm sheets: 0.80 mm.
pub fn latex_2x_0_4() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_2X_0_4,
        name: "Latex laminate 2 × 0.4 mm",
        thickness: 0.8,
        density: 0.69798,
        stretch: 696.548,
        shear: 348.318,
        bending: 1.9858e-5,
        provenance: LATEX_DERIVED,
    })
}

/// Three fully glued 0.40 mm sheets: 1.20 mm.
pub fn latex_3x_0_4() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_3X_0_4,
        name: "Latex laminate 3 × 0.4 mm",
        thickness: 1.2,
        density: 1.04697,
        stretch: 1044.822,
        shear: 522.477,
        bending: 6.7021e-5,
        provenance: LATEX_DERIVED,
    })
}

/// Two fully glued 1.0 mm sheets: 2.0 mm.
pub fn latex_2x_1_0() -> Fabric {
    latex_sheet(LatexSpec {
        id: ID_LATEX_2X_1_0,
        name: "Latex laminate 2 × 1.0 mm",
        thickness: 2.0,
        density: 1.74495,
        stretch: 1741.371,
        shear: 870.795,
        bending: 3.1028e-4,
        provenance: LATEX_DERIVED,
    })
}

/// The 8 latex sheet presets, in canonical order: 0.25, 0.4 (measured), 1.0, then the laminates.
pub fn latex_presets() -> Vec<Fabric> {
    vec![
        latex_0_25(),
        latex_0_4(),
        latex_1_0(),
        latex_2x_0_25(),
        latex_0_4_0_25(),
        latex_2x_0_4(),
        latex_3x_0_4(),
        latex_2x_1_0(),
    ]
}

/// The 13 standard presets in canonical order: the 5 textiles, then the 8 latex sheets.
pub fn presets() -> Vec<Fabric> {
    let mut all = vec![cotton_jersey(), denim(), poplin(), fleece(), wool_suiting()];
    all.extend(latex_presets());
    all
}

/// Finds a preset by its standard id or name (case-insensitive).
pub fn find_preset(id_or_name: &str) -> Option<Fabric> {
    presets()
        .into_iter()
        .find(|p| p.id.as_str() == id_or_name || p.name.eq_ignore_ascii_case(id_or_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::check;
    use std::collections::BTreeMap;

    #[test]
    fn latex_presets_are_isotropic_and_derived_from_the_measured_profile() {
        let all = latex_presets();
        assert_eq!(all.len(), 8);
        let base = all
            .iter()
            .find(|p| p.id.as_str() == ID_LATEX_0_4)
            .expect("the measured 0.4 mm profile");
        assert_eq!(base.category, Category::Nonwoven);
        assert!((base.density - 0.34899).abs() < 1e-12);
        assert!((base.stretch.warp - 348.274).abs() < 1e-9);
        assert!((base.stretch.shear - 174.159).abs() < 1e-9);
        assert!((base.bending.warp - 2.482e-6).abs() < 1e-15);

        let t0 = base.thickness.unwrap();
        let close = |a: f64, b: f64| (a - b).abs() / b.abs() < 5e-4;
        for p in &all {
            assert_eq!(p.category, Category::Nonwoven, "{}", p.name);
            assert_eq!(p.stretch.warp, p.stretch.weft, "{}", p.name);
            assert_eq!(p.bending.warp, p.bending.weft, "{}", p.name);
            assert_eq!(p.bending.bias, Some(p.bending.warp), "{}", p.name);
            assert_eq!(p.friction, LATEX_FRICTION, "{}", p.name);
            let s = p.thickness.unwrap() / t0;
            assert!(close(p.density, base.density * s), "{} density", p.name);
            assert!(
                close(p.stretch.warp, base.stretch.warp * s),
                "{} stretch",
                p.name
            );
            assert!(
                close(p.stretch.shear, base.stretch.shear * s),
                "{} shear",
                p.name
            );
            assert!(
                close(p.bending.warp, base.bending.warp * s.powi(3)),
                "{} bending",
                p.name
            );
        }
    }

    #[test]
    fn preset_ids_are_unique() {
        let mut ids = std::collections::HashSet::new();
        for preset in presets() {
            assert!(
                ids.insert(preset.id.as_str().to_owned()),
                "duplicate preset id {}",
                preset.id
            );
        }
    }

    #[test]
    fn presets_validate() {
        for preset in presets() {
            let doc = crate::Document {
                kind: crate::Kind::Fabric,
                project: None,
                garments: Vec::new(),
                fabrics: vec![preset.clone()],
                appearances: Vec::new(),
                avatars: Vec::new(),
                view: None,
                files: BTreeMap::new(),
                generator: Some("test".into()),
                extensions_used: Vec::new(),
                extensions_required: Vec::new(),
                format_version: crate::FORMAT_VERSION,
                min_reader_version: crate::MIN_READER_VERSION,
                manifest_other: Map::new(),
                part_other: BTreeMap::new(),
            };
            let report = check(&doc);
            assert!(
                report.errors.is_empty(),
                "preset {} has validation errors: {:?}",
                preset.name,
                report.errors
            );
        }
    }
}
