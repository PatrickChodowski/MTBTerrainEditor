
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoiseData {
    pub noise:          Noises,
    pub seed:           u32,
    pub height_scale:   f32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
    pub global:         bool
}

#[derive(Clone)]
pub struct Noise {
    pub noise:          Noises,
    pub seed:           u32,
    pub height_scale:   f32,
    pub scale:          f64,
    pub octaves:        Option<usize>,
    pub freq:           Option<f64>,
    pub global:         bool,
    pub noise_function: NoiseFunction
}


impl NoiseData {

    pub fn set(&self) -> Noise {

        let nfn: NoiseFunction;
        match self.noise {
            Noises::Perlin =>        {nfn = NoiseFunction::Perlin(Perlin::new(self.seed))}
            Noises::PerlinSurflet => {nfn = NoiseFunction::PerlinSurflet(PerlinSurflet::new(self.seed))}
            Noises::Value =>         {nfn = NoiseFunction::Value(Value::new(self.seed))}
            Noises::OpenSimplex =>   {nfn = NoiseFunction::OpenSimplex(OpenSimplex::new(self.seed))}
            Noises::SuperSimplex =>  {nfn = NoiseFunction::SuperSimplex(SuperSimplex::new(self.seed))}
            Noises::Worley =>        {nfn = NoiseFunction::Worley(Worley::new(self.seed))}
            Noises::Simplex =>       {nfn = NoiseFunction::Simplex(Simplex::new(self.seed))}
            Noises::FBMPerlin =>     {
                let mut noise_fn: Fbm<Perlin> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::FBMPerlin(noise_fn);
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::BMPerlin(noise_fn);
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::BPerlin(noise_fn);
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::RMPerlin(noise_fn);
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMPerlin(noise_fn);
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMPerlinSurflet(noise_fn);
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMPerlinSurflet(noise_fn);
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BPerlinSurflet(noise_fn);
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMPerlinSurflet(noise_fn);
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMPerlinSurflet(noise_fn);
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMValue(noise_fn);
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMValue(noise_fn);
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BValue(noise_fn);
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMValue(noise_fn);
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::HMValue(noise_fn);
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::FBMSS(noise_fn);
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BMSS(noise_fn);
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::BSS(noise_fn);
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn =  NoiseFunction::RMSS(noise_fn);
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                nfn = NoiseFunction::HMSS(noise_fn);
            }
        }

        return Noise {noise: self.noise.clone(), seed: self.seed, height_scale: self.height_scale, 
                      scale: self.scale, octaves: self.octaves, freq: self.freq, global: self.global, noise_function: nfn};
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

impl Noise {
    pub fn apply(&self, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {
        let mut gpos: [f32; 3] = *pos;
        if self.global {
            gpos[0] = pos[0] + loc[0];
            gpos[1] = pos[1] + loc[1];
            gpos[2] = pos[2] + loc[2];
        }

        let r: f64;
        match &self.noise_function {
            // XD but I really dont know how to make it better
            NoiseFunction::Perlin(f)                     => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::PerlinSurflet(f)              => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::Value(f)                      => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::OpenSimplex(f)                => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::SuperSimplex(f)               => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::Worley(f)                     => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::Simplex(f)                    => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::FBMPerlin(f)                  => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BMPerlin(f)                   => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BPerlin(f)                    => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::RMPerlin(f)                   => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::HMPerlin(f)                   => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::FBMPerlinSurflet(f)           => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BPerlinSurflet(f)             => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::RMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::HMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::FBMValue(f)                   => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BMValue(f)                    => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BValue(f)                     => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::RMValue(f)                    => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::HMValue(f)                    => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::FBMSS(f)                      => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BMSS(f)                       => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::BSS(f)                        => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::RMSS(f)                       => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
            NoiseFunction::HMSS(f)                       => {r = f.get([pos[0] as f64 * self.scale, pos[2] as f64 * self.scale])}
        }
        return r as f32 * self.height_scale;    
    }

}

