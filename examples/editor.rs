
use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, WindowPlugin, WindowResolution};
use mtb_editor::MTBEditorPlugin;

pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                    title: "MTB Terrain Generator Editor".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: true,
                    mode: WindowMode::Windowed,

                ..default()
            }), ..default()})
            .set(AssetPlugin {watch_for_changes: true,..default()})
        )
        .add_plugin(MTBEditorPlugin)
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.5, 0.7, 0.9, 1.0].into()))
        .run();
}

