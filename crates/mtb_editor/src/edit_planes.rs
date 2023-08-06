

use bevy::prelude::*;
use belly::prelude::*;
use crate::mtb_grid::EditPlaneEvent;

use mtb_core::planes::PlaneData;

pub struct PlaneEditorPlugin;

impl Plugin for PlaneEditorPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_plugin(BellyPlugin)
      .add_system(make_new_plane)
      ;
  }
}


fn make_new_plane(mut commands:     Commands, 
                  mut meshes:       ResMut<Assets<Mesh>>,
                  mut materials:    ResMut<Assets<StandardMaterial>>,   
                  mut edit_plane:   EventReader<EditPlaneEvent>){

    for _ep in edit_plane.iter(){
        info!(" Edit Plane triggered ");
        let pd = PlaneData::new();
        if let Some(pd_entity) = pd.spawn(&mut commands, &mut meshes, &mut materials){
            commands.add(eml! {
                <body s:padding="50px">
                    <button on:press=run!(for pd_entity |pd: &mut PlaneData| pd.subdivisions.0 += 1 )>"+"</button>
                        <span s:width="150px" s:justify-content="center">
                            <label bind:value=from!(pd_entity, PlaneData:name|fmt.c("Value: {c}"))/>
                        </span>
                    <button on:press=run!(for pd_entity |pd: &mut PlaneData| pd.subdivisions.0 -= 1 )>"-"</button>
                </body>
            });
        }
    }
}