
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::mtb_ui::ApplyModifierEvent;
use crate::core::vertex::{Vertex, PickedVertex, apply_modifiers};


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SceneStates::new())
        .add_systems(Update, save_state.before(apply_modifiers))
        .add_systems(Update, undo.run_if(input_just_pressed(KeyCode::Z)
                                 .and_then(input_pressed(KeyCode::ControlLeft))))
      ;                      
    }
  }

#[derive(Resource)]
pub struct SceneStates {
    pub index: u32,
    pub data: Vec<SceneState>
}
impl SceneStates {
    pub fn new() -> Self {
        SceneStates{index: 0, data: Vec::new()}
    }
}

pub struct SceneState {
    pub vertex_entities: HashMap<Entity, (Transform, Vertex, bool)>
}

pub fn save_state(mut apply_mod:      EventReader<ApplyModifierEvent>,
                  mut scene_states:   ResMut<SceneStates>,
                  vertex:  Query<(Entity, &Transform, &Vertex, Option<&PickedVertex>)>){

    for _ev in apply_mod.iter(){   
        info!("Saving state");     
        let mut ss = SceneState{ vertex_entities: HashMap::new()};
        for (entity, transform, vertex, picked_vertex) in vertex.iter(){
            ss.vertex_entities.insert(entity, (*transform, *vertex, picked_vertex.is_some()));
        }
        scene_states.data.push(ss);
        scene_states.index = scene_states.data.len() as u32 -1;
    }
}

// it doesnt undo actions, it just says undo
pub fn undo(mut scene_states:   ResMut<SceneStates>,
            mut commands:       Commands,
            mut vertex:         Query<(Entity, &mut Transform, &mut Vertex)>){

    info!("scene_states index: {}", scene_states.index);

    #[allow(unused_comparisons)]
    if scene_states.index >= 0 && (scene_states.index as usize) < scene_states.data.len() {

        info!("Undoing");

        let ss: &SceneState = &scene_states.data[scene_states.index as usize];    

        for (entity, mut transform, mut vertex) in vertex.iter_mut(){
            if let Some((old_transform, old_vertex, old_picked_vertex)) = ss.vertex_entities.get(&entity){

                *transform = *old_transform;
                *vertex = *old_vertex;
                
                if *old_picked_vertex {
                    commands.entity(entity).insert(PickedVertex);
                } else {
                    commands.entity(entity).remove::<PickedVertex>();
                }

            }
        }

        if scene_states.index > 0 {
            scene_states.index -= 1;
        }

    }
}
