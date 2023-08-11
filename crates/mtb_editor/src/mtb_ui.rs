
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::slice::Iter;

use mtb_core::colors::ColorsLib;
use mtb_core::planes::PlaneData;

use crate::mtb_grid::{GridData, HoverData, Hoverables};
use crate::{AppState, boxselect};
use crate::boxselect::{box_select,BoxSelectPlugin, BoxSelect};
use crate::vertex::Vertex;
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
        .add_plugin(BoxSelectPlugin)
        .add_plugin(ColorPickerPlugin)
        .add_plugin(TextInputPlugin)
        .add_plugin(ModalPlugin)
        .add_event::<ToggleSubmenuEvent>()
        .add_event::<OpenModalEvent>()
        .insert_resource(ColorsLib::new())
        .insert_resource(Picker::new())
        .add_startup_system(setup)
        .add_system(update_left_into_panel)
        .add_system(open_modal.run_if(in_state(ModalState::Off).and_then(on_event::<OpenModalEvent>())))
        .add_system(save_modal.in_schedule(OnExit(ModalState::On)))

        .add_system(open_editor.in_schedule(OnEnter(AppState::Edit)))
        .add_system(close_editor.in_schedule(OnExit(AppState::Edit)))
        .add_system(click_button.run_if(input_just_pressed(MouseButton::Left)))

        .add_system(pick.run_if(in_state(AppState::Edit)))

        // .add_system(apply.run_if(in_state(AppState::Editor)))
        ;
    }
}

#[derive(Resource)]
pub struct Picker {
  pub select: SelectOption
} 
impl Picker {
  pub fn new() -> Picker {
    Picker { select: SelectOption::Point }
  }
  pub fn reset(&mut self) {
    self.select = SelectOption::Point
  }
}
 
pub fn pick(mut commands:       Commands, 
            picker:             Res<Picker>, 
            hoverdata:          Res<HoverData>,
            mut meshes:         ResMut<Assets<Mesh>>,
            mut materials:      ResMut<Assets<StandardMaterial>>,
            mut boxselect:      Query<(&mut Transform, &mut BoxSelect)>,
            vertex:             Query<(Entity, &GlobalTransform), With<Vertex>>){

  match picker.select {
    SelectOption::Box    => {
      box_select(&mut commands, &mut boxselect, &mut meshes, &mut materials, &hoverdata);

    }
    SelectOption::Ellipse => {}
    SelectOption::Point   => {}
    SelectOption::Brush   => {}
  }

}



pub fn click_button(mut picker:  ResMut<Picker>,
                    mut buttons: Query<(&Interaction, &ButtonValue, Option<&SelectOption>), (Changed<Interaction>, With<Button>)>){

  for (interaction, value, area) in buttons.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        if let Some(area) = area {
          picker.select = *area;
          info!(" [DEBUG] Changed Picker area to {}", area.to_str());
        }
      }
      _ => {}
    }
  }
}


pub fn open_editor(mut commands: Commands, ass: Res<AssetServer>){
  let ent_sidepanel = spawn_side_panel(&mut commands);
  commands.entity(ent_sidepanel).insert(GUIElement);
  let mut v: Vec<Entity> = Vec::new();
  for area_option in SelectOption::iterator(){
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
pub enum SelectOption {
  Box,
  Ellipse,
  Point,
  Brush
} 

impl<'a> SelectOption {
  fn to_str(&self) -> &'a str {
    match self {
      SelectOption::Box => {"box"}
      SelectOption::Ellipse => {"ellipse"}
      SelectOption::Point => {"point"}
      SelectOption::Brush => {"brush"}
    }
  }
  pub fn iterator() -> Iter<'static, SelectOption> {
    static AREA_OPTIONS: [SelectOption; 4] = [SelectOption::Box, 
                                              SelectOption::Ellipse, 
                                              SelectOption::Point, 
                                              SelectOption::Brush];
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
