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
    cursor:         [u16; 3],
    sb_shape_owner: i64,
    last_mine_pos:  [u16; 3],
}

unsafe impl Send for VoxStruct { }

const VOL_SIZE    : usize = 128;
const SUBVOL_SIZE : usize = 32;
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
        self.vol.fill(0, 0, 0, VOL_SIZE as u16, VOL_SIZE as u16, VOL_SIZE as u16, 100.into());

        for i in 0..SUBVOLS.pow(3) {
            self.meshes.push(MeshInstance::new());

            let mut sb = StaticBody::new();

            unsafe {
                owner.add_child(self.meshes[i].cast::<Node>(), false);

                let sb_obj = Object::from_sys(sb.cast::<Object>().unwrap().to_sys());
                let id = sb.create_shape_owner(Some(sb_obj));
                self.collision_shapes.push((sb, id));

                owner.add_child(sb.cast::<Node>(), false);
            }

            self.octrees.push(Octree::new_from_size(SUBVOL_SIZE));
        }

        self.load_vol(owner);
    }

    #[export]
    fn load_vol(&mut self, mut owner: Spatial) {
        for z in 0..VOL_SIZE {
            let iz  = z / SUBVOLS;
            let izi = z % SUBVOLS;

            for y in 0..VOL_SIZE {
                let iy  = y / SUBVOLS;
                let iyi = y % SUBVOLS;

                for x in 0..VOL_SIZE {
                    let ix  = x / SUBVOLS;
                    let ixi = x % SUBVOLS;
                    println!("XX {},{},{}", ix, iy, iz);

                    self.octrees[
                          iz * (SUBVOLS * SUBVOLS)
                        + iy * SUBVOLS
                        + ix].set(ixi as u16, iyi as u16, izi as u16, *self.vol.at(Pos { x: ixi as u16, y: iyi as u16, z: izi as u16 }));
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

    fn reload_at(&mut self, x: usize, y: usize, z: usize) {
        let iz  = z / SUBVOLS;
        let izi = z % SUBVOLS;

        let iy  = y / SUBVOLS;
        let iyi = y % SUBVOLS;

        let ix  = x / SUBVOLS;
        let ixi = x % SUBVOLS;

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

            println!("FOO: {:?}", n);
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
