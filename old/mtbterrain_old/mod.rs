use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::pbr::wireframe::Wireframe;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_debug_grid::*;

pub mod mtb_grid;
pub mod editmode;
pub mod easings;
pub mod modifiers;
pub mod noises;
pub mod terraces;
pub mod planes;
pub mod value;
pub mod wanders;
pub mod utils;
pub mod smoothing;
pub mod wave;
pub mod mtb_gui;

use editmode::EditModePlugin;
use mtb_grid::GridPlugin;
use mtb_gui::MTBGuiPlugin;

use planes::{Planes, PlanesAsset, TerrainPlane, spawn_plane};
use utils::{MTBConfigData, MTBConfigAsset};

use self::{mtb_gui::DebugMode, modifiers::DebugModifierBox, editmode::ManualEdits};

pub struct MTBTerrainPlugin;

impl Plugin for MTBTerrainPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_state::<DisplayMode>()
      .add_state::<AppMode>()
      .add_plugin(TomlAssetPlugin::<Planes>::new(&["mtbscene.toml"]))
      .add_plugin(TomlAssetPlugin::<MTBConfigData>::new(&["mtbconfig.toml"]))
      .add_startup_system(setup_config)
      .add_system(setup_terrains_file_handle.run_if(on_event::<AssetEvent<MTBConfigData>>()))
      .add_system(planes_update.run_if(on_event::<AssetEvent<Planes>>()))
      .add_system(toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)))
      .add_system(toggle_appmode.run_if(input_just_pressed(KeyCode::Tab)))
      .add_plugin(GridPlugin)
      .add_plugin(MTBGuiPlugin)
      .add_plugin(EditModePlugin)
    //   .add_plugin(DebugGridPlugin::with_floor_grid())
      ;
  }
}


fn setup_terrains_file_handle(mut commands:         Commands, 
                              ass:                  Res<AssetServer>,
                              mtb_config_assets:    Res<Assets<MTBConfigData>>,
                              mtb_config_handle:    Res<MTBConfigAsset>){

    let scene_file = &mtb_config_assets.get(&mtb_config_handle.0).unwrap().scene_file;
    let path: &str = &format!("mtbterrain/scenes/{}.mtbscene.toml", scene_file);
    let planes_handle = PlanesAsset(ass.load(path));
    commands.insert_resource(planes_handle);
}


fn setup_config(mut commands:    Commands, 
                ass:             Res<AssetServer>,) {

    let config_handle = MTBConfigAsset(ass.load("mtbterrain/config.mtbconfig.toml"));
    commands.insert_resource(config_handle);

}

// generates planes
pub fn planes_update(mut commands:           Commands,
                     mut meshes:             ResMut<Assets<Mesh>>,
                     mut materials:          ResMut<Assets<StandardMaterial>>,
                     terrain_planes:         Query<Entity, With<TerrainPlane>>,
                     debug_boxes:            Query<Entity, With<DebugModifierBox>>,
                     planes_assets:          Res<Assets<Planes>>,
                     planes_handle:          Res<PlanesAsset>,
                     manual_planes:          Res<ManualEdits>,
                     display_mode:           Res<State<DisplayMode>>,
                     debug_mode:             Res<State<DebugMode>>
                    ){

    for entity in terrain_planes.iter(){
        commands.entity(entity).despawn_recursive();
    }
    for entity in debug_boxes.iter(){
        commands.entity(entity).despawn_recursive();
    }

    for pd in planes_assets.get(&planes_handle.0).unwrap().planes.iter(){
        if pd.active {
            spawn_plane(&mut commands, &mut meshes, &mut materials, &pd, &display_mode, &debug_mode); 
        }
    }

    // for pd in manual_planes.data.iter(){
    //     if pd.active {
    //         spawn_plane(&mut commands, &mut meshes, &mut materials, &pd, &display_mode, &debug_mode); 
    //     }
    // }
}



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DisplayMode {
    #[default]
    WireFrameOn,
    WireFrameOff
}

fn toggle_wireframe(
    mut commands:            Commands,
    planes:                  Query<Entity, With<TerrainPlane>>,
    display_mode:            Res<State<DisplayMode>>,
    mut next_display_mode:   ResMut<NextState<DisplayMode>>,
){
    
    match display_mode.0 {
        DisplayMode::WireFrameOn => {
            next_display_mode.set(DisplayMode::WireFrameOff);
            for entity in planes.iter() {
                commands.entity(entity).remove::<Wireframe>();
            }
        
        }
        DisplayMode::WireFrameOff => {
            next_display_mode.set(DisplayMode::WireFrameOn);
            for entity in planes.iter() {
                commands.entity(entity).insert(Wireframe);
            }
        }
    }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppMode {
    #[default]
    View,
    Edit
}

fn toggle_appmode(
    app_mode:                Res<State<AppMode>>,
    mut next_app_mode:       ResMut<NextState<AppMode>>,
){
    
    match app_mode.0 {
        AppMode::View => {
            next_app_mode.set(AppMode::Edit);        
        }
        AppMode::Edit => {
            next_app_mode.set(AppMode::View);
        }
    }
}
