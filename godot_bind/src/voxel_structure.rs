use crate::state::SSCG;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::voxeltree::*;
use crate::gd_voxel_impl::*;
use crate::util::WorkerPool;
use wlambda::VVal;

use std::sync::RwLock;
use std::sync::Arc;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxStruct {
    meshes:           std::vec::Vec<Ref<MeshInstance, Shared>>,
    collision_shapes: std::vec::Vec<(Ref<StaticBody, Shared>, i64)>,
    vol:              Vol<u8>,
    vol_generation:   usize,
    octrees:          std::vec::Vec<Arc<RwLock<Octree<u8>>>>,
    color_map:        ColorMap,

    cursor:           [u16; 3],
    workers:          WorkerPool<VoxRendJob,VoxRendResult>,
    last_load_vol:    std::time::Instant,
}

unsafe impl Send for VoxStruct { }

const VOL_SIZE    : usize = 128;
const SUBVOL_SIZE : usize = 16;
const SUBVOLS     : usize = VOL_SIZE / SUBVOL_SIZE;

fn vval2colors(clr: VVal) -> ColorMap {
    let mut colors = [[0.0; 3]; 256];
    use crate::gui::wlambda_api::color_hex24tpl;
    for (i, (c, _)) in clr.iter().enumerate() {
        c.with_s_ref(|s| {
            let tpl = color_hex24tpl(s);
            colors[i] = [
                tpl.0 as f32 / 255.0,
                tpl.1 as f32 / 255.0,
                tpl.2 as f32 / 255.0,
            ];
        })
    }
    ColorMap::new_from(colors)
}

struct VoxRendJob {
    vol_generation: usize,
    color_map: ColorMap,
    oct_subtree_idx: usize,
    oct_subtree: Arc<RwLock<Octree<u8>>>,
}

unsafe impl Send for VoxRendJob { }

impl VoxRendJob {
    pub fn render(&mut self) -> VoxRendResult {
//        if self.vol_generation < vg {
//            return VoxRendResult {
//                vol_generation:  self.vol_generation,
//                oct_subtree_idx: self.oct_subtree_idx,
//                empty: true,
//                arrs: None,
//            };
//        }

        let n = self.oct_subtree.write().unwrap().recompute();

        let arr =
            if !n.empty {
                let cm = self.color_map;
                let oct_guard = self.oct_subtree.read().unwrap();
                let arr = render_octree_to_am(&cm, &*oct_guard);
                Some(arr)
            } else {
                None
            };

        VoxRendResult {
            vol_generation:  self.vol_generation,
            oct_subtree_idx: self.oct_subtree_idx,
            empty: n.empty,
            arrs: arr,
        }
    }
}

#[allow(dead_code)]
struct VoxRendResult {
    vol_generation:  usize,
    oct_subtree_idx: usize,
    arrs:            Option<RenderedMeshArrays>,
    empty:           bool,
}

unsafe impl Send for VoxRendResult { }

#[methods]
impl VoxStruct {
    fn new(_owner: &Spatial) -> Self {
        Self {
            meshes:           vec![],
            collision_shapes: vec![],
            vol:              Vol::new(VOL_SIZE),
            vol_generation:   0,
            octrees:          vec![],
            color_map:        ColorMap::new_gray(),
            cursor:           [0, 0, 0],
            last_load_vol:    std::time::Instant::now(),
            workers:          WorkerPool::new(|mut j: VoxRendJob| {
                j.render()
            }, 8),
        }
    }

