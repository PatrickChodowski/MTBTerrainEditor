

use bevy::prelude::*;
use std::str::FromStr;
use std::slice::Iter;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use mtb_core::colors::ColorsLib;
use mtb_core::planes::PlaneData;

use crate::mtb_grid::{GridData, HoverData, Hoverables};
use crate::AppState;
use crate::brush::{BrushPlugin, BrushSettings};
use crate::boxselect::BoxSelectPlugin;
use crate::spawn_text_node;

pub struct MTBUIPlugin;

impl Plugin for MTBUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<PickerState>()
        .add_state::<ModifierState>()
        .add_plugins(BoxSelectPlugin)
        .add_plugins(BrushPlugin)
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .insert_resource(ColorsLib::new())
        .add_systems(Startup, setup)
        .add_systems(Update, update_egui)
        .add_systems(Update, update_left_into_panel)
        ;
    }
}


#[derive(Default, Resource, Debug)]
struct OccupiedScreenSpace {
    _left: f32,
    _top: f32,
    right: f32,
    _bottom: f32,
}



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum PickerState {
    #[default]
    Point,
    Brush,
    Box
}

impl<'a> PickerState {
  pub fn iterator() -> Iter<'static, PickerState> {
    static PICKER_OPTIONS: [PickerState; 3] = [PickerState::Box, PickerState::Point, PickerState::Brush];
    PICKER_OPTIONS.iter()
  }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum ModifierState {
    #[default]
    Color,
    Noise,
    Value,
    Wave,
    Terraces,
    Smoothing
}

impl<'a> ModifierState { 
  pub fn iterator() -> Iter<'static, ModifierState> {
    static MOD_OPTIONS: [ModifierState; 6] = [ModifierState::Color, ModifierState::Noise, ModifierState::Value,
                                              ModifierState::Wave, ModifierState::Terraces, ModifierState::Smoothing];
    MOD_OPTIONS.iter()
  }
  
}

impl FromStr for ModifierState {
  type Err = ();
  fn from_str(input: &str) -> Result<ModifierState, Self::Err> {
      match input {
          "Color"     => Ok(ModifierState::Color),
          "Noise"     => Ok(ModifierState::Noise),
          "Value"     => Ok(ModifierState::Value),
          "Wave"      => Ok(ModifierState::Wave),
          "Terraces"  => Ok(ModifierState::Terraces),
          "Smoothing" => Ok(ModifierState::Smoothing),
          _      => Err(()),
      }
  }
}

  
#[derive(Component)]
pub struct TopLeftInfoPanel;

fn setup(mut commands:  Commands){
  let _info_panel_entity = spawn_info_panel(&mut commands);
}

fn update_egui(mut contexts:              EguiContexts,
               mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
               picker_state:              Res<State<PickerState>>,
               mut next_picker_state:     ResMut<NextState<PickerState>>,
               mut brush_settings:        ResMut<BrushSettings>,
               modifier_state:            Res<State<ModifierState>>,
               mut next_modifier_state:   ResMut<NextState<ModifierState>>) {

  let mut color: egui::Color32 = egui::Color32::LIGHT_BLUE.linear_multiply(0.5);

  // let mut box_select = false;

  let ctx = contexts.ctx_mut();
  occupied_screen_space.right = egui::SidePanel::right("right_panel")
    .resizable(true)
    .show(ctx, |ui| {
        ui.label("Edit mode");
        ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        // ui.horizontal(|ui| {
        //   box_select = ui.button("Box Select").clicked();
        // });

        ui.vertical(|ui| {
          ui.label("Picker:");
          for &p in PickerState::iterator(){
              if ui.radio_value(&mut picker_state.get(), &p, format!("{p:?}")).clicked() {
                next_picker_state.set(p);
              };
          }
        });

        ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        if let PickerState::Brush = picker_state.get() {
          ui.add(egui::Slider::new(&mut brush_settings.radius, 1.0..=100.0).max_decimals(1));
        }

        ui.allocate_space(egui::Vec2::new(1.0, 20.0));
        ui.vertical(|ui| {
          ui.label("Modifier:");
          for &p in ModifierState::iterator(){
              if ui.radio_value(&mut modifier_state.get(), &p, format!("{p:?}")).clicked() {
                next_modifier_state.set(p);
              };
          }
        });

        ui.allocate_space(egui::Vec2::new(1.0, 20.0));

        match modifier_state.get() {
          ModifierState::Color => {
            ui.color_edit_button_srgba(&mut color);
          }
          _ => {}
        }
      


        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .width();


}

fn update_left_into_panel(mut commands:  Commands,
                          grid:          Res<GridData>,
                          hover_data:    Res<HoverData>,
                          ass:           Res<AssetServer>,
                          app_state:     Res<State<AppState>>,
                          mod_state:     Res<State<ModifierState>>,
                          planes:        Query<&PlaneData>,
                          top_left:      Query<Entity, With<TopLeftInfoPanel>>){

  let ent = top_left.get_single().unwrap();                  
  commands.entity(ent).despawn_descendants();

  let mut v: Vec<Entity> = Vec::new();
  v.push(spawn_text_node(&format!("    App State: {:?}", app_state.get()), &mut commands, &ass));  
  v.push(spawn_text_node(&format!("    Modifier: {:?}", mod_state.get()), &mut commands, &ass));  
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
      position_type:    PositionType::Absolute,
      left:             Val::Percent(0.0), 
      top:              Val::Percent(0.0), 
      width:            Val::Percent(100.0), 
      height:           Val::Px(25.0),
      flex_wrap:        FlexWrap::Wrap,
      flex_direction:   FlexDirection::Row,
      align_items:      AlignItems::FlexStart,
      justify_content:  JustifyContent::FlexStart,
      ..default()
    },
    ..default()
  })
  .insert(TopLeftInfoPanel)
  .id()
  ;
  return ent;
}
