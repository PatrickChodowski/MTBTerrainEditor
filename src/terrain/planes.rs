
use bevy::prelude::*;
use bevy::pbr::wireframe::Wireframe;
#[allow(unused_imports)]
use bevy::prelude::shape::Plane;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::reflect::TypeUuid;

use bevy::utils::HashMap;
use serde::{Serialize, Deserialize};

#[allow(unused_imports)]
use crate::settings::{MIN_X, MAX_X, MIN_Z, MAX_Z};
use crate::terrain::modifiers::{Modifier, ModifierFN};
use crate::terrain::utils::AABB;

pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_startup_system(setup)
      .add_system(update.run_if(on_event::<AssetEvent<Planes>>()))
      ;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaneData {
    pub loc:          [f32; 3],
    pub subdivisions: u32,
    pub dims:         (f32, f32),
    pub color:        PlaneColor,
    pub modifiers:    Vec<Modifier>
}

impl PlaneData {
  pub fn apply(&self, mesh: &mut Mesh) -> Mesh {
    let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();

    // Unpack modifiers
    let mut modifier_functions: Vec<ModifierFN> = Vec::new();
    for modifier in self.modifiers.iter(){
      modifier_functions.push(modifier.bake(&self));
    }

    // Iterate through vector and apply modifiers
    for pos in v_pos.iter_mut(){
      for m in modifier_functions.iter(){
        pos[1] = m.modifier.apply(&pos, &m.aabbs, &self.loc);
      }   
    }

    // Colors based on height
    let mut v_clr: Vec<[f32; 4]> = Vec::new();
    let min_height: f32 = 0.0;
    let max_height: f32 = 0.0;
    for pos in v_pos.iter(){
        v_clr.push(self.color.apply(pos[1], min_height, max_height));
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_clr);
    return mesh.clone()
  }

  pub fn get_aabb(&self) -> AABB {
    let min_x = -1.0*self.dims.0/2.0;
    let max_x = self.dims.0/2.0;
    let min_z = -1.0*self.dims.1/2.0;
    let max_z = self.dims.1/2.0;
    return AABB{min_x, max_x, min_z, max_z};
  }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlaneColor {
    Color([f32; 4]),
    Steps(Vec<(f32, [f32; 4])>), // Vector of tuples of (high limit, color)
    Gradient([f32; 4],[f32; 4])
}

impl PlaneColor {
    pub fn apply(&self, height: f32, min_height: f32, max_height: f32) -> [f32; 4] {
        match self {
            PlaneColor::Steps(v) => {[0.8, 0.5, 0.5, 1.0]}
            PlaneColor::Gradient(low, high) => {[0.8, 0.8, 0.5, 1.0]} // need min height and max height
            _ => {[0.5, 0.5, 0.5, 1.0]}
        }
    }
}




#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Planes(pub Vec<PlaneData>);

#[derive(Resource)]
pub struct PlanesAsset(pub Handle<Planes>);


#[derive(Component)]
pub struct TerrainPlane;

fn setup(mut commands: Commands, ass: Res<AssetServer>){
    let handle = PlanesAsset(ass.load("data/planes.json"));
    commands.insert_resource(handle);
}


// generates planes
fn update(mut commands:           Commands,
          mut meshes:             ResMut<Assets<Mesh>>,
          mut materials:          ResMut<Assets<StandardMaterial>>,
          terrain_planes:         Query<Entity, With<TerrainPlane>>,
          planes_assets:          Res<Assets<Planes>>,
          handle:                 Res<PlanesAsset>){

    for entity in terrain_planes.iter(){
        commands.entity(entity).despawn_recursive();
    }

    for pd in planes_assets.get(&handle.0).unwrap().0.iter(){
        spawn_plane(&mut commands, &mut meshes, &mut materials, &pd); 
    }
}


fn spawn_plane(commands:           &mut Commands, 
               meshes:             &mut ResMut<Assets<Mesh>>,
               materials:          &mut ResMut<Assets<StandardMaterial>>,   
               pd:                 &PlaneData){

    let mut mesh = plane_mesh(pd.subdivisions, &pd.dims);
    mesh = pd.apply(&mut mesh);

    let material: Handle<StandardMaterial>;
    match pd.color {
        PlaneColor::Color(clr) => {material = materials.add(StandardMaterial::from(Color::from(clr)));}
        _ => {material = materials.add(StandardMaterial{..default()});}
    }

    commands.spawn((PbrBundle {
        material,
        mesh: meshes.add(mesh),
        transform: Transform::from_translation(pd.loc.into()),
        ..default()
        },
        TerrainPlane,
        Wireframe
    ));
}

fn plane_mesh(subdivisions: u32, dims: &(f32, f32)) -> Mesh {
    let mesh = Mesh::from(RectPlane {
        width: dims.0,
        length: dims.1,  
        subdivisions
    });
    return mesh;
}

#[derive(Debug, Copy, Clone)]
pub struct RectPlane {
    pub width: f32,   // width
    pub length: f32,   // length or depth, depends on how you look
    pub subdivisions: u32
}


// Based stronly on plane code

impl From<RectPlane> for Mesh {
    fn from(rect_plane: RectPlane) -> Self {
        let z_vertex_count = rect_plane.subdivisions + 2;
        let x_vertex_count = rect_plane.subdivisions + 2;
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
