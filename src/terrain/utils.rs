use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Edge {X, NX, Z, NZ}  

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EdgeLine {
  pub axis:   Axis,
  pub start:  (f32, f32),
  pub end:    (f32, f32)
}

impl EdgeLine {
  pub fn to_aabb(&self, width: f32) -> AABB {
    let min_x: f32;
    let max_x: f32;
    let min_z: f32;
    let max_z: f32;

    match self.axis {
      Axis::Z => {
        min_x = self.start.0 - width/2.0; 
        max_x = self.end.0 + width/2.0;
        if self.start.1 < self.end.1 {
          min_z = self.start.1;
          max_z = self.end.1;
        } else {
          min_z = self.end.1;
          max_z = self.start.1;
        }
      }
      Axis::X => {
        min_z = self.start.1 - width/2.0; 
        max_z = self.end.1 + width/2.0;
        if self.start.0 < self.end.0 {
          min_x = self.start.0;
          max_x = self.end.0;
        } else {
          min_x = self.end.0;
          max_x = self.start.0;
        }
      }
    }
    let aabb = AABB{min_x, max_x, min_z, max_z};
    println!("self edge: {:?}  width: {} aabb: {:?}", self, width, aabb);
    return aabb;
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis {X, Z}

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

  // Convert aabb into edges (thin AABB box)
  pub fn to_edges(&self, plane: &AABB) -> Vec<EdgeLine> {
  
    let mut v: Vec<EdgeLine> = Vec::new();
    if self.min_x > plane.min_x && self.min_x < plane.max_x{
      v.push(EdgeLine{axis: Axis::Z, start: (self.min_x, self.min_z), end: (self.min_x, self.max_z)});
    }
    if self.max_x > plane.min_x && self.max_x < plane.max_x{
      v.push(EdgeLine{axis: Axis::Z, start: (self.max_x, self.min_z), end: (self.max_x, self.max_z)});
    }
    if self.min_z > plane.min_z && self.min_z < plane.max_z{
      v.push(EdgeLine{axis: Axis::X, start: (self.min_x, self.min_z), end: (self.max_x, self.min_z)});
    }
    if self.max_z > plane.min_z && self.max_z < plane.max_z{
      v.push(EdgeLine{axis: Axis::X, start: (self.min_x, self.max_z), end: (self.max_x, self.max_z)});
    }

    return v;
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

  pub fn to_edges(&self, plane: &AABB) -> Vec<EdgeLine> {
    let mut abs: Vec<EdgeLine> = Vec::new();
    for aabb in self.0.iter(){
      let mut edges = aabb.to_edges(plane);
      abs.append(&mut edges);
    }
    return abs;
  }

}

#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "201ce530-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct ConfigData {
    pub scene_file:         String
}

#[derive(Resource)]
pub struct ConfigAsset(pub Handle<ConfigData>);
