
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};
use serde::{Serialize, Deserialize};

use super::easings::Easings;
use super::modifiers::ModifierBase;
use super::utils::Area;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoiseData {
    pub mb:             ModifierBase,
    pub noise:          Noises,
    pub seed:           u32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
    pub easing:         Easings,
    pub global:         bool
}

#[derive(Clone)]
pub struct Noise {
    pub area:           Area,
    pub noise:          Noises,
    pub seed:           u32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
    pub easing:         Easings,
    pub global:         bool,
    pub noise_function: NoiseFunction
}


impl NoiseData {
    pub fn set(&self) -> Noise {
        let nfn = NoiseFunction::new(self.noise.clone(), self.seed, self.octaves, self.freq);
        return Noise {area:             self.mb.to_area(),
                      noise:            self.noise.clone(), 
                      seed:             self.seed, 
                      scale:            self.scale, 
                      octaves:          self.octaves, 
                      freq:             self.freq, 
                      easing:           self.easing,
                      global:           self.global, 
                      noise_function:   nfn};
    }
}





#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Noises {
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

impl Noise {
    pub fn apply(&self, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {

        let mut gpos: [f32; 3] = *pos;
        if self.global {
            gpos[0] = pos[0] + loc[0];
            gpos[1] = pos[1] + loc[1];
            gpos[2] = pos[2] + loc[2];
        }

        if !self.area.has_point(pos) {
            return gpos[1];
        }

        let r: f64 = self.noise_function.apply(self.scale, gpos[0] as f64, gpos[2] as f64);
        let eased_r = self.easing.apply(r as f32);
        return eased_r * gpos[1];    
    }
}

// Simpler noise used as argument in other modifiers

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimpleNoiseData {
    pub noise:          Noises,
    pub seed:           u32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
}

impl SimpleNoiseData {
    pub fn set(&self) -> SimpleNoise {
        let nfn = NoiseFunction::new(self.noise.clone(), self.seed, self.octaves, self.freq);
        return SimpleNoise {
                    noise:            self.noise.clone(), 
                    seed:             self.seed, 
                    scale:            self.scale, 
                    octaves:          self.octaves, 
                    freq:             self.freq, 
                    noise_function:   nfn};
    }
}


#[derive(Clone)]
pub struct SimpleNoise {
    pub noise:          Noises,
    pub seed:           u32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
    pub noise_function: NoiseFunction
}

impl SimpleNoise {
    pub fn apply(&self, x: f32, z: f32) -> f32 {
        let r: f64 = self.noise_function.apply(self.scale, x as f64, z as f64);
        return r as f32;    
    }
    pub fn _apply3d(&self, x: f32, y: f32, z: f32) -> f32 {
        let r: f64 = self.noise_function._apply3d(self.scale, x as f64, y as f64, z as f64);
        return r as f32;    
    }
}





#[derive(Clone)]
pub enum NoiseFunction {
    Perlin(Perlin),
    PerlinSurflet(PerlinSurflet),
    OpenSimplex(OpenSimplex),
    SuperSimplex(SuperSimplex),
    Simplex(Simplex),
    Value(Value),
    Worley(Worley),
    BMPerlin(BasicMulti<Perlin>),
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

impl NoiseFunction {
    pub fn apply(&self, scale: f64, x: f64, z: f64) -> f64 {
        let r: f64;
        match &self {
            // XD but I really dont know how to make it better
            NoiseFunction::Perlin(f)                     => {r = f.get([x* scale, z * scale])}
            NoiseFunction::PerlinSurflet(f)              => {r = f.get([x* scale, z * scale])}
            NoiseFunction::Value(f)                      => {r = f.get([x* scale, z * scale])}
            NoiseFunction::OpenSimplex(f)                => {r = f.get([x* scale, z * scale])}
            NoiseFunction::SuperSimplex(f)               => {r = f.get([x* scale, z * scale])}
            NoiseFunction::Worley(f)                     => {r = f.get([x* scale, z * scale])}
            NoiseFunction::Simplex(f)                    => {r = f.get([x* scale, z * scale])}
            NoiseFunction::FBMPerlin(f)                  => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BMPerlin(f)                   => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BPerlin(f)                    => {r = f.get([x* scale, z * scale])}
            NoiseFunction::RMPerlin(f)                   => {r = f.get([x* scale, z * scale])}
            NoiseFunction::HMPerlin(f)                   => {r = f.get([x* scale, z * scale])}
            NoiseFunction::FBMPerlinSurflet(f)           => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BMPerlinSurflet(f)            => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BPerlinSurflet(f)             => {r = f.get([x* scale, z * scale])}
            NoiseFunction::RMPerlinSurflet(f)            => {r = f.get([x* scale, z * scale])}
            NoiseFunction::HMPerlinSurflet(f)            => {r = f.get([x* scale, z * scale])}
            NoiseFunction::FBMValue(f)                   => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BMValue(f)                    => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BValue(f)                     => {r = f.get([x* scale, z * scale])}
            NoiseFunction::RMValue(f)                    => {r = f.get([x* scale, z * scale])}
            NoiseFunction::HMValue(f)                    => {r = f.get([x* scale, z * scale])}
            NoiseFunction::FBMSS(f)                      => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BMSS(f)                       => {r = f.get([x* scale, z * scale])}
            NoiseFunction::BSS(f)                        => {r = f.get([x* scale, z * scale])}
            NoiseFunction::RMSS(f)                       => {r = f.get([x* scale, z * scale])}
            NoiseFunction::HMSS(f)                       => {r = f.get([x* scale, z * scale])}
        }
        return r;
    }

