use bevy::prelude::*;

use bevy::window::WindowMode;
use bevy::window::PresentMode;
use bevy::window::WindowPlugin;
use bevy::log::LogPlugin;
use bevy::window::WindowResolution;
use bevy::pbr::wireframe::WireframePlugin;

mod camera;
use camera::CameraPlugin;

mod terrain;
use terrain::planes::PlanesPlugin;

mod tools;

mod utils;

pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                title: "Terrain Generator".to_string(),
                // present_mode: PresentMode::AutoNoVsync,
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                mode: WindowMode::Windowed,

            ..default()
        }), ..default()}).set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        // .set(RenderPlugin{wgpu_settings: WgpuSettings{features: WgpuFeatures::POLYGON_MODE_LINE, ..default()}})
        )
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlanesPlugin)
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.5, 0.7, 0.9, 1.0].into()))
        .run();
}
