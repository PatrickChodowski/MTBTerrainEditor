
use bevy::{prelude::*, utils::HashMap};
use bevy::window::PrimaryWindow;
use crate::core::planes::{TerrainPlane, PlaneData};
use crate::core::utils::AABB;
use bevy_egui::EguiContext;

use super::mtb_camera::MTBCamera;
pub const TILE_DIM: f32 = 10.0;

pub struct MTBGridPlugin;

impl Plugin for MTBGridPlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(GridData::new())
      .insert_resource(HoverData::new())
      .add_systems(PreUpdate, hover_check)
      ;
  }
}


// Update grid tiles height. After planes update step it takes all planes and gets height per tile.
fn _update(mut grid: ResMut<GridData>, 
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
pub fn hover_check(mut hover_data:      ResMut<HoverData>,
                   mut egui_context:    Query<(Entity, &mut EguiContext)>,
                   planes:              Query<(Entity, &AABB), With<PlaneData>>,
                   window:              Query<(Entity, &Window), With<PrimaryWindow>>,
                   camera:              Query<(&Camera, &GlobalTransform), With<MTBCamera>>,
                   grid:                Res<GridData>){

    hover_data.reset();
    let mut is_hovered_gui: bool = false;
    let mut hovered_entity: Option<Entity> = None;

    let Ok((window_entity, primary)) = window.get_single() else {return;};

    if let Some(pos) = primary.cursor_position(){
        hover_data.cursor_position = Some((pos.x, pos.y));

        if let Ok((_entity, mut ctx)) = egui_context.get_mut(window_entity) {
          if ctx.get_mut().is_pointer_over_area() {
            is_hovered_gui = true;
          }
        }
      
        let (camera, camera_transform) = camera.single();
        if let Some(ray) = camera.viewport_to_world(camera_transform, pos){
            let dist = (grid.y - ray.origin.y)/ray.direction.y;
            if dist >= 0.0 {
                let int_x: f32 = ray.origin.x + dist * ray.direction.x;
                let int_z: f32 = ray.origin.z + dist * ray.direction.z;

                for (entity, aabb) in planes.iter(){
                  if aabb.has_point(&[int_x,0.0,int_z]){
                    hovered_entity = Some(entity);
                  }
                }

                hover_data.hovered_tile_xz = grid.get_tile(int_x, int_z);
                hover_data.hovered_xz = (int_x, int_z);
            }
        }

        if is_hovered_gui {
          hover_data.hoverable = Hoverables::Gui;
        } else if hovered_entity.is_some() {
          hover_data.hoverable = Hoverables::Entity(hovered_entity.unwrap());
        } else {
          hover_data.hoverable = Hoverables::Grid;
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Hoverables {
  Grid,
  Gui,
  Entity(Entity),
  None
}

#[derive(Resource, Debug)]
pub struct HoverData {
  pub cursor_position:      Option<(f32,f32)>,
  pub hovered_xz:           (f32, f32),
  pub old_hovered_xz:       (f32, f32),
  pub hovered_tile_xz:      (i32, i32),
  pub hoverable:            Hoverables,
  pub old_hoverable:        Hoverables,

}

impl HoverData {
  pub fn new() -> HoverData {
        return HoverData{cursor_position: None,
                         hovered_xz: (0.0, 0.0),
                         old_hovered_xz: (0.0, 0.0),
                         hovered_tile_xz: (0, 0),
                         hoverable: Hoverables::None,
                         old_hoverable: Hoverables::None};
  }
  pub fn reset(&mut self){
    self.cursor_position = None;
    self.old_hovered_xz = self.hovered_xz;
    self.hovered_xz = (0.0, 0.0);
    self.hovered_tile_xz = (0, 0);
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
