use bevy_egui::egui::Color32;
use bevy_egui::egui::Ui;
use bevy::prelude::ResMut;
use crate::editor::mtb_ui::ModResources;


#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub color: [f32; 4],
    pub _color: Color32
}

impl Color {
    pub fn apply(&self) -> [f32; 4] {
        self.color
    }
    pub fn new() -> Self {
        let mut clr = Color{_color: Color32::LIGHT_BLUE.linear_multiply(0.5), 
                            color: [1.0, 1.0, 1.0, 1.0]};
        clr.set();
        return clr;
    }
    pub fn set(&mut self){
        self.color = [(self._color.r() as f32)/255.0, 
                      (self._color.g() as f32)/255.0, 
                      (self._color.b() as f32)/255.0, 
                      (self._color.a() as f32)/255.0];
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        ui.color_edit_button_srgba(&mut mod_res.color._color);
    }
}

