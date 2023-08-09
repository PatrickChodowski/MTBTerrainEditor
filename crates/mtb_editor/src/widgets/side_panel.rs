use bevy::prelude::*;

pub struct SidePanelPlugin;

impl Plugin for SidePanelPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_system(update.run_if(resource_changed::<ColorsLib>()))
        // .add_system(click.run_if(input_just_pressed(MouseButton::Left)).in_base_set(CoreSet::PreUpdate))
        ;
    }
}

#[derive(Component)]
pub struct SidePanel;

pub fn spawn_side_panel(commands: &mut Commands, 
                        ass: &Res<AssetServer>) -> Entity {

    let ent_textinput = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(90.0), 
                            top: Val::Percent(0.0), 
                            ..default()},
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.4, 0.4, 0.4, 1.0].into()),
        ..default()
      }, SidePanel, Name::new(format!("SidePanel")))).id();
      
    //   let txt_style = TextStyle {font_size: 15.0, color: Color::BLACK,font: ass.load("fonts/lambda.ttf")};
    //   let actual_text = commands.spawn((TextBundle::from_section("", txt_style), TextInput::new(id))).id();
      
    //   commands.entity(ent_textinput).push_children(&[actual_text]);
      return ent_textinput;
}