    #[export]
    fn on_wlambda_init(&mut self, owner: &Spatial) {
        let d = std::time::Instant::now();
        let (sysid, entid) = self.parent_info(owner);
        lock_sscg!(sscg);
        let ret = sscg.call_cb("on_draw_voxel_structure", &vec![sysid, entid]);
        if !ret.is_none() {
            sscg.vox_painters
                .borrow()[ret.v_i(0) as usize]
                .borrow()
                .write_into_u8_vol(ret.v_i(1) as usize, &mut self.vol);

            if ret.v_(2).is_str() {
                self.color_map =
                    match &ret.v_s_raw(2)[..] {
                        "8bit" => ColorMap::new_8bit(),
                        _      => ColorMap::new_gray(),
                    };
            } else if !ret.v_(2).is_none() {
                self.color_map = vval2colors(ret.v_(2));
            }

            println!("Drawing voxel volume, took {} ms", d.elapsed().as_millis());
            self.load_vol(owner);
            println!("Reloaded voxel volume, took {} ms", d.elapsed().as_millis());
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Spatial) {
        //use wlambda::util::*;

        let voxel_material =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://scenes/entities/materials/voxel_material.tres"),
                GodotString::from_str("ShaderMaterial"),
                false).unwrap().cast::<Material>().unwrap();

        let mut i = 0;
        for z in 0..SUBVOLS {
            for y in 0..SUBVOLS {
                for x in 0..SUBVOLS {
                    self.meshes.push(MeshInstance::new().into_shared());


                    let mesh = unsafe {
                        self.meshes[i].assume_safe()
                    };

                    mesh.set_material_override(voxel_material.clone());
                    let mut t = mesh.transform();
                    t.origin.x = (x * SUBVOL_SIZE) as f32;
                    t.origin.y = (y * SUBVOL_SIZE) as f32;
                    t.origin.z = (z * SUBVOL_SIZE) as f32;
                    mesh.set_transform(t);
                    mesh.set_layer_mask_bit(0, false);
                    mesh.set_layer_mask_bit(1, true);

                    let sb = StaticBody::new();
                    sb.set_transform(t);

                    owner.add_child(mesh, false);

                    let sb     = sb.into_shared();
                    let sbref  = unsafe { sb.assume_safe() };
                    let id = sbref.create_shape_owner(sbref);
                    owner.add_child(sbref, false);
                    self.collision_shapes.push((sb, id));

                    self.octrees.push(
                        std::sync::Arc::new(
                            std::sync::RwLock::new(
                                Octree::new_from_size(SUBVOL_SIZE))));

                    i += 1;
                }
            }
        }
        println!("initialized godot objects");

//        self.load_vol(owner);
//        println!("loaded godot objects");
    }

    #[allow(dead_code)]
    fn serialize_vol(&mut self) -> Vec<u8> {
        for z in 0..VOL_SIZE {
            let iz  = z / SUBVOL_SIZE;
            let izi = z % SUBVOL_SIZE;

            for y in 0..VOL_SIZE {
                let iy  = y / SUBVOL_SIZE;
                let iyi = y % SUBVOL_SIZE;

                for x in 0..VOL_SIZE {
                    let ix  = x / SUBVOL_SIZE;
                    let ixi = x % SUBVOL_SIZE;

                    self.vol.set(x as u16, y as u16, z as u16,
                        self.octrees[
                              iz * (SUBVOLS * SUBVOLS)
                            + iy * SUBVOLS
                            + ix].read().unwrap().get(
                                ixi as u16,
                                iyi as u16,
                                izi as u16));
                }
            }
        }

        self.vol.serialize()
    }

