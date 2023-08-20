
use bevy_egui::egui::{DragValue, Ui};
use bevy::prelude::ResMut;
use crate::editor::mtb_ui::ModResources;


#[derive(Debug, Clone, Copy)]
pub struct ColorGradient {
    pub min_height: f32,
    pub max_height: f32,
    pub min_color: [f32; 4],
    pub max_color: [f32; 4],
}


impl ColorGradient {
    pub fn apply(&self, height: f32) -> [f32; 4] {
        let scale = (height - self.min_height)/(self.max_height - self.min_height);
        return [self.min_color[0] + scale*(self.max_color[0] - self.min_color[0]), 
                self.min_color[1] + scale*(self.max_color[1] - self.min_color[1]),
                self.min_color[2] + scale*(self.max_color[2] - self.min_color[2]),
                self.min_color[3] + scale*(self.max_color[3] - self.min_color[3])];

    }
    pub fn new() -> Self {
        ColorGradient{min_color: [0.0, 0.0, 0.0, 1.0],
                      max_color: [1.0, 1.0, 1.0, 1.0],
                      min_height: 0.0,
                      max_height: 10.0}
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        ui.vertical(|ui| {
            ui.label("Color Gradient");
            ui.label("Min Height:");
            ui.add(DragValue::new(&mut mod_res.color_gradient.min_height).speed(1.0));
            ui.label("Max Height:");
            ui.add(DragValue::new(&mut mod_res.color_gradient.max_height).speed(1.0));
            ui.label("Min Color:");
            ui.color_edit_button_rgba_unmultiplied(&mut mod_res.color_gradient.min_color);
            ui.label("Max Color:");
            ui.color_edit_button_rgba_unmultiplied(&mut mod_res.color_gradient.max_color);
          });
    }
}

