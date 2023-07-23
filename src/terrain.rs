use bevy::prelude::*;
use bevy::pbr::wireframe::Wireframe;
use std::hash::{Hash, Hasher};
#[allow(unused_imports)]
use bevy::utils::{HashMap, HashSet};
#[allow(unused_imports)]
use bevy::render::mesh::Indices;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ordered_float::OrderedFloat;
use grid::*;
use libm::powf;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};

use crate::tools::mapgrid::{MIN_X, MAX_X, MIN_Z, MAX_Z};
use crate::utils::read_txt;

// const WATER_HEIGHT: f32 = 0.0;


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GridColors::new())
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 5.0})
        .insert_resource(ClearColor([0.7, 0.8, 0.99, 1.0].into()))
        .insert_resource(TerrainSettings::new())
        .insert_resource(Terraces::new())
        .add_event::<SetTerrainEvent>()
        .add_startup_system(setup)
        ;
    }
}



fn setup(mut commands:           Commands,
         mut meshes:             ResMut<Assets<Mesh>>,
         mut materials:          ResMut<Assets<StandardMaterial>>,
        terrain_settings:        ResMut<TerrainSettings>,
         terraces:               Res<Terraces>,
         colors:                 Res<GridColors>,
        ){

    if let Some((terrain_mesh, _positions_vec)) = generate_plane(&terrain_settings, &terraces, &colors) { 

        commands
        .spawn(PbrBundle {
            material: materials.add(StandardMaterial{..default()}),
            mesh: meshes.add(terrain_mesh),
            transform: Transform::from_xyz((MIN_X+MAX_X)/2.0, 0.0, (MIN_Z+MAX_Z)/2.0),
            ..default()
        })
        .insert(Wireframe)
        .insert(Name::new("Terrain Plane Mesh"));
    }
}

pub const SUBDIVISIONS: u32 = 500;
pub const WIDTH: f32 = 1000.0;

fn generate_plane(ts: &ResMut<TerrainSettings>, terraces: &Res<Terraces>, _colors: &Res<GridColors>)  -> Option<(Mesh, Vec<[f32; 3]>)> {

    let mut mesh = Mesh::from(shape::Plane {
        size: WIDTH,  
        subdivisions: SUBDIVISIONS
    });

    if let Some(pos) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {

        let mut positions_vec: Vec<[f32; 3]> = pos.as_float3().unwrap().to_vec();
        let mut colors_vec: Vec<[f32; 4]> = Vec::new();
        let noise_fn: NoiseFunc = NoiseFunc::build(ts.noise, ts.seed, ts.octaves, ts.freq);

        for pos in positions_vec.iter_mut() {
            let mut height: f32 = noise_fn.apply(pos[0], pos[2], ts.scale);
            height = apply_easing(height, ts.easing);
            height = height*ts.scale_up;

            let t_hc =  apply_smooth_terrace_color(height, &terraces.data, &_colors);
            pos[1] = t_hc.0;
            // let color = apply_gradient(height, ts.scale_up);
            colors_vec.push(t_hc.1);
            // colors_vec.push(color);
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions_vec.clone());
        // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors_vec);




        info!(" [START] Extracting mesh - subdivisions: {}", SUBDIVISIONS);
        let mut mesh_data = MeshData::extract(&mesh, SUBDIVISIONS, WIDTH);
        info!(" [DONE] Extracting mesh - subdivisions: {}", SUBDIVISIONS);
        // mesh_data.color_quad(0, &[1.0, 0.0, 0.0, 1.0], &mut colors_vec);

        info!(" [START] search merges" );
        let vec_vec_quads: Vec<Vec<QuadData>> = mesh_data.search_merges().clone();

        info!(" [DONE] search merges" );


        for vec_quads in vec_vec_quads.iter(){
            println!("Quads to add: {}/{}", vec_quads.len(), mesh_data.quads_count);
            let mut corners = mesh_data.merge_quads(vec_quads);
            mesh_data.reduce(&mut corners);
        }

        println!(" [Mesh Data] pos: {:?}",       mesh_data.pos);
        println!(" [Mesh Data] norms: {:?}",     mesh_data.norms);
        println!(" [Mesh Data] uvs: {:?}",       mesh_data.uvs);
        println!(" [Mesh Data] indices: {:?}",   mesh_data.indices);


        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.pos);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL,   mesh_data.norms);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0,     mesh_data.uvs);
        mesh.set_indices(Some(Indices::U32(mesh_data.indices)));

        return Some((mesh, positions_vec));
    }
    return None;
}

