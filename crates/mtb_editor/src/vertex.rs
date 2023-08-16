use bevy::{prelude::*, input::common_conditions::{input_pressed, input_just_pressed}};
use bevy_mod_picking::prelude::*;
use mtb_core::planes::TerrainPlane;
use crate::{mtb_grid::{HoverData, hover_check}, mtb_ui::PickerState};


pub struct VertexPlugin;

impl Plugin for VertexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PickVertex>()
        .add_startup_system(setup)
        .add_system(pick_vertex.run_if(on_event::<PickVertex>()))
        .add_system(clear.run_if(input_just_pressed(MouseButton::Right)).in_base_set(CoreSet::PreUpdate))
        .add_system(highlight_picked.after(pick_vertex)
                                    .in_base_set(CoreSet::PostUpdate))
        .add_system(drag.run_if(input_pressed(MouseButton::Left))
                        .after(hover_check)
                        .in_set(OnUpdate(PickerState::Point)))
        .add_system(vertex_update.after(drag).in_base_set(CoreSet::PostUpdate))
        ;
    }
}


// Click on grid in edit mode
fn clear(mut commands: Commands,
         picked_vertex: Query<Entity, With<PickedVertex>>){
    for v in picked_vertex.iter(){
      commands.entity(v).remove::<PickedVertex>();
    }
}


pub fn drag(mut picked_vertex: Query<&mut Transform, With<PickedVertex>>, 
            hover_data:        Res<HoverData>){

    let delta_x = hover_data.hovered_xz.0 - hover_data.old_hovered_xz.0;
    let delta_y = hover_data.hovered_xz.1 - hover_data.old_hovered_xz.1;

    for mut tr in picked_vertex.iter_mut(){
        tr.translation.x += delta_x;
        tr.translation.z += delta_y;
    }

}


pub fn vertex_update(mut vertex: Query<(&Transform, &mut Vertex, &Parent), Changed<Transform>>,
                     planes: Query<&Handle<Mesh>, With<TerrainPlane>>,
                     mut meshes: ResMut<Assets<Mesh>>
){
    for (transform, mut vertex, parent) in vertex.iter_mut(){

        if let Ok(plane_mesh_handle) = planes.get(**parent) {
            if let Some(plane_mesh) = meshes.get_mut(plane_mesh_handle) {
                let mut v_pos: Vec<[f32; 3]> = plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
                v_pos[vertex.index] = transform.translation.into();
                plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
            }
        }
        vertex.loc = transform.translation.into();
    }
}


pub struct PickVertex {
    pub entity: Entity
}
impl From<ListenedEvent<Down>> for PickVertex {
    fn from(event: ListenedEvent<Down>) -> Self {
        PickVertex{entity: event.target}
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

    // let ref_loc: [f32;3] = [0.0, 10.0, 0.0];
    let default_vertex_material = materials.add(Color::BLACK.with_a(0.85).into());
    let red_vertex_material = materials.add(Color::ORANGE_RED.with_a(0.85).into());
    let default_vertex_mesh = meshes.add(shape::UVSphere{radius: 3.0, ..default()}.into());

    // let default_vertex_mesh = meshes.add(shape::Cube{size: 25.0}.into());

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

pub fn pick_vertex(mut commands:          Commands,
                   mut pick_vertex_event: EventReader<PickVertex>,
                   picked_vertex:         Query<&PickedVertex>
                ){

    for ev in pick_vertex_event.iter(){
        if let Ok(_ent) = picked_vertex.get(ev.entity) {
            commands.entity(ev.entity).remove::<PickedVertex>();
        } else {
            commands.entity(ev.entity).insert(PickedVertex);
        }
    }
}


pub fn highlight_picked(
    mut commands:          Commands,
    mut vertex:            Query<(Entity, Option<&PickedVertex>), With<Vertex>>, 
    refs:                  Res<VertexRefs>){

    for (entity, picked) in vertex.iter_mut(){
        if picked.is_some(){
            commands.entity(entity).insert(refs.picked_mat.clone_weak());
        } else {
            commands.entity(entity).insert(refs.mat.clone_weak());
        }
    }
}





#[derive(Component)]
pub struct RefVertex;

#[derive(Component)]
pub struct PickedVertex;

#[derive(Component)]
pub struct Vertex {
    pub index: usize,
    pub loc: [f32;3],
    pub clr: [f32;4]
}
impl Vertex {
    pub fn from_loc(loc: &[f32;3], index: usize) -> Self{
        Vertex {loc: *loc, clr: [0.5, 0.5, 0.5, 1.0], index}
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
    for (index, pos) in v_pos.iter().enumerate(){

        let entity = commands.spawn((PbrBundle {
                                        material: refs.mat.clone_weak(),
                                        mesh: refs.mesh.clone_weak(),
                                    transform: Transform::from_translation(pos.clone().into()),
                                    ..default()}, 
                                    Vertex::from_loc(pos, index),
                                    PickableBundle::default(),
                                    RaycastPickTarget::default(),
                                    OnPointer::<Down>::send_event::<PickVertex>(),
                                    // OnPointer::<DragStart>::target_remove::<Pickable>(), // Disable picking
                                    // OnPointer::<DragEnd>::target_insert(Pickable), // Re-enable picking
                                    // OnPointer::<Drag>::target_component_mut::<Transform>(|drag, transform| {
                                    //     transform.translation += drag.delta.extend(0.0) // Make the square follow the mouse
                                    // }),
                                )).id();

        vertices.push(entity);

    }

    commands.entity(*plane_entity).push_children(&vertices);

}
