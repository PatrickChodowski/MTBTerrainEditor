
use bevy::prelude::*;
use std::slice::Iter;

use mtb_core::colors::ColorsLib;
use mtb_core::planes::PlaneData;
use crate::mtb_grid::{GridData, HoverData, Hoverables};

use crate::widgets::buttons::{spawn_button, ButtonValue};
use crate::widgets::modal::{ModalPlugin, ModalPanel, ModalState, spawn_modal};
use crate::widgets::side_panel::{spawn_side_panel, SidePanel};
use crate::widgets::button_group::spawn_button_group;
use crate::widgets::color_picker::{ColorPickerPlugin, ColorPickerData, spawn_color_picker};
use crate::widgets::text_input::{spawn_text_input, TextInputPlugin, TextInputBox};
use crate::widgets::text_node::spawn_text_node;

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
        .add_state::<AppState>()
        .add_plugin(ColorPickerPlugin)
        .add_plugin(TextInputPlugin)
        .add_plugin(ModalPlugin)
        .add_event::<ToggleSubmenuEvent>()
        .add_event::<OpenModalEvent>()
        .insert_resource(ColorsLib::new())
        .add_startup_system(setup)
        .add_system(update_left_into_panel)
        .add_system(open_modal.run_if(in_state(ModalState::Off).and_then(on_event::<OpenModalEvent>())))
        .add_system(save_modal.run_if(in_state(ModalState::On)))

        .add_system(open_editor.in_schedule(OnEnter(AppState::Editor)))
        .add_system(close_editor.in_schedule(OnExit(AppState::Editor)))
        // .add_system(apply.run_if(in_state(AppState::Editor)))
        ;
    }
}

pub fn open_editor(mut commands: Commands, ass: Res<AssetServer>){
  let ent_sidepanel = spawn_side_panel(&mut commands);
  commands.entity(ent_sidepanel).insert(GUIElement);
  let mut v: Vec<Entity> = Vec::new();
  for area_option in AreaOption::iterator(){
      let new_button = spawn_button(&mut commands, 
                                    &ass,
                                    ButtonValue::String(area_option.to_str().to_string()),
                                    (Val::Percent(10.0), Val::Percent(1.0)),
                                    (Val::Percent(80.0), Val::Px(20.0)),
                                    PositionType::Relative);
      commands.entity(new_button).insert(*area_option);
      v.push(new_button);
  }
  commands.entity(ent_sidepanel).push_children(&v);
}

pub fn close_editor(mut commands: Commands, sidepanel: Query<Entity, With<SidePanel>>){
  for entity in sidepanel.iter(){
    commands.entity(entity).despawn_recursive();
  }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    View,
    Editor
}



pub fn open_modal(mut commands:          Commands,
                  mut open_modal:        EventReader<OpenModalEvent>,
                  ass:                   Res<AssetServer>,
                  mut next_modal_state:  ResMut<NextState<ModalState>>,
                  colors_lib:            Res<ColorsLib>){

    for ev in open_modal.iter(){
      let modal = spawn_modal(&mut commands, &mut next_modal_state);
      commands.entity(modal).insert(ev.modal_type).insert(GUIElement);

      match ev.modal_type {
        ModalType::Color => {
          let color_picker = spawn_color_picker(&mut commands, &ass);
          let text_input = spawn_text_input(&mut commands, &ass, &(11.0, 55.0), &(200.0, 30.0), "ColorName".to_string());
          let button_group = spawn_button_group(&mut commands, &ass, &colors_lib, &(11.0, 65.0), &(625.0, 100.0));
          commands.entity(modal).push_children(&[color_picker, text_input, button_group]);
        }
        ModalType::PlaneColor => {
          let color_picker = spawn_color_picker(&mut commands, &ass);
          let text_input = spawn_text_input(&mut commands, &ass, &(11.0, 55.0), &(200.0, 30.0), "ColorName".to_string());
          let button_group = spawn_button_group(&mut commands, &ass, &colors_lib, &(11.0, 65.0), &(625.0, 100.0));
          commands.entity(modal).push_children(&[color_picker, text_input, button_group]);
        }

        ModalType::ColorGradient => {
          let color_picker = spawn_color_picker(&mut commands, &ass);
          commands.entity(modal).push_children(&[color_picker]);
        }
      }
    }
}

