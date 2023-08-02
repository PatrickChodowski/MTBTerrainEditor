use libm::{fabsf, powf, sqrtf};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Axis {
  X,Z
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AreaDims {
    pub x: f32,
    pub z: f32
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AreaData {
    AABB(AreaDims),
    Ellipse(AreaDims)
}

impl AreaData {
  pub fn to_area(&self, loc: &[f32; 2]) -> Area {
    let area: Area;
    match self {
        AreaData::AABB(dims) => {
            area = Area::AABB(AABB{min_x: loc[0]-1.0*dims.x/2.0, max_x: loc[0]+dims.x/2.0, min_z: loc[1]-1.0*dims.z/2.0, max_z: loc[1]+dims.z/2.0});
        }
        AreaData::Ellipse(dims) => {
            area = Area::Ellipse(Ellipse{ a: dims.x, b: dims.z, x: loc[0], z: loc[1] });
        }
    }
    return area;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Area {
    AABB(AABB),
    Ellipse(Ellipse)
}
impl Area {
  pub fn has_point(&self, pos: &[f32; 3]) -> bool {
    let has_point: bool;
    match &self {
      Area::AABB(aabb) => {has_point = aabb.has_point(pos)}
      Area::Ellipse(ellipse) => {has_point = ellipse.has_point(pos)}
    }
    return has_point;
  }

  pub fn get_center(&self) -> (f32, f32){
    match &self {
      Area::AABB(aabb) => {aabb.get_center()}
      Area::Ellipse(ellipse) => {ellipse.get_center()}
    }
  }

  pub fn get_radius(&self) -> f32 {
    match &self {
      Area::AABB(aabb) => {aabb.get_radius()}
      Area::Ellipse(ellipse) => {ellipse.get_radius()}
    }
  }

}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Edge {X, NX, Z, NZ}  

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct AABB {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_z:          f32,
  pub max_z:          f32
}

impl AABB {

  // abomination of mathematics, radius in aabb
  pub fn get_radius(&self) -> f32 {
    let x_r = (self.max_x - self.min_x)/2.0;
    let z_r = (self.max_z - self.min_z)/2.0;

    // get longest of two
    if x_r >= z_r {
      x_r
    } else {
      z_r
    }
  }

  pub fn get_center(&self) -> (f32,f32){
    ((self.min_x + self.max_x)/2.0, (self.min_z + self.max_z)/2.0)
  }

  pub fn _intersect(self, other: &AABB) -> bool {
    self.max_x >= other.min_x && self.min_x <= other.max_x &&
    self.max_z >= other.min_z && self.min_z <= other.max_z
  }

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    p[0] >= self.min_x && p[0] <= self.max_x && p[2] >= self.min_z && p[2] <= self.max_z
  }

  pub fn _has_point_excl(&self, p: &[f32; 3]) -> bool {
    p[0] > self.min_x && p[0] < self.max_x && p[2] > self.min_z && p[2] < self.max_z
  }

  pub fn from_point(xz: &(f32, f32), dims: &(f32, f32)) -> Self {
    AABB{min_x: xz.0 - dims.0/2.0, max_x: xz.0 + dims.0/2.0, min_z: xz.1 - dims.1/2.0, max_z: xz.1 + dims.1/2.0}
  }

}

#[derive(Serialize, Deserialize, Debug, Clone, TypeUuid)]
#[uuid = "201ce530-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct MTBConfigData {
    pub scene_file:         String
}

#[derive(Resource)]
pub struct MTBConfigAsset(pub Handle<MTBConfigData>);


// unpacks mesh stats
pub fn get_mesh_stats(mesh: &Mesh){
  if let Some(pos) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
    let v_pos = pos.as_float3().unwrap().to_vec();
    // pos: 3, color: 4, normals: 3, uvs: 2
    let v_memory = 3.0 + 4.0 + 3.0 + 2.0;

    info!("  Vertex count: {} Memory: {} Kb", 
        v_pos.len(), 
        (v_memory*v_pos.len() as f32)*4.0/1024.0
      );
  }

}


#[derive(Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Ellipse {
  pub a: f32,  // axis
  pub b: f32,  // axis
  pub x: f32,  // loc 
  pub z: f32   // loc
}

impl Ellipse {
  pub fn get_radius(&self) -> f32 {
    let x_r = self.x/2.0;
    let z_r = self.z/2.0;

    // get longest of two
    if x_r >= z_r {
      x_r
    } else {
      z_r
    }
  }

  pub fn get_center(&self) -> (f32,f32){
    (self.x, self.z)
  }

  pub fn has_point(&self, p: &[f32; 3]) -> bool {
    let dx = p[0] - self.x;
    let dz = p[2] - self.z;
    (dx * dx) / (self.a * self.a) + (dz * dz) / (self.b * self.b) <= 1.0
  }

  pub fn has_point_dist(&self, p: &[f32; 3]) -> Option<f32> {
    let dx = p[0] - self.x;
    let dz = p[2] - self.z;

    if (dx * dx) / (self.a * self.a) + (dz * dz) / (self.b * self.b) <= 1.0 {
      Some(get_distance_euclidean(&(self.x, self.z), &(p[0],p[2])))
    } else {
      None
    }
  }
}

pub fn get_distance_manhattan(xz: &(f32, f32), target: &(f32, f32)) -> f32 {
  return fabsf(target.0 - xz.0) + fabsf(target.1 - xz.1);
}

pub fn get_distance_euclidean(xz: &(f32, f32), target: &(f32, f32)) -> f32 {
  return sqrtf(powf(target.0 - xz.0, 2.0) + powf(target.1 - xz.1, 2.0));
}