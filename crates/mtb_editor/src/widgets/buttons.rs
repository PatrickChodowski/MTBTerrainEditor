
use bevy::prelude::*;

use super::text_node::spawn_text_node;

#[derive(Component, Debug, Clone)]
pub enum ButtonValue {
    Color([f32;4]),
    String(String)
}


pub fn spawn_button(commands: &mut Commands, 
                    ass:      &Res<AssetServer>,
                    value:    ButtonValue,
                    pos:      (Val, Val),
                    dims:     (Val, Val),
                    position_type: PositionType) -> Entity {

    let button_style = Style {

        width: dims.0, 
        height: dims.1,
        position_type,
        margin: UiRect::all(Val::Px(3.0)),
        padding: UiRect::all(Val::Px(0.0)),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::Center,
        left: pos.0, 
        top: pos.1,
        ..default()

      };

      let mut bkgc = BackgroundColor([0.3, 0.3, 0.3, 1.0].into());

      if let ButtonValue::Color(clr) = value {
        bkgc = BackgroundColor(clr.into());
      }

      let btn_id = commands.spawn((ButtonBundle{style: button_style,
        background_color: bkgc,
        ..default()}, value.clone())).id();
      
      if let ButtonValue::String(strval) = value {
        let label_ent = spawn_text_node(&strval, commands, ass);
        commands.entity(btn_id).push_children(&[label_ent]);
      }
      
      return btn_id;
    }

