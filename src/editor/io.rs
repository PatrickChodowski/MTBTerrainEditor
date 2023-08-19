use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write};
use std::fs::File;

use crate::core::{planes::PlaneData, vertex::Vertex};

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<WriteData>()
        .add_event::<LoadData>()
        .insert_resource(IOName::new())
        .add_systems(PostUpdate, write_data.run_if(on_event::<WriteData>()))
        .add_systems(PostUpdate, load_data.run_if(on_event::<LoadData>()))
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
pub struct WriteData;

#[derive(Event)]
pub struct LoadData;

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



pub fn write_data(vertex: Query<&Vertex>,
                  planes: Query<(&PlaneData, &Children)>,
                  ioname: Res<IOName>) {

    info!("Eriting data to {}", ioname.data);
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

pub fn load_data(vertex: Query<&Vertex>,
                  planes: Query<(&PlaneData, &Children)>,
                  ioname: Res<IOName>) {

    // info!("Eriting data to {}", ioname.data);
    // let mut v: Vec<SavePlaneData> = Vec::new();
    // for (pd, children) in planes.iter(){
    //     let mut spd = SavePlaneData::from_pd(pd);
    //     for child in children.iter(){
    //         if let Ok(p_vertex) = vertex.get(*child){
    //             spd.vertex.push(*p_vertex);
    //         }
    //     }
    //     v.push(spd);
    // }

    // let f = File::create(format!("./assets/saves/{}.json", ioname.data)).ok().unwrap();
    // let mut writer = BufWriter::new(f);
    // let _res = serde_json::to_writer(&mut writer, &v);
    // let _res = writer.flush();

}