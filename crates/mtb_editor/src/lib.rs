
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin,Wireframe};
use bevy_mod_picking::DefaultPickingPlugins;

pub mod boxselect;
pub mod brush;
pub mod mtb_camera;
pub mod mtb_colors;
pub mod mtb_console;
pub mod mtb_grid;
pub mod mtb_ui;
pub mod vertex;

pub mod widgets;

use mtb_console::{MTBConsolePlugin, ConsoleState};
use mtb_core::planes::PlanesPlugin;
use mtb_colors::MTBColorsPlugin;
use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;

use mtb_core::planes::TerrainPlane;
use vertex::{spawn_vertex, Vertex, VertexRefs, VertexPlugin};

pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_state::<DisplayState>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(MTBColorsPlugin)
        .add_plugins(MTBConsolePlugin)
        .add_plugins(MTBCameraPlugin)
        .add_plugins(MTBGridPlugin)
        .add_plugins(MTBUIPlugin)
        .add_plugins(PlanesPlugin)
        .add_plugins(VertexPlugin)
        .add_systems(Update, toggle_appstate.run_if(input_just_pressed(KeyCode::Tab)))
        .add_systems(Update, toggle_displaystate.run_if(input_just_pressed(KeyCode::Space).and_then(in_state(ConsoleState::Off))))

        .add_systems(OnEnter(DisplayState::Wireframe), show_wireframe)
        .add_systems(OnExit(DisplayState::Wireframe), hide_wireframe)

        .add_systems(OnEnter(DisplayState::Vertex), show_vertex)
        .add_systems(OnExit(DisplayState::Vertex), hide_vertex)

        .add_systems(OnEnter(DisplayState::VertexWireframe), show_vertex_wire)
        .add_systems(OnExit(DisplayState::VertexWireframe), hide_vertex_wire)
        ;
    }
 }

 pub fn show_vertex_wire(mut commands:     Commands, 
                         planes:           Query<(Entity, &Handle<Mesh>), With<TerrainPlane>>,                    
                         mut meshes:       ResMut<Assets<Mesh>>,
                         refs:             Res<VertexRefs>){

    for (entity, handle_mesh) in planes.iter(){
        commands.entity(entity).insert(Wireframe);
        spawn_vertex(&entity, &mut commands, handle_mesh, &mut meshes, &refs);
    }
    
}

pub fn hide_vertex_wire(mut commands: Commands, 
                        planes:       Query<Entity, With<Wireframe>>,
                        vertex:       Query<Entity, With<Vertex>>){
    for plane in planes.iter(){
        commands.entity(plane).remove::<Wireframe>();
    }
    for entity in vertex.iter(){
        commands.entity(entity).despawn_recursive();
    }
}



pub fn show_vertex(mut commands:     Commands, 
                   planes:           Query<(Entity, &Handle<Mesh>), With<TerrainPlane>>,
                   mut meshes:       ResMut<Assets<Mesh>>,
                   refs:             Res<VertexRefs>
                ){

    for (entity, handle_mesh) in planes.iter(){
        spawn_vertex(&entity, &mut commands, handle_mesh, &mut meshes, &refs);
    }
}

pub fn hide_vertex(mut commands: Commands,
                   vertex:       Query<Entity, With<Vertex>>
                ){

    for entity in vertex.iter(){
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
    match app_state.get() {
        AppState::Object => {next_app_state.set(AppState::Edit)}
        AppState::Edit => {next_app_state.set(AppState::Object);}
    }
}

fn toggle_displaystate(display_state:            Res<State<DisplayState>>,
                       mut next_display_state:   ResMut<NextState<DisplayState>>){

    match display_state.get() {
        DisplayState::Basic => {next_display_state.set(DisplayState::Wireframe)}
        DisplayState::Wireframe => {next_display_state.set(DisplayState::Vertex);}
        DisplayState::Vertex => {next_display_state.set(DisplayState::VertexWireframe);}
        DisplayState::VertexWireframe => {next_display_state.set(DisplayState::Basic);}
    }

}