use bevy::prelude::*;
//use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct VertexPlugin;

impl Plugin for VertexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_system(highlight_picked.in_base_set(CoreSet::PostUpdate))
        ;
    }
}

#[derive(Resource)]
pub struct VertexRefs {
    pub mesh:           Handle<Mesh>,
    pub mat:            Handle<StandardMaterial>,
    pub picked_mat:     Handle<StandardMaterial>
}

// spawn references
pub fn setup(mut commands:     Commands,
             mut materials:    ResMut<Assets<StandardMaterial>>,
             mut meshes:       ResMut<Assets<Mesh>>
){
    
    let ref_loc: [f32;3] = [-5000.0, -5000.0, -5000.0]; // basically hell
    let default_vertex_material = materials.add(Color::BLACK.into());
    let red_vertex_material = materials.add(Color::ORANGE_RED.into());
    let default_vertex_mesh = meshes.add(shape::UVSphere::default().into());

    commands.spawn((PbrBundle {
        material: default_vertex_material.clone(),
        mesh: default_vertex_mesh.clone(),
        transform: Transform::from_translation(ref_loc.clone().into()),
        ..default()}, RefVertex));

     commands.spawn((PbrBundle {
            material: red_vertex_material.clone(),
            mesh: default_vertex_mesh.clone(),
            transform: Transform::from_translation(ref_loc.clone().into()),
            ..default()}, RefVertex));

    let refs = VertexRefs{mesh: default_vertex_mesh, mat: default_vertex_material, picked_mat: red_vertex_material};
    commands.insert_resource(refs);

}


pub fn highlight_picked(mut vertex: Query<(&mut Handle<StandardMaterial>, Option<&PickedVertex>), With<Vertex>>, 
                        refs:       Res<VertexRefs>
                    ){

    for (mut mat, picked) in vertex.iter_mut(){

        // if picked.is_some(){
        //    mat = *refs.picked_mat.clone_weak();
        // }

    }

}




#[derive(Component)]
pub struct RefVertex;

#[derive(Component)]
pub struct PickedVertex;

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
                    refs:         &Res<VertexRefs>
                ){

    let plane_mesh = meshes.get_mut(handle_mesh).unwrap();

    let v_pos: Vec<[f32; 3]> = plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    // let mut v_clr: Vec<[f32; 4]> = Vec::new();
    let mut vertices: Vec<Entity> = Vec::new();
    for pos in v_pos.iter(){

        let entity = commands.spawn((PbrBundle {
                                        material: refs.mat.clone_weak(),
                                        mesh: refs.mesh.clone_weak(),
                                    transform: Transform::from_translation(pos.clone().into()),
                                    ..default()}, Vertex::from_loc(pos)
        )).id();

        vertices.push(entity);

    }

    commands.entity(*plane_entity).push_children(&vertices);

}
