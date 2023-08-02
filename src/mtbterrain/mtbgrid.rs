use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use grid::*;

pub const TILE_DIM: f32 = 10.0;

#[derive(Component)]
pub struct MTBCamera;

pub struct GridPlugin;

impl Plugin for GridPlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(GridData::new())
      .insert_resource(HoverData::new())
      .add_system(hover_check.in_base_set(CoreSet::PreUpdate))
      ;
  }
}

// check if mouse is hovering over grid, plane or gui
fn hover_check(//gui:                 Query<(&Node, &Style, &Visibility), With<GUIHoverable>>,
               mut hover_data:      ResMut<HoverData>,
            //    planes:              Query<Entity, With<TerrainPlane>>,
               window:              Query<&Window, With<PrimaryWindow>>,
               camera:              Query<(&Camera, &GlobalTransform), With<MTBCamera>>,
               grid:                Res<GridData>){

    hover_data.reset();
    let Ok(primary) = window.get_single() else {return;};
    // let window_width = primary.width();
    // let window_height = primary.height();

    if let Some(cursor_position) = primary.cursor_position(){
        hover_data.cursor_position = Some((cursor_position.x, cursor_position.y));
              
        let (camera, camera_transform) = camera.single();
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position){
            let dist = (grid.loc.1 - ray.origin.y)/ray.direction.y;
            if dist >= 0.0 {
                let int_x: f32 = ray.origin.x + dist * ray.direction.x;
                let int_z: f32 = ray.origin.z + dist * ray.direction.z;
                let tile_xz: (i32, i32) = grid.get_tile(int_x, int_z);
                hover_data.hovered_tile_xz = tile_xz;
                println!(" [Debug] Tile: {:?} Loc: ({},{})", hover_data.hovered_tile_xz, int_x, int_z);
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
struct HoverData {
  cursor_position:      Option<(f32,f32)>,
  hovered_entity:       Option<Entity>,
  hovered_tile_xz:      (i32, i32),
  is_hovered_gui:       bool,
  is_hovered_entity:    bool,
  hoverable:            Hoverables,
  old_hoverable:        Hoverables,
  old_hovered_tile_xz:  (i32, i32),
  old_hovered_entity:   Option<Entity>
}

impl HoverData {
  pub fn new() -> HoverData {
        return HoverData{cursor_position: None,
                         hovered_entity: None, 
                         hovered_tile_xz: (0, 0),
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
    pub data:           Grid<GridTile>,
    pub tile_dim:       f32,
    pub counts_xz:      (u32, u32),
    pub dims:           (f32, f32),
    pub loc:            (f32, f32, f32)
}

impl GridData {
    pub fn new() -> Self {
        let mut gp = GridData {
            data:       grid![],
            tile_dim:   TILE_DIM,
            loc:        (0.0, 0.0, 0.0),
            counts_xz:  (0, 0),
            dims:       (0.0, 0.0)
        };

        for _x in 0..gp.counts_xz.0{
            let mut row: Vec<GridTile> = Vec::with_capacity(gp.counts_xz.1 as usize);
            for _z in 0..gp.counts_xz.1 {
              row.push(GridTile::new(_x, _z));
            }
            gp.data.push_row(row);
          }

        return gp;
    }
    
    // Get tile of the position
    pub fn get_tile(&self, x: f32, z: f32) -> (i32, i32) {
        let x_i: i32 = (x/self.tile_dim) as i32;
        let z_i: i32 = (z/self.tile_dim) as i32;
        return (x_i,z_i);
    }

}


#[derive(Copy, Clone, Debug, Reflect)]
pub struct GridTile {
  pub height:   f32,
  pub tile:     (u32, u32),     
}

impl GridTile {
    pub fn new(x: u32, z: u32) -> Self {
        return GridTile{height: 0.0, tile:(x,z)};
    }
}
