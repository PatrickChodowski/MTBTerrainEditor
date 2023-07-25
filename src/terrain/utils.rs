use bevy::{
    prelude::*,
    reflect::TypeUuid
  };

use serde::{Serialize, Deserialize};
use super::modifiers::{Modifier, ModifierFN};
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

    let mut modifier_functions: Vec<ModifierFN> = Vec::new();
    for modifier in self.modifiers.iter(){
      modifier_functions.push(modifier.bake(&self));
    }

    for pos in v_pos.iter_mut(){

      if let Some(noise_fn) = &noise_fn {
        pos[1] = noise_fn.apply(&pos, &self.noise_data.as_ref().unwrap());
      }

      for m in modifier_functions.iter(){
        pos[1] = m.modifier.apply(&pos, &m.aabbs);
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




//         v.push(Modifier{
//             min_x: min_x + self.dist, 
//             max_x: min_x + self.dist + self.buffer,
//             min_z: min_z + self.buffer, 
//             max_z: max_z - self.buffer, 
//             modifier_type: ModifierType::Scale(0.5)});

//         v.push(Modifier{
//             min_x: max_x - self.dist - self.buffer, 
//             max_x: max_x - self.dist,
//             min_z: min_z + self.buffer, 
//             max_z: max_z - self.buffer, 
//             modifier_type: ModifierType::Scale(0.5)});

//         v.push(Modifier{
//             min_x: min_x + self.buffer, 
//             max_x: max_x - self.buffer,
//             min_z: min_z + self.dist,
//             max_z: min_z + self.dist + self.buffer, 
//             modifier_type: ModifierType::Scale(0.5)});

//         v.push(Modifier{
//             min_x: min_x + self.buffer, 
//             max_x: max_x - self.buffer,
//             min_z: max_z - self.dist - self.buffer,
//             max_z: max_z - self.dist, 
//             modifier_type: ModifierType::Scale(0.5)});
                    
//         return Modifiers(v);
//     }
// }
