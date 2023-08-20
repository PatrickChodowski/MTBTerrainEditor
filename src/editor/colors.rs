
use bevy::{prelude::*, utils::HashSet};
use bevy_egui::egui::{Ui, RichText, Color32, Grid};
use crate::editor::mtb_ui::ModResources;

use super::mtb_ui::ModifierState;

pub struct ColorsPlugin;

impl Plugin for ColorsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Colors::new())
      ;                      
    }
  }

pub fn to_clr32(aclr: &[u8; 4]) -> Color32 {
    Color32::from_rgba_unmultiplied(aclr[0] as u8, 
                                    aclr[1] as u8, 
                                    aclr[2] as u8, 
                                    aclr[3] as u8)
// cant really tell why those colors look this way, something with linear rgba blah blah blah
}

pub fn to_f32_norm(aclr: &[u8; 4]) -> [f32; 4] {
    [(aclr[0] as f32)/255.0, 
     (aclr[1] as f32)/255.0, 
     (aclr[2] as f32)/255.0, 
     (aclr[3] as f32)/255.0]
}



#[derive(Resource, Clone, Debug)]
pub struct Colors {
    pub data:       HashSet<[u8;4]>,
    pub mod_color:  ModifierColor
}
impl Colors {
    pub fn addf32(&mut self, clr: &[f32; 4]) {
        self.data.insert([(clr[0]*255.0) as u8, 
                          (clr[1]*255.0) as u8, 
                          (clr[2]*255.0) as u8, 
                          (clr[3]*255.0) as u8]);
    }


    pub fn new() -> Self {
        Colors{data: HashSet::new(), mod_color: ModifierColor::Color}
    }
    pub fn ui(&self, 
                ui:         &mut Ui, 
                mod_res:    &mut ResMut<ModResources>,
                mod_state:  ModifierState,
                mod_color:  ModifierColor
            ) {

    let ncols = 3;    

    Grid::new("ColorGrid").min_row_height(25.0).show(ui, |ui| {
        for (index, clr) in self.data.iter().enumerate(){
            if index % ncols == 0 && index > 0 {
                ui.end_row();
            }
            if ui.button(RichText::new("BUTTO").background_color(to_clr32(clr))
                                               .color(to_clr32(clr))).clicked() {
                    match (mod_state, mod_color) {
                        (ModifierState::Color, _)          => {
                            mod_res.color.color = to_f32_norm(clr);
                        }
                        (ModifierState::ColorGradient, ModifierColor::GradientMin) => {
                            mod_res.color_gradient.min_color = to_f32_norm(clr);
                        }
                        (ModifierState::ColorGradient, ModifierColor::GradientMax) => {
                            mod_res.color_gradient.max_color = to_f32_norm(clr);
                        }
                        (_,_)                          => {}
                    }
                }
            }
        });
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ModifierColor {
    Color,
    GradientMin,
    GradientMax
}