use gdnative::*;
use euclid::{vec2, vec3};
use std::rc::Rc;
use crate::voxeltree::*;
use crate::gd_voxel_impl::*;

#[derive(NativeClass)]
#[inherit(gdnative::MeshInstance)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct InstVoxVolume {
    box_in_focus:   bool,
    octree:         Option<Octree<u8>>,
    cursor:         [u16; 3],
    sb_shape_owner: i64,
    last_mine_pos:  [u16; 3],
}

#[methods]
impl InstVoxVolume {
    fn _init(owner: MeshInstance) -> Self {
        Self {
            box_in_focus:   false,
            octree:         None,
            cursor:         [0, 0, 0],
            last_mine_pos:  [0, 0, 0],
            sb_shape_owner: 0
        }
    }

    #[export]
    fn _ready(&mut self, mut owner: MeshInstance) {
        let mut am = ArrayMesh::new();
        let mut ot : Octree<u8> = Octree::new_from_size(16);
        ot.fill(0, 0, 0, 4, 4, 4, 128.into());
        ot.fill(1, 1, 1, 2, 2, 2, 0.into());
        ot.fill(4, 4, 4, 4, 4, 4, 240.into());
        ot.fill(8, 8, 8, 8, 8, 8, 250.into());
        ot.set(0, 0, 0, 0.into());
        ot.set(1, 1, 0, 0.into());
        ot.set(0, 7, 0, 72.into());
        ot.set(0, 7, 7, 23.into());
//        ot.set(0, 0, 3, 0.into());

        ot.recompute();

        self.octree = Some(ot);

        unsafe {
            let mut sb = StaticBody::new();
            let sb_obj = Object::from_sys(sb.cast::<Object>().unwrap().to_sys());
            let id = sb.create_shape_owner(Some(sb_obj));
            self.sb_shape_owner = id;
            sb.set_name(GodotString::from("box_selector"));

            owner.add_child(sb.cast::<Node>(), false);
            owner.show();
        }

        self.rebuild(&mut owner);
    }

    fn rebuild(&mut self, owner: &mut MeshInstance) {
        if let Some(ref mut ot) = self.octree {
            let n = ot.recompute();

            let mut am = ArrayMesh::new();
            let mut cvshape = ConcavePolygonShape::new();

            if !n.empty {
                let cm = ColorMap::new_8bit();

                render_octree_to_am(&mut am, &mut cvshape, &cm, ot);
            }

            unsafe {
                owner.set_mesh(am.cast::<Mesh>());
                let mut sb =
                    owner.get_node(NodePath::from_str("box_selector")).unwrap();
                let mut ssb = sb.cast::<StaticBody>().unwrap();
                ssb.shape_owner_clear_shapes(self.sb_shape_owner);

                if !n.empty {
                    ssb.shape_owner_add_shape(
                        self.sb_shape_owner,
                        cvshape.cast::<Shape>());
                }
            }
        }
    }

    #[export]
    fn mine_info_at(&mut self, mut owner: MeshInstance, x: f64, y: f64, z: f64) -> Variant {
        let v =
            self.octree.as_ref().unwrap().get_inv_y(
                self.cursor[0],
                self.cursor[1],
                self.cursor[2]);
        let mut dict = gdnative::Dictionary::new();
        dict.set(&Variant::from_str("material"), &Variant::from_i64(v.color as i64));
        dict.set(&Variant::from_str("time"),     &Variant::from_f64(1.2));
        Variant::from_dictionary(&dict)
    }

    #[export]
    fn looking_at(&mut self, mut owner: MeshInstance, x: f64, y: f64, z: f64) {
        unsafe {
            let mut c = owner.get_child(0).and_then(|n| n.cast::<Spatial>()).unwrap();
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
        }
    }

    #[export]
    fn mine(&mut self, mut owner: MeshInstance) {
        if self.last_mine_pos != self.cursor {
            let v =
                self.octree.as_ref().unwrap().get_inv_y(
                    self.cursor[0],
                    self.cursor[1],
                    self.cursor[2]);
            self.octree.as_mut().unwrap().set_inv_y(
                self.cursor[0],
                self.cursor[1],
                self.cursor[2],
                0.into());
            println!("MINED: {}", v.color);
            self.rebuild(&mut owner);

            self.last_mine_pos = self.cursor;
        }
    }

    #[export]
    fn looking_at_nothing(&mut self, mut owner: MeshInstance) {
        unsafe {
            let mut c = owner.get_child(0).and_then(|n| n.cast::<Spatial>()).unwrap();
            if self.box_in_focus {
                c.hide();
                self.box_in_focus = false;
            }
        }
    }

    #[export]
    fn _process(&mut self, mut owner: MeshInstance, delta: f64) {
        let rot_speed = (10.0_f64).to_radians();
        unsafe {
//            owner.rotate_x(rot_speed * 0.6 * delta);
//            owner.rotate_z(rot_speed * 0.6 * delta);
        }
    }
}

