
use bevy::prelude::*;
use bevy::input::common_conditions::{input_pressed, input_just_pressed};
use bevy_egui::EguiContexts;
use bevy_egui::egui::{DragValue, Window};
use bevy_mod_picking::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::reflect::{TypeUuid, TypePath};
use serde::{Serialize, Deserialize};
use crate::editor::{AppState, DoubleClick};
use super::utils::{AABB, get_mesh_stats};
use crate::editor::mtb_grid::{hover_check, HoverData, Hoverables};

pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpawnNewPlaneEvent>()
        .add_event::<PickPlane>()
        .add_systems(PreUpdate,  clear.run_if(input_just_pressed(MouseButton::Right)).run_if(in_state(AppState::Object)))
        .add_systems(Update,     spawn_new_plane.run_if(on_event::<SpawnNewPlaneEvent>()).run_if(in_state(AppState::Object)))
        .add_systems(Update,     pick_plane.run_if(in_state(AppState::Object)))
        .add_systems(Update,     drag.run_if(input_pressed(MouseButton::Left)
                                 .and_then(in_state(AppState::Object)))
                                 .after(hover_check))
        .add_systems(PostUpdate, highlight_picked_plane
                                        .after(pick_plane)
                                        .run_if(in_state(AppState::Object))
                                    )
        .add_systems(Update,     drop_plane.run_if(input_just_pressed(KeyCode::Back)))
        .add_systems(OnExit(AppState::Object), deselect_plane)

        .add_systems(PostUpdate, spawn_edit_plane.run_if(in_state(AppState::Object).and_then(on_event::<DoubleClick>())))
        .add_systems(PostUpdate, edit_plane.run_if(in_state(AppState::Object)))
        ;
    }
  }

  pub fn spawn_edit_plane(hover_data:        Res<HoverData>,
                          mut planes:        Query<&mut PlaneEdit>) {
    
      if let Hoverables::Entity(entity) = hover_data.hoverable {
          if let Ok(mut pd) = planes.get_mut(entity) {
              pd.0 = true;
          }
      }
      
    }

  pub fn edit_plane(mut contexts:      EguiContexts,
                    mut planes:        Query<(&mut PlaneData, &mut Transform, &mut AABB,  &mut PlaneEdit)>){

    let ctx = contexts.ctx_mut();
    for (mut pd, mut tr, mut aabb, mut pe) in planes.iter_mut() {
        if !pe.0 {
            continue;
        }

        Window::new(format!("Plane Edit {}", pd.label))
                .open(&mut pe.0)
                .resizable(true)
                .default_width(280.0)
                .show(ctx, |ui| {
                    
                    ui.columns(2, |columns| {
                        columns[0].label("Loc X ");
                        columns[1].add(DragValue::new(&mut pd.loc[0]).speed(1.0));
                        columns[0].label("Loc Y ");
                        columns[1].add(DragValue::new(&mut pd.loc[1]).speed(1.0));
                        columns[0].label("Loc Z ");
                        columns[1].add(DragValue::new(&mut pd.loc[2]).speed(1.0));
                    }
                );

        });

        tr.translation.x = pd.loc[0];  
        tr.translation.y = pd.loc[1];  
        tr.translation.z = pd.loc[2];   
        *aabb = pd.get_aabb();
    }
  }


  pub fn drop_plane(mut commands: Commands, 
                    planes:       Query<(Entity, &PickedPlane)>) {
    for (entity, picked) in planes.iter(){
        if picked.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
  }

  #[derive(Event)]
  pub struct PickPlane {
    pub entity: Entity
  }

  impl  From<ListenerInput<Pointer<Down>>> for PickPlane {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        PickPlane{entity: event.target}
    }
}

  #[derive(Event)]
  pub struct SpawnNewPlaneEvent {
    pub pd: PlaneData
  }
  impl SpawnNewPlaneEvent {
    pub fn new() -> Self {
        SpawnNewPlaneEvent{pd: PlaneData::new()}
    }
  }

#[derive(Component)]
pub struct PickedPlane(pub bool);

// Click on grid in object mode
fn clear(mut planes: Query<&mut PickedPlane>){
    for mut picked in planes.iter_mut(){
        picked.0 = false;
    }
}


pub fn pick_plane(mut pick_plane_event:    EventReader<PickPlane>,
                  mut planes:              Query<&mut PickedPlane>){
    for ev in pick_plane_event.iter(){
        if let Ok(mut picked) = planes.get_mut(ev.entity){
            picked.0 = true;
        }
    }
}

// it shouldnt work all the time
pub fn highlight_picked_plane(
    mut materials:         ResMut<Assets<StandardMaterial>>,
    planes:                Query<(&mut Handle<StandardMaterial>, &PickedPlane), Changed<PickedPlane>>){
    
    for (handle_mat, picked) in planes.iter(){
        if let Some(mat) = materials.get_mut(handle_mat){
            if picked.0 {
                mat.base_color.set_g(0.4);
                mat.base_color.set_b(0.4);
            } else {
                mat.base_color.set_g(1.0);
                mat.base_color.set_b(1.0);
            }
        }
    }
}
  pub fn spawn_new_plane(mut commands:     Commands, 
                         mut meshes:       ResMut<Assets<Mesh>>,
                         mut materials:    ResMut<Assets<StandardMaterial>>,
                         mut spawn_plane:  EventReader<SpawnNewPlaneEvent>,
                        ){
    for ev in spawn_plane.iter(){
        let _entity = ev.pd.spawn(&mut commands, &mut meshes, &mut materials);
    }
  }

