
use bevy::input::common_conditions::input_just_pressed;
use bevy::{prelude::*, utils::HashMap};
use bevy::window::PrimaryWindow;
use mtb_core::planes::{TerrainPlane, Planes};
use mtb_core::utils::AABB;

use crate::mtb_camera::MTBCamera;
use crate::mtb_ui::GUIElement;
pub const TILE_DIM: f32 = 10.0;

pub struct MTBGridPlugin;

impl Plugin for MTBGridPlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(GridData::new())
      .insert_resource(HoverData::new())
      .add_system(hover_check.in_base_set(CoreSet::PreUpdate))
      // .add_system(update.run_if(on_event::<AssetEvent<Planes>>()).in_base_set(CoreSet::PostUpdate))
      .add_system(click.run_if(input_just_pressed(MouseButton::Left)))
      ;
  }
}


// Click on grid in edit mode
fn click(hover_data:        Res<HoverData>){
  if let Hoverables::Grid = hover_data.hoverable {
    info!("just clicked in editor");
  }
}



// Update grid tiles height. After planes update step it takes all planes and gets height per tile.
fn update(mut grid: ResMut<GridData>, 
          meshes:   Res<Assets<Mesh>>,
          planes:   Query<&Handle<Mesh>, With<TerrainPlane>>
        ){
  
  // Hashmap key by tuple? well, it works
  grid.data.clear();
  let mut tiles: HashMap<(i32, i32), Vec<f32>> = HashMap::new();

  // println!("DEBUG: updating grid heights");
  for mesh in planes.iter(){
    let md = meshes.get(mesh).unwrap();
    let v_pos: Vec<[f32; 3]> = md.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    for pos in v_pos.iter(){
      let tile_xz = grid.get_tile(pos[0], pos[2]);
      tiles.entry(tile_xz).or_insert(Vec::new()).push(pos[1]);
    }
  }

  for (tile, heights) in tiles.iter(){
    let avg_height:f32 = heights.iter().sum::<f32>()/heights.len() as f32;
    grid.data.insert(*tile, avg_height);
  }

  // println!("DEBUG: updating grid heights");
}


// check if mouse is hovering over grid, plane or gui
fn hover_check(mut hover_data:      ResMut<HoverData>,
               gui:                 Query<(&Node, &Style, &Visibility), With<GUIElement>>,
               window:              Query<&Window, With<PrimaryWindow>>,
               camera:              Query<(&Camera, &GlobalTransform), With<MTBCamera>>,
               grid:                Res<GridData>){

    hover_data.reset();
    let Ok(primary) = window.get_single() else {return;};
    let window_width = primary.width();
    let window_height = primary.height();

    if let Some(pos) = primary.cursor_position(){
        hover_data.cursor_position = Some((pos.x, pos.y));

        for (n, s, v) in gui.iter(){
          if v != Visibility::Hidden {
            let ns = n.size();
            let aabb = AABB{min_x: s.position.left.evaluate(window_width).unwrap(), 
                              max_x: ns.x, 
                              max_z: window_height - s.position.top.evaluate(window_height).unwrap(),
                              min_z: window_height - ns.y};
            if aabb.has_point(&[pos.x, 0.0, pos.y]){
              hover_data.is_hovered_gui = true;
              break;
            }
          }
        }

        let (camera, camera_transform) = camera.single();
        if let Some(ray) = camera.viewport_to_world(camera_transform, pos){
            let dist = (grid.y - ray.origin.y)/ray.direction.y;
            if dist >= 0.0 {
                let int_x: f32 = ray.origin.x + dist * ray.direction.x;
                let int_z: f32 = ray.origin.z + dist * ray.direction.z;
                let tile_xz: (i32, i32) = grid.get_tile(int_x, int_z);
                hover_data.hovered_tile_xz = tile_xz;
                hover_data.hovered_xz = (int_x, int_z);
            }
        }

        if hover_data.is_hovered_gui {
          hover_data.hoverable = Hoverables::Gui;
        } else if hover_data.is_hovered_entity {
          hover_data.hoverable = Hoverables::Entity;
        } else {
          hover_data.hoverable = Hoverables::Grid;
        }
    }


}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Hoverables {
  Grid,
  Gui,
  Entity,
  None
}

#[derive(Resource, Debug)]
pub struct HoverData {
  pub cursor_position:      Option<(f32,f32)>,
  pub hovered_entity:       Option<Entity>,
  pub hovered_tile_xz:      (i32, i32),
  pub hovered_xz:           (f32, f32),
  pub is_hovered_gui:       bool,
  pub is_hovered_entity:    bool,
  pub hoverable:            Hoverables,
  pub old_hoverable:        Hoverables,
  pub old_hovered_tile_xz:  (i32, i32),
  pub old_hovered_entity:   Option<Entity>
}

impl HoverData {
  pub fn new() -> HoverData {
        return HoverData{cursor_position: None,
                         hovered_entity: None, 
                         hovered_tile_xz: (0, 0),
                         hovered_xz: (0.0, 0.0),
                         is_hovered_gui: false, 
                         is_hovered_entity: false,
                         hoverable: Hoverables::None,
                         old_hoverable: Hoverables::None,
                         old_hovered_tile_xz: (0, 0),
                         old_hovered_entity: None
                        };
  }
  pub fn reset(&mut self){
    self.cursor_position = None;
    self.old_hovered_entity = self.hovered_entity;
    self.hovered_entity = None;
    self.old_hovered_tile_xz = self.hovered_tile_xz;
    self.hovered_tile_xz = (0, 0);
    self.is_hovered_entity = false;
    self.is_hovered_gui = false;
    self.old_hoverable = self.hoverable;
    self.hoverable = Hoverables::None;
  }
}


#[derive(Resource, Debug)]
pub struct GridData {
    pub data:           HashMap<(i32, i32), f32>, // feels dodgy but works well?
    pub tile_dim:       f32,
    pub y:              f32
}

impl GridData {
    pub fn new() -> Self {
        let gp = GridData {
            data:       HashMap::new(),
            tile_dim:   TILE_DIM,
            y:          0.0
        };
        return gp;
    }
    
    // Get tile of the position
    pub fn get_tile(&self, x: f32, z: f32) -> (i32, i32) {
        let x_i: i32 = (x/self.tile_dim) as i32;
        let z_i: i32 = (z/self.tile_dim) as i32;
        return (x_i,z_i);
    }

}
