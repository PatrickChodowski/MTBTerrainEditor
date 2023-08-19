
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Actions::new())
        .add_systems(Update, undo.run_if(input_just_pressed(KeyCode::Z)
                                 .and_then(input_pressed(KeyCode::ControlLeft))))
      ;                      
    }
  }

#[derive(Resource, Clone, Debug)]
pub struct Actions {
    pub data: Vec<ActionData>
}
impl Actions {
    pub fn new() -> Self {
        Actions{data: Vec::new()}
    }
}

#[derive(Clone, Debug)]
pub struct ActionData {

}



pub fn undo(){
    info!("Undo");
}