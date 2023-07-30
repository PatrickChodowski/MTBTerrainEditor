use bevy::input::common_conditions::input_just_pressed;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::PresentMode;
use bevy::window::WindowPlugin;
use bevy::log::LogPlugin;
use bevy::window::WindowResolution;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_common_assets::toml::TomlAssetPlugin;

mod camera;
use camera::CameraPlugin;

mod terrain;
use terrain::planes::TerrainPlane;
use terrain::planes::{PlanesPlugin, Planes};
use terrain::utils::ConfigData;

pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                    title: "Terrain Generator".to_string(),
                    // present_mode: PresentMode::AutoNoVsync,
                    present_mode: PresentMode::AutoVsync,
                    resizable: true,
                    mode: WindowMode::Windowed,

                ..default()
            }), ..default()})
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                level: bevy::log::Level::DEBUG,
            })
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
        )
        .add_plugin(WireframePlugin)
        .add_plugin(TomlAssetPlugin::<Planes>::new(&["scene.toml"]))
        .add_plugin(TomlAssetPlugin::<ConfigData>::new(&["toml"]))
        .add_plugin(CameraPlugin)
        .add_plugin(PlanesPlugin)

        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.5, 0.7, 0.9, 1.0].into()))
        .add_state::<DisplayMode>()
        .add_system(toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)))
        .run();
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

