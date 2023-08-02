
use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;

use super::{mtb_grid::{HoverData, GridData}, modifiers::DebugModifierBox};

pub struct MTBGuiPlugin;

impl Plugin for MTBGuiPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_state::<DebugMode>()
      .add_startup_system(setup)
      .add_system(update_left_into_panel)
      .add_system(toggle_debug.run_if(input_just_pressed(KeyCode::Z)))
      ;
  }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DebugMode {
    DebugOn,
    #[default]
    DebugOff
}

fn toggle_debug(
      mut debug_boxes:       Query<&mut Visibility, With<DebugModifierBox>>,
      debug_mode:            Res<State<DebugMode>>,
      mut next_debug_mode:   ResMut<NextState<DebugMode>>){

  match debug_mode.0 {
    DebugMode::DebugOn => {
      next_debug_mode.set(DebugMode::DebugOff);
      for mut vis in debug_boxes.iter_mut(){
        *vis = Visibility::Hidden;
      }
    }
    DebugMode::DebugOff => {
      next_debug_mode.set(DebugMode::DebugOn);
      for mut vis in debug_boxes.iter_mut(){
        *vis = Visibility::Inherited;
      }
    }
}
}


#[derive(Component)]
pub struct TopLeftInfoPanel;

#[derive(Component)]
pub struct GUIElement;


fn setup(mut commands:  Commands) {
  let _info_panel_entity = spawn_info_panel(&mut commands);
}


fn update_left_into_panel(mut commands:  Commands,
                          grid:          Res<GridData>,
                          hover_data:    Res<HoverData>,
                          ass:           Res<AssetServer>,
                          top_left:      Query<Entity, With<TopLeftInfoPanel>>){
  let ent = top_left.get_single().unwrap();                  
  commands.entity(ent).despawn_descendants();

  let mut v: Vec<Entity> = Vec::new();
  v.push(make_text_node(&format!(" Tile: {:?}", hover_data.hovered_tile_xz), &mut commands, &ass));  
  v.push(make_text_node(&format!("    Pos: ({:.0}, {:.0})",  hover_data.hovered_xz.0, hover_data.hovered_xz.1), &mut commands, &ass)); 

  if let Some(height) = grid.data.get(&hover_data.hovered_tile_xz) {
    v.push(make_text_node(&format!("    Height: {}",  height), &mut commands, &ass)); 
  }

  commands.entity(ent).push_children(&v);
 
}









fn spawn_info_panel(commands: &mut Commands) -> Entity {

  let ent = commands.spawn(NodeBundle{
    style: Style {
      position_type: PositionType::Absolute,
      position: UiRect {left: Val::Percent(0.0), 
                        top: Val::Percent(0.0), 
                        ..default()},
      size: Size::new(Val::Percent(25.0), Val::Px(25.0)),
      flex_wrap: FlexWrap::Wrap,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::FlexStart,
      justify_content: JustifyContent::FlexStart,
      ..default()
    },
    // background_color: BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5)),
    ..default()
  })
  .insert(GUIElement)
  .insert(TopLeftInfoPanel)
  .id()
  ;
  return ent;
}


pub fn make_text_node(txt: &str, commands: &mut Commands, ass: &Res<AssetServer>,) -> Entity {

  let txt_style = TextStyle {
    font_size: 15.0,
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
      position: UiRect {left: Val::Px(0.0),top: Val::Px(0.0),..default()},
      ..default()
    },
    ..default()}).id();

  commands.entity(node_txt).push_children(&[txt]);

  return node_txt;

}


