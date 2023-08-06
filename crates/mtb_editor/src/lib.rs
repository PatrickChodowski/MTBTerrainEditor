
use bevy::prelude::*;

pub mod mtb_camera;
pub mod mtb_grid;
pub mod mtb_ui;

use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;


pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(MTBCameraPlugin)
        .add_plugin(MTBGridPlugin)
        .add_plugin(MTBUIPlugin)
        ;
    }
  }


