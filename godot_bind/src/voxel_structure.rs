use gdnative::*;
use euclid::{vec2, vec3};
use crate::voxeltree::*;
use crate::gd_voxel_impl::*;

#[derive(NativeClass)]
#[inherit(gdnative::Spatial)]
pub struct VoxStruct {
    meshes:           std::vec::Vec<MeshInstance>,
    collision_shapes: std::vec::Vec<(StaticBody, i64)>,
    vol:              Vol<u8>,
    octrees:          std::vec::Vec<Octree<u8>>,

    box_in_focus:   bool,
    sb_shape_owner: i64,

    cursor:         [u16; 3],
    last_mine_pos:  [u16; 3],
}

unsafe impl Send for VoxStruct { }

const VOL_SIZE    : usize = 128;
const SUBVOL_SIZE : usize = 16;
const SUBVOLS     : usize = VOL_SIZE / SUBVOL_SIZE;

#[methods]
impl VoxStruct {
    fn _init(owner: Spatial) -> Self {
        Self {
            meshes:           vec![],
            collision_shapes: vec![],
            vol:              Vol::new(VOL_SIZE),
            octrees:          vec![],
            box_in_focus:   false,
            cursor:         [0, 0, 0],
            last_mine_pos:  [0, 0, 0],
            sb_shape_owner: 0
        }
    }

    #[export]
    fn _ready(&mut self, mut owner: Spatial) {
        use wlambda::util::*;
        self.vol.fill(0, 0, 0, VOL_SIZE as u16, VOL_SIZE as u16, VOL_SIZE as u16, 100.into());
        let mut sm = SplitMix64::new(3489492);
        for z in 0..VOL_SIZE {
            for y in 0..VOL_SIZE {
                for x in 0..VOL_SIZE {
                    if u64_to_open01(sm.next_u64()) > 0.01 {
                        self.vol.set(x as u16, y as u16, z as u16, 0.into());
                    } else {
                        let color = (u64_to_open01(sm.next_u64()) * 256.0) as u8;
                        self.vol.set(x as u16, y as u16, z as u16, color.into());
                    }
                }
            }
        }
        println!("filled...");

        let voxel_material =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://scenes/system_map/voxel_material.tres"),
                GodotString::from_str("ShaderMaterial"),
                false).unwrap().cast::<Material>().unwrap();

        let mut i = 0;
        for z in 0..SUBVOLS {
            for y in 0..SUBVOLS {
                for x in 0..SUBVOLS {
                    self.meshes.push(MeshInstance::new());

                    let mut sb = StaticBody::new();

                    unsafe {
                        self.meshes[i].set_material_override(Some(voxel_material.clone()));
                        let mut t = self.meshes[i].get_transform();
                        t.origin.x = (x * SUBVOL_SIZE) as f32;
                        t.origin.y = (y * SUBVOL_SIZE) as f32;
                        t.origin.z = (z * SUBVOL_SIZE) as f32;
                        self.meshes[i].set_transform(t);
                        sb.set_transform(t);

                        owner.add_child(self.meshes[i].cast::<Node>(), false);

                        let sb_obj = Object::from_sys(sb.cast::<Object>().unwrap().to_sys());
                        let id = sb.create_shape_owner(Some(sb_obj));
                        self.collision_shapes.push((sb, id));

                        owner.add_child(sb.cast::<Node>(), false);
                    }

                    self.octrees.push(Octree::new_from_size(SUBVOL_SIZE));

                    i += 1;
                }
            }
        }
        println!("initialized godot objects");

