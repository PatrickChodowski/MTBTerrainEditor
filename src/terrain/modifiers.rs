
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::terrain::utils::AABB;


pub trait ModifierTrait {
    fn apply(&self, pos: &[f32; 3]) -> f32;
    fn bake(&mut self, dims: &(f32, f32));
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Edge {
    X, NX, Z, NZ
}  


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    FlatEdges(FlatEdges),
    FlatEdge(FlatEdge),
    Easing(Easing)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierFN {
    pub modifier: Modifier, // Data

    pub aabbs:    Vec<AABB>,

}







#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdge {
    pub edge:   Edge,
    pub height: f32,
    pub dist:   f32,
}
impl FlatEdge {
    pub fn into_fn(&self) -> Box<dyn ModifierTrait> {
      return Box::new(FlatEdgesFn{height: self.height, dist: self.dist, buffer:0.0, aabbs: Vec::new()});
    }
  }



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdges {
    pub height: f32,
    pub dist:   f32,
    pub buffer: f32,
}

impl FlatEdges {
  pub fn into_fn(&self) -> Box<dyn ModifierTrait> {
    return Box::new(FlatEdgesFn{height: self.height, dist: self.dist, buffer:self.buffer, aabbs: Vec::new()});
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatEdgesFn {
  pub height: f32,
  pub dist:   f32,
  pub buffer: f32,
  pub aabbs:  Vec<AABB>
}

impl ModifierTrait for FlatEdgesFn {
  fn apply(&self, pos: &[f32; 3]) -> f32{
    for aabb in self.aabbs.iter(){
      if aabb.has_point(&pos){
        return self.height;
      }
    }
    return pos[1];
  }

  fn bake(&mut self, dims: &(f32, f32)){

    let min_x = -1.0*dims.0/2.0;
    let max_x = dims.0/2.0;
    let min_z = -1.0*dims.1/2.0;
    let max_z = dims.1/2.0;

    let mut v: Vec<AABB> = Vec::with_capacity(8);
    v.push(AABB{min_x, max_x: min_x + self.dist, min_z, max_z});
    v.push(AABB{min_x: max_x - self.dist, max_x, min_z, max_z});
    v.push(AABB{min_x, max_x, min_z, max_z: min_z + self.dist});
    v.push(AABB{min_x, max_x, min_z: max_z-self.dist, max_z});

    self.aabbs = v;

  }
}

/*  EASING   */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Easing {
    pub height: f32,
    pub dist: f32
}

impl Easing {
  pub fn into_fn(&self) -> Box<dyn ModifierTrait> {
    return Box::new(EasingFn{height: self.height, dist: self.dist, aabbs: Vec::new()});
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EasingFn {
  pub height: f32,
  pub dist: f32,
  pub aabbs:  Vec<AABB>
}


impl ModifierTrait for EasingFn {
    fn apply(&self, pos: &[f32; 3]) -> f32{
      for aabb in self.aabbs.iter(){
        if aabb.has_point(&pos){
          return self.height;
        }
      }
      return pos[1];
    }
  
    fn bake(&mut self, dims: &(f32, f32)){
  
      let min_x = -1.0*dims.0/2.0;
      let max_x = dims.0/2.0;
      let min_z = -1.0*dims.1/2.0;
      let max_z = dims.1/2.0;
  
      let mut v: Vec<AABB> = Vec::with_capacity(8);
      v.push(AABB{min_x, max_x: min_x + self.dist, min_z, max_z});
      v.push(AABB{min_x: max_x - self.dist, max_x, min_z, max_z});
      v.push(AABB{min_x, max_x, min_z, max_z: min_z + self.dist});
      v.push(AABB{min_x, max_x, min_z: max_z-self.dist, max_z});
  
      self.aabbs = v;
  
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
