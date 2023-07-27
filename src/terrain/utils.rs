use bevy::{
    prelude::*,
    reflect::TypeUuid
  };

use serde::{Serialize, Deserialize};
use super::modifiers::{Modifier, ModifierFN};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaneData {
    pub loc:          (f32, f32, f32),
    pub subdivisions: u32,
    pub dims:         (f32, f32),
    pub color:        [f32; 4],
    pub modifiers:    Vec<Modifier>
}

impl PlaneData {
  pub fn apply(&self, mesh: &mut Mesh) -> Mesh {
    let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();

    let mut modifier_functions: Vec<ModifierFN> = Vec::new();
    for modifier in self.modifiers.iter(){
      modifier_functions.push(modifier.bake(&self));
    }

    for pos in v_pos.iter_mut(){
      for m in modifier_functions.iter(){
        pos[1] = m.modifier.apply(&pos, &m.aabbs);
      }        
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    return mesh.clone()
  }

  pub fn get_aabb(&self) -> AABB {
    let min_x = -1.0*self.dims.0/2.0;
    let max_x = self.dims.0/2.0;
    let min_z = -1.0*self.dims.1/2.0;
    let max_z = self.dims.1/2.0;
    return AABB{min_x, max_x, min_z, max_z};
  }

}

#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Planes(pub Vec<PlaneData>);

#[derive(Resource)]
pub struct PlanesAsset(pub Handle<Planes>);


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