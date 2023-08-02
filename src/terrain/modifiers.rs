
use serde::{Deserialize,Serialize};

use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::smoothing::{SmoothingData,Smoothing};
use crate::terrain::terraces::{TerracesData, Terraces};
use crate::terrain::planes::PlaneData;
use crate::terrain::value::{ValueData, Value};
use crate::terrain::wanders::{TargetWanderNoiseData,TargetWanderNoise};
use crate::terrain::wave::{WaveData,Wave};
use crate::terrain::utils::{AreaData, Area};


// Struct needed for every modifier. Added as component to all modifiers
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    Smoothing(SmoothingData),
    Terraces(TerracesData),
    TargetWanderNoise(TargetWanderNoiseData),
    Wave(WaveData),
    Value(ValueData)
} 

impl ModifierData {
    pub fn set(&self, pd: &PlaneData) -> Modifier {
        match self {
            ModifierData::Noise(data)               => {return Modifier::Noise(data.set())}
            ModifierData::Smoothing(data)           => {return Modifier::Smoothing(data.set())}
            ModifierData::TargetWanderNoise(data)   => {return Modifier::TargetWanderNoise(data.set(pd))}
            ModifierData::Terraces(data)            => {return Modifier::Terraces(data.set())}
            ModifierData::Wave(data)                => {return Modifier::Wave(data.set())}
            ModifierData::Value(data)               => {return Modifier::Value(data.set())}
        }
    }

}


#[derive(Clone)]
pub enum Modifier {
    Noise(Noise),
    Smoothing(Smoothing),
    Terraces(Terraces),
    Value(Value),
    Wave(Wave),
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
            Modifier::Smoothing(_data)         => {pos[1]}

            // XZ only:
            Modifier::Wave(_data)              => {pos[1]}
        }           
    }


    pub fn apply_area(&mut self, v_pos: &mut Vec<[f32; 3]>){
        match self {
            Modifier::Smoothing(data)          => {data.apply(v_pos)}
            
            // point only:
            Modifier::Noise(_data)             => {}
            Modifier::Value(_data)             => {}
            Modifier::Terraces(_data)          => {}
            Modifier::TargetWanderNoise(_data) => {}

            // XZ only:
            Modifier::Wave(data)               => {data.apply(v_pos)}
        }   
    }


}
