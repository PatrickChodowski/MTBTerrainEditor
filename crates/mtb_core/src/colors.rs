use std::str::FromStr;
use bevy::{utils::HashMap, prelude::Resource};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Resource)]
pub struct ColorsLib {
    pub data: HashMap<String, [f32; 4]>
}
impl ColorsLib {
    pub fn new() -> Self{
        ColorsLib{data: HashMap::new()}
    }
    pub fn add(&mut self, id: String, color: [f32; 4]){
        self.data.insert(id, color);
    }
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorData {
    pub id:  String,
    pub clr: [f32; 4]
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlaneColor {
    Color([f32; 4]),        // Single Color
    Gradient(ColorGradient) // Tuple of 2 color (low, high)
}

impl PlaneColor {
    pub fn apply(&self, height: f32, min_height: f32, max_height: f32) -> [f32; 4] {
        match self {
            PlaneColor::Color(clr) => {return *clr;}
            PlaneColor::Gradient(cgr) => {
                let scale = (height - min_height)/(max_height - min_height);
                return [cgr.low[0] + scale*(cgr.high[0] - cgr.low[0]), 
                        cgr.low[1] + scale*(cgr.high[1] - cgr.low[1]),
                        cgr.low[2] + scale*(cgr.high[2] - cgr.low[2]),
                        cgr.low[3] + scale*(cgr.high[3] - cgr.low[3])];
            } 
        }
    }
    pub fn new() -> Self {
        PlaneColor::Color([0.0, 0.0, 0.0, 0.0])
    }
}


impl FromStr for PlaneColor {
    type Err = ();
    fn from_str(input: &str) -> Result<PlaneColor, Self::Err> {
        match input {
            "c"    => Ok(PlaneColor::Color([0.0, 0.0, 0.0, 0.0])),
            "g"    => Ok(PlaneColor::Gradient(ColorGradient::new())),
            _      => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorGradient {
    pub low: [f32; 4],
    pub high: [f32; 4]
}

impl ColorGradient {
    pub fn new() -> Self {
        ColorGradient{low: [0.0, 0.0, 0.0, 0.0], high: [1.0, 1.0, 1.0, 1.0]}
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorRange {
    pub from: f32,
    pub to:   f32,
    pub clr:  PlaneColor
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Colors {
    pub data: Vec<ColorRange>
}



impl Colors {
    pub fn apply(&self, height: f32, _min_height: f32, _max_height: f32) -> [f32; 4] {
        for color_range in self.data.iter() {
            if height >= color_range.from && height < color_range.to {
                return color_range.clr.apply(height, color_range.from, color_range.to);
            }
        }
        return [1.0, 1.0, 1.0, 1.0];
    }

    pub fn new() -> Self {
        Colors{data: Vec::new()}
    }
}

  
