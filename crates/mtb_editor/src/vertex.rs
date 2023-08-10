use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

#[derive(Component)]
pub struct RefVertex;

#[derive(Component)]
pub struct Vertex {
    pub loc: [f32;3],
    pub clr: [f32;4]
}
impl Vertex {
    pub fn from_loc(loc: &[f32;3]) -> Self{
        Vertex {loc: *loc, clr: [0.5, 0.5, 0.5, 1.0]}
    }
}

pub fn spawn_vertex(plane_entity: &Entity,
                    commands:     &mut Commands, 
                    handle_mesh:  &Handle<Mesh>, 
                    meshes:       &mut ResMut<Assets<Mesh>>,
                    materials:    &mut ResMut<Assets<StandardMaterial>>
                ){

    let plane_mesh = meshes.get_mut(handle_mesh).unwrap();

    let v_pos: Vec<[f32; 3]> = plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    // let mut v_clr: Vec<[f32; 4]> = Vec::new();

    let ref_loc: [f32;3] = [-1000.0, -1000.0, -1000.0];
    let default_vertex_material = materials.add(Color::BLACK.into());
    let default_vertex_mesh = meshes.add(shape::Icosphere{radius: 4.0, subdivisions: 0}.try_into().unwrap());
    let _ref_vertex =commands.spawn((PbrBundle {
                                            material: default_vertex_material.clone(),
                                            mesh: default_vertex_mesh.clone(),
                                            transform: Transform::from_translation(ref_loc.clone().into()),
                                            ..default()}, RefVertex)).id();

    let mut vertices: Vec<Entity> = Vec::new();
    for pos in v_pos.iter(){

        let entity = commands.spawn((PbrBundle {
                                        material: default_vertex_material.clone_weak(),
                                        mesh: default_vertex_mesh.clone_weak(),
                                    transform: Transform::from_translation(pos.clone().into()),
                                    ..default()}, Vertex::from_loc(pos)
        )).id();

        vertices.push(entity);

    }

    commands.entity(*plane_entity).push_children(&vertices);

}
