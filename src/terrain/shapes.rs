
use serde::{Deserialize,Serialize};

use crate::terrain::utils::{AABB, Ellipse};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ShapeDims {
    pub x: f32,
    pub z: f32
}

// apply simple height modifier of given area and given shape to the plane

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ShapeTypeData {
    AABB(ShapeDims),
    Ellipse(ShapeDims)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ShapeType {
    AABB(AABB),
    Ellipse(Ellipse)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeData {
    pub shapetypedata: ShapeTypeData,
    pub loc:       [f32; 2],
    pub height:    f32
}
impl ShapeData {
    pub fn set(&self) -> Shape {
        // let shape = Shape{loc: self.loc, 
        //                   height: self.height, 
        //                   shapetype: self.shapetypedata};

        let shapetype: ShapeType;
        match self.shapetypedata {
            ShapeTypeData::AABB(dims) => {
                shapetype = ShapeType::AABB(AABB{min_x: self.loc[0]-1.0*dims.x/2.0, max_x: self.loc[0]+dims.x/2.0, min_z: self.loc[1]-1.0*dims.z/2.0, max_z: self.loc[1]+dims.z/2.0});
            }
            ShapeTypeData::Ellipse(dims) => {
                shapetype = ShapeType::Ellipse(Ellipse{ a: dims.x, b: dims.z, x: self.loc[0], z: self.loc[1] });
            }
        }

        let shape = Shape{height: self.height, shapetype};
        return shape;

    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shape {
    pub height:    f32,
    pub shapetype: ShapeType,
}

impl Shape {
    pub fn apply(&self, pos: &[f32; 3]) -> f32 { 
        match &self.shapetype {
            ShapeType::AABB(aabb) => {
                if aabb.has_point(pos) {
                    return self.height;
                }
            }
            ShapeType::Ellipse(ellipse) => {
                if ellipse.has_point(pos){
                    return self.height;
                }
            }
        }
        return pos[1];

    }
}