// Close with saving data
pub fn save_modal(modals:                Query<(&ModalType, &Children), With<ModalPanel>>,
                  color_picker:          Query<&ColorPickerData>,
                  text_inputs:           Query<&TextInputBox>,
                  mut colors_lib:        ResMut<ColorsLib>){

  for (modal_type, children) in modals.iter(){
    match modal_type {
      ModalType::Color => {
        let mut id: Option<String> = None;
        let mut clr: Option<[f32;4]> = None;
        for child in children.iter(){
          if let Ok(cpd) = color_picker.get(*child) {
            clr = Some([cpd.r, cpd.g, cpd.b, cpd.a]);
          }
          if let Ok(text) = text_inputs.get(*child){
            id = Some(text.text.clone());
          }
        }
        if id.is_some() && clr.is_some(){
          colors_lib.add(id.unwrap(), clr.unwrap());
        }
      }
      ModalType::ColorGradient => {}
      ModalType::PlaneColor => {}
    }
  }
}


pub struct OpenModalEvent {
  pub modal_type: ModalType
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum ModalType {
  #[default]
  PlaneColor,
  Color,
  ColorGradient
}

#[derive(Component, Debug, Clone, Copy)]
pub enum AreaOption {
  AABB,
  Ellipse,
  Mark
} 

impl<'a> AreaOption {
  fn to_str(&self) -> &'a str {
    match self {
      AreaOption::AABB => {"aabb"}
      AreaOption::Ellipse => {"ellipse"}
      AreaOption::Mark => {"mark"}
    }
  }
  pub fn iterator() -> Iter<'static, AreaOption> {
    static AREA_OPTIONS: [AreaOption; 3] = [AreaOption::AABB, AreaOption::Ellipse, AreaOption::Mark];
    AREA_OPTIONS.iter()
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
pub struct GUIElement;

#[allow(dead_code)]
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



fn setup(mut commands:  Commands,  _ass: Res<AssetServer>,) {
  let _info_panel_entity = spawn_info_panel(&mut commands);
}

