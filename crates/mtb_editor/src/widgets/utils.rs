use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, Component)]
pub struct AABB {
  pub min_x:          f32,
  pub max_x:          f32,
  pub min_y:          f32,
  pub max_y:          f32
}

impl AABB {
    pub fn new(xy: &(f32, f32), dims: &(f32, f32))  -> AABB {
        AABB{min_x: xy.0 - dims.0/2.0, max_x: xy.0 + dims.0/2.0, min_y: xy.1 - dims.1/2.0, max_y: xy.1 + dims.1/2.0}
    }

    pub fn has_point(&self, p: &(f32, f32)) -> bool {
        p.0 >= self.min_x && p.0 <= self.max_x && p.1 >= self.min_y && p.1 <= self.max_y
    }
    
}