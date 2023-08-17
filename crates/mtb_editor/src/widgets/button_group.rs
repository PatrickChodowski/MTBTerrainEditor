
use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;

use mtb_core::colors::ColorsLib;

use crate::widgets::buttons::{spawn_button, ButtonValue};

pub struct ButtonGroupPlugin;

impl Plugin for ButtonGroupPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_system(update.run_if(resource_changed::<ColorsLib>()))
        .add_systems(PreUpdate, click.run_if(input_just_pressed(MouseButton::Left)))
        ;
    }
}

// pub fn update(){

// }

#[derive(Component)]
pub struct ButtonGroup;

pub fn click(){

}

pub fn spawn_button_group(commands:  &mut Commands,
                          ass:       &Res<AssetServer>,
                          colorslib: &Res<ColorsLib>,
                          xy:        &(f32, f32), 
                          dims:      &(f32, f32),) -> Entity {

    let ent_bg = commands.spawn((NodeBundle{
      style: Style {
        position_type:    PositionType::Absolute,
        left:             Val::Percent(xy.0), 
        top:              Val::Percent(xy.1), 
        width:            Val::Px(dims.0), 
        height:           Val::Px(dims.1),
        flex_wrap:        FlexWrap::Wrap,
        flex_direction:   FlexDirection::Row,
        align_items:      AlignItems::FlexStart,
        justify_content:  JustifyContent::FlexStart,
        ..default()
      },
      background_color: BackgroundColor([0.5, 0.5, 0.5, 1.0].into()),
      ..default()
    }, ButtonGroup, Name::new(format!("ButtonGroup")))).id();

    let mut v: Vec<Entity> = Vec::new();
    for (_key, value) in colorslib.data.iter(){
        let new_button = spawn_button(commands, 
                                      ass,
                                      ButtonValue::Color(*value),
                                      (Val::Px(0.0), Val::Px(1.0)),
                                      (Val::Px(40.0), Val::Px(40.0)),
                                      PositionType::Relative);
        v.push(new_button);
    }
    commands.entity(ent_bg).push_children(&v);
    return ent_bg;
}
