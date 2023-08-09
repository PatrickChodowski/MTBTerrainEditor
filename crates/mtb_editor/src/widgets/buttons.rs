
use bevy::prelude::*;

#[derive(Component)]
pub enum ButtonValue {
    Color([f32;4]),
    String(String),
    NumberI32(i32),
    NumberU32(u32),
    NumberF32(f32)
}


pub fn spawn_button(commands: &mut Commands, 
                value:    ButtonValue,
                dims:     (Val, Val),
                pos_type: PositionType) -> Entity {

    let button_style = Style {
        size: Size::new(dims.0, dims.1),
        position_type: PositionType::Relative,
        margin: UiRect::all(Val::Px(3.0)),
        padding: UiRect::all(Val::Px(0.0)),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::Center,
        position: UiRect {left: Val::Px(0.0), top: Val::Px(1.0),..default()},
        ..default()
      };
    
      let btn_id = commands.spawn(ButtonBundle{style: button_style,
                                               background_color: BackgroundColor(color.into()),
                                               ..default()}).id();
      
      return btn_id;
    }

