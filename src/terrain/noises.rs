
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};
use serde::{Serialize, Deserialize};

use crate::terrain::modifiers::ModifierTrait;
use crate::terrain::planes::PlaneData;
use crate::terrain::utils::AABBs;

#[derive(Clone)]
pub struct Noise {
    pub noise_function: NoiseFunction,
    pub noise_data:     NoiseData,
}
impl Noise {
    pub fn from_noise_data(nd: &NoiseData) -> Self {
        Noise {noise_function: nd.bake(), 
               noise_data: nd.clone()
            }
    }

    pub fn aabbs(pd: &PlaneData) -> AABBs {
        let mut aabbs = AABBs::new();
        aabbs.0.push(pd.get_aabb());
        return aabbs;
    }
}

impl ModifierTrait for Noise {
    fn apply(&self, pos: &[f32; 3], aabbs: &AABBs, loc: &[f32; 3]) -> f32 {
        if aabbs.has_point(pos) {

            let mut g_pos = *pos;
            if self.noise_data.global {
                g_pos = [pos[0] + loc[0], pos[1] + loc[1], pos[2]+loc[2]];
            }

            return self.noise_function.apply(&g_pos, &self.noise_data);
        }
        return pos[1];
    }
}




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

impl NoiseData {

    pub fn bake(&self) -> NoiseFunction {

        match self.noise {
            Noises::Perlin =>        {NoiseFunction::Perlin(Perlin::new(self.seed))}
            Noises::PerlinSurflet => {NoiseFunction::PerlinSurflet(PerlinSurflet::new(self.seed))}
            Noises::Value =>         {NoiseFunction::Value(Value::new(self.seed))}
            Noises::OpenSimplex =>   {NoiseFunction::OpenSimplex(OpenSimplex::new(self.seed))}
            Noises::SuperSimplex =>  {NoiseFunction::SuperSimplex(SuperSimplex::new(self.seed))}
            Noises::Worley =>        {NoiseFunction::Worley(Worley::new(self.seed))}
            Noises::Simplex =>       {NoiseFunction::Simplex(Simplex::new(self.seed))}
            Noises::FBMPerlin =>     {
                let mut noise_fn: Fbm<Perlin> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::FBMPerlin(noise_fn)
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BMPerlin(noise_fn)
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BPerlin(noise_fn)
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::RMPerlin(noise_fn)
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::HMPerlin(noise_fn)
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::FBMPerlinSurflet(noise_fn)
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BMPerlinSurflet(noise_fn)
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BPerlinSurflet(noise_fn)
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::RMPerlinSurflet(noise_fn)
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::HMPerlinSurflet(noise_fn)
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::FBMValue(noise_fn)
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BMValue(noise_fn)
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BValue(noise_fn)
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::RMValue(noise_fn)
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::HMValue(noise_fn)
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::FBMSS(noise_fn)
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BMSS(noise_fn)
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::BSS(noise_fn)
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::RMSS(noise_fn)
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(self.seed);
                if let Some(octaves) = self.octaves {
                    noise_fn.octaves = octaves;
                }
                if let Some(freq) = self.freq {
                    noise_fn.frequency = freq
                }
                return NoiseFunction::HMSS(noise_fn)
            }
        }
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

impl NoiseFunction {
    pub fn apply(&self, pos: &[f32; 3], nd: &NoiseData) -> f32 {

        let r: f64;
        match self {
            // XD but I really dont know how to make it better
            NoiseFunction::Perlin(f)                     => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::PerlinSurflet(f)              => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::Value(f)                      => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::OpenSimplex(f)                => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::SuperSimplex(f)               => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::Worley(f)                     => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::Simplex(f)                    => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::FBMPerlin(f)                  => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BMPerlin(f)                   => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BPerlin(f)                    => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::RMPerlin(f)                   => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::HMPerlin(f)                   => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::FBMPerlinSurflet(f)           => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BPerlinSurflet(f)             => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::RMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::HMPerlinSurflet(f)            => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::FBMValue(f)                   => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BMValue(f)                    => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BValue(f)                     => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::RMValue(f)                    => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::HMValue(f)                    => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::FBMSS(f)                      => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BMSS(f)                       => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::BSS(f)                        => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::RMSS(f)                       => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
            NoiseFunction::HMSS(f)                       => {r = f.get([pos[0] as f64 * nd.scale, pos[2] as f64 * nd.scale])}
        }
        return r as f32 * nd.height_scale;    
    }

}

