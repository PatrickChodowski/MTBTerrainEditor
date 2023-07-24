use bevy::{
    prelude::*,
    reflect::TypeUuid
  };
use serde::{Serialize, Deserialize};
use super::modifiers::Modifier;
use super::noises::{NoiseData, NoiseFunction};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaneData {
    pub loc: (f32, f32, f32),
    pub subdivisions: u32,
    pub dims: (f32, f32),
    pub color: [f32; 4],
    pub noise_data: Option<NoiseData>,
    pub modifiers:  Option<Vec<Modifier>>

}

impl PlaneData {
  pub fn apply(&self, mesh: &mut Mesh) -> Mesh {
    let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();

    let mut noise_fn: Option<NoiseFunction> = None;
    if let Some(noise_data) = &self.noise_data {
      noise_fn = Some(noise_data.bake());
    }

    for pos in v_pos.iter_mut(){

      if let Some(noise_fn) = &noise_fn {
        pos[1] = noise_fn.apply(&pos, &self.noise_data.as_ref().unwrap());
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
