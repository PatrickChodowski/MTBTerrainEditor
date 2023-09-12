
use bevy::prelude::Resource;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley, Fbm, Billow, BasicMulti, RidgedMulti, HybridMulti};
use serde::{Serialize, Deserialize};
use std::slice::Iter;
use super::easings::Easings;
use bevy_egui::{egui, egui::Ui};
use crate::editor::mtb_ui::ModResources;
use bevy::prelude::ResMut;


#[derive(Clone, Resource, Debug, Serialize, Deserialize)]
pub struct Noise {
    pub noise:          Noises,
    pub seed:           u32,
    pub scale:          f64,
    pub octaves:        usize,
    pub freq:           f64,
    pub easing:         Easings,
    pub global:         bool,
    pub reset:          bool,
    pub reset_value:    f32
}
impl Noise {
    pub fn new() -> Self {
        Noise { 
                noise:        Noises::Perlin, 
                seed:         0, 
                scale:        0.01, 
                octaves:      6, 
                freq:         1.0,
                easing:       Easings::None, 
                global:       false,
                reset:        false,
                reset_value:  10.0
            }

    }
}

impl Noise {
    pub fn set(&self) -> NoiseFunction {
        let nfn = NoiseFunction::new(self.noise.clone(), self.seed, self.octaves, self.freq);
        return nfn;
    }
    pub fn apply(&self, noise_fn: &NoiseFunction, pos: &[f32; 3], loc: &[f32; 3]) -> f32 {
        let mut gpos: [f32; 3] = *pos;
        if self.global {
            gpos[0] = pos[0] + loc[0];
            gpos[1] = pos[1] + loc[1];
            gpos[2] = pos[2] + loc[2];
        }

        if self.reset {
            gpos[1] = self.reset_value;
        }

        let r: f64 = noise_fn.apply(self.scale, gpos[0] as f64, gpos[2] as f64);
        let eased_r = self.easing.apply(r as f32);
        return eased_r * gpos[1];    
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {

        egui::ComboBox::from_label("Noise")
        .width(140.0)
        .selected_text(format!("{:?}", mod_res.noise.noise))
        .show_ui(ui, |ui| {
          for &p in Noises::iterator(){
            ui.selectable_value(&mut mod_res.noise.noise, p, format!("{p:?}"));
          }
        });

        ui.separator();

        ui.columns(2, |columns| {
          columns[1].label("Seed");
          columns[0].add(egui::DragValue::new(&mut mod_res.noise.seed).speed(1.0));
          columns[1].label("Scale");
          columns[0].add(egui::DragValue::new(&mut mod_res.noise.scale).speed(0.0001));
          columns[1].label("Frequency");
          columns[0].add(egui::DragValue::new(&mut mod_res.noise.freq).speed(0.1));
          columns[1].label("Octaves");
          columns[0].add(egui::DragValue::new(&mut mod_res.noise.octaves).speed(1.0));
        });

        egui::ComboBox::from_label("Easing")
        .width(140.0)
        .selected_text(format!("{:?}", mod_res.noise.easing))
        .show_ui(ui, |ui| {
          for &p in Easings::iterator(){
            ui.selectable_value(&mut mod_res.noise.easing, p, format!("{p:?}"));
          }
        });
        ui.checkbox(&mut mod_res.noise.global, "Use global position?");

        ui.checkbox(&mut mod_res.noise.reset, "Reset everytime?");
        if mod_res.noise.reset {
            ui.add(egui::DragValue::new(&mut mod_res.noise.reset_value).speed(1.0));
        }
    }
}





#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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


impl<'a> Noises {
      pub fn iterator() -> Iter<'static, Noises> {
    static NOISES_OPTIONS: [Noises; 27] = [
        Noises::Perlin,
        Noises::PerlinSurflet,
        Noises::OpenSimplex,
        Noises::Value,
        Noises::SuperSimplex,
        Noises::Worley,
        Noises::Simplex,
        Noises::FBMPerlin,
        Noises::BMPerlin,
        Noises::BPerlin,
        Noises::RMPerlin,
        Noises::HMPerlin,
        Noises::FBMPerlinSurflet,
        Noises::BMPerlinSurflet,
        Noises::BPerlinSurflet,
        Noises::RMPerlinSurflet,
        Noises::HMPerlinSurflet,
        Noises::FBMValue,
        Noises::BMValue,
        Noises::BValue,
        Noises::RMValue,
        Noises::HMValue,
        Noises::FBMSS,
        Noises::BMSS,
        Noises::BSS,
        Noises::RMSS,
        Noises::HMSS 
    ];
    NOISES_OPTIONS.iter()
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

 