        self.load_vol(owner);
        println!("loaded godot objects");
    }

    #[export]
    fn load_vol(&mut self, mut owner: Spatial) {
        for z in 0..VOL_SIZE {
            let iz  = z / SUBVOL_SIZE;
            let izi = z % SUBVOL_SIZE;

            for y in 0..VOL_SIZE {
                let iy  = y / SUBVOL_SIZE;
                let iyi = y % SUBVOL_SIZE;

                for x in 0..VOL_SIZE {
                    let ix  = x / SUBVOL_SIZE;
                    let ixi = x % SUBVOL_SIZE;

                    self.octrees[
                          iz * (SUBVOLS * SUBVOLS)
                        + iy * SUBVOLS
                        + ix].set(
                            ixi as u16,
                            iyi as u16,
                            izi as u16,
                            *self.vol.at(Pos {
                              x: x as u16,
                              y: y as u16,
                              z: z as u16
                            }));
                }
            }
        }

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
    }

    fn get_octree_at(&mut self, x: usize, y: usize, z: usize) -> (&mut Octree<u8>, [u16; 3]) {
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

        (&mut self.octrees[sub_idx], [ixi as u16, iyi as u16, izi as u16])
    }

    #[export]
    fn mine_info_at(&mut self, mut owner: Spatial, x: f64, y: f64, z: f64) -> Variant {
        let (ot, pos) =
            self.get_octree_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);
        let v =
            ot.get_inv_y(
                pos[0],
                pos[1],
                pos[2]);
        let mut dict = gdnative::Dictionary::new();
        dict.set(&Variant::from_str("material"), &Variant::from_i64(v.color as i64));
        dict.set(&Variant::from_str("time"),     &Variant::from_f64(1.2));
        Variant::from_dictionary(&dict)
    }

    #[export]
    fn looking_at(&mut self, mut owner: Spatial, x: f64, y: f64, z: f64) {
        unsafe {
            let mut c =
                owner.get_child(0)
                     .and_then(|n| n.cast::<Spatial>())
                     .unwrap();

            if !self.box_in_focus {
                c.show();
                self.box_in_focus = true;
            }

            let mut t = c.get_transform();
            t.origin.x = x.floor() as f32 + 0.5;
            t.origin.y = y.floor() as f32 + 0.5;
            t.origin.z = z.floor() as f32 + 0.5;
            c.set_transform(t);

            self.cursor = [
                t.origin.x as u16,
                t.origin.y as u16,
                t.origin.z as u16,
            ];

            {
                let (ot, pos) =
                    self.get_octree_at(
                        self.cursor[0] as usize,
                        self.cursor[1] as usize,
                        self.cursor[2] as usize);
                let v = ot.get_inv_y(pos[0], pos[1], pos[2]);
                if v.color == 0 {
                    c.hide();
                    self.box_in_focus = true;
                } else {
                    c.show();
                    self.box_in_focus = true;
                }
            }
        }
    }

    #[export]
    fn mine(&mut self, mut owner: Spatial) {
        if self.last_mine_pos != self.cursor {
            {
                let (ot, pos) =
                    self.get_octree_at(
                        self.cursor[0] as usize,
                        self.cursor[1] as usize,
                        self.cursor[2] as usize);
                ot.set_inv_y(pos[0], pos[1], pos[2], 0.into());
            }

            self.reload_at(
                self.cursor[0] as usize,
                self.cursor[1] as usize,
                self.cursor[2] as usize);

            self.last_mine_pos = self.cursor;
        }
    }

    #[export]
    fn looking_at_nothing(&mut self, mut owner: Spatial) {
        unsafe {
            let mut c =
                owner.get_child(0)
                     .and_then(|n| n.cast::<Spatial>())
                     .unwrap();
            if self.box_in_focus {
                c.hide();
                self.box_in_focus = false;
            }
        }
    }


    fn reload_at(&mut self, x: usize, y: usize, z: usize) {
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

        let n = self.octrees[sub_idx].recompute();

        let mut am = ArrayMesh::new();
        let mut cvshape = ConcavePolygonShape::new();
        let (mut static_body, shape_owner_idx) = self.collision_shapes[sub_idx];

        if !n.empty {
            let cm = ColorMap::new_8bit();

            render_octree_to_am(
                &mut am, &mut cvshape, &cm, &self.octrees[sub_idx]);
        }

        unsafe {
            self.meshes[sub_idx].set_mesh(am.cast::<Mesh>());
            let mut ssb = static_body.cast::<StaticBody>().unwrap();
            ssb.shape_owner_clear_shapes(shape_owner_idx);

            if !n.empty {
                ssb.shape_owner_add_shape(
                    shape_owner_idx,
                    cvshape.cast::<Shape>());
                self.meshes[sub_idx].show();
                static_body.show();
            } else {
                self.meshes[sub_idx].hide();
                static_body.hide();
            }
        }
    }
}
