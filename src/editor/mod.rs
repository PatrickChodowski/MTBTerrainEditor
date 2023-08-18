
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin,Wireframe};
use bevy_mod_picking::DefaultPickingPlugins;

pub mod boxselect;
pub mod brush;
pub mod mtb_camera;
pub mod mtb_colors;
pub mod mtb_grid;
pub mod mtb_ui;

use super::core::planes::{PlanesPlugin, TerrainPlane};
use super::core::vertex::{spawn_vertex, Vertex, VertexRefs, VertexPlugin};

use mtb_colors::MTBColorsPlugin;
use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;

pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_state::<DisplayState>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(MTBColorsPlugin)
        .add_plugins(MTBCameraPlugin)
        .add_plugins(MTBGridPlugin)
        .add_plugins(MTBUIPlugin)
        .add_plugins(PlanesPlugin)
        .add_plugins(VertexPlugin)
        .add_systems(Update, spawn_new_plane_vertex)
        .add_systems(Update, toggle_appstate.run_if(input_just_pressed(KeyCode::Tab)))
        .add_systems(Update, toggle_displaystate.run_if(input_just_pressed(KeyCode::Space)))

        .add_systems(OnEnter(DisplayState::Wireframe), show_wireframe)
        .add_systems(OnExit(DisplayState::Wireframe), hide_wireframe)

        .add_systems(OnEnter(DisplayState::Vertex), show_vertex)
        .add_systems(OnExit(DisplayState::Vertex), hide_vertex)

        .add_systems(OnEnter(DisplayState::VertexWireframe), show_vertex_wire)
        .add_systems(OnExit(DisplayState::VertexWireframe), hide_vertex_wire)
        ;
    }
 }


 pub fn spawn_new_plane_vertex(mut commands:     Commands, 
                               planes:              Query<(Entity, &Handle<Mesh>), Added<TerrainPlane>>,                    
                               mut meshes:          ResMut<Assets<Mesh>>,
                               refs:                Res<VertexRefs>){

        for (entity, handle_mesh) in planes.iter(){
            spawn_vertex(&entity, &mut commands, handle_mesh, &mut meshes, &refs);
        }
    }



 pub fn show_vertex_wire(mut commands:     Commands, 
                         planes:           Query<Entity, With<TerrainPlane>>,   
                         mut vertex:       Query<&mut Visibility, With<Vertex>>){

    for entity in planes.iter(){
        commands.entity(entity).insert(Wireframe);
    }

    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Inherited;
    }
    
}

pub fn hide_vertex_wire(mut commands: Commands, 
                        planes:       Query<Entity, With<Wireframe>>,
                        mut vertex:   Query<&mut Visibility, With<Vertex>>){
    for plane in planes.iter(){
        commands.entity(plane).remove::<Wireframe>();
    }
    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Hidden;
    }
}



pub fn show_vertex(mut vertex:       Query<&mut Visibility, With<Vertex>>){

    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Inherited;
    }
}

pub fn hide_vertex(mut vertex:   Query<&mut Visibility, With<Vertex>>){
    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Hidden;
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
      #[default]
      Object,
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

pub fn spawn_text_node(txt: &str, commands: &mut Commands, ass: &Res<AssetServer>,) -> Entity {

    let txt_style = TextStyle {
      font_size: 20.0,
      color: Color::WHITE,
      font: ass.load("fonts/lambda.ttf")
    };
  
    let txt = commands.spawn(TextBundle::from_section(txt, txt_style)).id();
    let node_txt = commands.spawn(NodeBundle{
      style: Style {
        position_type: PositionType::Relative,
        margin: UiRect::all(Val::Px(0.0)),
        padding: UiRect::all(Val::Px(0.0)),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        align_content: AlignContent::Center,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        ..default()
      },
      ..default()}).id();
  
    commands.entity(node_txt).push_children(&[txt]);
  
    return node_txt;
  
  }
  