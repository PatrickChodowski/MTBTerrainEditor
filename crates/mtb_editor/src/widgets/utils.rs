
// minimal node aabb [min_x, max_x, min_y, max_y]
pub fn get_aabb(xy: &(f32, f32), dims: &(f32, f32)) -> [f32; 4] {
    [xy.0 - dims.0/2.0, xy.0 + dims.0/2.0, xy.1 - dims.1/2.0, xy.1 + dims.1/2.0]
}

pub fn has_point(aabb: &[f32; 4], p: &(f32, f32)) -> bool {
    p.0 >= aabb[0] && p.0 <= aabb[1] && p.1 >= aabb[2] && p.1 <= aabb[3]
}