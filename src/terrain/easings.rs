use libm::powf;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EasingData {
    pub easing: Easings,
    pub global: bool,
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
