
use serde::{Serialize, Deserialize};

use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::planes::PlaneData;
use crate::terrain::wanders::TargetWanderNoise;
use crate::terrain::utils::{AABB, AABBs};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Edge {
    X, NX, Z, NZ
}  

pub trait ModifierTrait {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs, loc: &[f32; 3]) -> f32;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    FlatEdges(FlatEdges),
    FlatEdge(FlatEdge),
    Noise(NoiseData),
    TargetWanderNoise(TargetWanderNoise)
}

impl Modifier {
  pub fn bake(&self, pd: &PlaneData) -> ModifierFN {
    match self {
      Modifier::FlatEdges(data) => {
        ModifierFN { 
          modifier: Box::new(data.clone()),
          aabbs: data.aabbs(pd)
        }
      }
      Modifier::FlatEdge(data) => {
        ModifierFN { 
          modifier: Box::new(data.clone()),
          aabbs: data.aabbs(pd)
        }
      }
      Modifier::Noise(data) => {
        ModifierFN { 
          modifier: Box::new(Noise::from_noise_data(data)),
          aabbs: Noise::aabbs(pd)
        }
      }
      Modifier::TargetWanderNoise(data) => {
        ModifierFN { 
          modifier: Box::new(data.clone()),
          aabbs:  data.aabbs(pd)
        }
      }

    }
  }
}

pub struct ModifierFN {
    pub modifier: Box<dyn ModifierTrait>,
    pub aabbs:    AABBs
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdges {
    pub height: f32,
    pub dist:   f32,
    pub buffer: f32,
}

impl FlatEdges {
  pub fn aabbs(&self, pd: &PlaneData) -> AABBs{
    let min_x = -1.0*pd.dims.0/2.0;
    let max_x = pd.dims.0/2.0;
    let min_z = -1.0*pd.dims.1/2.0;
    let max_z = pd.dims.1/2.0;
    
    let mut v: Vec<AABB> = Vec::with_capacity(8);
    v.push(AABB{min_x, max_x: min_x + self.dist, min_z, max_z});
    v.push(AABB{min_x: max_x - self.dist, max_x, min_z, max_z});
    v.push(AABB{min_x, max_x, min_z, max_z: min_z + self.dist});
    v.push(AABB{min_x, max_x, min_z: max_z-self.dist, max_z});

    return AABBs(v);
  }
}

impl ModifierTrait for FlatEdges {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs, _loc: &[f32; 3]) -> f32 {
    if aabbs.has_point(pos) {
      return self.height;
    }
    return pos[1];
  }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdge {
    pub edge:   Edge,
    pub height: f32,
    pub dist:   f32,
}

impl FlatEdge {
  pub fn aabbs(&self, pd: &PlaneData) -> AABBs {
    let min_x = -1.0*pd.dims.0/2.0;
    let max_x = pd.dims.0/2.0;
    let min_z = -1.0*pd.dims.1/2.0;
    let max_z = pd.dims.1/2.0;

    match self.edge {
      Edge::X   => {AABBs(vec![AABB{min_x: max_x - self.dist, max_x, min_z, max_z}])}
      Edge::NX  => {AABBs(vec![AABB{min_x, max_x: min_x+self.dist, min_z, max_z}])}
      Edge::Z   => {AABBs(vec![AABB{min_x, max_x, min_z: max_z-self.dist, max_z}])}
      Edge::NZ  => {AABBs(vec![AABB{min_x, max_x, min_z, max_z: min_z+self.dist}])}
    }

  }
}

impl ModifierTrait for FlatEdge {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs, _loc: &[f32; 3]) -> f32 {
    if aabbs.has_point(pos) {
      return self.height;
    }
    return pos[1];
  }
}