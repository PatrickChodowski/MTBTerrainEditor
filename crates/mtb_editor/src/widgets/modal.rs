
use bevy::{prelude::*, input::common_conditions::input_just_pressed, ecs::schedule::ScheduleLabel};

#[derive(Component)]
pub struct ModalPanel;

pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<ModalState>()
        .add_systems(ModalState::On, close_modal.run_if(input_just_pressed(KeyCode::Escape)))
        ;
    }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, ScheduleLabel)]
pub enum ModalState {
    #[default]
    Off,
    On
}

// match modal_state.0 {
//     ModalState::On => {next_modal_state.set(ModalState::Off);}
//     ModalState::Off => {next_modal_state.set(ModalState::On);}
// }

pub fn close_modal(mut commands:          Commands, 
                   modals:                Query<Entity, With<ModalPanel>>,
                   mut next_modal_state:  ResMut<NextState<ModalState>>){
    for entity in modals.iter(){
        commands.entity(entity).despawn_recursive();
    }
    next_modal_state.set(ModalState::Off);
}



pub fn spawn_modal(commands:          &mut Commands, 
                   next_modal_state:  &mut ResMut<NextState<ModalState>>,) -> Entity {

    let ent = commands.spawn(NodeBundle{
      style: Style {
        position_type:    PositionType::Absolute,
        left:             Val::Percent(25.0), 
        top:              Val::Percent(25.0), 
        width:            Val::Percent(50.0), 
        height:           Val::Percent(50.0),
        flex_wrap:        FlexWrap::Wrap,
        flex_direction:   FlexDirection::Row,
        align_items:      AlignItems::FlexStart,
        justify_content:  JustifyContent::FlexStart,
        ..default()
      },
      background_color: BackgroundColor([0.5, 0.5, 0.5, 1.0].into()),
      ..default()
    })
    .insert(ModalPanel)
    .insert(Name::new("Modal"))
    .id()
    ;

    next_modal_state.set(ModalState::On);

    return ent;
  }
  