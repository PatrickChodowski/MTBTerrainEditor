
use bevy::prelude::*;
use serde::{Deserialize,Serialize};

use super::mtb_gui::DebugMode;
use super::noises::{NoiseData, Noise};
use super::smoothing::{SmoothingData,Smoothing};
use super::terraces::{TerracesData, Terraces};
use super::planes::PlaneData;
use super::value::{ValueData, Value};
use super::wanders::{TargetWanderNoiseData,TargetWanderNoise};
use super::wave::{WaveData,Wave};
use super::utils::{AreaData, Area};


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

    pub fn spawn_debug(&self, 
                       commands:    &mut Commands, 
                       meshes:      &mut ResMut<Assets<Mesh>>, 
                       materials:   &mut ResMut<Assets<StandardMaterial>>,
                       debug_mode:  &Res<State<DebugMode>>
                    ) {
        
        let mut spawn: bool = true;
        let mut clr: Color = Color::WHITE;
        let mut loc: [f32;2] = [0.0, 0.0]; 
        let mut dims: (f32, f32) = (10.0, 10.0);
        const HEIGHT: f32 = 20.0;
        
        match &self {
            ModifierData::Wave(data)      => {
                clr = Color::PINK; 
                loc = data.mb.loc;
                dims = data.mb.area.get_dims();
            }
            ModifierData::Smoothing(data) => {
                clr = Color::GREEN; 
                loc = data.mb.loc;
                dims = data.mb.area.get_dims();
            }
            _ => {spawn=false;}
        }

        if spawn {
            let vis: Visibility;
            match &debug_mode.0 {
                DebugMode::DebugOn  => {vis = Visibility::Inherited;}
                DebugMode::DebugOff => {vis = Visibility::Hidden;}
            }
            clr.set_a(0.5);
            commands.spawn((PbrBundle {
                mesh: meshes.add(shape::Box::new(dims.0, HEIGHT, dims.1).into()),
                material: materials.add(clr.into()),
                transform: Transform::from_xyz(loc[0], 0.0, loc[1]),
                ..default()
            }, DebugModifierBox))
            .insert(vis);
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



#[derive(Component)]
pub struct DebugModifierBox;