use bevy_egui::egui::Ui;
use bevy::prelude::ResMut;
use crate::editor::mtb_ui::ModResources;


#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub color: [f32; 4],
}

impl Color {
    pub fn apply(&self) -> [f32; 4] {
        self.color
    }
    pub fn new() -> Self {
        Color{color: [1.0, 1.0, 1.0, 1.0]}
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        ui.color_edit_button_rgba_unmultiplied(&mut mod_res.color.color);
    }
}

