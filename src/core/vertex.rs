use bevy::{prelude::*, input::common_conditions::{input_pressed, input_just_pressed}, render::mesh::VertexAttributeValues};
use bevy_mod_picking::prelude::*;
use super::planes::TerrainPlane;
use crate::editor::{mtb_grid::{HoverData, hover_check}, mtb_ui::{PickerState, ApplyModifierEvent, ModResources, ModifierState}, AppState};


pub struct VertexPlugin;

impl Plugin for VertexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PickVertex>()
        .add_systems(Startup, setup)
        .add_systems(Update, pick_vertex.run_if(on_event::<PickVertex>()).run_if(in_state(AppState::Edit)))
        .add_systems(PreUpdate, clear.run_if(input_just_pressed(MouseButton::Right)).run_if(in_state(AppState::Edit)))
        .add_systems(PostUpdate, highlight_picked.after(pick_vertex).run_if(in_state(AppState::Edit)))
        .add_systems(Update, drag.run_if(input_pressed(MouseButton::Left)
                                 .and_then(in_state(PickerState::Point))
                                 .and_then(in_state(AppState::Edit))
                                ).after(hover_check))
        .add_systems(Update, apply_modifiers.run_if(in_state(AppState::Edit)))
        .add_systems(PostUpdate, vertex_update_transform.after(drag).after(apply_modifiers).run_if(in_state(AppState::Edit)))
        .add_systems(PostUpdate, vertex_update_vertex.after(apply_modifiers).run_if(in_state(AppState::Edit)))
        .add_systems(OnExit(AppState::Edit), deselect_vertex)

        ;
    }
}


fn apply_modifiers(
    mut apply_mod:      EventReader<ApplyModifierEvent>,
    mut mod_res:        ResMut<ModResources>,
    mut picked_vertex:  Query<(&mut Transform, &mut Vertex), With<PickedVertex>>
) {

    for ev in apply_mod.iter(){
        info!(" Applied modifier {:?}", ev.mod_type);

        mod_res.color.set();
        mod_res.color_gradient.set();
        let nfn = mod_res.noise.set();
        let wnfn = mod_res.wave.noise.set();

        for (mut tr, mut v) in picked_vertex.iter_mut(){

            match ev.mod_type {
                ModifierState::Color => {
                    v.clr = mod_res.color.apply();
                }
                ModifierState::ColorGradient => {
                    v.clr = mod_res.color_gradient.apply(v.loc[1]);
                }
                ModifierState::Value => {
                    let height = mod_res.value.apply(&v.loc);
                    v.loc[1] = height;
                    tr.translation[1] = height;
                }
                ModifierState::Noise => {
                    let noise_height = mod_res.noise.apply(&nfn, &v.loc, &v.loc);
                    v.loc[1] = noise_height;
                    tr.translation[1] = noise_height;
                }
                ModifierState::Wave => {
                    let pos = mod_res.wave.noise.apply(&wnfn, &v.loc, &v.loc);
                    v.loc[1] = pos;
                    tr.translation[1] = pos;
                }
                _ => {}
            }
        }
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


pub fn vertex_update_transform(mut vertex: Query<(&Transform, &mut Vertex, &Parent), Changed<Transform>>,
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

pub fn vertex_update_vertex(vertex: Query<(&mut Vertex, &Parent), Changed<Vertex>>,
                            planes: Query<&Handle<Mesh>, With<TerrainPlane>>,
                            mut meshes: ResMut<Assets<Mesh>>
){
    let mut plane_ref: Entity = Entity::PLACEHOLDER;
    let mut v_clr: Option<Vec<[f32;4]>> = None;
    let mut v_pos: Option<Vec<[f32; 3]>> = None;

    for (index, (_vertex, parent)) in vertex.iter().enumerate(){
        if index == 0 {
            plane_ref = **parent;
            if let Ok(plane_mesh_handle) = planes.get(plane_ref) {
                if let Some(_plane_mesh) = meshes.get_mut(plane_mesh_handle) {
                    v_pos = Some(_plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec());

                    if let Some(attr_vcolor) = _plane_mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
                        if let VertexAttributeValues::Float32x4(vcolors) = attr_vcolor {
                            v_clr = Some(vcolors.to_vec());
                        }
                    } else {
                        v_clr = Some(vec![[1.0, 1.0, 1.0, 1.0]; v_pos.as_ref().unwrap().len()]);
                    }
                }
            }
            break; // only first needed to get the plane
        }
    }

    if v_pos.is_some() && v_clr.is_some() {
        for (vertex, _parent) in vertex.iter(){
            v_pos.as_mut().unwrap()[vertex.index] = vertex.loc;
            v_clr.as_mut().unwrap()[vertex.index] = vertex.clr;
        }
        if let Ok(plane_mesh_handle) = planes.get(plane_ref) {
            if let Some(plane_mesh) = meshes.get_mut(plane_mesh_handle) {
                plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos.unwrap());
                plane_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_clr.unwrap());
            }
        }
    }    
}


#[derive(Event)]
pub struct PickVertex {
    pub entity: Entity
}
impl  From<ListenerInput<Pointer<Down>>> for PickVertex {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
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
                   mut pick_vertex_event: EventReader<PickVertex>){
    for ev in pick_vertex_event.iter(){
        commands.entity(ev.entity).insert(PickedVertex);
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
                                    On::<Pointer<Down>>::send_event::<PickVertex>(),
                                )).id();

        commands.entity(entity).insert(Visibility::Hidden);
        vertices.push(entity);

    }

    commands.entity(*plane_entity).push_children(&vertices);

}

pub fn deselect_vertex(mut commands:    Commands,
                       picked_vertex:   Query<Entity, With<PickedVertex>>,
                       refs:            Res<VertexRefs>
                    ){
    for v in picked_vertex.iter(){
        commands.entity(v).remove::<PickedVertex>();
        commands.entity(v).insert(refs.mat.clone_weak());
    }
}
