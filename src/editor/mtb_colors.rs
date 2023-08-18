
use bevy::prelude::*;

use crate::core::colors::ColorData;

pub struct MTBColorsPlugin;


impl Plugin for MTBColorsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Colors::new())
        ;
    }
}

#[derive(Resource)]
pub struct Colors {
    pub data: Vec<ColorData>
}
impl Colors {
    pub fn new() -> Self {
        Colors{data: Vec::new()}
    }
}
