use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write};
use std::fs::File;

use crate::core::{planes::PlaneData, vertex::Vertex};

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ExportPlanes>()
        .insert_resource(IOName::new())
        .add_systems(PostUpdate, export_planes.run_if(on_event::<ExportPlanes>()))
      ;                      
    }
  }


#[derive(Resource)]
pub struct IOName {
    pub data: String
}
impl IOName {
    pub fn new() -> IOName {
        IOName { data: "".to_string() }
    }
}


#[derive(Event)]
pub struct ExportPlanes;

#[derive(Serialize, Deserialize)]
pub struct SavePlaneData {
    pub plane:        PlaneData,
    pub vertex:       Vec<Vertex>
}
impl SavePlaneData {
    pub fn from_pd(pd: &PlaneData) -> Self {
        SavePlaneData{plane: pd.clone(), vertex: Vec::new()}
    }
}



pub fn export_planes(vertex: Query<&Vertex>,
                     planes: Query<(&PlaneData, &Children)>,
                     ioname: Res<IOName>) {

    info!("Exporting planes to {}", ioname.data);
    let mut v: Vec<SavePlaneData> = Vec::new();
    for (pd, children) in planes.iter(){
        let mut spd = SavePlaneData::from_pd(pd);
        for child in children.iter(){
            if let Ok(p_vertex) = vertex.get(*child){
                spd.vertex.push(*p_vertex);
            }
        }
        v.push(spd);
    }

    let f = File::create(format!("./assets/saves/{}.json", ioname.data)).ok().unwrap();
    let mut writer = BufWriter::new(f);
    let _res = serde_json::to_writer(&mut writer, &v);
    let _res = writer.flush();

}