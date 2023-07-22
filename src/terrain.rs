use bevy::prelude::*;
use bevy::pbr::wireframe::Wireframe;

use crate::tools::mapgrid::{MIN_X, MAX_X, MIN_Z, MAX_Z};

pub struct TerrainPlugin;



impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.7, 0.8, 0.99, 1.0].into()))
        .add_startup_system(setup)
        ;
    }
}



fn setup(mut commands:           Commands,
         mut meshes:             ResMut<Assets<Mesh>>,
         mut materials:          ResMut<Assets<StandardMaterial>>,){

    let mesh = Mesh::from(shape::Plane {size: 1000.0, subdivisions: 0,});
    let clr: Color = [0.3, 0.6, 0.6, 1.0].into();
    commands.spawn(PbrBundle {
            material: materials.add(clr.into()),
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz((MAX_X-MIN_X)/2.0, 0.0, (MAX_Z-MIN_Z)/2.0),
            ..default()
    })
    .insert(Wireframe)
    ;

}