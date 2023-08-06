
use serde::{Deserialize,Serialize};

use super::modifiers::ModifierBase;
use super::utils::Area;
use super::value::Val;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Terrace {
  pub from: f32,
  pub to: f32,
  pub val: Val
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerracesData {
  pub mb:       ModifierBase,
  pub terraces: Vec<Terrace>
}

impl TerracesData {
  pub fn set(&self) -> Terraces {
    return Terraces { area: self.mb.to_area(), terraces: self.terraces.clone() };
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Terraces {
  pub area:     Area,
  pub terraces: Vec<Terrace>, // height, from, to
}


impl Terraces {
  pub fn apply(&self, pos: &[f32; 3]) -> f32 {
    if self.area.has_point(pos){
      for terrace in self.terraces.iter(){
        if pos[1] >= terrace.from && pos[1] < terrace.to {
          return terrace.val.apply(pos[1]);
        }
      }
    }
    return pos[1];
  }
}