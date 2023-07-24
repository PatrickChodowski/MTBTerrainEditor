use bevy::prelude::*;
use std::fs::{self};
use libm::atan2f; 

pub fn get_yaw(q: Quat) -> f32 {
  //float Yaw = Mathf.Rad2Deg * Mathf.Atan2(2 * q.y * q.w - 2 * q.x * q.z, 1 - 2 * q.y * q.y - 2 * q.z * q.z);
  return atan2f(2.0*q.y*q.w - 2.0*q.x *q.z, 1.0 - 2.0*q.y*q.y - 2.0*q.z*q.z);
}

// Get Pitch from quaternion rotation
pub fn get_pitch(q: Quat) -> f32 {
  // float Pitch = Mathf.Rad2Deg * Mathf.Atan2(2 * q.x * q.w - 2 * q.y * q.z, 1 - 2 * q.x * q.x - 2 * q.z * q.z);
  return atan2f(2.0*q.x*q.w - 2.0*q.y*q.z, 1.0 - 2.0*q.x*q.x - 2.0*q.z*q.z);
}

pub fn _read_txt(file_path: &str) -> String {
  info!(" [UTILS] Reading text file {file_path}");
  let data: String = fs::read_to_string(file_path)
                        .expect(&format!("\n [ERROR utils.read_txt] Unable to read file {file_path}  \n"));
  return data;
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Reflect)]
pub struct AABBf {
  pub min_x: f32,
  pub max_x: f32,
  pub min_z: f32,
  pub max_z: f32,
}
impl AABBf {
  pub fn intersect(self, other: &AABBf) -> bool {
    self.max_x >= other.min_x && self.min_x <= other.max_x &&
    self.max_z >= other.min_z && self.min_z <= other.max_z
  }
  pub fn has_point3(&self, p: &(f32, f32, f32)) -> bool {
    p.0 >= self.min_x && p.0 <= self.max_x && p.2 >= self.min_z && p.2 <= self.max_z
  }
  pub fn has_point(&self, p: &(f32, f32)) -> bool {
    p.0 >= self.min_x && p.0 <= self.max_x && p.1 >= self.min_z && p.1 <= self.max_z
  }
}

#[derive(Debug)]
pub struct AABBfs(pub Vec<AABBf>);

impl AABBfs {
  pub fn has_point3(&self, p: &(f32, f32, f32)) -> bool {
    for aabb in self.0.iter(){
      return p.0 >= aabb.min_x && p.0 <= aabb.max_x && p.2 >= aabb.min_z && p.2 <= aabb.max_z
    }
    return false;
  }
  pub fn has_point_3array(&self, p: &[f32; 3]) -> bool {
    for aabb in self.0.iter(){
      if p[0]>= aabb.min_x && p[0] <= aabb.max_x && p[2] >= aabb.min_z && p[2] <= aabb.max_z {
        return true;
      }
    }
    return false;
  }
}