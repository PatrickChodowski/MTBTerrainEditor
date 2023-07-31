
use serde::{Deserialize,Serialize};

use crate::terrain::utils::Area;
use crate::terrain::modifiers::ModifierBase;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Val {
    Value(f32),
    Scale(f32)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueData {
    pub mb:        ModifierBase,
    pub val:       Val
}
impl ValueData {
    pub fn set(&self) -> Value {
        let v = Value{val: self.val, area: self.mb.to_area()};
        return v;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Value {
    pub val:       Val,
    pub area:      Area,
}

impl Value {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 { 
        if self.area.has_point(pos){
            match &self.val {
                Val::Value(v) => { return *v;}
                Val::Scale(s) => { return pos[1]*s;}
            }
        }
        return pos[1];
    }
}