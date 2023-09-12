
use bevy_egui::egui::{Ui, DragValue};
use bevy::prelude::ResMut;
use serde::{Serialize, Deserialize};
use crate::editor::mtb_ui::ModResources;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrace {
  pub from:   f32,
  pub to:     f32,
  pub value:  f32
}


impl Terrace {
  pub fn new() -> Self {
    Terrace { from: 0.0, to: 10.0, value: 5.0 }
  }
  pub fn apply(&self, v: f32) -> f32 {
    if v >= self.from && v < self.to {
      return self.value;
    } else {
      return v;
    }
  } 
  pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
    ui.label("Terrace");
    ui.separator();

    ui.columns(2, |columns| {
        columns[1].label("From");
        columns[0].add(DragValue::new(&mut mod_res.terrace.from).speed(1.0));
        columns[1].label("To");
        columns[0].add(DragValue::new(&mut mod_res.terrace.to).speed(1.0));
        columns[1].label("Value");
        columns[0].add(DragValue::new(&mut mod_res.terrace.value).speed(1.0));
    });
  }
}