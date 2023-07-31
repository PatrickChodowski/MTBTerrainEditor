
use serde::{Deserialize,Serialize};

use crate::terrain::utils::{Area, Axis, get_distance_euclidean};
use crate::terrain::modifiers::ModifierBase;

use super::easings::Easings;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Val {
    Value(f32),
    Diff(f32),
    Scale(f32)
}
impl Val {
    pub fn apply(&self, x: f32) -> f32 {
        match &self {
            Val::Value(v) => {return *v;}
            Val::Diff(d)  => {return x + d;}
            Val::Scale(s) => {return x*s;}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ValueScaling {
    None,
    DistanceCenter(f32),
    DistanceCenterRev(f32),
    DistanceAxis((Axis, f32))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueData {
    pub mb:        ModifierBase,
    pub scaling:   ValueScaling,
    pub val:       Val,
    pub easing:    Easings
}
impl ValueData {
    pub fn set(&self) -> Value {
        let v = Value{val: self.val, area: self.mb.to_area(), scaling: self.scaling, easing: self.easing};
        return v;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Value {
    pub val:       Val,
    pub area:      Area,
    pub easing:    Easings,
    pub scaling:   ValueScaling,
}

impl Value {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 { 

        let center = self.area.get_center();
        let r = self.area.get_radius();

        if self.area.has_point(pos){
            let v2 = self.val.apply(pos[1]);
            match self.scaling {
                ValueScaling::DistanceCenterRev(d) => {
                    let dist = get_distance_euclidean(&(pos[0], pos[2]), &center);
                    let mut scale: f32 = (dist/r*d).clamp(0.0, 1.0);
                    // println!("Radius: {} Distance: {}, scale: {} v2: {}", r, dist, scale, v2);
                    scale = self.easing.apply(scale);
                    return v2*scale;
                }
                ValueScaling::DistanceCenter(d) => {
                    let dist = get_distance_euclidean(&(pos[0], pos[2]), &center);
                    let mut scale: f32 = 1.0-(dist/r*d).clamp(0.0, 1.0);
                    // println!("Radius: {} Distance: {}, scale: {} v2: {}", r, dist, scale, v2);
                    scale = self.easing.apply(scale);
                    return v2*scale;
                }
                ValueScaling::DistanceAxis((axis, d)) => {
                    let dist: f32;
                    match axis {
                        Axis::X => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(center.0, pos[2]));}
                        Axis::Z => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(pos[0], center.1));}
                    }
                    let mut scale: f32 = 1.0-(dist/r*d).clamp(0.0, 1.0);
                    scale = self.easing.apply(scale);
                    return v2*scale;
                }
                ValueScaling::None => {
                    return v2;
                }
            }
        }
        return pos[1];
    }
}