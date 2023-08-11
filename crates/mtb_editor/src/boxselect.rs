
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use libm::fabsf;

use crate::mtb_ui::Picker;
use crate::mtb_grid::HoverData;
pub struct BoxSelectPlugin;

impl Plugin for BoxSelectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(deselect.run_if(input_just_pressed(MouseButton::Right)))
        .add_system(update_box_select.run_if(input_pressed(MouseButton::Left)).in_base_set(CoreSet::PreUpdate))
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

// click and drag selection
pub fn box_select(commands:       &mut Commands, 
              boxselect:          &mut Query<(&mut Transform, &mut BoxSelect)>,
              meshes:             &mut ResMut<Assets<Mesh>>,
              materials:          &mut ResMut<Assets<StandardMaterial>>,
              hoverdata:          &Res<HoverData>
            ){

  let loc = hoverdata.hovered_xz;

  if boxselect.is_empty(){

    commands.spawn((PbrBundle {
        material: materials.add(Color::rgba(0.3, 0.9, 0.3, 0.3).into()),
        mesh: meshes.add(Mesh::from(shape::Plane {size: 1.0, subdivisions: 0,})),
        transform: Transform::from_xyz(loc.0, 20.0, loc.1),
        ..default()
    }, BoxSelect::new(&loc)));

  } else {

    let (mut t, bs) = boxselect.get_single_mut().unwrap();
    let new_x = (loc.0 + bs.start_loc.0)/2.0;
    let new_z = (loc.1 + bs.start_loc.2)/2.0;
    let scale_x = fabsf(bs.start_loc.0- loc.0);
    let scale_z = fabsf(bs.start_loc.2- loc.1);
    t.translation = (new_x, 20.0, new_z).into();
    t.scale = (scale_x, 1.0, scale_z).into();

  }

}


// Updates and fills out box select with ghost entities during drag
fn update_box_select(picker:            Res<Picker>,
                     selectbox:         Query<&Transform, With<BoxSelect>>,
                     hover_data:        Res<HoverData>,){

  if let Ok(t) = selectbox.get_single(){




  }

}

// Deselects all selectable on mouse right click or button change
pub fn deselect(mut commands:     Commands,
                mut picker:       ResMut<Picker>,
                boxselect:       Query<Entity, With<BoxSelect>>
              ){ 
  picker.reset();
  
  for ent in boxselect.iter(){
    commands.entity(ent).despawn();
  }
  
}