#[derive(Debug, Default, Clone)]
pub struct QuadData {
    pub id:             u32,
    pub row:            u32, // position within the plane
    pub col:            u32, // position within the plane
    pub pos:            Vec<[f32; 3]>,
    pub norms:          Vec<[f32; 3]>,
    pub uvs:            Vec<[f32; 2]>,
    pub colors:         Vec<[f32; 4]>,
    pub indices:        Vec<u32>,
    pub unique_indices: Vec<u32>,
    pub heights:        HashSet<OrderedFloat<f32>>      // any reddit post tells me not to compare floats but the vertex position is float and need to compare vertex positions...
}

impl PartialEq for QuadData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for QuadData {}

impl Hash for QuadData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl QuadData {
    pub fn new() -> Self {
        QuadData{id: 0, 
                 row: 0, 
                 col: 0,
                 pos: Vec::new(), 
                 norms: Vec::new(), 
                 uvs: Vec::new(), 
                 colors: Vec::new(), 
                 indices: Vec::new(), 
                 unique_indices: Vec::new(), 
                 heights: HashSet::new()
        }
    }

    pub fn can_merge(&self, other: &QuadData) -> bool {
        let is_same: bool = self.heights == other.heights;
        // println!("  Can merge {} with {}? Answer: {} ({:?},{:?})", self.id, other.id, is_same, self.heights, other.heights);
        return is_same;
    }
}


#[derive(Debug)]
pub struct MeshData {
    pub pos:            Vec<[f32; 3]>,
    pub norms:          Vec<[f32; 3]>,
    pub uvs:            Vec<[f32; 2]>,
    pub indices:        Vec<u32>,
    pub index6:         Vec<u32>,

    pub size:           f32,
    pub subdivisions:   u32,
    pub vertex_count:   u32,
    pub quads_count:    u32,
    pub dimn:           u32, //count of quads in row/col

    pub quads:          Grid<QuadData>
}

impl MeshData {
    pub fn new(subdivisions: u32, size: f32) -> Self {
        MeshData{pos:           Vec::new(), 
                 norms:         Vec::new(), 
                 indices:       Vec::new(), 
                 uvs:           Vec::new(),
                 index6:        (0..6).collect(), 
                 subdivisions, 
                 size,  
                 vertex_count:  (subdivisions+2)*(subdivisions+2), 
                 quads_count:   (subdivisions+1)*(subdivisions+1),
                 dimn:          subdivisions +1, 
                 quads:         Grid::new((subdivisions +1) as usize, (subdivisions +1) as usize)
        }
    }

    pub fn get_vertex_index(&self, pos: &[f32; 3]) -> u32 {
        return self.pos.iter().position(|x| x == pos).unwrap() as u32;
    } 

