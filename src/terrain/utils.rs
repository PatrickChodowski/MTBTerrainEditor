use bevy::prelude::*;

use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct AABB {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_z:          f32,
  pub max_z:          f32,
}

impl AABB {

  pub fn _intersect(self, other: &AABB) -> bool {
    self.max_x >= other.min_x && self.min_x <= other.max_x &&
    self.max_z >= other.min_z && self.min_z <= other.max_z
  }

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    p[0] >= self.min_x && p[0] <= self.max_x && p[2] >= self.min_z && p[2] <= self.max_z
  }

  pub fn from_point(xz: &(f32, f32), dims: &(f32, f32)) -> Self {
    AABB{min_x: xz.0 - dims.0/2.0, max_x: xz.0 + dims.0/2.0, min_z: xz.1 - dims.1/2.0, max_z: xz.1 + dims.1/2.0}
  }

}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AABBs(pub Vec<AABB>);

impl AABBs {
  pub fn new() -> Self {
    AABBs(Vec::new())
  }
  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    for aabb in self.0.iter() {
      if aabb.has_point(p){
        return true;
      }
    }
    return false;
  }

}