    pub fn new(noise: Noises, seed: u32, octaves: usize, freq: f64) -> Self {
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
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn = NoiseFunction::FBMPerlin(noise_fn);
            }
            Noises::BMPerlin => {
                let mut noise_fn: BasicMulti<Perlin> = BasicMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn = NoiseFunction::BMPerlin(noise_fn);
            }
            Noises::BPerlin => {
                let mut noise_fn: Billow<Perlin> = Billow::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn = NoiseFunction::BPerlin(noise_fn);
            }
            Noises::RMPerlin => {
                let mut noise_fn: RidgedMulti<Perlin> = RidgedMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn = NoiseFunction::RMPerlin(noise_fn);
            }
            Noises::HMPerlin => {
                let mut noise_fn: HybridMulti<Perlin> = HybridMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::HMPerlin(noise_fn);
            }
            Noises::FBMPerlinSurflet =>     {
                let mut noise_fn: Fbm<PerlinSurflet> = Fbm::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::FBMPerlinSurflet(noise_fn);
            }
            Noises::BMPerlinSurflet => {
                let mut noise_fn: BasicMulti<PerlinSurflet> = BasicMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BMPerlinSurflet(noise_fn);
            }
            Noises::BPerlinSurflet => {
                let mut noise_fn: Billow<PerlinSurflet> = Billow::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BPerlinSurflet(noise_fn);
            }
            Noises::RMPerlinSurflet => {
                let mut noise_fn: RidgedMulti<PerlinSurflet> = RidgedMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::RMPerlinSurflet(noise_fn);
            }
            Noises::HMPerlinSurflet => {
                let mut noise_fn: HybridMulti<PerlinSurflet> = HybridMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::HMPerlinSurflet(noise_fn);
            }
            Noises::FBMValue =>     {
                let mut noise_fn: Fbm<Value> = Fbm::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::FBMValue(noise_fn);
            }
            Noises::BMValue => {
                let mut noise_fn: BasicMulti<Value> = BasicMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BMValue(noise_fn);
            }
            Noises::BValue => {
                let mut noise_fn: Billow<Value> = Billow::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BValue(noise_fn);
            }
            Noises::RMValue => {
                let mut noise_fn: RidgedMulti<Value> = RidgedMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::RMValue(noise_fn);
            }
            Noises::HMValue => {
                let mut noise_fn: HybridMulti<Value> = HybridMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::HMValue(noise_fn);
            }
            Noises::FBMSS =>     {
                let mut noise_fn: Fbm<SuperSimplex> = Fbm::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::FBMSS(noise_fn);
            }
            Noises::BMSS => {
                let mut noise_fn: BasicMulti<SuperSimplex> = BasicMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BMSS(noise_fn);
            }
            Noises::BSS => {
                let mut noise_fn: Billow<SuperSimplex> = Billow::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::BSS(noise_fn);
            }
            Noises::RMSS => {
                let mut noise_fn: RidgedMulti<SuperSimplex> = RidgedMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn =  NoiseFunction::RMSS(noise_fn);
            }
            Noises::HMSS => {
                let mut noise_fn: HybridMulti<SuperSimplex> = HybridMulti::new(seed);
                noise_fn.octaves = octaves;
                noise_fn.frequency = freq;
                nfn = NoiseFunction::HMSS(noise_fn);
            }
            
        }
        return nfn;
    }
}
