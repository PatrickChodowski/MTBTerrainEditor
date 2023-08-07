
use bevy::prelude::*;
use crate::mtb_grid::{GridData, HoverData};
use mtb_core::planes::{SpawnNewPlaneEvent, PlaneData};

pub const MENU_BTN_COLOR: Color = Color::rgb(0.4, 0.4, 0.4); 
pub const MENU_BTN_COLOR_HOVER: Color = Color::rgb(0.45, 0.45, 0.45); 
pub const MENU_BTN_COLOR_PRESSED: Color = Color::rgb(0.5, 0.5, 0.5); 
pub const MENU_BTN_HEIGHT: Val = Val::Px(25.0);
pub const MENU_BTN_WIDTH: Val = Val::Px(100.0);
pub const MENU_CHILD_BTN_WIDTH: Val = Val::Px(80.0);
pub const MENU_TEXT_COLOR: Color= Color::rgb(1.0, 1.0, 1.0); 

pub struct MTBUIPlugin;

impl Plugin for MTBUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ToggleSubmenuEvent>()
        .add_startup_system(setup)
        .add_system(update_left_into_panel)
        .add_system(click)
        ;
    }
}

pub struct ToggleSubmenuEvent {
  pub button_entity: Entity,
  pub height_diff: f32,
  pub is_open: bool
}
  
#[derive(Component)]
pub struct TopLeftInfoPanel;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct GUIElement;

struct ButtonData {
  label: String,
  children: Option<Vec<ButtonData>> 
}

#[derive(Component)]
pub struct Expandable {
  pub is_open: bool
}

#[derive(Component)]
pub struct ButtonLabel;



fn setup(mut commands:  Commands,  ass: Res<AssetServer>,) {
  let _info_panel_entity = spawn_info_panel(&mut commands);
  let menu = spawn_menu(&mut commands);
  let menu_buttons = spawn_menu_buttons(&mut commands, &ass);
  commands.entity(menu).push_children(&menu_buttons);
}

fn update_left_into_panel(mut commands:  Commands,
                          grid:          Res<GridData>,
                          hover_data:    Res<HoverData>,
                          ass:           Res<AssetServer>,
                          top_left:      Query<Entity, With<TopLeftInfoPanel>>){

  let ent = top_left.get_single().unwrap();                  
  commands.entity(ent).despawn_descendants();

  let mut v: Vec<Entity> = Vec::new();
  v.push(make_text_node(&format!("    Tile: {:?}", hover_data.hovered_tile_xz), &mut commands, &ass));  
  v.push(make_text_node(&format!("    Pos: ({:.0}, {:.0})",  hover_data.hovered_xz.0, hover_data.hovered_xz.1), &mut commands, &ass)); 

  if let Some(height) = grid.data.get(&hover_data.hovered_tile_xz) {
    v.push(make_text_node(&format!("    Height: {}",  height), &mut commands, &ass)); 
  }

  commands.entity(ent).push_children(&v);
}

// Click on button
fn click(mut btn_q: Query<(Entity, &Interaction, &mut BackgroundColor, &mut Style, Option<&Children>, Option<&mut Expandable>),
                          (Changed<Interaction>, With<Button>)>, 
    mut event_toggle_submenu: EventWriter<ToggleSubmenuEvent>, 
    mut spawn_new_plane:      EventWriter<SpawnNewPlaneEvent>
){
    for (entity, interaction, mut color, mut style, children, expandable) in &mut btn_q {
        match *interaction {
            Interaction::Clicked => {
                *color = MENU_BTN_COLOR_PRESSED.into();
                if let Some(mut expandable) = expandable {
                    let mut height_diff: f32 = 0.0;
                    if expandable.is_open {
                        let new_height = MENU_BTN_HEIGHT;
                        height_diff = new_height.evaluate(1.0).ok().unwrap() - style.size.height.evaluate(1.0).ok().unwrap();
                        style.size.height = new_height;
                    } else {
                        if children.is_some(){
                            let new_height = Val::Px((children.unwrap().len() as f32)*30.0);
                            height_diff = new_height.evaluate(1.0).ok().unwrap() - style.size.height.evaluate(1.0).ok().unwrap();
                            style.size.height = new_height;
                        }
                    }
                    expandable.is_open = !expandable.is_open;
                    event_toggle_submenu.send(ToggleSubmenuEvent{button_entity: entity, height_diff, is_open: expandable.is_open});
                } else {
                  spawn_new_plane.send(SpawnNewPlaneEvent{pd: PlaneData::new()});
                }
            }
            Interaction::Hovered => {
            *color = MENU_BTN_COLOR_HOVER.into();
            }
            Interaction::None => {
            *color = MENU_BTN_COLOR.into();
            }
        }
    }
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
      ..default()
    })
    .insert(GUIElement)
    .insert(TopLeftInfoPanel)
    .id()
    ;
    return ent;
}
  