    pub fn get_quad_by_id(&self, quad_id: &u32) -> Option<&QuadData> {
        for qd in self.quads.iter(){
            if &qd.id == quad_id {
                return Some(&qd);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn get_quad_by_pos(&self, row: u32, col: u32) -> &QuadData {
        return &self.quads[row as usize][col as usize];
    }

    #[allow(dead_code)]
    // Takes color vector and changes quad by id
    pub fn color_quad(&self, quad_id: &u32, color: &[f32; 4], colors_vec: &mut Vec<[f32; 4]>) {
        if let Some(qd) = self.get_quad_by_id(quad_id){
            for idx in qd.indices.iter(){
                colors_vec[*idx as usize] = *color;
            }
        }
    } 

    pub fn reduce(&mut self, corners: &mut Vec<u32>){

        // println!("reduce vdelete: {:?}", vdelete);
        corners.sort_by(|a,b| a.partial_cmp(&b).unwrap());
        let mut corner_map: HashMap<u32, u32> = HashMap::new();
        for (c, corner) in corners.iter().enumerate(){corner_map.insert(*corner, c as u32);};

        let mut set_index: Vec<u32> = vec![3,1,2,0,2,1];
        set_index.reverse();

        let min_idx = corners.iter().min().unwrap();
        let max_idx = corners.iter().max().unwrap();
        // println!("  reduce corners: {:?}", corners);
        // println!("  corners mapping: {:?}", corner_map);
        // println!("min: {:?} max: {:?}", min_idx, max_idx);

        self.pos = corners.iter().map(|&index| self.pos[index as usize]).collect();
        self.norms = corners.iter().map(|&index| self.norms[index as usize]).collect();

        // println!("old indices: {:?}", self.indices);
        let mut new_indices: Vec<u32> = Vec::with_capacity(self.indices.len());

        for idx in self.indices.iter(){
            if idx > max_idx {
                new_indices.push((idx - max_idx) as u32);
            } else if corners.contains(idx){
                new_indices.push(set_index.pop().unwrap())
            } else if idx < min_idx {
                new_indices.push(*idx);
            }
        }
        // println!("new indices: {:?}", new_indices);
 
        self.indices = new_indices;
        self.calculate_uvs();

    }

    // Method merging 2 quads
    pub fn merge_quads<'a>(&self, quads: &Vec<QuadData>) -> Vec<u32> {

        // find corner vertices
        let absurd_value: f32 = 99999.0;

        let mut left_top: (u32, [f32; 3]) = (0, [absurd_value, 0.0, absurd_value]);
        let mut right_top: (u32, [f32; 3]) = (0, [-absurd_value, 0.0, absurd_value]);
        let mut left_bottom: (u32, [f32; 3]) = (0, [absurd_value, 0.0, -absurd_value]);
        let mut right_bottom: (u32, [f32; 3]) = (0, [-absurd_value, 0.0, -absurd_value]);

        for quad in quads.iter(){
            for pos in quad.pos.iter(){
                if pos[0] <= left_top.1[0] && pos[2] <= left_top.1[2] {
                    left_top = (self.get_vertex_index(pos), *pos);
                }
                if pos[0] >= right_top.1[0] && pos[2] <= right_top.1[2] {
                    right_top = (self.get_vertex_index(pos), *pos);
                }
                if pos[0] <= left_bottom.1[0] && pos[2] >= left_bottom.1[2] {
                    left_bottom = (self.get_vertex_index(pos), *pos);
                }
                if pos[0] >= right_bottom.1[0] && pos[2] >= right_bottom.1[2] {
                    right_bottom = (self.get_vertex_index(pos), *pos);
                }
            }
        }

        println!(" [CORNER] left top: {:?}", left_top);
        println!(" [CORNER] right top: {:?}", right_top);
        println!(" [CORNER] left bottom: {:?}", left_bottom);
        println!(" [CORNER] right bottom: {:?}", right_bottom);
        
        let corners: Vec<u32> = vec![left_top.0, right_top.0, left_bottom.0, right_bottom.0];
        return corners;
    }


    pub fn search_merges(&self) -> Vec<Vec<QuadData>>  {

        let mut quads_to_merge: HashMap<&QuadData, Vec<&QuadData>> = HashMap::new();
        let start_adjs: Vec<(u32, u32)> = vec![(1,1), (0,1), (1,0)];

        #[allow(unused_assignments)]
        let mut expand_adjs: Vec<(u32, u32)> = Vec::with_capacity(1000);
        let mut adjs_checks: Vec<bool> = Vec::new();
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        while x < self.subdivisions && y < self.subdivisions {

            let qd = &self.quads[x as usize][y as usize];
            let mut adj_qds: Vec<&QuadData> = Vec::new();
            expand_adjs = start_adjs.clone();
            let mut level: u32 = 1;

            while expand_adjs.len() > 0 {

                let adj = expand_adjs.pop().unwrap();
                let adj_x = x + adj.0;
                let adj_y = y + adj.1;

                if !(adj_x < self.dimn && adj_y < self.dimn) {
                    continue; // Edge quad
                }

                let adj_qd = &self.quads[adj_x as usize][adj_y as usize];
                if qd.can_merge(adj_qd){
                    adjs_checks.push(true);
                    adj_qds.push(adj_qd);
                }

                if expand_adjs.len() == 0 {

                    // if all match
                    if adjs_checks.iter().all(|a| *a){
                        quads_to_merge.entry(qd).or_insert(Vec::new()).append(&mut adj_qds);
                        level += 1;
                        for a in 0..level {
                            expand_adjs.push((a, level));
                            expand_adjs.push((level, a));
                        }
                        expand_adjs.push((level,level));
                    }
        
                    adjs_checks.clear();
                    adj_qds.clear();
                }
            }
            x += level;
            y += level;
        }

        // compile result. its because I need a full copy of those quads to merge
        let mut result: Vec<Vec<QuadData>> = Vec::new(); 
        for (k,v) in quads_to_merge.iter(){
            let mut v2: Vec<QuadData> = v.into_iter().cloned().cloned().collect(); // XD XD
            v2.push(k.clone().clone());
            result.push(v2);
        }
        return result;
    }

    pub fn calculate_uvs(&mut self){
        for pos in self.pos.iter(){
            self.uvs.push([(pos[0] + self.size/2.0)/self.size, (pos[2] + self.size/2.0)/self.size]);
        }
    }
            

    pub fn extract(mesh: &Mesh, subdivisions: u32, size: f32) -> Self {
        let mut md = MeshData::new(subdivisions, size);

        if let Some(pos) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            md.pos = pos.as_float3().unwrap().to_vec();
        }
        if let Some(norm) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            md.norms  = norm.as_float3().unwrap().to_vec();
        }
        if let Some(indices) = mesh.indices() {
            md.indices = indices.iter().map(|x| x as u32).collect();
        }
        info!(" [EXTRACT MESH DATA] subs: {}, quads: {} vertex: {} dimns: {}", md.subdivisions, md.quads_count, md.vertex_count, md.dimn);

        // info!("{:?}", md);

        md.generate_quads();
    
        return md;
    }

