
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin,Wireframe};
use bevy::pbr::{CascadeShadowConfigBuilder, ScreenSpaceAmbientOcclusionBundle};
use bevy::time::Stopwatch;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_egui::{egui, egui::Ui};
use serde::{Serialize, Deserialize};


pub mod actions;
pub mod boxselect;
pub mod brush;
pub mod mtb_camera;
pub mod mtb_grid;
pub mod mtb_ui;
pub mod io;
pub mod colors;

use super::core::planes::{PlanesPlugin, TerrainPlane};
use super::core::vertex::{spawn_vertex, Vertex, VertexRefs, VertexPlugin};

use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;

pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AppState>()
        .add_state::<DisplayState>()
        .add_event::<DoubleClick>()
        .insert_resource(DoubleClickTimer::new())
        .insert_resource(GlobalSettings::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(MTBCameraPlugin)
        .add_plugins(MTBGridPlugin)
        .add_plugins(MTBUIPlugin)
        .add_plugins(PlanesPlugin)
        .add_plugins(VertexPlugin)
        .add_systems(Startup, spawn_lights)
        .add_systems(Update,  update_lights.run_if(is_settings_changed))

        .add_systems(Update, spawn_new_plane_vertex)
        .add_systems(Update, toggle_appstate.run_if(input_just_pressed(KeyCode::Tab)))
        .add_systems(Update, toggle_displaystate.run_if(input_just_pressed(KeyCode::Space)))

        .add_systems(OnEnter(DisplayState::Wireframe), show_wireframe)
        .add_systems(OnExit(DisplayState::Wireframe), hide_wireframe)

        .add_systems(OnEnter(DisplayState::Vertex), show_vertex)
        .add_systems(OnExit(DisplayState::Vertex), hide_vertex)

        .add_systems(OnEnter(DisplayState::VertexWireframe), show_vertex_wire)
        .add_systems(OnExit(DisplayState::VertexWireframe), hide_vertex_wire)

        .add_systems(Update, record_dbl_click)
        ;
    }
 }


#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub dir_light:      DirLightData,
    pub ambient_light:  AmbientLightData,
    pub vertex_radius:  f32
}
impl GlobalSettings {
    pub fn new() -> Self {
        GlobalSettings {
            vertex_radius: 3.0,
            ambient_light: AmbientLightData::new(),
            dir_light:     DirLightData::new()
        }
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Settings");
        ui.separator();

        ui.label("Vertex Radius");
        ui.add(egui::DragValue::new(&mut self.vertex_radius).speed(1.0));
        ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        ui.label("Ambient light");
        ui.checkbox(&mut self.ambient_light.active, "Active");
        ui.add(egui::DragValue::new(&mut self.ambient_light.brightness).speed(1.0));
        ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        ui.label("Directional light");
        ui.checkbox(&mut self.dir_light.active, "Active");

        ui.columns(2, |columns| {
            columns[0].label("Illuminance");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.illuminance).speed(1.0));
            columns[0].label("Max Distance");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.max_dist).speed(1.0));
            columns[0].label("Loc X");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.loc[0]).speed(1.0));
            columns[0].label("Loc Y");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.loc[1]).speed(1.0));
            columns[0].label("Loc Z");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.loc[2]).speed(1.0));
            columns[0].label("LookAt X");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.look_at[0]).speed(1.0));
            columns[0].label("LookAt Y");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.look_at[1]).speed(1.0));
            columns[0].label("LookAt Z");
            columns[1].add(egui::DragValue::new(&mut self.dir_light.look_at[2]).speed(1.0));
          });
          ui.checkbox(&mut self.dir_light.shadows_enabled, "Shadows Enabled");

    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbientLightData {
    pub active:     bool,
    pub brightness: f32
}
impl AmbientLightData {
    pub fn new() -> Self {
        AmbientLightData { active: true, brightness: 5.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirLightData {
    pub loc:                [f32; 3],
    pub look_at:            [f32; 3],
    pub max_dist:           f32,
    pub illuminance:        f32,
    pub shadows_enabled:    bool,
    pub active:             bool
}
impl DirLightData {
    pub fn new() -> Self {
        DirLightData {active: true, 
                      loc: [0.0, 2500.0, 1000.0],
                      look_at: [0.0, 0.0, 0.0],
                      illuminance: 25000.0,
                      max_dist: 1000.0,
                      shadows_enabled: true
                    }
    }
}

pub fn spawn_lights(mut commands:     Commands,
                    settings:         Res<GlobalSettings>) {

    commands.spawn(ScreenSpaceAmbientOcclusionBundle {..default()});

    if settings.ambient_light.active {
        commands.insert_resource(AmbientLight {color: Color::WHITE, 
                                               brightness: settings.ambient_light.brightness});
    }

    if settings.dir_light.active {
        let loc = settings.dir_light.loc;
        let lookat = settings.dir_light.look_at;
        
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: settings.dir_light.illuminance,
                shadows_enabled: settings.dir_light.shadows_enabled,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(loc[0], loc[1], loc[2])
                                  .looking_at([lookat[0], lookat[1], lookat[2]].into(), Vec3::Y),
              
            cascade_shadow_config: CascadeShadowConfigBuilder {
                maximum_distance: settings.dir_light.max_dist,
                ..default()
            }
            .into(),
            ..default()
        });

    }
}

pub fn update_lights(settings:      Res<GlobalSettings>, 
                     mut amb_light: ResMut<AmbientLight>,
                     mut dir_light: Query<&mut DirectionalLight>
                    ) {

    if settings.ambient_light.active {
        amb_light.brightness = settings.ambient_light.brightness;
    } else {
        amb_light.brightness = 0.0;
    }

    for mut dl in dir_light.iter_mut(){
        if settings.dir_light.active {
            dl.illuminance = settings.dir_light.illuminance
        }
        else {
            dl.illuminance = 0.0;
        }
    }

}

pub fn is_settings_changed(settings: Res<GlobalSettings>) -> bool {
    settings.is_changed()
}