fn spawn_menu(commands:  &mut Commands) -> Entity {
    let ent = commands.spawn(NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(0.0), 
                            top: Val::Px(25.0), 
                            ..default()},
          size: Size::new(Val::Px(110.0), Val::Px(200.0)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Column,
          align_content: AlignContent::FlexStart,
          ..default()
        },
        // background_color: BackgroundColor([0.5, 0.5, 0.5, 0.5].into()),
        ..default()
      })
      .insert(GUIElement)
      .insert(Menu)
      .id()
      ;
    return ent;
}
  
  
fn spawn_menu_buttons(commands:  &mut Commands,  ass: &Res<AssetServer>) -> Vec<Entity> {
  let mut v = Vec::new();
  let buttons_data: [ButtonData; 2] = [ButtonData{label: "Add Plane".to_string(), children: None}, 
                                       ButtonData{label: "Add Modifier".to_string(), children: None}];
  let button_style = Style {
      size: Size::new(MENU_BTN_WIDTH, MENU_BTN_HEIGHT),
      position_type: PositionType::Relative,
      margin: UiRect::all(Val::Px(3.0)),
      padding: UiRect::all(Val::Px(0.0)),
      align_items: AlignItems::Center,
      flex_direction: FlexDirection::Column,
      align_content: AlignContent::Center,
      position: UiRect {left: Val::Px(0.0),top: Val::Px(1.0),..default()},
      ..default()
    };
  
  let child_button_style = Style {
    size: Size::new(MENU_CHILD_BTN_WIDTH, MENU_BTN_HEIGHT),
    position_type: PositionType::Relative,
    margin: UiRect::all(Val::Px(0.0)),
    padding: UiRect::all(Val::Px(0.0)),
    align_items: AlignItems::Center,
    flex_direction: FlexDirection::Column,
    align_content: AlignContent::Center,
    position: UiRect {left: Val::Px(0.0),top: Val::Px(10.0),..default()},
    ..default()
  };

  let text_style = TextStyle {
    font: ass.load("fonts/lambda.ttf"),
    font_size: 20.0,
    color: MENU_TEXT_COLOR
  };


  for btn in buttons_data {
    let btn_id = spawn_button(commands, &btn, &button_style, &text_style, true);
    if let Some(children) = btn.children{
      for cbtn in children {
        if cbtn.children.is_some(){
          panic!(" [GUI] Child button not allowed to have children (Button label: {})", cbtn.label);
        }
        let cbtn_id = spawn_button(commands, &cbtn, &child_button_style, &text_style, false);
        commands.entity(btn_id).push_children(&[cbtn_id]);
      }
      commands.entity(btn_id).insert(Expandable{is_open: false});
    }
    v.push(btn_id);
  }

  return v;
}

fn spawn_button(commands: &mut Commands, btn: &ButtonData, btn_style: &Style, txt_style: &TextStyle, visible: bool) -> Entity {

    let mut btn_style_vis = btn_style.clone();
    if !visible {
      btn_style_vis.display = Display::None;
    }
  
    let btn_id = commands.spawn(ButtonBundle{style: btn_style_vis,
                                             background_color: MENU_BTN_COLOR.into(),
                                             ..default()})
                         .insert(Name::new(format!("{} Button", btn.label)))
                         .id();
    
    let btn_label = commands.spawn(TextBundle::from_section(&btn.label, txt_style.clone())).insert(ButtonLabel).id();
    commands.entity(btn_id).push_children(&[btn_label]);
  
    return btn_id;
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