#[derive(Serialize, Deserialize, Debug, Clone, Component, Resource)]
pub struct PlaneData {
    pub label:        String,
    pub loc:          [f32; 3],
    pub subdivisions: [u32; 2],
    pub dims:         [f32; 2]
}

#[derive(Component)]
pub struct PlaneEdit(pub bool);

impl PlaneData {

  pub fn get_aabb(&self) -> AABB {
    let min_x = self.loc[0] -1.0*self.dims[0]/2.0;
    let max_x = self.loc[0] + self.dims[0]/2.0;
    let min_z = self.loc[2] -1.0*self.dims[1]/2.0;
    let max_z = self.loc[2] + self.dims[1]/2.0;
    return AABB{min_x, max_x, min_z, max_z};
  }



  pub fn new() -> PlaneData {
    return PlaneData{label: "Default Plane".to_string(),
                     loc: [0.0, 0.0, 0.0], 
                     dims: [200.0, 200.0], 
                     subdivisions: [10,10]
                    };
    }




    pub fn spawn(&self,
                 commands:           &mut Commands, 
                 meshes:             &mut ResMut<Assets<Mesh>>,
                 materials:          &mut ResMut<Assets<StandardMaterial>>) -> Entity {

        let mesh = plane_mesh(&self.subdivisions, &self.dims);
        get_mesh_stats(&mesh);
    
        let entity = commands.spawn((PbrBundle {
            material: materials.add(StandardMaterial{alpha_mode: AlphaMode::Mask(0.5),..default()}),
            mesh: meshes.add(mesh),
            transform: Transform::from_translation(self.loc.into()),
            ..default()
            },
            PickedPlane(false),
            TerrainPlane,
            PlaneEdit(false),
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Down>>::send_event::<PickPlane>(),
            self.clone(),
            self.get_aabb()
        )).id();
    
        return entity;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Planes {
    pub planes: Vec<PlaneData>
}

#[derive(Resource)]
pub struct PlanesAsset(pub Handle<Planes>);

#[derive(Component)]
pub struct TerrainPlane;

pub fn plane_mesh(subdivisions: &[u32; 2], dims: &[f32; 2]) -> Mesh {
    let mesh = Mesh::from(RectPlane {
        width: dims[0],
        length: dims[1],  
        x_subdivisions: subdivisions[0],
        z_subdivisions: subdivisions[1]
    });
    return mesh;
}

#[derive(Debug, Copy, Clone)]
pub struct RectPlane {
    pub width: f32,   // width
    pub length: f32,   // length or depth, depends on how you look
    pub x_subdivisions: u32,
    pub z_subdivisions: u32
}


// Based stronly on plane code

impl From<RectPlane> for Mesh {
    fn from(rect_plane: RectPlane) -> Self {
        let z_vertex_count = rect_plane.x_subdivisions + 2;
        let x_vertex_count = rect_plane.z_subdivisions + 2;
        let num_vertices = (z_vertex_count * x_vertex_count) as usize;
        let num_indices = ((z_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;
        let up = Vec3::Y.to_array();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        for y in 0..z_vertex_count {
            for x in 0..x_vertex_count {
                let tx = x as f32 / (x_vertex_count - 1) as f32;
                let ty = y as f32 / (z_vertex_count - 1) as f32;
                positions.push([(-0.5 + tx) * rect_plane.width, 0.0, (-0.5 + ty) * rect_plane.length]);
                normals.push(up);
                uvs.push([tx, 1.0 - ty]);
            }
        }

        for y in 0..z_vertex_count - 1 {
            for x in 0..x_vertex_count - 1 {
                let quad = y * x_vertex_count + x;
                indices.push(quad + x_vertex_count + 1);
                indices.push(quad + 1);
                indices.push(quad + x_vertex_count);
                indices.push(quad);
                indices.push(quad + x_vertex_count);
                indices.push(quad + 1);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

pub fn deselect_plane(mut materials:    ResMut<Assets<StandardMaterial>>,
                      mut planes:       Query<(&mut Handle<StandardMaterial>, &mut PickedPlane)>){
    for (handle_mat, mut picked) in planes.iter_mut(){
        if picked.0 {
            picked.0 = false;
            if let Some(mat) = materials.get_mut(&handle_mat){
                mat.base_color.set_g(1.0);
                mat.base_color.set_b(1.0);
            }

        }
    }
}

pub fn drag(mut picked_plane:  Query<(&mut Transform, &mut AABB, &mut PlaneData, &PickedPlane)>, 
            hover_data:        Res<HoverData>){

    let delta_x = hover_data.hovered_xz.0 - hover_data.old_hovered_xz.0;
    let delta_y = hover_data.hovered_xz.1 - hover_data.old_hovered_xz.1;

    for (mut tr, mut aabb, mut pd, picked)  in picked_plane.iter_mut(){
        if picked.0 {
            tr.translation.x += delta_x;
            tr.translation.z += delta_y;
    
            pd.loc[0] = tr.translation.x;
            pd.loc[2] = tr.translation.z;
    
            *aabb = pd.get_aabb();
        }
    }

}
