
use bevy::input::common_conditions::{input_just_pressed, input_pressed, input_just_released};
use bevy::prelude::*;
use libm::fabsf;

use crate::mtb_ui::{Picker, SelectOption};
use crate::mtb_grid::{HoverData, Hoverables};
pub struct BoxSelectPlugin;

impl Plugin for BoxSelectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(despawn_box_select
                        .run_if(input_just_pressed(MouseButton::Right)
                        .or_else(input_just_released(MouseButton::Left))))
        .add_system(spawn_box_select
                        .run_if(input_just_pressed(MouseButton::Left))
                        .in_base_set(CoreSet::PreUpdate))
        .add_system(update_box_select
                        .run_if(input_pressed(MouseButton::Left))
                        .in_base_set(CoreSet::PreUpdate))
        ;
    }
}

#[derive(Component)]
pub struct BoxSelect {
  pub start_loc: (f32, f32, f32)
}
impl BoxSelect {
  fn new(xz: &(f32, f32)) -> Self {
    BoxSelect {start_loc: (xz.0, 20.0, xz.1)}
  }
}

pub fn update_box_select(hover_data:        Res<HoverData>,
                         mut box_select:    Query<(&mut Transform, &BoxSelect)>){

    if let Ok((mut t, bs)) = box_select.get_single_mut(){
        let loc = hover_data.hovered_xz;
        let new_x = (loc.0 + bs.start_loc.0)/2.0;
        let new_z = (loc.1 + bs.start_loc.2)/2.0;
        let scale_x = fabsf(bs.start_loc.0- loc.0);
        let scale_z = fabsf(bs.start_loc.2- loc.1);
        t.translation = (new_x, 20.0, new_z).into();
        t.scale = (scale_x, 1.0, scale_z).into();
    }
}

pub fn spawn_box_select(mut commands:      Commands, 
                        mut materials:     ResMut<Assets<StandardMaterial>>,
                        mut meshes:        ResMut<Assets<Mesh>>, 
                        box_select:        Query<&Transform, With<BoxSelect>>,
                        hover_data:        Res<HoverData>,
                        picker:            Res<Picker>){

    if let SelectOption::Box = picker.select {
        match hover_data.hoverable {
            Hoverables::Entity(_) | Hoverables::Grid => {
                let loc = hover_data.hovered_xz;
                if box_select.is_empty(){
                    commands.spawn((PbrBundle {
                        material: materials.add(Color::rgba(0.3, 0.9, 0.3, 0.3).into()),
                        mesh: meshes.add(Mesh::from(shape::Plane {size: 1.0, subdivisions: 0,})),
                        transform: Transform::from_xyz(loc.0, 20.0, loc.1),
                        ..default()
                    }, BoxSelect::new(&loc)));
                }
            }
            _ => {}
        }
    }
}

// Deselects all selectable on mouse right click or button change
pub fn despawn_box_select(mut commands:     Commands,
                          boxselect:        Query<Entity, With<BoxSelect>>){ 
  for ent in boxselect.iter(){
    commands.entity(ent).despawn();
  }
}
