
use libm::powf;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use bevy::prelude::*;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct NoiseData {
    pub noise:          Noises,
    pub seed:           u32,
    pub height_scale:   f32,
    pub scale:          f64,

    pub flatten:        Option<Flatten>
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Flatten {
    pub height: f32,
    pub dist:   f32
}




// Apply basic noise function to the mesh
pub fn apply_noise(mesh: &mut Mesh, nd: NoiseData) -> &Mesh {

    let noise_fn: NoiseFunc = NoiseFunc::build(nd.noise, nd.seed, None, None);
    let mut v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    for pos in v_pos.iter_mut() {
        let height: f32 = noise_fn.apply(pos[0], pos[2], nd.scale);
        pos[1] = height*nd.height_scale;
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    return mesh;
}


pub fn _smooth_step(x: f32, min_x: f32, max_x:f32) -> f32{
    let xc = x.clamp(min_x, max_x);
    return xc * xc * (3.0 - 2.0 * xc);
}

// Applies easing function to the noise output
pub fn _apply_easing(height: f32, easings: Easings) -> f32 {
    match easings {
        Easings::SmoothStart => {
            return height*height;
        }
        Easings::SmoothStop => {
            return 1.0 - ((1.0 - height)*(1.0-height));
        }
        Easings::SmoothEnd => {
            return 1.0 - (1.0 - height).powi(2);
        }
        Easings::SmoothStep => {
            return _smooth_step(height, 0.0, 1.0);
        }
        Easings::AbsoluteValue => {
            return height.abs();
        }
        Easings::AbsoluteValue2 => {
            return powf(height.abs(), 2.0);
        }
        Easings::AbsoluteValue3 => {
            return powf(height.abs(), 3.0);
        }
        Easings::AbsoluteValue6 => {
            return powf(height.abs(), 6.0);
        }
        Easings::AbsoluteValue10 => {
            return powf(height.abs(), 10.0);
        }
        Easings::None => {
            return height;
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Easings {
    SmoothStep,
    SmoothStop,
    SmoothStart,
    
    SmoothEnd,
    AbsoluteValue,
    AbsoluteValue2,
    AbsoluteValue3,
    AbsoluteValue6,
    AbsoluteValue10,
    None,
}

impl FromStr for Easings {
    type Err = ();
    fn from_str(input: &str) -> Result<Easings, Self::Err> {
        match input {
            "SmoothStep" => Ok(Easings::SmoothStep),
            "SmoothStop" => Ok(Easings::SmoothStop),
            "SmoothEnd" => Ok(Easings::SmoothEnd),
            "SmoothStart" => Ok(Easings::SmoothStart),
            "Abs" => Ok(Easings::AbsoluteValue),
            "Abs2" => Ok(Easings::AbsoluteValue2),
            "Abs3" => Ok(Easings::AbsoluteValue3),
            "Abs6" => Ok(Easings::AbsoluteValue6),
            "Abs10" => Ok(Easings::AbsoluteValue10),
            "None" => Ok(Easings::None),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Noises {
    None,
    Perlin,
    PerlinSurflet,
    OpenSimplex,
    Value,
    SuperSimplex,
    Worley,
    Simplex,
    FBMPerlin, // Fractal
    BMPerlin, //Basic Multi
    BPerlin,
    RMPerlin, // RidgedMultiPerlin
    HMPerlin, //Hybrid Multi perlin
    FBMPerlinSurflet,
    BMPerlinSurflet,
    BPerlinSurflet,
    RMPerlinSurflet,
    HMPerlinSurflet,
    FBMValue,
    BMValue,
    BValue,
    RMValue,
    HMValue,
    FBMSS,
    BMSS,
    BSS,
    RMSS,
    HMSS 
}

impl FromStr for Noises {
    type Err = ();
    fn from_str(input: &str) -> Result<Noises, Self::Err> {
        match input {
            "None" => Ok(Noises::None),
            "Perlin" => Ok(Noises::Perlin),
            "PerlinSurflet" => Ok(Noises::PerlinSurflet),
            "OpenSimplex" => Ok(Noises::OpenSimplex),
            "Value" => Ok(Noises::Value),
            "SuperSimplex" => Ok(Noises::SuperSimplex),
            "Worley" => Ok(Noises::Worley),
            "Simplex" => Ok(Noises::Simplex),
            "FBMPerlin" => Ok(Noises::FBMPerlin),
            "BMPerlin" => Ok(Noises::BMPerlin),
            "BPerlin" => Ok(Noises::BPerlin),
            "RMPerlin" => Ok(Noises::RMPerlin),
            "HMPerlin" => Ok(Noises::HMPerlin),
            "FBMPerlinSurflet" => Ok(Noises::FBMPerlinSurflet),
            "BMPerlinSurflet" => Ok(Noises::BMPerlinSurflet),
            "BPerlinSurflet" => Ok(Noises::BPerlinSurflet),
            "RMPerlinSurflet" => Ok(Noises::RMPerlinSurflet),
            "HMPerlinSurflet" => Ok(Noises::HMPerlinSurflet),
            "FBMValue" => Ok(Noises::FBMValue),
            "BMValue" => Ok(Noises::BMValue),
            "BValue" => Ok(Noises::BValue),
            "RMValue" => Ok(Noises::RMValue),
            "HMValue" => Ok(Noises::HMValue),
            "FBMSS" => Ok(Noises::FBMSS),
            "BMSS" => Ok(Noises::BMSS),
            "BSS" => Ok(Noises::BSS),
            "RMSS" => Ok(Noises::RMSS),
            "HMSS" => Ok(Noises::HMSS),
            _ => Err(()),
        }
    }
}



#[derive(Clone)]
pub enum NoiseFunc {
    None,
    Perlin(Perlin),
    PerlinSurflet(PerlinSurflet),
    OpenSimplex(OpenSimplex),
    SuperSimplex(SuperSimplex),
    Simplex(Simplex),
    Value(Value),
    Worley(Worley),
    BMPerlin(BasicMulti<Perlin>), // very nice!
    FBMPerlin(Fbm<Perlin>),
    BPerlin(Billow<Perlin>),
    RMPerlin(RidgedMulti<Perlin>),
    HMPerlin(HybridMulti<Perlin>),
    BMPerlinSurflet(BasicMulti<PerlinSurflet>),
    FBMPerlinSurflet(Fbm<PerlinSurflet>),
    BPerlinSurflet(Billow<PerlinSurflet>),
    RMPerlinSurflet(RidgedMulti<PerlinSurflet>),
    HMPerlinSurflet(HybridMulti<PerlinSurflet>),
    BMValue(BasicMulti<Value>),
    FBMValue(Fbm<Value>),
    BValue(Billow<Value>),
    RMValue(RidgedMulti<Value>),
    HMValue(HybridMulti<Value>),
    BMSS(BasicMulti<SuperSimplex>),
    FBMSS(Fbm<SuperSimplex>),
    BSS(Billow<SuperSimplex>),
    RMSS(RidgedMulti<SuperSimplex>),
    HMSS(HybridMulti<SuperSimplex>)
}

// noise_fn.octaves; // number of detail
// noise_fn.frequency; // The number of cycles per unit length that the noise function outputs.
// noise_fn.lacunarity; // A lacunarity of 2.0 results in the frequency doubling every octave. For almost all cases, 2.0 is a good value to use.
// noise_fn.persistence; // Increasing the persistence produces “rougher” noise.

impl NoiseFunc {
    fn build(noise: Noises, seed: u32, octaves: Option<usize>, freq: Option<f64>) -> NoiseFunc {
        match noise {
            Noises::None =>          {return NoiseFunc::None}
            Noises::Perlin =>        {return NoiseFunc::Perlin(Perlin::new(seed))}
            Noises::PerlinSurflet => {return NoiseFunc::PerlinSurflet(PerlinSurflet::new(seed))}
            Noises::Value =>         {return NoiseFunc::Value(Value::new(seed))}
            Noises::OpenSimplex =>   {return NoiseFunc::OpenSimplex(OpenSimplex::new(seed))}
            Noises::SuperSimplex =>  {return NoiseFunc::SuperSimplex(SuperSimplex::new(seed))}
            Noises::Worley =>        {return NoiseFunc::Worley(Worley::new(seed))}
            Noises::Simplex =>       {return NoiseFunc::Simplex(Simplex::new(seed))}
            Noises::FBMPerlin =>     {
                let mut noise_fn: Fbm<Perlin> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMPerlin(noise_fn)
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMPerlin(noise_fn)
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BPerlin(noise_fn)
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMPerlin(noise_fn)
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMPerlin(noise_fn)
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMPerlinSurflet(noise_fn)
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMPerlinSurflet(noise_fn)
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BPerlinSurflet(noise_fn)
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMPerlinSurflet(noise_fn)
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMPerlinSurflet(noise_fn)
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMValue(noise_fn)
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMValue(noise_fn)
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BValue(noise_fn)
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMValue(noise_fn)
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMValue(noise_fn)
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::FBMSS(noise_fn)
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BMSS(noise_fn)
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::BSS(noise_fn)
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::RMSS(noise_fn)
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunc::HMSS(noise_fn)
            }
        }
    }

    // not sure if there is any better way to do it :/ wish to just have one arm as the code is the same
    fn apply(&self, x: f32, y: f32, scale: f64) -> f32 {
        match self {
            NoiseFunc::None                          => {1.0*scale as f32}
            NoiseFunc::Perlin(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::PerlinSurflet(f)              => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Value(f)                      => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::OpenSimplex(f)                => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::SuperSimplex(f)               => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Worley(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::Simplex(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMPerlin(f)                  => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BPerlin(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMPerlin(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMPerlinSurflet(f)           => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BPerlinSurflet(f)             => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMPerlinSurflet(f)            => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMValue(f)                   => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BValue(f)                     => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMValue(f)                    => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::FBMSS(f)                      => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::BSS(f)                        => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::RMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
            NoiseFunc::HMSS(f)                       => {f.get([x as f64 * scale, y as f64 * scale]) as f32}
        }
    }
}