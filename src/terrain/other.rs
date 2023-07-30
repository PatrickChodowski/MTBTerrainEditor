
use libm::powf;
use serde::{Deserialize,Serialize};
use bevy::utils::HashMap;
use crate::terrain::planes::PlaneData;
use crate::terrain::utils::{AABB, AABBs, Edge, EdgeLine, Axis};

use super::wanders::get_distance_manhattan;


// Smooth edge line data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerracesData {
  pub terraces: Vec<[f32;3]>, // height, from, to
}

impl TerracesData {
  pub fn apply(&self, pos: &[f32; 3]) -> f32 {
    for terrace in self.terraces.iter(){
      if pos[1] >= terrace[1] && pos[1] < terrace[2] {
        return terrace[0];
      }
    }
    return pos[1];
  }
}

// pub enum TerraceValue {
//   Value(f32),
// }


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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SmoothMethod {
  Distance,
  DistanceReverse,
  DistanceSquare,
  DistancePower(f32),
  Value(f32)
}

// Smooth edge line data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmoothEdgeData {
  pub buffer: f32,
  pub method: SmoothMethod
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmoothEdge {
  pub buffer: f32,
  pub method: SmoothMethod,
  pub aabbs:  Vec<AABB>,
  pub axes:   Vec<Axis>,
  pub mains:  Vec<f32>
}

impl SmoothEdge {

    pub fn update(&mut self, edges: &Vec<EdgeLine>){

      let mut aabbs: Vec<AABB> = Vec::new();
      let mut axes:  Vec<Axis> = Vec::new();
      let mut mains: Vec<f32> = Vec::new();

      for edge in edges.iter(){
        let (aabb, main) = edge.to_aabb(self.buffer);
        aabbs.push(aabb);
        axes.push(edge.axis);
        mains.push(main);
      }

      self.aabbs = aabbs;
      self.axes = axes;
      self.mains = mains;

    }

    pub fn apply(&self, v_pos: &mut Vec<[f32; 3]>) -> HashMap<usize, f32>{
      let mut abpoints: HashMap<usize, Vec<(&[f32; 3], usize)>> = HashMap::new();
      let mut index_heights: HashMap<usize, f32> = HashMap::new();

      // Mapping first
      for (pos_index, pos) in v_pos.iter().enumerate(){
        for (index, aabb) in self.aabbs.iter().enumerate() {
          if aabb.has_point_excl(pos){
            abpoints.entry(index).or_insert(Vec::new()).push((pos, pos_index));
          }
        }
      }

      // per aabb
      for (index, points) in abpoints.iter_mut(){

        let axis: Axis = self.axes[*index];
        let main: f32 = self.mains[*index];

        for (pos, pos_index) in points.iter_mut(){
          let dist: f32;
          match axis {
            Axis::X => {
              dist = get_distance_manhattan(&(pos[0], pos[2]), &(pos[0], main));
            }
            Axis::Z => {
              dist = get_distance_manhattan(&(pos[0], pos[2]), &(main, pos[2]));
            }
          }

          let scaled_height: f32;
          match self.method {
            SmoothMethod::Distance          => {scaled_height =  (dist/self.buffer).clamp(0.0, 1.0)* pos[1];}
            SmoothMethod::DistanceReverse   => {scaled_height =  (1.0- (dist/self.buffer).clamp(0.0, 1.0))*pos[1];}
            SmoothMethod::DistanceSquare    => {scaled_height =  powf((dist/self.buffer).clamp(0.0, 1.0), 2.0)* pos[1];}
            SmoothMethod::DistancePower(p)  => {scaled_height =  powf((dist/self.buffer).clamp(0.0, 1.0), p)* pos[1];}
            SmoothMethod::Value(p)          => {scaled_height = p;}
          }
          index_heights.insert(*pos_index, scaled_height);
        }

      }
      return index_heights;
    }
}

impl SmoothEdgeData{
  pub fn set(&self) -> SmoothEdge {
    return SmoothEdge{buffer: self.buffer, method: self.method.clone(), axes: Vec::new(), aabbs: Vec::new(), mains: Vec::new()};
  }
}
