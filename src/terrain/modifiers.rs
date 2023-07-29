
use serde::{Deserialize,Serialize};

use crate::terrain::easings::EasingData;
use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::planes::PlaneData;
use crate::terrain::other::{FlatEdgeData,FlatEdgesData,SmoothEdgeData,SmoothEdge,FlatEdge,FlatEdges};
use crate::terrain::wanders::{TargetWanderNoiseData,TargetWanderNoise};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModifierData {
    Easing(EasingData),
    FlatEdges(FlatEdgesData),
    FlatEdge(FlatEdgeData),
    Noise(NoiseData),
    SmoothEdge(SmoothEdgeData),
    TargetWanderNoise(TargetWanderNoiseData)
} 

impl ModifierData {
    pub fn set(&self, pd: &PlaneData) -> Modifier {
        match self {
            ModifierData::Easing(data)              => {return Modifier::Easing(data.clone())}
            ModifierData::FlatEdges(data)           => {return Modifier::FlatEdges(data.set(pd))}
            ModifierData::FlatEdge(data)            => {return Modifier::FlatEdge(data.set(pd))}
            ModifierData::Noise(data)               => {return Modifier::Noise(data.set())}
            ModifierData::TargetWanderNoise(data)   => {return Modifier::TargetWanderNoise(data.set(pd))}
            ModifierData::SmoothEdge(data)          => {return Modifier::SmoothEdge(data.set())}
        }
    }

}


#[derive(Clone)]
pub enum Modifier {
    Easing(EasingData),
    FlatEdges(FlatEdges),
    FlatEdge(FlatEdge),
    Noise(Noise),
    SmoothEdge(SmoothEdge),
    TargetWanderNoise(TargetWanderNoise)
} 

impl Modifier {

    pub fn apply_point(&self, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {
        match self {
            Modifier::Easing(data) => {return data.easing.apply(pos[1])}
            Modifier::FlatEdges(data) => {return data.apply(pos)}
            Modifier::FlatEdge(data) => {return data.apply(pos)}    
            Modifier::Noise(data) => {return data.apply(pos, loc)}
            Modifier::SmoothEdge(_data) => {pos[1]} // no impact on point
            Modifier::TargetWanderNoise(data) => {return data.apply(pos)}
        }           
    }
}
