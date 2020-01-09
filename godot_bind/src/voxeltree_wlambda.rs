use crate::voxeltree::*;
use wlambda::vval::*;
use std::rc::Rc;
use std::cell::RefCell;
//use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};
use wlambda::set_vval_method;

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

    pub fn fill(&mut self, vol_id: usize, mask: usize,
                x: u16, y: u16, z: u16, w: u16, h: u16, d: u16,
                val: f64)
    {
        self.volumes[vol_id].fill(x, y, z, w, h, d, val.into());
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

    set_vval_method!(o, painter, fill, Some(9), Some(9), env, _argc, {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        painter.borrow_mut().fill(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            env.arg(2).i() as u16,
            env.arg(3).i() as u16,
            env.arg(4).i() as u16,
            env.arg(5).i() as u16,
            env.arg(6).i() as u16,
            env.arg(7).i() as u16,
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
