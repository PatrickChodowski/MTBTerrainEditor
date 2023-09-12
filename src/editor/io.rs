use bevy::input::common_conditions::{input_pressed, input_just_pressed};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::render::mesh::VertexAttributeValues;
use bevy_mod_picking::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write};
use std::fs::{self, File};

use super::GlobalSettings;
use crate::core::planes::{PlaneData, TerrainPlane, PickPlane, PlaneEdit, plane_mesh};
use crate::core::vertex::Vertex;
use super::colors::Colors;
use super::mtb_ui::ModResources;

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<WriteData>()
        .add_event::<LoadData>()
        .insert_resource(IOName::new())
        .add_systems(PreUpdate, input_write_data.run_if(input_pressed(KeyCode::ControlLeft)
                                                .and_then(input_just_pressed(KeyCode::S))))
        .add_systems(PostUpdate, write_data.run_if(on_event::<WriteData>()))
        .add_systems(PostUpdate, load_data.run_if(on_event::<LoadData>()))
      ;                      
    }
  }

fn input_write_data(mut write_data: EventWriter<WriteData>){
    write_data.send(WriteData);
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

    pub fn spawn(&self,
                 commands:           &mut Commands, 
                 meshes:             &mut ResMut<Assets<Mesh>>,
                 materials:          &mut ResMut<Assets<StandardMaterial>>) -> Entity {

        
        let mut mesh = plane_mesh(&self.plane.subdivisions, &self.plane.dims);

        // Adjust vertices:
        let mut v_clr: Option<Vec<[f32;4]>> = None;
        let mut v_pos: Option<Vec<[f32; 3]>>;

        v_pos = Some(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec());
        if let Some(attr_vcolor) = mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
            if let VertexAttributeValues::Float32x4(vcolors) = attr_vcolor {
                v_clr = Some(vcolors.to_vec());
            }
        } else {
            v_clr = Some(vec![[1.0, 1.0, 1.0, 1.0]; v_pos.as_ref().unwrap().len()]);
        }

        if v_pos.is_some() && v_clr.is_some() {
            for vertex in self.vertex.iter(){
                v_pos.as_mut().unwrap()[vertex.index] = vertex.loc;
                v_clr.as_mut().unwrap()[vertex.index] = vertex.clr;
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos.unwrap());
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_clr.unwrap());
        } 

        let entity = commands.spawn((PbrBundle {
            material: materials.add(StandardMaterial{..default()}),
            mesh: meshes.add(mesh),
            transform: Transform::from_translation(self.plane.loc.into()),
            ..default()
            },
            TerrainPlane,
            PlaneEdit(false),
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Down>>::send_event::<PickPlane>(),
            self.plane.clone(),
            self.plane.get_aabb()
        )).id();

        return entity;

    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveProjectData {
    // planes:     Vec<SavePlaneData>,
    colors:     HashSet<[u8; 4]>,
    mod_res:    ModResources,
    settings:   GlobalSettings
}

pub fn write_data(vertex:   Query<&Vertex>,
                  planes:   Query<(&PlaneData, &Children)>,
                  colors:   Res<Colors>,
                  mod_res:  Res<ModResources>,
                  settings: Res<GlobalSettings>,
                  ioname:   Res<IOName>) {

    info!("Writing data to {}", ioname.data);

    let mut v_planes: Vec<SavePlaneData> = Vec::new();
    for (pd, children) in planes.iter(){
        let mut spd = SavePlaneData::from_pd(pd);
        for child in children.iter(){
            if let Ok(p_vertex) = vertex.get(*child){
                spd.vertex.push(*p_vertex);
            }
        }
        v_planes.push(spd);
    }
    let f = File::create(format!("./assets/saves/{}.json", ioname.data)).ok().unwrap();
    let mut writer = BufWriter::new(f);
    let _res = serde_json::to_writer(&mut writer, &v_planes);
    let _res = writer.flush();


    let project_data = SaveProjectData{colors: colors.selects.clone(), 
        mod_res: mod_res.to_owned(), 
        settings: settings.to_owned()};

    let f_proj = File::create(format!("./assets/saves/{}.proj.json", ioname.data)).ok().unwrap();
    let mut writer_proj = BufWriter::new(f_proj);
    let _res_proj = serde_json::to_writer(&mut writer_proj, &project_data);
    let _res_proj = writer_proj.flush();

}

pub fn load_data(mut commands:      Commands,
                 mut meshes:        ResMut<Assets<Mesh>>,
                 mut materials:     ResMut<Assets<StandardMaterial>>,
                 mut colors:        ResMut<Colors>,
                 mut mod_res:       ResMut<ModResources>,
                 mut settings:      ResMut<GlobalSettings>,
                 planes:            Query<Entity, With<PlaneData>>,
                 ioname:            Res<IOName>) {

    info!("Loading data from {}", ioname.data);

    let path: &str = &format!("./assets/saves/{}.json", ioname.data);
    if let Ok(data) = fs::read_to_string(path){
        if let Ok(vspds) = serde_json::from_str::<Vec<SavePlaneData>>(&data) {
            for entity in planes.iter(){
                commands.entity(entity).despawn_recursive();
            }
            for spd in vspds.iter(){
                spd.spawn(&mut commands, &mut meshes, &mut materials);
            }
            info!("Success! Loaded data from {}", ioname.data);
        } else {
            info!("Failed to parse Vec<SavePlaneData> from {}", ioname.data);
        }
    } else {
        info!("Failed to read data from save file: {}", ioname.data);
    }                    

    let path_proj: &str = &format!("./assets/saves/{}.proj.json", ioname.data);
    if let Ok(data) = fs::read_to_string(path_proj){
        if let Ok(projdata) = serde_json::from_str::<SaveProjectData>(&data) {
            colors.selects = projdata.colors.clone();
            *mod_res = projdata.mod_res;
            *settings = projdata.settings;
            info!("Success! Loaded project data from {}", ioname.data);
        } else {
            info!("Failed to parse SaveProjectData from {}", ioname.data);
        }
    } else {
        info!("Failed to read data from save file: {}", ioname.data);
    }                    
}
