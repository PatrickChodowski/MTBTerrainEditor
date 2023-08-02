// Moves vertices x's and z's positions slightly
use serde::{Deserialize,Serialize};
use bevy::utils::HashMap;

use crate::terrain::modifiers::ModifierBase;
use crate::terrain::utils::Area;
use crate::terrain::noises::{SimpleNoiseData, SimpleNoise};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveData {
    pub mb:        ModifierBase,
    pub noise:     SimpleNoiseData,
    pub scale_x:   f32,
    pub scale_z:   f32,
}

impl WaveData {
    pub fn set(&self) -> Wave {
        Wave{area: self.mb.to_area(), noise: self.noise.set(), scale_x: self.scale_x, scale_z: self.scale_z}
    }
}

#[derive(Clone)]
pub struct Wave {
    pub area:       Area,
    pub noise:      SimpleNoise,
    pub scale_x:   f32,
    pub scale_z:   f32,
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
            let nudge_x = self.noise.apply(pos[1], pos[2]);
            let nudge_z = self.noise.apply(pos[0], pos[1]);
            let nudged_x = nudge_x*self.scale_x;
            let nudged_z = nudge_z*self.scale_z;
            v_pos[*index] = [pos[0]+nudged_x, pos[1], pos[2]+nudged_z];
        }

    }
}

