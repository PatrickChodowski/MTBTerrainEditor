
use serde::{Deserialize,Serialize};

use crate::terrain::easings::EasingData;
use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::other::{FlatEdgeData,FlatEdgesData,SmoothEdgeData,SmoothEdge,FlatEdge,FlatEdges};
use crate::terrain::planes::PlaneData;
use crate::terrain::utils::{EdgeLine, AABB};
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
            Modifier::TargetWanderNoise(data) => {return data.apply(pos)}

            // Area only:
            Modifier::SmoothEdge(_data) => {pos[1]}
        }           
    }

    pub fn apply_area(&self, v_pos: &mut Vec<[f32; 3]>, edges: &Vec<EdgeLine> ){
        match self {
            Modifier::SmoothEdge(data) => {
                println!("edges: {:?}", edges);
                println!("buffer: {:?}", data.buffer);
                println!("self edges: {:?}", data.edges);

            }

            // point only:
            Modifier::Easing(_data) => {} 
            Modifier::FlatEdges(_data) => {}
            Modifier::FlatEdge(_data) => {}
            Modifier::Noise(_data) => {}
            Modifier::TargetWanderNoise(_data) => {}
        }   
    }

    // Everything that has aabb can produce edgelines
    pub fn get_inner_edges(&self, plane: &AABB) -> Vec<EdgeLine> {
        let v: Vec<EdgeLine> = Vec::new();

        match self {
            Modifier::FlatEdges(data) => {
                return data.aabbs.to_edges(plane);
            }
            Modifier::FlatEdge(data) => {
                return data.aabbs.to_edges(plane);
            }
            Modifier::TargetWanderNoise(_data) => {}

            // not used
            Modifier::Easing(_data) => {} 
            Modifier::Noise(_data) => {}
            Modifier::SmoothEdge(_data) => {}
        }

        return v;
    }

}
