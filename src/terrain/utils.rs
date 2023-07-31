use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Serialize, Deserialize};

use super::wanders::get_distance_manhattan;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Edge {X, NX, Z, NZ}  

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EdgeLine {
  pub axis:     Axis,
  pub start:    (f32, f32),
  pub end:      (f32, f32),
  pub outside:  Edge // hard to explain but its needed
}

impl EdgeLine {
  pub fn _to_aabb(&self, width: f32) -> (AABB, f32) {
    let min_x: f32;
    let max_x: f32;
    let min_z: f32;
    let max_z: f32;

    let main: f32;

    match self.axis {
      Axis::Z => {
        main = self.start.0;
        match self.outside {
          Edge::X => {
            min_x = self.start.0;
            max_x = self.start.0 + width;
          }
          Edge::NX => {
            min_x = self.start.0- width;
            max_x = self.start.0;
          }
          _ => {min_x = 0.0; max_x = 0.0} // will not happen :)
        }
        if self.start.1 < self.end.1 {
          min_z = self.start.1;
          max_z = self.end.1;
        } else {
          min_z = self.end.1;
          max_z = self.start.1;
        }
      }
      Axis::X => {
        main = self.start.1;
        match self.outside {
          Edge::Z => {
            min_z = self.start.1;
            max_z = self.start.1 + width;
          }
          Edge::NZ => {
            min_z = self.start.1 - width;
            max_z = self.start.1;
          }
          _ => {min_z = 0.0; max_z = 0.0} // will not happen :)
        }
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
    return (aabb, main);
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis {X, Z}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct AABB {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_z:          f32,
  pub max_z:          f32
}

impl AABB {

  pub fn _intersect(self, other: &AABB) -> bool {
    self.max_x >= other.min_x && self.min_x <= other.max_x &&
    self.max_z >= other.min_z && self.min_z <= other.max_z
  }

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    p[0] >= self.min_x && p[0] <= self.max_x && p[2] >= self.min_z && p[2] <= self.max_z
  }

  pub fn _has_point_excl(&self, p: &[f32; 3]) -> bool {
    p[0] > self.min_x && p[0] < self.max_x && p[2] > self.min_z && p[2] < self.max_z
  }

  pub fn from_point(xz: &(f32, f32), dims: &(f32, f32)) -> Self {
    AABB{min_x: xz.0 - dims.0/2.0, max_x: xz.0 + dims.0/2.0, min_z: xz.1 - dims.1/2.0, max_z: xz.1 + dims.1/2.0}
  }

  // Convert aabb into edges (thin AABB box)
  pub fn _to_edges(&self, plane: &AABB) -> Vec<EdgeLine> {
  
    let mut v: Vec<EdgeLine> = Vec::new();
    if self.min_x > plane.min_x && self.min_x < plane.max_x{
      v.push(EdgeLine{axis: Axis::Z, start: (self.min_x, self.min_z), end: (self.min_x, self.max_z), outside: Edge::NX});
    }
    if self.max_x > plane.min_x && self.max_x < plane.max_x{
      v.push(EdgeLine{axis: Axis::Z, start: (self.max_x, self.min_z), end: (self.max_x, self.max_z), outside: Edge::X});
    }
    if self.min_z > plane.min_z && self.min_z < plane.max_z{
      v.push(EdgeLine{axis: Axis::X, start: (self.min_x, self.min_z), end: (self.max_x, self.min_z), outside: Edge::NZ});
    }
    if self.max_z > plane.min_z && self.max_z < plane.max_z{
      v.push(EdgeLine{axis: Axis::X, start: (self.min_x, self.max_z), end: (self.max_x, self.max_z), outside: Edge::Z});
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

  pub fn _to_edges(&self, plane: &AABB) -> Vec<EdgeLine> {
    let mut abs: Vec<EdgeLine> = Vec::new();
    for aabb in self.0.iter(){
      let mut edges = aabb._to_edges(plane);
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


// unpacks mesh stats
pub fn get_mesh_stats(mesh: &Mesh){
  if let Some(pos) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
    let v_pos = pos.as_float3().unwrap().to_vec();
    // pos: 3, color: 4, normals: 3, uvs: 2
    let v_memory = 3.0 + 4.0 + 3.0 + 2.0;

    info!("  Vertex count: {} Memory: {} Kb", 
        v_pos.len(), 
        (v_memory*v_pos.len() as f32)*4.0/1024.0
      );
  }

}


#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Ellipse {
  pub a: f32,  // axis
  pub b: f32,  // axis
  pub x: f32,  // loc 
  pub z: f32   // loc
}

impl Ellipse {

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    let dx = p[0] - self.x;
    let dz = p[2] - self.z;
    (dx * dx) / (self.a * self.a) + (dz * dz) / (self.b * self.b) <= 1.0
  }

  pub fn has_point_dist(&self, p: &[f32; 3]) -> Option<f32> {
    let dx = p[0] - self.x;
    let dz = p[2] - self.z;

    if (dx * dx) / (self.a * self.a) + (dz * dz) / (self.b * self.b) <= 1.0 {
      Some(get_distance_manhattan(&(self.x, self.z), &(p[0],p[2])))
    } else {
      None
    }

  }
}