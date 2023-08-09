use bevy::prelude::*;

#[derive(Component)]
pub struct SidePanel;

pub fn spawn_side_panel(commands: &mut Commands) -> Entity {

    let ent_textinput = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(90.0), 
                            top: Val::Percent(0.0), 
                            ..default()},
          size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.4, 0.4, 0.4, 1.0].into()),
        ..default()
      }, SidePanel, Name::new(format!("SidePanel")))).id();
      
      return ent_textinput;
}
