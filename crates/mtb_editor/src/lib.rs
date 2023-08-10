
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin,Wireframe};

pub mod mtb_camera;
pub mod mtb_colors;
pub mod mtb_console;
pub mod mtb_grid;
pub mod mtb_ui;
pub mod vertex;

pub mod widgets;

use mtb_console::MTBConsolePlugin;
use mtb_core::planes::PlanesPlugin;
use mtb_colors::MTBColorsPlugin;
use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;

use mtb_core::planes::TerrainPlane;

#[allow(unused_imports)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use vertex::{spawn_vertex, RefVertex, Vertex};

pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_state::<DisplayState>()

        .add_plugin(WireframePlugin)
        .add_plugin(MTBColorsPlugin)
        .add_plugin(MTBConsolePlugin)
        .add_plugin(MTBCameraPlugin)
        .add_plugin(MTBGridPlugin)
        .add_plugin(MTBUIPlugin)
        .add_plugin(PlanesPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_system(toggle_appstate.run_if(input_just_pressed(KeyCode::Tab)))
        .add_system(toggle_displaystate.run_if(input_just_pressed(KeyCode::Space)))
        .add_system(show_wireframe.in_schedule(OnEnter(DisplayState::Wireframe)))
        .add_system(hide_wireframe.in_schedule(OnExit(DisplayState::Wireframe)))

        .add_system(show_vertex.in_schedule(OnEnter(DisplayState::Vertex)))
        .add_system(hide_vertex.in_schedule(OnExit(DisplayState::Vertex)))
        ;
    }
 }


pub fn show_vertex(mut commands:     Commands, 
                   planes:           Query<(Entity, &Handle<Mesh>), With<TerrainPlane>>,
                   mut meshes:       ResMut<Assets<Mesh>>,
                   mut materials:    ResMut<Assets<StandardMaterial>>
                ){

    for (entity, handle_mesh) in planes.iter(){
        spawn_vertex(&entity, &mut commands, handle_mesh, &mut meshes, &mut materials);
    }


}

pub fn hide_vertex(mut commands: Commands,
                   vertex:       Query<Entity, With<Vertex>>,
                   vertex_ref:   Query<Entity, With<RefVertex>>
                ){

    for entity in vertex.iter(){
        commands.entity(entity).despawn_recursive();
    }

    for entity in vertex_ref.iter(){
        commands.entity(entity).despawn_recursive();
    }

}



pub fn show_wireframe(mut commands: Commands, planes: Query<Entity, With<TerrainPlane>>){
    for plane in planes.iter(){
        commands.entity(plane).insert(Wireframe);
    }
}

pub fn hide_wireframe(mut commands: Commands, planes: Query<Entity, With<Wireframe>>){
    for plane in planes.iter(){
        commands.entity(plane).remove::<Wireframe>();
    }
}

  #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
  pub enum DisplayState {
      #[default]
      Basic,
      Wireframe,
      Vertex,
      VertexWireframe
  }

  #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
  pub enum AppState {
      Object,
      #[default]
      Edit
  }
  
  fn toggle_appstate(
    app_state:            Res<State<AppState>>,
    mut next_app_state:   ResMut<NextState<AppState>>,
){
    match app_state.0 {
        AppState::Object => {next_app_state.set(AppState::Edit)}
        AppState::Edit => {next_app_state.set(AppState::Object);}
    }
}

fn toggle_displaystate(display_state:            Res<State<DisplayState>>,
                       mut next_display_state:   ResMut<NextState<DisplayState>>){

    match display_state.0 {
        DisplayState::Basic => {next_display_state.set(DisplayState::Wireframe)}
        DisplayState::Wireframe => {next_display_state.set(DisplayState::Vertex);}
        DisplayState::Vertex => {next_display_state.set(DisplayState::VertexWireframe);}
        DisplayState::VertexWireframe => {next_display_state.set(DisplayState::Basic);}
    }

}