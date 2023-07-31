
use serde::{Deserialize,Serialize};

use crate::terrain::utils::{Area, get_distance_euclidean};
use crate::terrain::modifiers::ModifierBase;

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
    Distance
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueData {
    pub mb:        ModifierBase,
    pub scaling:   ValueScaling,
    pub val:       Val
}
impl ValueData {
    pub fn set(&self) -> Value {
        let v = Value{val: self.val, area: self.mb.to_area(), scaling: self.scaling};
        return v;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Value {
    pub val:       Val,
    pub area:      Area,
    pub scaling:   ValueScaling,
}

impl Value {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 { 
        if self.area.has_point(pos){
            return self.val.apply(pos[1]);
        }
        return pos[1];
    }
}