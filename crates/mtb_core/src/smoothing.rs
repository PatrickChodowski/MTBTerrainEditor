
use bevy::utils::HashMap;
use serde::{Deserialize,Serialize};
use libm::logf;

use super::modifiers::ModifierBase;
use super::utils::{Area,get_distance_euclidean};

// takes area of points and smoothes them out
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmoothingData {
    pub mb:     ModifierBase,
    pub method: SmoothingMethod
}

impl SmoothingData {
    pub fn set(&self) -> Smoothing {
        return Smoothing{
            area: self.mb.to_area(),
            method: self.method
        };
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Smoothing {
    pub area:   Area,
    pub method: SmoothingMethod
}

impl Smoothing {
    pub fn apply(&self, v_pos: &mut Vec<[f32; 3]>) {
        let mut min_height = f32::MAX;
        let mut max_height = f32::MIN;
        let mut points: HashMap<usize, [f32; 3]> = HashMap::new();

        for (index, pos) in v_pos.iter().enumerate(){
            if self.area.has_point(pos){
                if pos[1] > max_height {
                    max_height = pos[1];
                }
                if pos[1] < min_height {
                    min_height = pos[1];
                }
                points.insert(index, *pos);
            }  
        }

        self.method.apply(&mut points, min_height, max_height);

        for (index, pos) in points.iter(){
            v_pos[*index] = *pos;
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SmoothingMethod {
    HeightNormalize,
    DistanceToPoint((f32, f32, f32))
}

impl SmoothingMethod {
    pub fn apply(&self, points:  &mut HashMap<usize, [f32;3]>, min_height: f32, max_height: f32){
        match self {
            SmoothingMethod::HeightNormalize => {
                let range = max_height - min_height;
                if range <= 0.0 {
                    return;
                }
                let mut new_points: Vec<(usize, (f32, f32, f32))> = Vec::with_capacity(points.len());
                for (index, point) in points.iter(){
                    let mut p2: [f32; 3] = point.clone();
                    let st_point: f32 = (p2[1] - min_height)/range + 1.0; // +1 to standardadize between 1 and 2 instead 0 and for logarithms
                    let ph: f32 = p2[1].clone()*(1.0-logf(st_point));
                    p2[1] = ph;
                    new_points.push((*index, (p2[0], p2[1], p2[2])));
                }
                for np in new_points.iter(){
                    points.insert(np.0, [np.1.0, np.1.1, np.1.2]);
                }

            }
            SmoothingMethod::DistanceToPoint((x, z, height)) => {
                let mut dists: Vec<(usize,f32)> = Vec::with_capacity(points.len());
                let mut max_dist = f32::MIN;
                for (index, point) in points.iter(){
                    let dist = get_distance_euclidean(&(*x,*z), &(point[0], point[2]));
                    if dist > max_dist {
                        max_dist = dist;
                    }
                    dists.push((*index, dist));
                }

                for (index, dist) in dists.iter(){
                    let old_point = points[index];
                    let scale = 1.0 - dist/max_dist;
                    let mut height2 = height*scale;

                    if height < &old_point[1] && height2 > old_point[1]{
                        height2 = old_point[1];
                    }

                    if height > &old_point[1] && height2 < old_point[1]{
                        height2 = old_point[1];
                    }

                    points.insert(*index, [old_point[0], height2, old_point[2]]);
                }

    
            }
        }
    }
}
