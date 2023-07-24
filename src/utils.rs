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
