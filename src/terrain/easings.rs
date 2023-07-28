use libm::powf;
use serde::{Serialize, Deserialize};

use crate::terrain::modifiers::ModifierTrait;
use crate::terrain::planes::PlaneData;
use crate::terrain::utils::AABBs;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EasingData {
    pub easing: Easings,
    pub global: bool
}

impl EasingData {
    pub fn aabbs(&self, pd: &PlaneData) -> AABBs {
        let mut aabbs = AABBs::new();
        aabbs.0.push(pd.get_aabb());
        return aabbs;
    }
}

impl ModifierTrait for EasingData {
    fn apply(&self, pos: &[f32; 3], aabbs: &AABBs, loc: &[f32; 3]) -> f32 {
        if aabbs.has_point(pos) {
            let mut g_pos = *pos;
            if self.global {
                g_pos = [pos[0] + loc[0], pos[1] + loc[1], pos[2]+loc[2]];
            }
            return self.easing.apply(g_pos[1]);
        }
        return pos[1];
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Easings {
    SmoothStep,
    SmoothStop,
    SmoothStart,
    SmoothEnd,
    AbsoluteValue,
    AbsoluteValuePow(f32)
}

impl Easings {
    pub fn apply(&self, x: f32) -> f32 {
        match self {
            Easings::SmoothStart => {
                return x*x;
            }
            Easings::SmoothStop => {
                return 1.0 - ((1.0 - x)*(1.0-x));
            }
            Easings::SmoothEnd => {
                return 1.0 - (1.0 - x).powi(2);
            }
            Easings::SmoothStep => {
                let xc = x.clamp(0.0, 1.0);
                return xc * xc * (3.0 - 2.0 * xc);
            }
            Easings::AbsoluteValue => {
                return x.abs();
            }
            Easings::AbsoluteValuePow(p) => {
                return powf(x.abs(), *p);
            }
        }
    }
}