    #[export]
    fn load_vol(&mut self, mut _owner: &Spatial) {
        self.last_load_vol = std::time::Instant::now();

        for z in 0..SUBVOLS {
            for y in 0..SUBVOLS {
                for x in 0..SUBVOLS {
                    let mut ot = self.octrees[
                          z * (SUBVOLS * SUBVOLS)
                        + (SUBVOLS - (y + 1)) * SUBVOLS
                        + x].write().unwrap();

                    for sz in 0..SUBVOL_SIZE {
                        for sy in 0..SUBVOL_SIZE {
                            for sx in 0..SUBVOL_SIZE {
                                ot.set(sx as u16, sy as u16, sz as u16,
                                       *self.vol.at(Pos {
                                           x: x as u16 * SUBVOL_SIZE as u16 + sx as u16,
                                           y: y as u16 * SUBVOL_SIZE as u16 + sy as u16,
                                           z: z as u16 * SUBVOL_SIZE as u16 + sz as u16,
                                       }));
                            }
                        }
                    }
                }
            }
        }
//        for z in 0..VOL_SIZE {
//            let iz  = z / SUBVOL_SIZE;
//            let izi = z % SUBVOL_SIZE;
//
//            for y in 0..VOL_SIZE {
//                let iy  = y / SUBVOL_SIZE;
//                let iyi = (SUBVOL_SIZE - 1) - (y % SUBVOL_SIZE);
//
//                for x in 0..VOL_SIZE {
//                    let ix  = x / SUBVOL_SIZE;
//                    let ixi = x % SUBVOL_SIZE;
//
//                    self.octrees[
//                          iz * (SUBVOLS * SUBVOLS)
//                        + iy * SUBVOLS
//                        + ix].write().unwrap().set(
//                            ixi as u16,
//                            iyi as u16,
//                            izi as u16,
//                            *self.vol.at(Pos {
//                              x: x as u16,
//                              y: y as u16,
//                              z: z as u16
//                            }));
//                }
//            }
//        }
        println!("Copy To sub octrees took {}ms",
                 self.last_load_vol.elapsed().as_millis());

        self.inc_vol_generation();
        for z in 0..SUBVOLS {
            for y in 0..SUBVOLS {
                for x in 0..SUBVOLS {
                    self.reload_at(
                        x * SUBVOL_SIZE,
                        y * SUBVOL_SIZE,
                        z * SUBVOL_SIZE);
                }
            }
        }

        println!("Issue reload jobs took {}ms",
                 self.last_load_vol.elapsed().as_millis());
    }

    fn inc_vol_generation(&mut self) {
        self.vol_generation = self.vol_generation.wrapping_add(1);

//        let new_vol_gen = (*self.vol_generation.get_mut()).wrapping_add(1);
//        *self.vol_generation.get_mut() = new_vol_gen;
    }

    fn get_octree_at(&mut self, x: usize, y: usize, z: usize) -> (&std::sync::Arc<std::sync::RwLock<Octree<u8>>>, [u16; 3]) {
        let iz  = z / SUBVOL_SIZE;
        let izi = z % SUBVOL_SIZE;

        let iy  = y / SUBVOL_SIZE;
        let iyi = y % SUBVOL_SIZE;

        let ix  = x / SUBVOL_SIZE;
        let ixi = x % SUBVOL_SIZE;

        let sub_idx =
              iz * (SUBVOLS * SUBVOLS)
            + iy * SUBVOLS
            + ix;

        (&self.octrees[sub_idx], [ixi as u16, iyi as u16, izi as u16])
    }

    #[export]
    fn mine_info_at_cursor(&mut self, mut _owner: &Spatial) -> Variant {
        let (ot, pos) =
            self.get_octree_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);
        let v = ot.read().unwrap().get_inv_y(pos[0], pos[1], pos[2]);
        let dict = Dictionary::new();
        dict.insert(&Variant::from_str("material"), &Variant::from_i64(v.color as i64));
        dict.insert(&Variant::from_str("time"),     &Variant::from_f64(1.2));
        dict.insert(&Variant::from_str("x"),        &Variant::from_i64(self.cursor[0] as i64));
        dict.insert(&Variant::from_str("y"),        &Variant::from_i64(self.cursor[1] as i64));
        dict.insert(&Variant::from_str("z"),        &Variant::from_i64(self.cursor[2] as i64));
        Variant::from_dictionary(&dict.into_shared())
    }

    #[export]
    fn looking_at(&mut self, owner: &Spatial, x: f64, y: f64, z: f64) -> bool {
        let c = unsafe {
            owner.get_child(0)
                 .map(|n| n.assume_safe())
                 .and_then(|n| n.cast::<Spatial>())
                 .unwrap()
        };
        let c2 = unsafe {
            owner.get_child(1)
                 .map(|n| n.assume_safe())
                 .and_then(|n| n.cast::<Spatial>())
                 .unwrap()
        };

        let mut t = c.transform();
        t.origin.x = x.floor() as f32 + 0.5;
        t.origin.y = y.floor() as f32 + 0.5;
        t.origin.z = z.floor() as f32 + 0.5;
        c.set_transform(t);
        c2.set_transform(t);

        self.cursor = [
            t.origin.x as u16,
            t.origin.y as u16,
            t.origin.z as u16,
        ];

        let (ot, pos) =
            self.get_octree_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);
        let v = ot.read().unwrap().get_inv_y(pos[0], pos[1], pos[2]);
        v.color != 0

