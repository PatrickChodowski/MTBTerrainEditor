// Moves vertices x's and z's positions slightly
use serde::{Deserialize,Serialize};
use bevy::utils::HashMap;

use crate::terrain::modifiers::ModifierBase;
use crate::terrain::utils::Area;
use crate::terrain::noises::{SimpleNoiseData, SimpleNoise};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveData {
    pub mb:        ModifierBase,
    pub noise:     SimpleNoiseData
}

impl WaveData {
    pub fn set(&self) -> Wave {
        Wave{area: self.mb.to_area(), noise: self.noise.set()}
    }
}

#[derive(Clone)]
pub struct Wave {
    pub area:       Area,
    pub noise:      SimpleNoise
}

impl Wave {
    pub fn apply(&self, v_pos: &mut Vec<[f32; 3]>) {
        let mut points: HashMap<usize, [f32; 3]> = HashMap::new();
        for (index, pos) in v_pos.iter().enumerate(){
            if self.area.has_point(pos){
                points.insert(index, *pos);
            }  
        }


        for (index, pos) in points.iter(){
            let nudge = self.noise.apply(pos[1], pos[2]);
            v_pos[*index] = [pos[0]+nudge, pos[1], pos[2]];
        }



    }
}

