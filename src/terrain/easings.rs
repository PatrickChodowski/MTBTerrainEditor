use libm::powf;
use serde::{Serialize, Deserialize};

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
    pub fn apply(&self, height: f32) -> f32 {
        match self {
            Easings::SmoothStart => {
                return height*height;
            }
            Easings::SmoothStop => {
                return 1.0 - ((1.0 - height)*(1.0-height));
            }
            Easings::SmoothEnd => {
                return 1.0 - (1.0 - height).powi(2);
            }
            Easings::SmoothStep => {
                let x = height.clamp(0.0, 1.0);
                return x * x * (3.0 - 2.0 * x);
            }
            Easings::AbsoluteValue => {
                return height.abs();
            }
            Easings::AbsoluteValuePow(p) => {
                return powf(height.abs(), *p);
            }
        }
    }
}
