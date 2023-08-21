use bevy_egui::egui::{DragValue, Ui, Context, RichText};
use bevy::prelude::ResMut;
use crate::editor::{mtb_ui::ModResources, colors::Colors, colors::f32_to_clr32};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub color: [f32; 4],
    pub open:   bool,
}

impl Color {
    pub fn apply(&self) -> [f32; 4] {
        self.color
    }
    pub fn new() -> Self {
        Color{color: [1.0, 1.0, 1.0, 1.0], open: false}
    }

    pub fn ui(ctx: &Context, ui: &mut Ui, mod_res: &mut ResMut<ModResources>, colors: &mut ResMut<Colors>){
        ui.label("Color:");
        ui.toggle_value(&mut mod_res.color.open, "Pick color");
        colors.show(ctx, &mut mod_res.color.open, "Color");
        mod_res.color.color = colors.input;
    }
}




#[derive(Debug, Clone, Copy)]
pub struct ColorGradient {
    pub min_height: f32,
    pub max_height: f32,
    pub min_color: [f32; 4],
    pub max_color: [f32; 4],
    pub min_open:   bool,
    pub max_open:   bool
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
                      max_height: 10.0,
                      min_open: false,
                      max_open: false}
    }

    pub fn ui(ctx: &Context, ui: &mut Ui, mod_res: &mut ResMut<ModResources>, colors: &mut ResMut<Colors>) {
        ui.vertical(|ui| {
            ui.label("Color Gradient");
            ui.separator();

            ui.columns(2, |columns| {
                columns[0].label("Min Height:");
                columns[0].add(DragValue::new(&mut mod_res.color_gradient.min_height).speed(1.0));
                columns[1].label("Max Height:");
                columns[1].add(DragValue::new(&mut mod_res.color_gradient.max_height).speed(1.0));
            });
            
            ui.separator();

            ui.columns(2, |columns| {
                columns[0].label("Min Color:");
                columns[1].label("Max Color:");

                columns[0].toggle_value(&mut mod_res.color_gradient.min_open,  "Pick min color");
                colors.show(ctx, &mut mod_res.color_gradient.min_open, "Gradient Min Color");
                if mod_res.color_gradient.min_open {
                    mod_res.color_gradient.max_open = false;
                    if colors.clicked {
                        mod_res.color_gradient.min_color = colors.input;
                        colors.clicked = false;
                    }
                }


                columns[1].toggle_value(&mut mod_res.color_gradient.max_open,  "Pick max color");            
                colors.show(ctx, &mut mod_res.color_gradient.max_open, "Gradient Max Color");
                if mod_res.color_gradient.max_open {
                    mod_res.color_gradient.min_open = false;
                    if colors.clicked {
                        mod_res.color_gradient.max_color = colors.input;
                        colors.clicked = false;
                    }
                }

                let _ = columns[0].button(RichText::new("_____________")
                          .background_color(f32_to_clr32(&mod_res.color_gradient.min_color))
                          .color(f32_to_clr32(&mod_res.color_gradient.min_color)));

                let _ = columns[1].button(RichText::new("_____________")
                          .background_color(f32_to_clr32(&mod_res.color_gradient.max_color))
                          .color(f32_to_clr32(&mod_res.color_gradient.max_color)));

            });
          });
    }
}

