use serde::{Deserialize,Serialize};
use crate::terrain::planes::PlaneData;
use crate::terrain::utils::{AABB,AABBs, EdgeLine, Edge};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdgesData {
    pub height: f32,
    pub dist:   f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdges {
    pub height: f32,
    pub dist:   f32,
    pub aabbs:  AABBs
}



impl FlatEdgesData {
  pub fn set(&self, pd: &PlaneData) -> FlatEdges {
    let min_x = -1.0*pd.dims.0/2.0;
    let max_x = pd.dims.0/2.0;
    let min_z = -1.0*pd.dims.1/2.0;
    let max_z = pd.dims.1/2.0;
    
    let mut v: Vec<AABB> = Vec::with_capacity(8);
    v.push(AABB{min_x, max_x: min_x + self.dist, min_z, max_z});
    v.push(AABB{min_x: max_x - self.dist, max_x, min_z, max_z});
    v.push(AABB{min_x, max_x, min_z, max_z: min_z + self.dist});
    v.push(AABB{min_x, max_x, min_z: max_z-self.dist, max_z});

    return FlatEdges{dist: self.dist, height: self.height, aabbs: AABBs(v)};
  }
}

impl FlatEdges {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 {
        if self.aabbs.has_point(pos) {
          return self.height;
        }
        return pos[1];
      }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdgeData {
    pub edge:   Edge,
    pub height: f32,
    pub dist:   f32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdge {
    pub edge:   Edge,
    pub height: f32,
    pub dist:   f32,
    pub aabbs:  AABBs
}



impl FlatEdgeData {
  pub fn set(&self, pd: &PlaneData) -> FlatEdge {
    let min_x = -1.0*pd.dims.0/2.0;
    let max_x = pd.dims.0/2.0;
    let min_z = -1.0*pd.dims.1/2.0;
    let max_z = pd.dims.1/2.0;

    let v: AABBs;

    match self.edge {
      Edge::X   => {v = AABBs(vec![AABB{min_x: max_x - self.dist, max_x, min_z, max_z}])}
      Edge::NX  => {v = AABBs(vec![AABB{min_x, max_x: min_x+self.dist, min_z, max_z}])}
      Edge::Z   => {v = AABBs(vec![AABB{min_x, max_x, min_z: max_z-self.dist, max_z}])}
      Edge::NZ  => {v = AABBs(vec![AABB{min_x, max_x, min_z, max_z: min_z+self.dist}])}
    }
    return FlatEdge{edge: self.edge, dist: self.dist, height: self.height, aabbs: v};
  }

}

impl FlatEdge {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 {
        if self.aabbs.has_point(pos) {
          return self.height;
        }
        return pos[1];
      }
}



// Smooth edge line data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmoothEdgeData {
  pub buffer: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmoothEdge {
  pub edges:  Vec<EdgeLine>,
  pub buffer: f32,
  pub aabbs:  AABBs
}

impl SmoothEdge {
    pub fn apply(&self) {

    }

}

impl SmoothEdgeData{
  pub fn set(&self) -> SmoothEdge {
    return SmoothEdge{buffer: self.buffer, edges: Vec::new(), aabbs: AABBs::new()};
  }
}
