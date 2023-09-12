use bevy_egui::{egui, egui::Ui};
use bevy::prelude::ResMut;
use serde::{Serialize, Deserialize};
use crate::core::easings::Easings;
use crate::editor::mtb_ui::ModResources;
use super::noises::{Noise, NoiseFunction, Noises};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn apply(&self, noise_fn: &NoiseFunction, pos: &[f32; 3]) -> [f32; 3] {
        let nudge_x = noise_fn.apply(self.noise.scale, pos[1].into(), pos[2].into());
        let nudge_z = noise_fn.apply(self.noise.scale, pos[0].into(), pos[1].into());
        let nudged_x = nudge_x as f32*self.scale_x;
        let nudged_z = nudge_z as f32*self.scale_z;
        return [pos[0]+nudged_x, pos[1], pos[2]+nudged_z];
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        
        
        ui.label("Wave");
        ui.separator();

        ui.columns(2, |columns| {
            columns[1].label("Scale X");
            columns[0].add(egui::DragValue::new(&mut mod_res.wave.scale_x).speed(0.1));
            columns[1].label("Scale Z");
            columns[0].add(egui::DragValue::new(&mut mod_res.wave.scale_z).speed(0.1));
        });

        ui.separator();
        egui::ComboBox::from_label("Noise")
        .width(140.0)
        .selected_text(format!("{:?}", mod_res.wave.noise.noise))
        .show_ui(ui, |ui| {
          for &p in Noises::iterator(){
            ui.selectable_value(&mut mod_res.wave.noise.noise, p, format!("{p:?}"));
          }
        });

        ui.separator();

        ui.columns(2, |columns| {
          columns[1].label("Seed");
          columns[0].add(egui::DragValue::new(&mut mod_res.wave.noise.seed).speed(1.0));
          columns[1].label("Scale");
          columns[0].add(egui::DragValue::new(&mut mod_res.wave.noise.scale).speed(1.0));
          columns[1].label("Frequency");
          columns[0].add(egui::DragValue::new(&mut mod_res.wave.noise.freq).speed(0.1));
          columns[1].label("Octaves");
          columns[0].add(egui::DragValue::new(&mut mod_res.wave.noise.octaves).speed(1.0));
        });

        egui::ComboBox::from_label("Easing")
        .width(140.0)
        .selected_text(format!("{:?}", mod_res.wave.noise.easing))
        .show_ui(ui, |ui| {
          for &p in Easings::iterator(){
            ui.selectable_value(&mut mod_res.wave.noise.easing, p, format!("{p:?}"));
          }
        });
        ui.checkbox(&mut mod_res.wave.noise.global, "Use global position?");
    
    }
}

