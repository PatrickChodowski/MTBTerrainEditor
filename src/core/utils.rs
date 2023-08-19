use libm::{fabsf, powf, sqrtf};
use bevy::prelude::*;
use std::slice::Iter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Axis {
  X,Z
}

impl<'a> Axis {
  pub fn iterator() -> Iter<'static, Axis> {
      static OPTIONS: [Axis; 2] = [
          Axis::X,
          Axis::Z,
      ];
      OPTIONS.iter()
  }
}


#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize, Component)]
pub struct AABB {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_z:          f32,
  pub max_z:          f32
}

impl AABB {

  pub fn from_gui(n: &Node, s: &Style, window_width: f32, window_height: f32) -> Self {
    let ns = n.size();
    let aabb = AABB{min_x: s.left.evaluate(window_width).unwrap(), 
                    max_x: ns.x, 
                    max_z: window_height - s.top.evaluate(window_height).unwrap(),
                    min_z: window_height - ns.y};
      return aabb;
    }
  
  // abomination of mathematics, radius in aabb
  pub fn get_radius(&self) -> f32 {
    let x_r = (self.max_x - self.min_x)/2.0;
    let z_r = (self.max_z - self.min_z)/2.0;

    // get longest of two
    if x_r >= z_r {
      x_r
    } else {
      z_r
    }
  }

  pub fn get_center(&self) -> (f32,f32){
    ((self.min_x + self.max_x)/2.0, (self.min_z + self.max_z)/2.0)
  }

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    p[0] >= self.min_x && p[0] <= self.max_x && p[2] >= self.min_z && p[2] <= self.max_z
  }

  pub fn from_point(xz: &(f32, f32), dims: &(f32, f32)) -> Self {
    AABB{min_x: xz.0 - dims.0/2.0, max_x: xz.0 + dims.0/2.0, min_z: xz.1 - dims.1/2.0, max_z: xz.1 + dims.1/2.0}
  }

}










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

pub fn get_distance_manhattan(xz: &(f32, f32), target: &(f32, f32)) -> f32 {
  return fabsf(target.0 - xz.0) + fabsf(target.1 - xz.1);
}

pub fn get_distance_euclidean(xz: &(f32, f32), target: &(f32, f32)) -> f32 {
  return sqrtf(powf(target.0 - xz.0, 2.0) + powf(target.1 - xz.1, 2.0));
}
