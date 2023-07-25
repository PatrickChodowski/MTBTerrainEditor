use bevy::{
    prelude::*,
    reflect::TypeUuid
  };
use serde::{Serialize, Deserialize};
use super::modifiers::{Modifier, ModifierTrait};
use super::noises::{NoiseData, NoiseFunction};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaneData {
    pub loc: (f32, f32, f32),
    pub subdivisions: u32,
    pub dims: (f32, f32),
    pub color: [f32; 4],
    pub noise_data: Option<NoiseData>,
    pub modifiers:  Vec<Modifier>
}

impl PlaneData {
  pub fn apply(&self, mesh: &mut Mesh) -> Mesh {
    let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();

    let mut noise_fn: Option<NoiseFunction> = None;
    if let Some(noise_data) = &self.noise_data {
      noise_fn = Some(noise_data.bake());
    }

    // Grab modifiers and convert into functions
    let mut mods: Vec<Box<dyn ModifierTrait>> = Vec::new();
    for m in self.modifiers.iter(){
      // dont know better way :/
      match m {
        Modifier::FlatEdges(a) => {mods.push(a.into_fn())}
        Modifier::Easing(a)    => {mods.push(a.into_fn())}
        Modifier::FlatEdge(a)    => {mods.push(a.into_fn())}
      }
    }

    // Bake modifiers
    for m in mods.iter_mut(){
      m.bake(&self.dims)
    }

    for pos in v_pos.iter_mut(){

      if let Some(noise_fn) = &noise_fn {
        pos[1] = noise_fn.apply(&pos, &self.noise_data.as_ref().unwrap());
      }

      for m in mods.iter(){
        pos[1] = m.apply(&pos);
      }
        
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    return mesh.clone()
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
}