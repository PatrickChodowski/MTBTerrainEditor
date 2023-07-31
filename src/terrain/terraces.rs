
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerracesData {
  pub terraces: Vec<[f32;3]>, // height, from, to
}

impl TerracesData {
  pub fn apply(&self, pos: &[f32; 3]) -> f32 {
    for terrace in self.terraces.iter(){
      if pos[1] >= terrace[1] && pos[1] < terrace[2] {
        return terrace[0];
      }
    }
    return pos[1];
  }
}