
use libm::powf;
use serde::{Deserialize,Serialize};
use bevy::utils::HashMap;

use crate::terrain::planes::PlaneData;
use crate::terrain::utils::{AABB, AABBs, Edge, Axis};
use crate::terrains::wanders::get_distance_manhattan;



// takes area of points and smoothes them out
























// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum SmoothMethod {
//   Distance,
//   DistanceReverse,
//   DistanceSquare,
//   DistancePower(f32),
//   Value(f32)
// }

// // Smooth edge line data
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SmoothEdgeData {
//   pub buffer: f32,
//   pub method: SmoothMethod
// }


//     pub fn _apply(&self, v_pos: &mut Vec<[f32; 3]>) -> HashMap<usize, f32>{
//       let mut abpoints: HashMap<usize, Vec<(&[f32; 3], usize)>> = HashMap::new();
//       let mut index_heights: HashMap<usize, f32> = HashMap::new();

//       // Mapping first
//       for (pos_index, pos) in v_pos.iter().enumerate(){
//         for (index, aabb) in self.aabbs.iter().enumerate() {
//           if aabb._has_point_excl(pos){
//             abpoints.entry(index).or_insert(Vec::new()).push((pos, pos_index));
//           }
//         }
//       }

//       // per aabb
//       for (index, points) in abpoints.iter_mut(){

//         let axis: Axis = self.axes[*index];
//         let main: f32 = self.mains[*index];

//         for (pos, pos_index) in points.iter_mut(){
//           let dist: f32;
//           match axis {
//             Axis::X => {
//               dist = get_distance_manhattan(&(pos[0], pos[2]), &(pos[0], main));
//             }
//             Axis::Z => {
//               dist = get_distance_manhattan(&(pos[0], pos[2]), &(main, pos[2]));
//             }
//           }

//           let scaled_height: f32;
//           match self.method {
//             SmoothMethod::Distance          => {scaled_height =  (dist/self.buffer).clamp(0.0, 1.0)* pos[1];}
//             SmoothMethod::DistanceReverse   => {scaled_height =  (1.0- (dist/self.buffer).clamp(0.0, 1.0))*pos[1];}
//             SmoothMethod::DistanceSquare    => {scaled_height =  powf((dist/self.buffer).clamp(0.0, 1.0), 2.0)* pos[1];}
//             SmoothMethod::DistancePower(p)  => {scaled_height =  powf((dist/self.buffer).clamp(0.0, 1.0), p)* pos[1];}
//             SmoothMethod::Value(p)          => {scaled_height = p;}
//           }
//           index_heights.insert(*pos_index, scaled_height);
//         }

//       }
//       return index_heights;
//     }
// }

// impl SmoothEdgeData{
//   pub fn set(&self) -> SmoothEdge {
//     return SmoothEdge{buffer: self.buffer, method: self.method.clone(), axes: Vec::new(), aabbs: Vec::new(), mains: Vec::new()};
//   }
// }