 #[derive(Event)]
 pub struct DoubleClick;

 #[derive(Resource)]
 pub struct DoubleClickTimer {
    pub clicked: bool,
    pub timer:   Stopwatch
 }
 impl DoubleClickTimer {
    pub fn new() -> Self {
        DoubleClickTimer { clicked: false, timer: Stopwatch::new() }
    }
 }
 

 pub fn record_dbl_click(time:              Res<Time>, 
                         mouse:             Res<Input<MouseButton>>,
                         mut dblc_timer:    ResMut<DoubleClickTimer>,
                         mut dbl_click:     EventWriter<DoubleClick>){

    dblc_timer.timer.tick(time.delta());
    if dblc_timer.timer.elapsed_secs() > 0.2 {
        dblc_timer.clicked = false;
    }

    if mouse.just_pressed(MouseButton::Left) {
        match dblc_timer.clicked {
            false => {dblc_timer.clicked = true}
            true  => {
                dblc_timer.clicked = false;
                dbl_click.send(DoubleClick)
            }
        }
        dblc_timer.timer.reset();
    }

 }


 pub fn spawn_new_plane_vertex(mut commands:     Commands, 
                               planes:           Query<(Entity, &Handle<Mesh>), Added<TerrainPlane>>,                    
                               mut meshes:       ResMut<Assets<Mesh>>,
                               refs:             Res<VertexRefs>,
                               settings:         Res<GlobalSettings>
                            ){

        for (entity, handle_mesh) in planes.iter(){
            info!("spawning vertices");
            spawn_vertex(&entity, &mut commands, handle_mesh, &mut meshes, &refs, &settings);
        }
    }



 pub fn show_vertex_wire(mut commands:     Commands, 
                         planes:           Query<(Entity, &mut Handle<StandardMaterial>), With<TerrainPlane>>,   
                         mut materials:    ResMut<Assets<StandardMaterial>>,
                         mut vertex:       Query<&mut Visibility, With<Vertex>>){

    for (entity, handle_mat) in planes.iter(){
        if let Some(mat) = materials.get_mut(handle_mat){
            mat.base_color.set_a(0.1);
        }
        commands.entity(entity).insert(Wireframe);

    }

    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Inherited;
    }
    
}

pub fn hide_vertex_wire(mut commands:     Commands, 
                        mut materials:    ResMut<Assets<StandardMaterial>>,
                        planes:           Query<(Entity, &mut Handle<StandardMaterial>), With<Wireframe>>,
                        mut vertex:       Query<&mut Visibility, With<Vertex>>){

    for (entity, handle_mat) in planes.iter(){
        commands.entity(entity).remove::<Wireframe>();
        if let Some(mat) = materials.get_mut(handle_mat){
            mat.base_color.set_a(1.0);
        }
    }
    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Hidden;
    }
}



pub fn show_vertex(mut vertex: Query<&mut Visibility, With<Vertex>>){

    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Inherited;
    }
}

pub fn hide_vertex(mut vertex:   Query<&mut Visibility, With<Vertex>>){
    for mut vis in vertex.iter_mut(){
        *vis = Visibility::Hidden;
    }
}

pub fn show_wireframe(mut commands: Commands, 
                      mut materials:    ResMut<Assets<StandardMaterial>>,
                      planes: Query<(Entity, &mut Handle<StandardMaterial>), With<TerrainPlane>>){

    for (entity, handle_mat) in planes.iter(){
        if let Some(mat) = materials.get_mut(handle_mat){
            mat.base_color.set_a(0.1);
        }
        commands.entity(entity).insert(Wireframe);
    }
}

pub fn hide_wireframe(mut commands:     Commands,
                      mut materials:    ResMut<Assets<StandardMaterial>>, 
                      planes:           Query<(Entity, &mut Handle<StandardMaterial>), With<Wireframe>>){

    for (entity, handle_mat) in planes.iter(){
        if let Some(mat) = materials.get_mut(handle_mat){
            mat.base_color.set_a(1.0);
        }
        commands.entity(entity).remove::<Wireframe>();
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
  