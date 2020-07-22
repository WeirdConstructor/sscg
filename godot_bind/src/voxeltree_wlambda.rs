use crate::voxeltree::*;
use wlambda::vval::*;
use std::rc::Rc;
use std::cell::RefCell;
//use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};
use wlambda::set_vval_method;
use crate::util::Sampled3DNoise;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct FColor(f64);

impl From<u8> for FColor {
    fn from(i: u8) -> Self {
        Self(i as f64 / 255.0)
    }
}

impl Into<u8> for FColor {
    fn into(self) -> u8 {
        (self.0.min(1.0).max(0.0) * 255.0).floor() as u8
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

impl Into<f64> for FColor {
    fn into(self) -> f64 {
        self.0
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

#[derive(Debug, Clone, PartialEq)]
pub enum DrawOp {
    UseSrc,                     // output = src
    UseDst,                     // output = dst
    AddSrcDst,                  // output = src + dst
    SubDstSrc,                  // output = dst - src
    SubSrcDst,                  // output = src - dst
    MulSrcDst,                  // output = src * dst
    Mul(f64),                   // output = src * v
    Add(f64),                   // output = src + v
    Value(f64),                 // output = v
    Clamp(f64, f64),            // output = clamp(src, a, b)
    Map(f64, f64, f64, f64),    // output = map(src, a1, b1, a2, b2) or src if outside a1/b1
    MaskMap(f64, f64, f64, f64),    // output = map(src, a1, b1, a2, b2) or dst if outside a1/b1
    ClampedMap(f64, f64, f64, f64), // output = map(clamp(src, a1, b1), a1, b1, a2, b2)
    MultiRemap(std::vec::Vec<(f64, f64, f64, f64)>), // output = multimap
    Min(f64),                   // output = min(src, v)
    Max(f64),                   // output = max(src, v)
    Chain(Box<DrawOp>, Box<DrawOp>),// output = drawop2(drawop1(src, dst), dst)
}

impl DrawOp {
    pub fn from_vval(vv: VVal) -> Self {
        if vv.v_(0).is_vec() {
            return
                DrawOp::Chain(
                    Box::new(Self::from_vval(vv.v_(0))),
                    Box::new(Self::from_vval(vv.v_(1))));
        }

        match &vv.v_s_raw(0)[..] {
            "use_src"       => DrawOp::UseSrc,
            "use_dst"       => DrawOp::UseDst,
            "mul"           => DrawOp::Mul(vv.v_f(1)),
            "add"           => DrawOp::Add(vv.v_f(1)),
            "add_src_dst"   => DrawOp::AddSrcDst,
            "sub_dst_src"   => DrawOp::SubDstSrc,
            "sub_src_dst"   => DrawOp::SubSrcDst,
            "mul_src_dst"   => DrawOp::MulSrcDst,
            "value"         => DrawOp::Value(vv.v_f(1)),
            "clamp"         => DrawOp::Clamp(vv.v_f(1), vv.v_f(2)),
            "multi_remap"   => {
                let mut v = vec![];
                let mut i = 1;
                while i < vv.len() {
                    v.push((vv.v_f(i), vv.v_f(i + 1), vv.v_f(i + 2), vv.v_f(i + 3)));
                    i += 4;
                }
                DrawOp::MultiRemap(v)
            },
            "map"           => DrawOp::Map(
                                   vv.v_f(1), vv.v_f(2),
                                   vv.v_f(3), vv.v_f(4)),
            "mask_map"      => DrawOp::MaskMap(
                                   vv.v_f(1), vv.v_f(2),
                                   vv.v_f(3), vv.v_f(4)),
            "clamped_map"   => DrawOp::ClampedMap(
                                   vv.v_f(1), vv.v_f(2),
                                   vv.v_f(3), vv.v_f(4)),
            "min"           => DrawOp::Min(vv.v_f(1)),
            "max"           => DrawOp::Min(vv.v_f(1)),
            "chain"         => DrawOp::Chain(
                                   Box::new(Self::from_vval(vv.v_(1))),
                                   Box::new(Self::from_vval(vv.v_(2)))),
            _ => DrawOp::UseSrc,
        }
    }

    pub fn apply(&self, src: f64, dest: f64) -> f64 {
        match self {
            DrawOp::AddSrcDst => src + dest,
            DrawOp::SubDstSrc => dest - src,
            DrawOp::SubSrcDst => src - dest,
            DrawOp::MulSrcDst => src + dest,
            DrawOp::UseSrc    => src,
            DrawOp::UseDst    => dest,
            DrawOp::Add(v)   => *v + src,
            DrawOp::Mul(v)   => *v * src,
            DrawOp::Value(v) => *v,
            DrawOp::Clamp(a, b) => {
                if src < *a { return *a }
                if src > *b { return *b }
                src
            },
            DrawOp::Map(a1, b1, a2, b2) => {
                if src < *a1 { return src; }
                else if src > *b1 { return src; }

                let x = (*b1 - src) / (*b1 - *a1);
                *a2 * x + *b2 * (1.0 - x)
            },
            DrawOp::MaskMap(a1, b1, a2, b2) => {
                if src < *a1 { return dest; }
                else if src > *b1 { return dest; }

                let x = (*b1 - src) / (*b1 - *a1);
                *a2 * x + *b2 * (1.0 - x)
            },
            DrawOp::ClampedMap(a1, b1, a2, b2) => {
                let src = if src > *b1 { *b1 }
                          else if src < *a1 { *a1 }
                          else { src };

                let x = (*b1 - src) / (*b1 - *a1);
                *a2 * x + *b2 * (1.0 - x)
            },
            DrawOp::MultiRemap(v) => {
                for mapop in v {
                    if src >= mapop.0 && src < mapop.1 {
                        let x = (mapop.0 - src) / (mapop.1 - mapop.0);
                        return mapop.2 * x + mapop.3 * (1.0 - x);
                    }
                }
                src
            },
            DrawOp::Min(min) => {
                if src < *min { src }
                else { *min }
            },
            DrawOp::Max(max) => {
                if src > *max { src }
                else { *max }
            },
            DrawOp::Chain(op_a, op_b) => {
                let next_src_val = op_a.apply(src, dest);
                op_b.apply(next_src_val, dest)
            }
        }
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

    pub fn fill_noise(
        &mut self, vol_id: usize, _mask: usize,
        rect: Rect,
        seed: i64, noise_size: usize, noise_scale: f64, op: DrawOp)
    {
        let vol = &mut self.volumes[vol_id];
        let n = Sampled3DNoise::new(noise_size, seed);

        for z in 0..rect.d {
            for y in 0..rect.h {
                for x in 0..rect.w {
                    let dst_val =
                        vol.color_at(Pos {
                            x: rect.x + x,
                            y: rect.y + y,
                            z: rect.z + z,
                        });
                    let src_val =
                        n.at(
                            noise_scale * x as f64 / (rect.w as f64),
                            noise_scale * y as f64 / (rect.h as f64),
                            noise_scale * z as f64 / (rect.d as f64));
                    vol.set(
                        rect.x + x as u16,
                        rect.y + y as u16,
                        rect.z + z as u16,
                        op.apply(src_val, (*dst_val).into()).into());
                }
            }
        }
    }

    pub fn sample_fbm(
        &mut self, vol_id: usize, _mask: usize,
        rect: Rect,
        seed: i64,
        noise_size: usize,
        noise_scale: f64,
        octaves: usize,
        lacunarity: f64,
        gain: f64,
        op: DrawOp)
    {
        let vol = &mut self.volumes[vol_id];
        let n = Sampled3DNoise::new(noise_size, seed);

        for z in 0..rect.d {
            for y in 0..rect.h {
                for x in 0..rect.w {
                    let dst_val = vol.color_at(Pos {
                        x: rect.x + x as u16,
                        y: rect.y + y as u16,
                        z: rect.z + z as u16,
                    });
                    let src_val =
                        n.at_fbm(
                            noise_scale * (x as f64) / (rect.w as f64),
                            noise_scale * (y as f64) / (rect.h as f64),
                            noise_scale * (z as f64) / (rect.d as f64),
                            octaves, lacunarity, gain);
                    vol.set(
                        rect.x + x as u16,
                        rect.y + y as u16,
                        rect.z + z as u16,
                        op.apply(src_val, (*dst_val).into()).into());
                }
            }
        }
    }

    pub fn fill(&mut self, vol_id: usize, _mask: usize,
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

#[allow(unused_variables)]
pub fn new_voxel_painter(id: usize) -> (Rc<RefCell<VoxelPainter>>, VVal) {
    let o = VVal::map();

    let painter = Rc::new(RefCell::new(VoxelPainter::new()));

    set_vval_method!(o, painter, new, Some(2), Some(2), env, _argc, {
        println!("NEW VOL!");
        Ok(VVal::Int(painter.borrow_mut().new_vol(
            env.arg(0).i() as usize,
            env.arg(1).f())))
    });

    set_vval_method!(o, painter, fill_noise, Some(11), Some(12), env, _argc, {
        painter.borrow_mut().fill_noise(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            Rect::from_wlambda_env(env, 2),
            env.arg(8).i(),
            env.arg(9).i() as usize, // seed
            env.arg(10).f(),
            DrawOp::from_vval(env.arg(11)));        // noise scale

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, sample_fbm, Some(14), Some(15), env, _argc, {
        painter.borrow_mut().sample_fbm(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            Rect::from_wlambda_env(env, 2),
            env.arg(8).i(),           // seed
            env.arg(9).i() as usize,  // noise size
            env.arg(10).f(),          // noise scale
            env.arg(11).i() as usize, // octaves
            env.arg(12).f(),          // lacunarity
            env.arg(13).f(),
            DrawOp::from_vval(env.arg(14)));         // gain

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, fill, Some(9), Some(9), env, _argc, {
        painter.borrow_mut().fill(
            env.arg(0).i() as usize,
            env.arg(1).i() as usize,
            Rect::from_wlambda_env(env, 2),
            env.arg(8).f(),
        );

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, clear, Some(0), Some(0), _env, _argc, {
        painter.borrow_mut().clear();
        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, painter, id, Some(0), Some(0), _env, _argc, {
        Ok(VVal::Int(id as i64))
    });

    (painter, o)
}
