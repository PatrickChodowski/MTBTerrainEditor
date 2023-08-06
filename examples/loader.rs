
use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, WindowPlugin, WindowResolution};
use bevy::pbr::wireframe::WireframePlugin;

use mtb_loader::MTBLoaderPlugin;

pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                    title: "MTB Terrain Generator Loader".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: true,
                    mode: WindowMode::Windowed,
                ..default()
            }), ..default()})
            .set(AssetPlugin {watch_for_changes: true, ..default()})
        )
        .add_plugin(WireframePlugin)
        .add_plugin(MTBLoaderPlugin)
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.5, 0.7, 0.9, 1.0].into()))
        .add_startup_system(setup)
        .run();
}


fn setup(mut commands: Commands){
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(500.0, 1000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));
}


