use bevy::prelude::*;

use super::{AppMode, planes::PlaneData, modifiers::ModifierData};

#[derive(Resource)]
pub struct ManualEdits {
    pub data: Vec<ManualEdit>
}
impl ManualEdits {
    pub fn new() -> Self {
        return ManualEdits{data: Vec::new()};
    }
}

pub struct ManualEdit {
    pub plane_id:   u32,
    pub modifiers:  Vec<ModifierData>
}



pub struct EditModePlugin;

impl Plugin for EditModePlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(ManualEdits::new())
    //   .add_system(update.run_if(in_state(AppMode::Edit)))
      ;
  }
}

