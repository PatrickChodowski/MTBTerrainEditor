
use bevy::prelude::*;
use bevy::pbr::wireframe::Wireframe;
#[allow(unused_imports)]
use bevy::prelude::shape::Plane;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::terrain::noises::apply_noise;
use crate::terrain::utils::{PlaneData, Planes, PlanesAsset};

#[allow(unused_imports)]
use crate::tools::mapgrid::{MIN_X, MAX_X, MIN_Z, MAX_Z};


pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_startup_system(setup)
      .add_system(update.run_if(on_event::<AssetEvent<Planes>>()))
      ;
  }
}

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
               pd: &PlaneData){

    let mut mesh = plane_mesh(pd.subdivisions, &pd.dims);

    if let Some(noise_data) = pd.noise_data {
        mesh = apply_noise(&mut mesh, noise_data).clone();
    }

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
