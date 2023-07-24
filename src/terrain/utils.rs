use bevy::{
    prelude::*,
    reflect::TypeUuid
  };
  use serde::{Serialize, Deserialize};

  use super::noises::NoiseData;


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlaneData {
    pub loc: (f32, f32, f32),
    pub subdivisions: u32,
    pub dims: (f32, f32),
    pub color: [f32; 4],
    pub noise_data: Option<NoiseData>

}
#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Planes(pub Vec<PlaneData>);

#[derive(Resource)]
pub struct PlanesAsset(pub Handle<Planes>);
