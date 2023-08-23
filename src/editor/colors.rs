
use bevy::{prelude::*, utils::HashSet};
use bevy_egui::egui::{Ui, RichText, Color32, Grid, Context, Window, DragValue};

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

pub fn f32_to_clr32(aclr: &[f32; 4]) -> Color32 {
    Color32::from_rgba_unmultiplied((aclr[0]*255.0) as u8, 
                                    (aclr[1]*255.0) as u8, 
                                    (aclr[2]*255.0) as u8, 
                                    (aclr[3]*255.0) as u8)
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
    pub selects:       HashSet<[u8;4]>,
    pub input:         [f32; 4], 
    pub clicked:       bool
}
impl Colors {
    pub fn addf32(&mut self, clr: &[f32; 4]) {
        let clr_conv = [(clr[0]*255.0) as u8, 
                        (clr[1]*255.0) as u8, 
                        (clr[2]*255.0) as u8, 
                        (clr[3]*255.0) as u8];
        self.selects.insert(clr_conv);
        
        //info!("debug added color: {:?}", clr_conv);
        //info!("debug all colors: {:?}", self);
    }

    pub fn reset(&mut self) {
        self.input = [1.0, 1.0, 1.0, 1.0];
        self.clicked = false;
    }

    pub fn new() -> Self {
        Colors{selects: HashSet::new(), input: [1.0, 1.0, 1.0, 1.0], clicked: false}
    }

    pub fn show(&mut            self, 
                ctx:            &Context, 
                open:           &mut bool,
                label:          &str
            ) {

        Window::new(format!("Color Selection {}", label))
                .open(open)
                .resizable(true)
                .default_width(280.0)
                .show(ctx, |ui| {
                    self.ui(ui);
                });
    }



    pub fn ui(&mut self, ui: &mut Ui) {

        if ui.button("Confirm").clicked(){
            self.addf32(&self.input.clone());
            self.clicked = true;
        }

        ui.label("Insert values manually: ");
        ui.columns(2, |columns| {
            columns[1].label("Red");
            columns[0].add(DragValue::new(&mut self.input[0]).speed(1.0));
            columns[1].label("Green");
            columns[0].add(DragValue::new(&mut self.input[1]).speed(1.0));
            columns[1].label("Blue");
            columns[0].add(DragValue::new(&mut self.input[2]).speed(1.0));
            columns[1].label("Alpha");
            columns[0].add(DragValue::new(&mut self.input[3]).speed(1.0));
        });

        ui.separator();

        ui.label("Use color picker: ");
        ui.color_edit_button_rgba_unmultiplied(&mut self.input);

        ui.separator();

        ui.label("Or pick from already created colors: ");
        let ncols = 4;  
        Grid::new("ColorGrid").min_row_height(25.0).show(ui, |ui| {
            for (index, clr) in self.selects.iter().enumerate(){
                if index % ncols == 0 && index > 0 {
                    ui.end_row();
                }
                if ui.button(RichText::new("BUTTO").background_color(to_clr32(clr))
                                                   .color(to_clr32(clr))).clicked() {
                        self.input = to_f32_norm(clr);
                    }
                }
            });
        }
}
