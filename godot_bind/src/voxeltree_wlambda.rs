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
struct VoxelPainter {
    volumes: std::vec::Vec<Vol<FColor>>,
    masks:   std::vec::Vec<Mask>,
}

impl VoxelPainter {
    fn new() -> Self {
        Self {
            volumes: vec![],
            masks: vec![],
        }
    }

    fn clear(&mut self) {
        self.volumes.clear();
        self.masks.clear();
    }

    fn new_vol(&mut self, size: usize, def: f64) -> i64 {
        self.volumes.push(Vol::new_default(size, def.into()));
        (self.volumes.len() - 1) as i64
    }
}

pub fn new_voxel_painter() -> VVal {
    let o = VVal::map();

    let painter = Rc::new(RefCell::new(VoxelPainter::new()));

    set_vval_method!(o, painter, new, Some(2), Some(2), env, _argc, {
        Ok(VVal::Int(painter.borrow_mut().new_vol(
            env.arg(0).i() as usize,
            env.arg(1).f())))
    });

    o
}