    pub fn _apply3d(&self, scale: f64, x: f64, y: f64, z: f64) -> f64 {
        let r: f64;
        match &self {
            // XD but I really dont know how to make it better
            NoiseFunction::Perlin(f)                     => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::PerlinSurflet(f)              => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::Value(f)                      => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::OpenSimplex(f)                => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::SuperSimplex(f)               => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::Worley(f)                     => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::Simplex(f)                    => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::FBMPerlin(f)                  => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BMPerlin(f)                   => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BPerlin(f)                    => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::RMPerlin(f)                   => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::HMPerlin(f)                   => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::FBMPerlinSurflet(f)           => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BMPerlinSurflet(f)            => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BPerlinSurflet(f)             => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::RMPerlinSurflet(f)            => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::HMPerlinSurflet(f)            => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::FBMValue(f)                   => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BMValue(f)                    => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BValue(f)                     => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::RMValue(f)                    => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::HMValue(f)                    => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::FBMSS(f)                      => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BMSS(f)                       => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::BSS(f)                        => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::RMSS(f)                       => {r = f.get([x* scale, y*scale, z * scale])}
            NoiseFunction::HMSS(f)                       => {r = f.get([x* scale, y*scale, z * scale])}
        }
        return r;
    }

 

    pub fn new(noise: Noises, seed: u32, octaves: Option<usize>, freq: Option<f64>) -> Self {
        let nfn: NoiseFunction;
        match noise {
            Noises::Perlin =>        {nfn = NoiseFunction::Perlin(Perlin::new(seed))}
            Noises::PerlinSurflet => {nfn = NoiseFunction::PerlinSurflet(PerlinSurflet::new(seed))}
            Noises::Value =>         {nfn = NoiseFunction::Value(Value::new(seed))}
            Noises::OpenSimplex =>   {nfn = NoiseFunction::OpenSimplex(OpenSimplex::new(seed))}
            Noises::SuperSimplex =>  {nfn = NoiseFunction::SuperSimplex(SuperSimplex::new(seed))}
            Noises::Worley =>        {nfn = NoiseFunction::Worley(Worley::new(seed))}
            Noises::Simplex =>       {nfn = NoiseFunction::Simplex(Simplex::new(seed))}
            Noises::FBMPerlin =>     {
                let mut noise_fn: Fbm<Perlin> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::FBMPerlin(noise_fn);
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::BMPerlin(noise_fn);
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::BPerlin(noise_fn);
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::RMPerlin(noise_fn);
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMPerlin(noise_fn);
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMPerlinSurflet(noise_fn);
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMPerlinSurflet(noise_fn);
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BPerlinSurflet(noise_fn);
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMPerlinSurflet(noise_fn);
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMPerlinSurflet(noise_fn);
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMValue(noise_fn);
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMValue(noise_fn);
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BValue(noise_fn);
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMValue(noise_fn);
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMValue(noise_fn);
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMSS(noise_fn);
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMSS(noise_fn);
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BSS(noise_fn);
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMSS(noise_fn);
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(seed);
                if let Some(octaves) = octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::HMSS(noise_fn);
            }
            
        }
        return nfn;
    }
}
