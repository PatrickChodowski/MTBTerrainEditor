use std::slice::Iter;
use bevy::reflect::Reflect;
// use libm::powf;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Reflect)]
pub enum Easings {
    None,
    SmoothStep,
    SmoothStop,
    SmoothStart,
    SmoothEnd,
    AbsoluteValue
}

impl<'a> Easings {
    pub fn iterator() -> Iter<'static, Easings> {
        static EASING_OPTIONS: [Easings; 6] = [
                Easings::None,
                Easings::SmoothStep,
                Easings::SmoothStop,
                Easings::SmoothStart,
                Easings::SmoothEnd,
                Easings::AbsoluteValue
        ];
        EASING_OPTIONS.iter()
    }
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
            // Easings::AbsoluteValuePow(p) => {
            //     return powf(x.abs(), *p);
            // }
            Easings::None => {
                return x;
            }
        }
    }
}
