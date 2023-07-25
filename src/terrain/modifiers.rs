
use serde::{Serialize, Deserialize};
use crate::terrain::utils::{AABB, AABBs};

use super::utils::PlaneData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Edge {
    X, NX, Z, NZ
}  

pub trait ModifierTrait {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs) -> f32;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    FlatEdges(FlatEdges),
    FlatEdge(FlatEdge)
}

impl Modifier {
  pub fn bake(&self, pd: &PlaneData) -> ModifierFN {
    match self {
      Modifier::FlatEdges(data) => {
        ModifierFN { 
          modifier: Box::new(data.clone()),
          aabbs: AABBs::from_dims_dist(&pd.dims, data.dist)
        }
      }
      Modifier::FlatEdge(data) => {
        ModifierFN { 
          modifier: Box::new(data.clone()),
          aabbs: AABBs(vec![AABB::from_edge(&data.edge, &pd.dims, data.dist)])
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

impl ModifierTrait for FlatEdges {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs) -> f32 {
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

impl ModifierTrait for FlatEdge {
  fn apply(&self, pos: &[f32; 3], aabbs: &AABBs) -> f32 {
    if aabbs.has_point(pos) {
      return self.height;
    }
    return pos[1];
  }
}