use bevy::prelude::*;

pub fn spawn_text_node(txt: &str, commands: &mut Commands, ass: &Res<AssetServer>,) -> Entity {

    let txt_style = TextStyle {
      font_size: 20.0,
      color: Color::WHITE,
      font: ass.load("fonts/lambda.ttf")
    };
  
    let txt = commands.spawn(TextBundle::from_section(txt, txt_style)).id();
    let node_txt = commands.spawn(NodeBundle{
      style: Style {
        position_type: PositionType::Relative,
        margin: UiRect::all(Val::Px(0.0)),
        padding: UiRect::all(Val::Px(0.0)),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        align_content: AlignContent::Center,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        ..default()
      },
      ..default()}).id();
  
    commands.entity(node_txt).push_children(&[txt]);
  
    return node_txt;
  
  }
  