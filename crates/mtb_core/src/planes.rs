
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::reflect::TypeUuid;
use serde::{Serialize, Deserialize};

use crate::colors::Colors;
use crate::modifiers::{Modifier, ModifierData};
use crate::utils::{AABB, get_mesh_stats};

pub const DEFAULT_PLANE_ID: u32 = 0;
pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpawnNewPlaneEvent>()
        .add_event::<EditPlaneEvent>()
        .add_system(spawn_new_plane.run_if(on_event::<SpawnNewPlaneEvent>()))
        .add_system(edit_planes.after(spawn_new_plane).run_if(on_event::<EditPlaneEvent>()))
        .add_system(update_planes.after(edit_planes).in_base_set(CoreSet::PostUpdate))
        ;
    }
  }

  pub struct EditPlaneEvent{
    pub id:     u32,
    pub loc:    Option<[f32;3]>,
    pub dims:   Option<[f32;2]>,
    pub subs:   Option<[u32;2]>,
  }
  impl EditPlaneEvent {
    pub fn new() -> Self {
        return EditPlaneEvent{id: DEFAULT_PLANE_ID, loc: None, dims: None, subs: None};
    }
  }

  pub struct SpawnNewPlaneEvent{
    pub pd: PlaneData
  }
  impl SpawnNewPlaneEvent {
    pub fn new() -> Self {
        SpawnNewPlaneEvent{pd: PlaneData::new()}
    }
  }

  pub fn edit_planes(mut planes:       Query<&mut PlaneData>,
                     mut spawn_plane:  EventReader<EditPlaneEvent>,){

    for ev in spawn_plane.iter(){
        for mut plane in planes.iter_mut(){
            if plane.id != ev.id {
                continue; // wrong plane
            }
            if let Some(new_loc) = ev.loc{
                plane.loc = new_loc;
            }
            if let Some(new_dims) = ev.dims{
                plane.dims = new_dims;
            }
            if let Some(new_subs) = ev.subs{
                plane.subdivisions = new_subs;
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
        if let Some(entity) = ev.pd.spawn(&mut commands, &mut meshes, &mut materials){
            info!("DEBUG spawn plane {:?}", entity);
            commands.entity(entity).insert(PlaneStatus::Edit);
        }
    }
  }

  pub fn update_planes(mut commands:     Commands, 
                       mut meshes:       ResMut<Assets<Mesh>>,
                       mut planes:       Query<(Entity, &PlaneData, &mut Transform), Changed<PlaneData>>){
    for (entity, pd, mut tr) in planes.iter_mut(){
        info!("DEBUG updating plane {:?}", entity);
        pd.remesh(entity, &mut commands, &mut meshes);
        commands.entity(entity).insert(pd.get_aabb());
        *tr = Transform::from_translation(pd.loc.into());
    }
  }



#[derive(Serialize, Deserialize, Debug, Clone, Component)]
pub struct PlaneData {
    pub id:           u32,
    pub loc:          [f32; 3],
    pub subdivisions: [u32; 2],
    pub dims:         [f32; 2],
    pub color:        Colors,
    pub modifiers:    Vec<ModifierData>,
    pub active:       bool
}




impl PlaneData {

    // it just may be much much more robust to iterate every time one by one on positions per modifier
    pub fn apply(&self, mesh: &mut Mesh) -> Mesh {
        let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();

        // Convert modifier data's to modifiers, extract meta data like edges for other modifiers
        let mut mods: Vec<Modifier> = Vec::new();
        for modifier in self.modifiers.iter(){
            let m = modifier.set(self);
            mods.push(m);
        }

        let mut min_height = f32::MAX;
        let mut max_height = f32::MIN;

        // Applying modifiers to point by point
        for pos in v_pos.iter_mut(){
            for m in mods.iter(){
                pos[1] = m.apply_point(&pos, &self.loc);
            }   
            if pos[1] > max_height {
                max_height = pos[1];
            }
            if pos[1] < min_height {
                min_height = pos[1];
            }
        }

        // Applying modifiers to local area
        for m in mods.iter_mut(){
            m.apply_area(&mut v_pos);
        }

        let mut v_clr: Vec<[f32; 4]> = Vec::new();
        for pos in v_pos.iter(){
            v_clr.push(self.color.apply(pos[1], min_height, max_height));
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_clr);
        return mesh.clone()
    }

  pub fn get_aabb(&self) -> AABB {
    let min_x = self.loc[0] -1.0*self.dims[0]/2.0;
    let max_x = self.loc[0] + self.dims[0]/2.0;
    let min_z = self.loc[2] -1.0*self.dims[1]/2.0;
    let max_z = self.loc[2] + self.dims[1]/2.0;
    return AABB{min_x, max_x, min_z, max_z};
  }

  pub fn new() -> PlaneData {
    return PlaneData{id: DEFAULT_PLANE_ID,
                     loc: [0.0, 0.0, 0.0], 
                     dims: [20.0, 20.0], 
                     subdivisions: [0,0], 
                     color: Colors::new(), 
                     modifiers: Vec::new(), 
                     active: true};
    }




    pub fn spawn(&self,
                 commands:           &mut Commands, 
                 meshes:             &mut ResMut<Assets<Mesh>>,
                 materials:          &mut ResMut<Assets<StandardMaterial>>) -> Option<Entity> {

        if !self.active {
            return None;
        } else {

            let mut mesh = plane_mesh(&self.subdivisions, &self.dims);
            mesh = self.apply(&mut mesh);
            
            get_mesh_stats(&mesh);
    
            let entity = commands.spawn((PbrBundle {
                material: materials.add(StandardMaterial{..default()}),
                mesh: meshes.add(mesh),
                transform: Transform::from_translation(self.loc.into()),
                ..default()
                },
                TerrainPlane,
                self.clone(),
                self.get_aabb()
            )).id();
    
            return Some(entity);

        }
    }

    pub fn remesh(&self,  
                   entity:             Entity,               
                   commands:           &mut Commands, 
                   meshes:             &mut ResMut<Assets<Mesh>>){

        commands.entity(entity).remove::<Handle<Mesh>>();
        let mut mesh = plane_mesh(&self.subdivisions, &self.dims);
        mesh = self.apply(&mut mesh);
        get_mesh_stats(&mesh);
        commands.entity(entity).insert(meshes.add(mesh));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Planes {
    pub planes: Vec<PlaneData>
}

#[derive(Resource)]
pub struct PlanesAsset(pub Handle<Planes>);

#[derive(Component)]
pub enum PlaneStatus {
    Set,
    Edit
}

#[derive(Component)]
pub struct TerrainPlane;

fn plane_mesh(subdivisions: &[u32; 2], dims: &[f32; 2]) -> Mesh {
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

