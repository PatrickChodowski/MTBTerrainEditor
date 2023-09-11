use bevy_egui::{egui, egui::Ui};
use bevy::prelude::ResMut;
use serde::{Serialize,Deserialize};
use crate::editor::mtb_ui::ModResources;


#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
    pub z: f32
}


impl Offset {
    pub fn new() -> Self {
        Offset { x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn apply(&self, loc: &[f32;3] ) -> [f32;3] {
        return [loc[0] + self.x, loc[1] + self.y, loc[2] + self.z];
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {

        ui.separator();
        ui.vertical(|ui| {
            ui.label("Offsets");
            ui.columns(2, |columns| {
              columns[1].label("X");
              columns[0].add(egui::DragValue::new(&mut mod_res.offset.x).speed(0.1));
              columns[1].label("Y");
              columns[0].add(egui::DragValue::new(&mut mod_res.offset.y).speed(0.1));
              columns[1].label("Z");
              columns[0].add(egui::DragValue::new(&mut mod_res.offset.z).speed(0.1));
            });
        });
    }
}