fn update_left_into_panel(mut commands:  Commands,
                          grid:          Res<GridData>,
                          hover_data:    Res<HoverData>,
                          ass:           Res<AssetServer>,
                          app_state:     Res<State<AppState>>,
                          planes:        Query<&PlaneData>,
                          top_left:      Query<Entity, With<TopLeftInfoPanel>>){

  let ent = top_left.get_single().unwrap();                  
  commands.entity(ent).despawn_descendants();

  let mut v: Vec<Entity> = Vec::new();
  v.push(spawn_text_node(&format!("    App State: {:?}", app_state.0), &mut commands, &ass));  
  v.push(spawn_text_node(&format!("    Planes Count: {:?}", planes.iter().len()), &mut commands, &ass));  
  v.push(spawn_text_node(&format!("    Tile: {:?}", hover_data.hovered_tile_xz), &mut commands, &ass));  
  v.push(spawn_text_node(&format!("    Pos: ({:.0}, {:.0})",  hover_data.hovered_xz.0, hover_data.hovered_xz.1), &mut commands, &ass)); 

  if let Some(height) = grid.data.get(&hover_data.hovered_tile_xz) {
    v.push(spawn_text_node(&format!("    Height: {}",  height), &mut commands, &ass)); 
  }

  if let Hoverables::Entity(entity) = hover_data.hoverable {
    if let Ok(pd) = planes.get(entity) {
      v.push(spawn_text_node(&format!("    Plane ID: {:?}", pd.id), &mut commands, &ass));  
      v.push(spawn_text_node(&format!("    Loc: {:?}",      pd.loc), &mut commands, &ass));  
      v.push(spawn_text_node(&format!("    Dims: {:?}",     pd.dims), &mut commands, &ass));  
      v.push(spawn_text_node(&format!("    Subs: {:?}",     pd.subdivisions), &mut commands, &ass));  
    }
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
      size: Size::new(Val::Percent(100.0), Val::Px(25.0)),
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



// // Click on button
// fn click(mut btn_q: Query<(Entity, &Interaction, &mut BackgroundColor, &mut Style, Option<&Children>, Option<&mut Expandable>),
//                           (Changed<Interaction>, With<Button>)>, 
//     mut event_toggle_submenu: EventWriter<ToggleSubmenuEvent>, 
//     mut spawn_new_plane:      EventWriter<SpawnNewPlaneEvent>
// ){
//     for (entity, interaction, mut color, mut style, children, expandable) in &mut btn_q {
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = MENU_BTN_COLOR_PRESSED.into();
//                 if let Some(mut expandable) = expandable {
//                     let mut height_diff: f32 = 0.0;
//                     if expandable.is_open {
//                         let new_height = MENU_BTN_HEIGHT;
//                         height_diff = new_height.evaluate(1.0).ok().unwrap() - style.size.height.evaluate(1.0).ok().unwrap();
//                         style.size.height = new_height;
//                     } else {
//                         if children.is_some(){
//                             let new_height = Val::Px((children.unwrap().len() as f32)*30.0);
//                             height_diff = new_height.evaluate(1.0).ok().unwrap() - style.size.height.evaluate(1.0).ok().unwrap();
//                             style.size.height = new_height;
//                         }
//                     }
//                     expandable.is_open = !expandable.is_open;
//                     event_toggle_submenu.send(ToggleSubmenuEvent{button_entity: entity, height_diff, is_open: expandable.is_open});
//                 } else {
//                   spawn_new_plane.send(SpawnNewPlaneEvent{pd: PlaneData::new()});
//                 }
//             }
//             Interaction::Hovered => {
//             *color = MENU_BTN_COLOR_HOVER.into();
//             }
//             Interaction::None => {
//             *color = MENU_BTN_COLOR.into();
//             }
//         }
//     }
// }
// fn spawn_menu(commands:  &mut Commands) -> Entity {
//     let ent = commands.spawn(NodeBundle{
//         style: Style {
//           position_type: PositionType::Absolute,
//           position: UiRect {left: Val::Percent(0.0), 
//                             top: Val::Px(25.0), 
//                             ..default()},
//           size: Size::new(Val::Px(110.0), Val::Px(200.0)),
//           flex_wrap: FlexWrap::Wrap,
//           flex_direction: FlexDirection::Column,
//           align_content: AlignContent::FlexStart,
//           ..default()
//         },
//         // background_color: BackgroundColor([0.5, 0.5, 0.5, 0.5].into()),
//         ..default()
//       })
//       .insert(GUIElement)
//       .insert(Menu)
//       .id()
//       ;
//     return ent;
// }
  
  
// fn spawn_menu_buttons(commands:  &mut Commands,  ass: &Res<AssetServer>) -> Vec<Entity> {
//   let mut v = Vec::new();
//   let buttons_data: [ButtonData; 2] = [ButtonData{label: "Add Plane".to_string(), children: None}, 
//                                        ButtonData{label: "Add Modifier".to_string(), children: None}];
//   let button_style = Style {
//       size: Size::new(MENU_BTN_WIDTH, MENU_BTN_HEIGHT),
//       position_type: PositionType::Relative,
//       margin: UiRect::all(Val::Px(3.0)),
//       padding: UiRect::all(Val::Px(0.0)),
//       align_items: AlignItems::Center,
//       flex_direction: FlexDirection::Column,
//       align_content: AlignContent::Center,
//       position: UiRect {left: Val::Px(0.0),top: Val::Px(1.0),..default()},
//       ..default()
//     };
  
//   let child_button_style = Style {
//     size: Size::new(MENU_CHILD_BTN_WIDTH, MENU_BTN_HEIGHT),
//     position_type: PositionType::Relative,
//     margin: UiRect::all(Val::Px(0.0)),
//     padding: UiRect::all(Val::Px(0.0)),
//     align_items: AlignItems::Center,
//     flex_direction: FlexDirection::Column,
//     align_content: AlignContent::Center,
//     position: UiRect {left: Val::Px(0.0),top: Val::Px(10.0),..default()},
//     ..default()
//   };

//   let text_style = TextStyle {
//     font: ass.load("fonts/lambda.ttf"),
//     font_size: 20.0,
//     color: MENU_TEXT_COLOR
//   };


//   for btn in buttons_data {
//     let btn_id = spawn_button(commands, &btn, &button_style, &text_style, true);
//     if let Some(children) = btn.children{
//       for cbtn in children {
//         if cbtn.children.is_some(){
//           panic!(" [GUI] Child button not allowed to have children (Button label: {})", cbtn.label);
//         }
//         let cbtn_id = spawn_button(commands, &cbtn, &child_button_style, &text_style, false);
//         commands.entity(btn_id).push_children(&[cbtn_id]);
//       }
//       commands.entity(btn_id).insert(Expandable{is_open: false});
//     }
//     v.push(btn_id);
//   }

//   return v;
// }

// fn spawn_button(commands: &mut Commands, btn: &ButtonData, btn_style: &Style, txt_style: &TextStyle, visible: bool) -> Entity {

//     let mut btn_style_vis = btn_style.clone();
//     if !visible {
//       btn_style_vis.display = Display::None;
//     }
  
//     let btn_id = commands.spawn(ButtonBundle{style: btn_style_vis,
//                                              background_color: MENU_BTN_COLOR.into(),
//                                              ..default()})
//                          .insert(Name::new(format!("{} Button", btn.label)))
//                          .id();
    
//     let btn_label = commands.spawn(TextBundle::from_section(&btn.label, txt_style.clone())).insert(ButtonLabel).id();
//     commands.entity(btn_id).push_children(&[btn_label]);
  
//     return btn_id;
//   }