    pub fn generate_quad(&self, id: u32) -> QuadData {
        let mut qd = QuadData::new();
        qd.id = id;
        qd.row = id/self.dimn;
        qd.col = id - qd.row*self.dimn;
        qd.indices = self.index6.iter().map(|&index| self.indices[index as usize +((id*6) as usize)] as u32).collect();
        // rust sometimes is just absurd syntax
        qd.unique_indices = HashSet::from_iter(qd.indices.iter().cloned()).iter().cloned().collect();
        qd.pos = qd.unique_indices.iter().map(|&index| self.pos[index as usize]).collect();
        qd.norms = qd.unique_indices.iter().map(|&index| self.norms[index as usize]).collect();
        qd.heights = HashSet::from_iter(qd.pos.iter().map(|pos| OrderedFloat(pos[1])));
        // println!("Quad Data: {:?}", qd);
        return qd;
    }

    pub fn generate_quads(&mut self) {
        for quad_id in 0..self.quads_count {
            let qd = self.generate_quad(quad_id);
            self.quads[qd.row as usize][qd.col as usize] = qd.clone();
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TerrainPreset {
    pub label: String,
    pub noise: Noises,
    pub easing: Easings,
    pub scale: f32,
    pub seed: u32,
    pub scale_up: f32,
    pub water_height_plane: f32,
    pub octaves: Option<usize>,
    pub freq: Option<f64>,
    pub terraces: Vec<Terrace>,
    pub models: Vec<TerrainModel>,
    pub entry: Option<(u32, u32)>,
    pub exit: Option<(u32, u32)>,
}

#[derive(Resource)]
pub struct TerrainPresets {
    pub data: HashMap<String, TerrainPreset>
}

impl<'a> TerrainPresets {
    pub fn _new() -> TerrainPresets {
        return TerrainPresets{data: HashMap::new()}
    }
}



pub fn apply_smooth_terrace_color(height: f32, terraces: &Vec<Terrace>, colors: &Res<GridColors>) -> (f32, [f32; 4]) {
    for t in terraces.iter(){
        if height >= t.min && height < t.max {
            if height >= t.padding*t.max {
                return (height, colors.get_color_array_linear_rgba(&t.padd_color));
            }
            return (t.value, colors.get_color_array_linear_rgba(&t.base_color)); 
        }
    }
    return (height, [0.0, 0.0, 0.0, 1.0]);
}


pub fn smooth_step(x: f32, min_x: f32, max_x:f32) -> f32{
    let xc = x.clamp(min_x, max_x);
    return xc * xc * (3.0 - 2.0 * xc);
}

// Applies easing function to the noise output
pub fn apply_easing(height: f32, easings: Easings) -> f32 {
    match easings {
        Easings::SmoothStart => {
            return height*height;
        }
        Easings::SmoothStop => {
            return 1.0 - ((1.0 - height)*(1.0-height));
        }
        Easings::SmoothEnd => {
            return 1.0 - (1.0 - height).powi(2);
        }
        Easings::SmoothStep => {
            return smooth_step(height, 0.0, 1.0);
        }
        Easings::AbsoluteValue => {
            return height.abs();
        }
        Easings::AbsoluteValue2 => {
            return powf(height.abs(), 2.0);
        }
        Easings::AbsoluteValue3 => {
            return powf(height.abs(), 3.0);
        }
        Easings::AbsoluteValue6 => {
            return powf(height.abs(), 6.0);
        }
        Easings::AbsoluteValue10 => {
            return powf(height.abs(), 10.0);
        }
        Easings::None => {
            return height;
        }
    }
}



#[derive(Resource, Clone)]
pub struct GridColors {
    pub data: HashMap<String, Color>,
}

#[derive(Serialize, Deserialize)]
pub struct ColorData<'a> {
    label: &'a str,
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl GridColors {
    #[allow(dead_code)]
    pub fn get_color_array_linear_rgba(&self, color_name: &str) -> [f32; 4] {
        if let Some(clr) = self.data.get(color_name) {
            return clr.as_linear_rgba_f32();
        } else {
            return [0.0, 0.0, 0.0, 1.0];
        }

    }
    #[allow(dead_code)]
    pub fn get_color_array(&self, color_name: &str) -> [f32; 4] {
        // returns black if its empty because options are annoying here
        if let Some(clr) = self.data.get(color_name) {
            return [
                clr.as_rgba().r(),
                clr.as_rgba().g(),
                clr.as_rgba().b(),
                clr.as_rgba().a()
            ];
        } else {
            return [0.0, 0.0, 0.0, 1.0];
        }
    }
    #[allow(dead_code)]
    pub fn get_color(&self, color_name: &str) -> Color {
        // returns black if its empty because options are annoying here
        if let Some(clr) = self.data.get(color_name) {
            return *clr;
        } else {
            return Color::BLACK;
        }
    }

    fn new() -> GridColors {
        //https://colorcodefinder.com/ -> works 
        //https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr.wgsl
        let mut gc = GridColors{data: HashMap::new()};
        let a: f32 = 255.0;
        let path: &str = "./assets/data/colors.json";
        let data: String = read_txt(path);
        let colors_data: Vec<ColorData> = serde_json::from_str(&data).
                            expect(&format!("\n [ERROR models.setup] Unable to get data from {path} \n"));

        for c in colors_data {
            gc.data.insert(c.label.to_string(), 
                        //    Color::rgba(c.r/a, c.g/a, c.b/a, c.a/a),
                        Color::rgba(c.r/a, c.g/a, c.b/a, 1.0),
            );
        }
        return gc;
    }
}

#[derive(Resource)]
pub struct Terraces {
    pub data: Vec<Terrace>,
    pub heights: Vec<f32>
}
impl Terraces  {
    pub fn new() -> Terraces {
        Terraces {
            data:vec![Terrace{min:-100.0, max:10.0, value: 0.0, padding:0.6, base_color: "blue3".to_string(), padd_color: "yellow1".to_string()},
                      Terrace{min:10.0, max:1000.0, value: 10.0, padding:0.0, base_color: "green1".to_string(), padd_color: "brown1".to_string()},
                    //   Terrace{min:10.0, max:30.0, value: 10.0, padding:0.0, base_color: "dark_green", padd_color: "dark_brown"},
                    //   Terrace{min:30.0, max:1000.0, value: 30.0, padding:0.0, base_color: "light_brown", padd_color: "dark_brown"}
                      ],
            heights: vec![0.0, 10.0]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Terrace {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub padding: f32,
    pub base_color: String,
    pub padd_color: String
}


#[derive(Serialize, Deserialize, Clone)]
pub struct TerrainModel {
    pub x: u32,
    pub z: u32,
    pub w: u32,
    pub h: u32,
    pub angle: f32,
    pub model_name: String,
}


pub struct SetTerrainEvent {
    pub noise:              Option<Noises>,
    pub scale:              Option<f64>,
    pub seed:               Option<u32>,
    pub easing:             Option<Easings>,
    pub scale_up:           Option<f32>,
    pub octaves:            Option<usize>,
    pub freq:               Option<f64>,
    pub water_height_plane: Option<f32>,
    pub models:             Option<Vec<TerrainModel>>,
    pub preset_name:        Option<String>,
    pub gen_trees:          Option<bool>
}
impl Default for SetTerrainEvent {
    fn default() -> SetTerrainEvent {
        SetTerrainEvent{
            noise:              None,
            scale:              None,
            seed:               None,
            easing:             None,
            scale_up:           None,
            octaves:            None,
            freq:               None,
            water_height_plane: None,
            models:             None,
            preset_name:        None,
            gen_trees:          None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Easings {
    SmoothStep,
    SmoothStop,
    SmoothStart,
    
    SmoothEnd,
    AbsoluteValue,
    AbsoluteValue2,
    AbsoluteValue3,
    AbsoluteValue6,
    AbsoluteValue10,
    None,
}

impl FromStr for Easings {
    type Err = ();
    fn from_str(input: &str) -> Result<Easings, Self::Err> {
        match input {
            "SmoothStep" => Ok(Easings::SmoothStep),
            "SmoothStop" => Ok(Easings::SmoothStop),
            "SmoothEnd" => Ok(Easings::SmoothEnd),
            "SmoothStart" => Ok(Easings::SmoothStart),
            "Abs" => Ok(Easings::AbsoluteValue),
            "Abs2" => Ok(Easings::AbsoluteValue2),
            "Abs3" => Ok(Easings::AbsoluteValue3),
            "Abs6" => Ok(Easings::AbsoluteValue6),
            "Abs10" => Ok(Easings::AbsoluteValue10),
            "None" => Ok(Easings::None),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Noises {
    None,
    Perlin,
    PerlinSurflet,
    OpenSimplex,
    Value,
    SuperSimplex,
    Worley,
    Simplex,
    FBMPerlin, // Fractal
    BMPerlin, //Basic Multi
    BPerlin,
    RMPerlin, // RidgedMultiPerlin
    HMPerlin, //Hybrid Multi perlin
    FBMPerlinSurflet,
    BMPerlinSurflet,
    BPerlinSurflet,
    RMPerlinSurflet,
    HMPerlinSurflet,
    FBMValue,
    BMValue,
    BValue,
    RMValue,
    HMValue,
    FBMSS,
    BMSS,
    BSS,
    RMSS,
    HMSS 
}

impl FromStr for Noises {
    type Err = ();
    fn from_str(input: &str) -> Result<Noises, Self::Err> {
        match input {
            "None" => Ok(Noises::None),
            "Perlin" => Ok(Noises::Perlin),
            "PerlinSurflet" => Ok(Noises::PerlinSurflet),
            "OpenSimplex" => Ok(Noises::OpenSimplex),
            "Value" => Ok(Noises::Value),
            "SuperSimplex" => Ok(Noises::SuperSimplex),
            "Worley" => Ok(Noises::Worley),
            "Simplex" => Ok(Noises::Simplex),
            "FBMPerlin" => Ok(Noises::FBMPerlin),
            "BMPerlin" => Ok(Noises::BMPerlin),
            "BPerlin" => Ok(Noises::BPerlin),
            "RMPerlin" => Ok(Noises::RMPerlin),
            "HMPerlin" => Ok(Noises::HMPerlin),
            "FBMPerlinSurflet" => Ok(Noises::FBMPerlinSurflet),
            "BMPerlinSurflet" => Ok(Noises::BMPerlinSurflet),
            "BPerlinSurflet" => Ok(Noises::BPerlinSurflet),
            "RMPerlinSurflet" => Ok(Noises::RMPerlinSurflet),
            "HMPerlinSurflet" => Ok(Noises::HMPerlinSurflet),
            "FBMValue" => Ok(Noises::FBMValue),
            "BMValue" => Ok(Noises::BMValue),
            "BValue" => Ok(Noises::BValue),
            "RMValue" => Ok(Noises::RMValue),
            "HMValue" => Ok(Noises::HMValue),
            "FBMSS" => Ok(Noises::FBMSS),
            "BMSS" => Ok(Noises::BMSS),
            "BSS" => Ok(Noises::BSS),
            "RMSS" => Ok(Noises::RMSS),
            "HMSS" => Ok(Noises::HMSS),
            _ => Err(()),
        }
    }
}



#[derive(Resource)]
pub struct TerrainSettings {
    pub noise: Noises,
    pub scale: f64,
    pub seed: u32,
    pub easing: Easings,
    pub scale_up: f32,
    pub water_height_plane: f32,
    pub octaves: Option<usize>,
    pub freq: Option<f64>,
    pub preset_name: Option<String>
}
impl TerrainSettings {
    pub fn new() -> TerrainSettings {
        TerrainSettings{
            noise: Noises::Perlin, 
            scale: 0.001, 
            seed:  1, 
            water_height_plane: 1.0,
            easing: Easings::None,
            scale_up: 100.0,
            octaves: None,
            freq: None,
            preset_name: None
        }
    }
    pub fn _update(&mut self, t: &SetTerrainEvent){
        if let Some(noise) = t.noise{
            self.noise = noise;
        }
        if let Some(scale) = t.scale{
            self.scale = scale;
        }
        if let Some(seed) = t.seed{
            self.seed = seed;
        }
        if let Some(easing) = t.easing{
            self.easing = easing;
        }
        if let Some(scale_up) = t.scale_up{
            self.scale_up = scale_up;
        }
        if let Some(whp) = t.water_height_plane{
            self.water_height_plane = whp;
        }
        self.octaves = t.octaves;
        self.freq = t.freq;
        self.preset_name = t.preset_name.clone();
    }
}


#[derive(Clone)]
pub enum NoiseFunc {
    None,
    Perlin(Perlin),
    PerlinSurflet(PerlinSurflet),
    OpenSimplex(OpenSimplex),
    SuperSimplex(SuperSimplex),
    Simplex(Simplex),
    Value(Value),
    Worley(Worley),
    BMPerlin(BasicMulti<Perlin>), // very nice!
    FBMPerlin(Fbm<Perlin>),
    BPerlin(Billow<Perlin>),
    RMPerlin(RidgedMulti<Perlin>),
    HMPerlin(HybridMulti<Perlin>),
    BMPerlinSurflet(BasicMulti<PerlinSurflet>),
    FBMPerlinSurflet(Fbm<PerlinSurflet>),
    BPerlinSurflet(Billow<PerlinSurflet>),
    RMPerlinSurflet(RidgedMulti<PerlinSurflet>),
    HMPerlinSurflet(HybridMulti<PerlinSurflet>),
    BMValue(BasicMulti<Value>),
    FBMValue(Fbm<Value>),
    BValue(Billow<Value>),
    RMValue(RidgedMulti<Value>),
    HMValue(HybridMulti<Value>),
    BMSS(BasicMulti<SuperSimplex>),
    FBMSS(Fbm<SuperSimplex>),
    BSS(Billow<SuperSimplex>),
    RMSS(RidgedMulti<SuperSimplex>),
    HMSS(HybridMulti<SuperSimplex>)
}

// noise_fn.octaves; // number of detail
// noise_fn.frequency; // The number of cycles per unit length that the noise function outputs.
// noise_fn.lacunarity; // A lacunarity of 2.0 results in the frequency doubling every octave. For almost all cases, 2.0 is a good value to use.
// noise_fn.persistence; // Increasing the persistence produces “rougher” noise.

impl NoiseFunc {
    fn build(noise: Noises, seed: u32, octaves: Option<usize>, freq: Option<f64>) -> NoiseFunc {
        match noise {
            Noises::None =>          {return NoiseFunc::None}
            Noises::Perlin =>        {return NoiseFunc::Perlin(Perlin::new(seed))}
            Noises::PerlinSurflet => {return NoiseFunc::PerlinSurflet(PerlinSurflet::new(seed))}
            Noises::Value =>         {return NoiseFunc::Value(Value::new(seed))}
            Noises::OpenSimplex =>   {return NoiseFunc::OpenSimplex(OpenSimplex::new(seed))}
            Noises::SuperSimplex =>  {return NoiseFunc::SuperSimplex(SuperSimplex::new(seed))}
            Noises::Worley =>        {return NoiseFunc::Worley(Worley::new(seed))}
            Noises::Simplex =>       {return NoiseFunc::Simplex(Simplex::new(seed))}
            Noises::FBMPerlin =>     {
                let mut noise_fn: Fbm<Perlin> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMPerlin(noise_fn)
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMPerlin(noise_fn)
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BPerlin(noise_fn)
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMPerlin(noise_fn)
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMPerlin(noise_fn)
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMPerlinSurflet(noise_fn)
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMPerlinSurflet(noise_fn)
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BPerlinSurflet(noise_fn)
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMPerlinSurflet(noise_fn)
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMPerlinSurflet(noise_fn)
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMValue(noise_fn)
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMValue(noise_fn)
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BValue(noise_fn)
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMValue(noise_fn)
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMValue(noise_fn)
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMSS(noise_fn)
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMSS(noise_fn)
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BSS(noise_fn)
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMSS(noise_fn)
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMSS(noise_fn)
            }
        }
    }

    // not sure if there is any better way to do it :/ wish to just have one arm as the code is the same
    fn apply(&self, x: f32, y: f32, scale: f64) -> f32 {
        match self {
            NoiseFunc::None                          => {1.0*scale as f32}
            NoiseFunc::Perlin(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::PerlinSurflet(f)              => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Value(f)                      => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::OpenSimplex(f)                => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::SuperSimplex(f)               => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Worley(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Simplex(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMPerlin(f)                  => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BPerlin(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMPerlinSurflet(f)           => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BPerlinSurflet(f)             => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMValue(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BValue(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMSS(f)                      => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BSS(f)                        => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
        }
    }
}