//            {
//                let (ot, pos) =
//                    self.get_octree_at(
//                        self.cursor[0] as usize,
//                        self.cursor[1] as usize,
//                        self.cursor[2] as usize);
//                let v = ot.get_inv_y(pos[0], pos[1], pos[2]);
//                if v.color == 0 {
//                    c.hide();
//                } else {
//                    c.show();
//                }
//            }
    }

    #[export]
    fn set_marker_status(&mut self, owner: &Spatial, show: bool, mining: bool) {
        let looking_cursor = unsafe {
            owner.get_child(0)
                 .map(|n| n.assume_safe())
                 .and_then(|n| n.cast::<Spatial>())
                 .unwrap()
        };
        let mining_cursor = unsafe {
            owner.get_child(1)
                 .map(|n| n.assume_safe())
                 .and_then(|n| n.cast::<Spatial>())
                 .unwrap()
        };

        if show {
            if mining {
                looking_cursor.hide();
                mining_cursor.show();
            } else {
                looking_cursor.show();
                mining_cursor.hide();
            }
        } else {
            looking_cursor.hide();
            mining_cursor.hide();
        }
    }

    fn parent_info(&self, owner: &Spatial) -> (VVal, VVal) {
        unsafe {
            let sysid =
                owner.get_parent().unwrap().assume_safe().get(
                    GodotString::from_str("system_id")).to_i64();
            let entid =
                owner.get_parent().unwrap().assume_safe().get(
                    GodotString::from_str("entity_id")).to_i64();
            (VVal::Int(sysid), VVal::Int(entid))
        }
    }

    #[export]
    fn spawn_mine_pop_at_cursor(&mut self, owner: &Spatial, color: u8) {
        use palette::Hsv;
        use palette::rgb::Rgb;
        let part = unsafe {
            owner.get_child(2)
                 .map(|n| n.assume_safe())
                 .and_then(|n| n.cast::<Particles>())
                 .unwrap()
        };

        let mat_ovr = part.material_override().unwrap();
        let m = unsafe {
            mat_ovr.assume_safe().cast::<SpatialMaterial>().unwrap()
        };
        let cm = self.color_map;
        let mut clr = cm.map(color);
        let mut fx_color : Rgb = Rgb::new(clr.r, clr.g, clr.b);
        let mut fx_color_hsv : Hsv = fx_color.into();
        fx_color_hsv.saturation = 1.0;
        fx_color_hsv.value = 0.7;
        fx_color = fx_color_hsv.into();
        clr.r = fx_color.red;
        clr.g = fx_color.green;
        clr.b = fx_color.blue;
        m.set_albedo(clr);
        m.set_emission(clr);
        part.set_material_override(m);

        let mut t = part.transform();
        t.origin.x = self.cursor[0] as f32 + 0.5;
        t.origin.y = self.cursor[1] as f32 + 0.5;
        t.origin.z = self.cursor[2] as f32 + 0.5;
        part.set_transform(t);

        part.show();
        part.set_one_shot(true);
        part.set_emitting(true);
        part.restart();
    }

    #[export]
    fn mine_status(&mut self, owner: &Spatial, started: bool) -> bool {
        let (ot, pos) =
            self.get_octree_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);
        let m = ot.read().unwrap().get_inv_y(pos[0], pos[1], pos[2]);

        let (sysid, entid) = self.parent_info(owner);
        lock_sscg!(sscg);
        let ret = sscg.call_cb(
            "on_mine",
            &vec![sysid, entid,
                  VVal::Bol(started),
                  VVal::Int(m.color as i64),
                  VVal::Int(self.cursor[0] as i64),
                  VVal::Int(self.cursor[1] as i64),
                  VVal::Int(self.cursor[2] as i64),
                  ]);

        ret.b()
    }

    #[export]
    fn mine_at_cursor(&mut self, owner: &Spatial) -> bool {
        // Prevent any change of the volume while it's being rerendered.
        if self.workers.queued_job_count() > 0 {
            return false;
        }

        let (ot, pos) =
            self.get_octree_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);
        let m = ot.read().unwrap().get_inv_y(pos[0], pos[1], pos[2]);

        if m.color != 0 {
            ot.write().unwrap().set_inv_y(pos[0], pos[1], pos[2], 0.into());
            self.inc_vol_generation();
            self.reload_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);

            lock_sscg!(sscg);
            let (sysid, entid) = self.parent_info(owner);
            sscg.call_cb(
                "on_mined_voxel",
                &vec![sysid, entid,
                      VVal::Int(m.color as i64),
                      VVal::Int(self.cursor[0] as i64),
                      VVal::Int(self.cursor[1] as i64),
                      VVal::Int(self.cursor[2] as i64),
                      ]);

            self.spawn_mine_pop_at_cursor(owner, m.color);

            true
        } else {
            false
        }
    }

    #[export]
    fn looking_at_nothing(&mut self, owner: &Spatial) {
        self.set_marker_status(owner, false, false);
    }

    #[export]
    fn _process(&mut self, mut _owner: &Spatial, _delta: f64) {
        self.wait_for_mesh_rendering();
    }

    fn wait_for_mesh_rendering(&mut self) {
        let cur_vol_gen = self.vol_generation;
        let mut max = 5;
        while let Some(VoxRendResult {
                            arrs, oct_subtree_idx,
                            empty: _, vol_generation })
            = self.workers.get_result()
        {

            if vol_generation < cur_vol_gen {
                continue;
            }

            let _d = std::time::Instant::now();
            let (static_body, shape_owner_idx) =
                self.collision_shapes[oct_subtree_idx];
            let static_body = unsafe { static_body.assume_safe() };
            let am = ArrayMesh::new();
            let cvshape = ConcavePolygonShape::new();

            let ssb = static_body.cast::<StaticBody>().unwrap();
            ssb.shape_owner_clear_shapes(shape_owner_idx);

            let mesh = unsafe { self.meshes[oct_subtree_idx].assume_safe() };

            if let Some(rend_arrs) = arrs {
                rend_arrs.write_to(am.as_ref(), cvshape.as_ref());

                if cvshape.faces().len() > 0 {
                    ssb.shape_owner_add_shape(shape_owner_idx, cvshape);
                }
                mesh.set_mesh(am);
                mesh.show();
                static_body.show();
            } else {
                mesh.set_mesh(am);
                mesh.hide();
                static_body.hide();
            }

            if self.workers.queued_job_count() == 0 {
                println!("Workers done after {}ms", self.last_load_vol.elapsed().as_millis());
                return;
            }

            max -= 1;
            if max == 0 { return; }
        }
    }

    fn reload_at(&mut self, x: usize, y: usize, z: usize) {
        let iz  = z / SUBVOL_SIZE;
        let iy  = y / SUBVOL_SIZE;
        let ix  = x / SUBVOL_SIZE;

        let sub_idx =
              iz * (SUBVOLS * SUBVOLS)
            + iy * SUBVOLS
            + ix;

          self.workers.send(VoxRendJob {
              vol_generation:  self.vol_generation,
              color_map:       self.color_map,
              oct_subtree_idx: sub_idx,
              oct_subtree:     self.octrees[sub_idx].clone()
          });
    }
}
