use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    FlatEdges
}

















#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum ModifierType {
  Value(f32),
  Scale(f32),
  DistanceScale
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub struct Modifier {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_z:          f32,
  pub max_z:          f32,
  pub modifier_type:  ModifierType
}
impl Modifier {
  pub fn _intersect(self, other: &Modifier) -> bool {
    self.max_x >= other.min_x && self.min_x <= other.max_x &&
    self.max_z >= other.min_z && self.min_z <= other.max_z
  }
  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    p[0] >= self.min_x && p[0] <= self.max_x && p[2] >= self.min_z && p[2] <= self.max_z
  }
}

#[derive(Debug)]
pub struct Modifiers(pub Vec<Modifier>);

impl Modifiers {

  pub fn apply(&self, p: &[f32; 3]) -> f32 {
    for modifier in self.0.iter(){
      if modifier.has_point(p) {
        match modifier.modifier_type {
          ModifierType::Value(v)  =>  {return v;}
          ModifierType::Scale(s)  =>  {return p[1]*s;}
          ModifierType::DistanceScale => {
            //
          }
        }

      }
    }
    return p[1];
  }
}


pub trait ModifierTrait {
    fn setup(&self, dims: &(f32, f32)) -> Modifiers;
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct FlatEdges {
    pub height: f32,
    pub dist:   f32,
    pub buffer: f32
}

impl ModifierTrait for FlatEdges {
    fn setup<'a>(&self, dims: &'a (f32, f32)) -> Modifiers {
        let mut v: Vec<Modifier> = Vec::with_capacity(8);
        return Modifiers(v);
    }
}


impl FlatEdges {
    fn to_modifiers<'a>(&self, dims: &'a (f32, f32)) -> Modifiers {
        let min_x = -1.0*dims.0/2.0;
        let max_x = dims.0/2.0;
        let min_z = -1.0*dims.1/2.0;
        let max_z = dims.1/2.0;

        let mut v: Vec<Modifier> = Vec::with_capacity(8);
        v.push(Modifier{min_x, max_x: min_x + self.dist, min_z, max_z, modifier_type: ModifierType::Value(self.height)});
        v.push(Modifier{min_x: max_x - self.dist, max_x, min_z, max_z, modifier_type: ModifierType::Value(self.height)});
        v.push(Modifier{min_x, max_x, min_z, max_z: min_z + self.dist, modifier_type: ModifierType::Value(self.height)});
        v.push(Modifier{min_x, max_x, min_z: max_z-self.dist, max_z,   modifier_type: ModifierType::Value(self.height)});

        v.push(Modifier{
            min_x: min_x + self.dist, 
            max_x: min_x + self.dist + self.buffer,
            min_z: min_z + self.buffer, 
            max_z: max_z - self.buffer, 
            modifier_type: ModifierType::Scale(0.5)});

        v.push(Modifier{
            min_x: max_x - self.dist - self.buffer, 
            max_x: max_x - self.dist,
            min_z: min_z + self.buffer, 
            max_z: max_z - self.buffer, 
            modifier_type: ModifierType::Scale(0.5)});

        v.push(Modifier{
            min_x: min_x + self.buffer, 
            max_x: max_x - self.buffer,
            min_z: min_z + self.dist,
            max_z: min_z + self.dist + self.buffer, 
            modifier_type: ModifierType::Scale(0.5)});

        v.push(Modifier{
            min_x: min_x + self.buffer, 
            max_x: max_x - self.buffer,
            min_z: max_z - self.dist - self.buffer,
            max_z: max_z - self.dist, 
            modifier_type: ModifierType::Scale(0.5)});
                    
        return Modifiers(v);
    }



}