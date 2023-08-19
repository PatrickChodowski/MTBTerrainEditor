use bevy_egui::{egui, egui::Ui};
use bevy::prelude::ResMut;
use crate::editor::mtb_ui::ModResources;
use super::noises::{Noise, NoiseFunction};

#[derive(Debug, Clone)]
pub struct Wave {
    pub noise:     Noise,
    pub scale_x:   f32,
    pub scale_z:   f32,
}

impl Wave {
    pub fn new() -> Wave {
        Wave{noise: Noise::new(), scale_x: 0.01, scale_z: 0.01}
    }
}

impl Wave {
    pub fn apply(&self, pos: &[f32; 3], noise_fn: &NoiseFunction) -> [f32; 3] {
        let nudge_x = noise_fn.apply(self.noise.scale, pos[1].into(), pos[2].into());
        let nudge_z = noise_fn.apply(self.noise.scale, pos[0].into(), pos[1].into());
        let nudged_x = nudge_x as f32*self.scale_x;
        let nudged_z = nudge_z as f32*self.scale_z;
        return [pos[0]+nudged_x, pos[1], pos[2]+nudged_z];
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {

    }
}

