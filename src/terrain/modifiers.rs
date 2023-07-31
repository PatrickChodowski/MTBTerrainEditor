
use serde::{Deserialize,Serialize};

use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::terraces::TerracesData;
use crate::terrain::planes::PlaneData;
use crate::terrain::value::{ValueData, Value};
use crate::terrain::wanders::{TargetWanderNoiseData,TargetWanderNoise};
use crate::terrain::utils::{AreaData, Area};


// Struct needed for every modifier. Added as component to all modifiers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierBase {
    pub area:      AreaData,
    pub loc:       [f32; 2],
}
impl ModifierBase {
    pub fn to_area(&self) -> Area {
        return self.area.to_area(&self.loc);
    }
  }



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModifierData {
    Noise(NoiseData),
    Terraces(TerracesData),
    TargetWanderNoise(TargetWanderNoiseData),
    Value(ValueData)
} 

impl ModifierData {
    pub fn set(&self, pd: &PlaneData, v_pos: &Vec<[f32; 3]>) -> Modifier {
        match self {
            ModifierData::Noise(data)               => {return Modifier::Noise(data.set())}
            ModifierData::Value(data)               => {return Modifier::Value(data.set())}
            ModifierData::TargetWanderNoise(data)   => {return Modifier::TargetWanderNoise(data.set(pd))}
            ModifierData::Terraces(data)            => {return Modifier::Terraces(data.clone())}
        }
    }

}


#[derive(Clone)]
pub enum Modifier {
    Noise(Noise),
    Value(Value),
    Terraces(TerracesData),
    TargetWanderNoise(TargetWanderNoise)
} 

impl Modifier {

    pub fn apply_point(&self, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {
        match self {
            Modifier::Noise(data)               => {return data.apply(pos, loc)}
            Modifier::Value(data)               => {return data.apply(pos)}
            Modifier::Terraces(data)            => {return data.apply(pos)}
            Modifier::TargetWanderNoise(data)   => {return data.apply(pos)}
            

            // Area only:
            // Modifier::SmoothArea(_data)         => {pos[1]}
        }           
    }

    pub fn apply_area(&mut self, _v_pos: &mut Vec<[f32; 3]>){
        match self {

            // Modifier::SmoothEdge(_data) => {
            //     // data.update(edges);
            //     // let index_heights: HashMap<usize, f32> = data.apply(v_pos);
            //     // for (index, height) in index_heights.iter(){
            //     //     v_pos[*index][1] = *height;
            //     // }
            // }

            // point only:
            Modifier::Noise(_data)             => {}
            Modifier::Value(_data)             => {}
            Modifier::Terraces(_data)          => {}
            Modifier::TargetWanderNoise(_data) => {}
        }   
    }


}
