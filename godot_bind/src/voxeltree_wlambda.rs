use crate::voxeltree::*;
use wlambda::vval::*;
use std::rc::Rc;
use std::cell::RefCell;
//use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};
use wlambda::set_vval_method;
use wlambda::util::{Sampled3DNoise};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct FColor(f64);

impl From<u8> for FColor {
    fn from(i: u8) -> Self {
        Self(i as f64 / 255.0)
    }
}

impl Into<u8> for FColor {
    fn into(self) -> u8 {
        (self.0.min(1.0).max(0.0) * 255.0).round() as u8
    }
}

//impl Into<FColor> for u8 {
//    fn into(self) -> FColor {
//        FColor((self as f64) / 255.0)
//    }
//}

impl Into<Voxel<u8>> for FColor {
    fn into(self) -> Voxel<u8> {
        Voxel {
            color: (self.0.min(1.0).max(0.0) * 255.0) as u8,
            faces: 0,
        }
    }
}

impl Into<f64> for Voxel<FColor> {
    fn into(self) -> f64 {
        self.color.0
    }
}

impl Into<Voxel<FColor>> for f64 {
    fn into(self) -> Voxel<FColor> {
        Voxel {
            color: FColor(self),
            faces: 0,
        }
    }
}


#[derive(Clone, Debug)]
enum Mask {
    Value { a: f64, b: f64 },
    Area  { x: u16, y: u16, z: u16, w: u16, h: u16, d: u16 },
    Volume{ vol_idx: usize, a: f64, b: f64 },
}

#[derive(Clone, Debug)]
pub struct VoxelPainter {
    volumes: std::vec::Vec<Vol<FColor>>,
    masks:   std::vec::Vec<Mask>,
}

pub struct Rect {
    x: u16,
    y: u16,
    z: u16,
    w: u16,
    h: u16,
    d: u16,
}

impl Rect {
    pub fn from_wlambda_env(env: &Env, offs: usize) -> Self {
        Self {
            x: env.arg(offs).i()     as u16,
            y: env.arg(offs + 1).i() as u16,
            z: env.arg(offs + 2).i() as u16,
            w: env.arg(offs + 3).i() as u16,
            h: env.arg(offs + 4).i() as u16,
            d: env.arg(offs + 5).i() as u16,
        }
    }
    pub fn from_usize(x: usize, y: usize, z: usize, w: usize, h: usize, d: usize) -> Self {
        Self {
            x: x as u16,
            y: y as u16,
            z: z as u16,
            w: w as u16,
            h: h as u16,
            d: d as u16,
        }
    }

    pub fn pos(&self) -> Pos {
        Pos { x: self.x, y: self.y, z: self.z }
    }
}

impl VoxelPainter {
    pub fn new() -> Self {
        Self {
            volumes: vec![],
            masks: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.volumes.clear();
        self.masks.clear();
    }

    pub fn write_into_u8_vol(&self, vol_id: usize, vol: &mut Vol<u8>) {
        if self.volumes.len() == 0 { return; }

        for z in 0..vol.size {
            for y in 0..vol.size {
                for x in 0..vol.size {
                    vol.set(
                        x as u16, y as u16, z as u16,
                        self.volumes[vol_id]
                        .at(Pos { x: x as u16, y: y as u16, z: z as u16 })
                        .color.into());
                }
            }
        }
    }

    pub fn sample_3dnoise_octaves(
        &mut self, vol_id: usize, mask: usize, rect: Rect,
        seed: i64, octaves: usize, factor: f64, persistence: f64)
    {
        let vol = &mut self.volumes[vol_id];
        let n = Sampled3DNoise::new(vol.size, seed);

        for z in 0..vol.size {
            for y in 0..vol.size {
                for x in 0..vol.size {
                    let mut val =
                        n.at_octaved(
                            x as f64, y as f64, z as f64,
                            octaves, factor, persistence);
                    if val < 0.1 { val = 0.0; }
                    vol.set(x as u16, y as u16, z as u16, val.into());
                }
            }
        }
    }

    pub fn fill(&mut self, vol_id: usize, mask: usize,
                rect: Rect, val: f64)
    {
        self.volumes[vol_id].fill(
            rect.x, rect.y, rect.z,
            rect.w, rect.h, rect.d, val.into());
    }

    pub fn new_vol(&mut self, size: usize, def: f64) -> i64 {
        self.volumes.push(Vol::new_default(size, def.into()));
        (self.volumes.len() - 1) as i64
    }
}

pub fn new_voxel_painter(id: usize) -> (Rc<RefCell<VoxelPainter>>, VVal) {
    let o = VVal::map();

    let painter = Rc::new(RefCell::new(VoxelPainter::new()));

    set_vval_method!(o, painter, new, Some(2), Some(2), env, _argc, {
        println!("NEW VOL!");
        Ok(VVal::Int(painter.borrow_mut().new_vol(
            env.arg(0).i() as usize,
            env.arg(1).f())))
    });

    set_vval_method!(o, painter, sample_3dnoise_octaves, Some(12), Some(12), env, _argc, {
        painter.borrow_mut().sample_3dnoise_octaves(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            Rect::from_wlambda_env(env, 2),
            env.arg(8).i(), // seed
            env.arg(9).i() as usize, // octaves
            env.arg(10).f(), // factor
            env.arg(11).f()); // persistence

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, fill, Some(9), Some(9), env, _argc, {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        painter.borrow_mut().fill(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            Rect::from_wlambda_env(env, 2),
            env.arg(8).f(),
        );

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, clear, Some(0), Some(0), env, _argc, {
        painter.borrow_mut().clear();
        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, id, Some(0), Some(0), env, _argc, {
        Ok(VVal::Int(id as i64))
    });

    (painter, o)
}
