use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::pbr::wireframe::Wireframe;
#[allow(unused_imports)]
use bevy::prelude::shape::Plane;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use serde::{Serialize, Deserialize};

use crate::terrain::noises::{apply_noise, Noises};
#[allow(unused_imports)]
use crate::tools::mapgrid::{MIN_X, MAX_X, MIN_Z, MAX_Z};
use crate::utils::read_txt;

pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(Planes::load())
      .add_system(update.run_if(input_just_pressed(KeyCode::Space)))
      ;
  }
}

#[derive(Component)]
pub struct TerrainPlane;

#[derive(Resource)]
pub struct Planes {
    pub data: Vec<PlaneData>
}

impl Planes {
    pub fn load() -> Self {
        let path: &str = &format!("./assets/data/planes.json");
        let data: String = read_txt(path);
        let planes: Vec<PlaneData> = serde_json::from_str(&data).expect(&format!(
            "\n [ERROR models.setup] Unable to get data from {path} \n"
        ));
        return Planes{data: planes};
    }
    pub fn reload(&mut self){
        let path: &str = &format!("./assets/data/planes.json");
        let data: String = read_txt(path);
        let planes: Vec<PlaneData> = serde_json::from_str(&data).expect(&format!(
            "\n [ERROR models.setup] Unable to get data from {path} \n"
        ));
        self.data = planes;
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlaneData {
    pub loc: (f32, f32, f32),
    pub subdivisions: u32,
    pub dims: (f32, f32),
    pub color: [f32; 4]
}


// generates planes
fn update(mut commands:           Commands,
          mut meshes:             ResMut<Assets<Mesh>>,
          mut materials:          ResMut<Assets<StandardMaterial>>,
          terrain_planes:         Query<Entity, With<TerrainPlane>>,
          mut planes:             ResMut<Planes>
        ){

    for entity in terrain_planes.iter(){
        commands.entity(entity).despawn_recursive();
    }

    planes.reload();

    for pd in planes.data.iter(){
        spawn_plane(&mut commands, &mut meshes, &mut materials, &pd); 
    }
}


fn spawn_plane(commands:           &mut Commands, 
               meshes:             &mut ResMut<Assets<Mesh>>,
               materials:          &mut ResMut<Assets<StandardMaterial>>,   
               pd: &PlaneData){

    let mut mesh = plane_mesh(pd.subdivisions, &pd.dims);
    mesh = apply_noise(&mut mesh, Noises::Perlin).clone();

    commands.spawn((PbrBundle {
        material: materials.add(StandardMaterial::from(Color::from(pd.color))),
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
