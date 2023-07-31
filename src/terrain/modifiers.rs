
use serde::{Deserialize,Serialize};

use crate::terrain::easings::EasingData;
use crate::terrain::noises::{NoiseData, Noise};
use crate::terrain::other::TerracesData;
use crate::terrain::planes::PlaneData;
use crate::terrain::shapes::{ShapeData, Shape};
use crate::terrain::wanders::{TargetWanderNoiseData,TargetWanderNoise};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModifierData {
    Easing(EasingData),
    Noise(NoiseData),
    Shape(ShapeData),
    Terraces(TerracesData),
    TargetWanderNoise(TargetWanderNoiseData)
} 

impl ModifierData {
    pub fn set(&self, pd: &PlaneData, v_pos: &Vec<[f32; 3]>) -> Modifier {
        match self {
            ModifierData::Easing(data)              => {return Modifier::Easing(data.clone())}
            ModifierData::Noise(data)               => {return Modifier::Noise(data.set())}
            ModifierData::Shape(data)               => {return Modifier::Shape(data.set())}
            ModifierData::TargetWanderNoise(data)   => {return Modifier::TargetWanderNoise(data.set(pd))}
            ModifierData::Terraces(data)            => {return Modifier::Terraces(data.clone())}
        }
    }

}


#[derive(Clone)]
pub enum Modifier {
    Easing(EasingData),
    Noise(Noise),
    Shape(Shape),
    // SmoothArea,
    Terraces(TerracesData),
    TargetWanderNoise(TargetWanderNoise)
} 

impl Modifier {

    pub fn apply_point(&self, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {
        match self {
            Modifier::Easing(data)              => {return data.easing.apply(pos[1])}
            Modifier::Noise(data)               => {return data.apply(pos, loc)}
            Modifier::Shape(data)               => {return data.apply(pos)}
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
            Modifier::Easing(_data)            => {} 
            Modifier::Noise(_data)             => {}
            Modifier::Shape(_data)             => {}
            Modifier::Terraces(_data)          => {}
            Modifier::TargetWanderNoise(_data) => {}
        }   
    }


}
