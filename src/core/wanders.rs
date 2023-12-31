
use serde::{Serialize, Deserialize};
use libm::atan2f;
use rand::prelude::*;

use super::easings::Easings;
use super::planes::PlaneData;
use super::utils::{AABB, get_distance_manhattan};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetWanderNoiseData {
    pub wander:               TargetWanders,
    pub width:                f32,
    pub height:               f32,
    pub step:                 f32,
    pub max_steps:            u32,
    pub source:               WanderLoc,
    pub target:               WanderLoc,
    pub seed:                 NoiseSeed,
    pub easing:               Easings
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetWanderNoise {
    pub wander:               TargetWanders,
    pub width:                f32,
    pub height:               f32,
    pub step:                 f32,
    pub max_steps:            u32,
    pub source:               WanderLoc,
    pub target:               WanderLoc,
    pub seed:                 NoiseSeed,
    pub easing:               Easings
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum NoiseSeed {
    SetSeed(u64),
    NoSeed
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum WanderLoc {
    Point((f32,f32)),
    Edge(Edge)
}

impl WanderLoc {
    fn get_point(&self, pab: &AABB, seed: NoiseSeed) -> (f32, f32) {
        match self {
            WanderLoc::Point(p)   => {return *p;}
            WanderLoc::Edge(edge) =>  {
                match edge {
                    Edge::X  => {(pab.max_x, get_random_range(pab.min_z, pab.max_z, seed))}
                    Edge::NX => {(pab.min_x, get_random_range(pab.min_z, pab.max_z, seed))}
                    Edge::Z  => {(pab.max_z, get_random_range(pab.min_x, pab.max_x, seed))}
                    Edge::NZ => {(pab.min_z, get_random_range(pab.min_x, pab.max_x, seed))}
                }
            }
        }
    }
}


impl TargetWanderNoiseData {
    pub fn set(&self, pd: &PlaneData) -> TargetWanderNoise {
        let pab = pd.get_aabb();
        let mut xz: (f32, f32) = self.source.get_point(&pab, self.seed);
        let end: (f32, f32) = self.target.get_point(&pab, self.seed);

        // Generate step points using wandering function. Step points will be central points of aabb boxes
        let mut points: Vec<(f32, f32)> = vec![xz];
        let mut i: u32 = 0;
        let mut direction = get_direction(&xz, &end);
        let mut last_angle: f32 = direction;
        let mut distance = get_distance_manhattan(&xz, &end);
        let inloop: bool = true;
        // println!(" WANDERS DEBUG from source: ({:?}) to target ({:?}), direction: {}", xz, end, direction);

        while inloop {
            i += 1;

            // end conditions
            if i >= self.max_steps {
                // println!(" WANDERS DEBUG breaking becasue of max steps. points count: {}", points.len());
                break;
            }

            if distance <= self.step {
                // println!(" WANDERS DEBUG breaking becasue of distance. points count: {}", points.len());
                break;
            }

            let (w_x, w_z, w_angle) = self.wander.apply(&xz, last_angle, self.step, direction, self.seed, i as u64);
            points.push((w_x,w_z));
            xz.0 = w_x;
            xz.1 = w_z;
            last_angle = w_angle;

            direction = get_direction(&xz, &end);
            distance = get_distance_manhattan(&xz, &end);
            
        }

        // construct aabbs from points
        let mut aabbs = Vec::new();
        for p in points.iter(){
            aabbs.push(AABB::from_point(p, &(self.step*2.0, self.width)));
        }

        // construct ellipses from points
        let mut ellipses = Vec::new();
        for p in points.iter(){
            ellipses.push(Ellipse{a: self.step*2.0, b: self.width, x: p.0, z: p.1});
        }


        return TargetWanderNoise{wander: self.wander, width: self.width, height: self.height, step: self.step,
            max_steps: self.max_steps, source: self.source, target: self.target, seed: self.seed, aabbs, ellipses, easing: self.easing};

    }
}

impl TargetWanderNoise {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 {
        for ellipse in self.ellipses.iter(){
            if let Some(dist) = ellipse.has_point_dist(pos){
                let scale = 1.0 - dist/ellipse.b.clamp(0.0, 1.0)/100.0;
                return self.height*self.easing.apply(scale);
            }
        }

        return pos[1];
    }
}




#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TargetWanders {
    Emu(f32),     // not, grid, steers randomly in degrees range
    EmuPerlin, // next angle is generated by perlin
    Wanderer   // nswe, goal goal goal, when you have a start and end point
}

impl TargetWanders {
    // returns new x, z, last_angle
    fn apply(&self, xz: &(f32,f32), last_angle: f32, step: f32, direction: f32, seed: NoiseSeed, i: u64) -> (f32, f32, f32) {
        let mut new_x: f32 = xz.0;
        let mut new_z: f32 = xz.1;
        let mut new_angle: f32 = last_angle;

        match self {
            // Wanders::Robot => {
            //     // Change direction every 5 steps
            //     if step_count % 5 == 0 {
            //         let angles: Vec<f32> = vec![0.0, 1.578, 3.142, -1.578]; 
            //         let final_angle = get_random_element(&angles);
            //         new_angle = final_angle;
            //     }
            //     (new_x, new_z) = get_point_angle(xz, step, new_angle);
            // }
            // Wanders::Zax => {
            //     // Change direction every 5 steps
            //     if step_count % 5 == 0 {
            //         let angles: Vec<f32> = vec![0.0, 1.578, 3.142, -1.578]; 
            //         let final_angle = get_random_element(&angles);
            //         new_angle = final_angle;
            //     }
            //     (new_x, new_z) = get_point_angle(xz, step, new_angle);
            // }

            TargetWanders::Emu(degrees) => {
                let r2 = degrees.to_radians()/2.0;
                (new_x, new_z, new_angle) = get_random_arc(xz,step, direction-r2, direction+r2, seed, i);
            }
            TargetWanders::Wanderer => {
                let angles: Vec<f32> = vec![direction+1.578, direction-1.578, direction - 3.142, direction, direction, direction]; // :)
                let final_angle = get_random_element(&angles, seed, i);
                new_angle = final_angle;
                (new_x, new_z) = get_point_angle(xz, step, new_angle);
            }
            _ => {}
        }

        return (new_x, new_z, new_angle);

    }
}

pub fn get_random_element(elements: &Vec<f32>, seed: NoiseSeed, i: u64) -> f32{
    match seed {
        NoiseSeed::NoSeed => {
            let mut rng  = thread_rng();
            let random_index = rng.gen_range(0..elements.len());
            return elements[random_index];
        }
        NoiseSeed::SetSeed(seed) => {
            let mut rng = StdRng::seed_from_u64(seed+i);
            let random_index = rng.gen_range(0..elements.len());
            return elements[random_index];
        }
    }
}

fn get_point_angle(xz: &(f32,f32), r: f32, angle: f32) -> (f32, f32){
    let point_x = xz.0 + r * angle.cos();
    let point_z = xz.1 + r * angle.sin();
    (point_x, point_z)
}

fn get_random_arc(xz: &(f32,f32), r: f32, start_angle: f32, end_angle: f32, seed: NoiseSeed, i: u64) -> (f32, f32, f32) {
    let random_angle: f32;
    match seed {
        NoiseSeed::NoSeed => {
            let mut rng  = thread_rng();
            random_angle = rng.gen_range(start_angle..=end_angle);
        }
        NoiseSeed::SetSeed(seed) => {
            let mut rng = StdRng::seed_from_u64(seed+i);
            random_angle = rng.gen_range(start_angle..=end_angle);
        }
    }
    let point_x = xz.0 + r * random_angle.cos();
    let point_z = xz.1 + r * random_angle.sin();
    (point_x, point_z, random_angle)
}

fn get_random_range(min: f32, max: f32, seed: NoiseSeed) -> f32 {
    match seed {
        NoiseSeed::NoSeed => {
            let mut rng  = thread_rng();
            return rng.gen_range(min..=max);
            
        }
        NoiseSeed::SetSeed(seed) => {
            let mut rng = StdRng::seed_from_u64(seed);
            return rng.gen_range(min..=max);
        }
    }
}

pub fn get_direction(xz: &(f32, f32), target: &(f32, f32)) -> f32 {
    return atan2f(target.1 - xz.1, target.0 - xz.0);
}

