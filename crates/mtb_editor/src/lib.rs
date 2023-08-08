
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin,Wireframe};

pub mod mtb_camera;
pub mod mtb_colors;
pub mod mtb_console;
pub mod mtb_grid;
pub mod mtb_ui;

pub mod widgets;

use mtb_console::MTBConsolePlugin;
use mtb_core::planes::PlanesPlugin;
use mtb_colors::MTBColorsPlugin;
use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;

use mtb_core::planes::TerrainPlane;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<WireFrameState>()
        .add_event::<ToggleWireframeEvent>()
        .add_plugin(WireframePlugin)
        .add_plugin(MTBColorsPlugin)
        .add_plugin(MTBConsolePlugin)
        .add_plugin(MTBCameraPlugin)
        .add_plugin(MTBGridPlugin)
        .add_plugin(MTBUIPlugin)
        .add_plugin(PlanesPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_system(toggle_wireframe.run_if(on_event::<ToggleWireframeEvent>()))
        ;
    }
  }


  pub struct ToggleWireframeEvent;

  #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
  pub enum WireFrameState {
      #[default]
      Off,
      On
  }
  

  fn toggle_wireframe(
    mut commands:        Commands,
    planes:              Query<Entity, With<TerrainPlane>>,
    wf_state:            Res<State<WireFrameState>>,
    mut next_wf_state:   ResMut<NextState<WireFrameState>>,
){
    
    match wf_state.0 {
        WireFrameState::On => {
            next_wf_state.set(WireFrameState::Off);
            for entity in planes.iter() {
                commands.entity(entity).remove::<Wireframe>();
            }
        
        }
        WireFrameState::Off => {
            next_wf_state.set(WireFrameState::On);
            for entity in planes.iter() {
                commands.entity(entity).insert(Wireframe);
            }
        }
    }
}