use gdnative::*;
use euclid::{vec2, vec3};
use crate::voxeltree::*;
use crate::gd_voxel_impl::*;

#[derive(NativeClass)]
#[inherit(gdnative::Spatial)]
pub struct VoxStruct {
    box_in_focus:   bool,
    octree:         Option<Octree<u8>>,
    cursor:         [u16; 3],
    sb_shape_owner: i64,
    last_mine_pos:  [u16; 3],
}

#[methods]
impl VoxStruct {
    fn _init(owner: Spatial) -> Self {
        Self {
            box_in_focus:   false,
            octree:         None,
            cursor:         [0, 0, 0],
            last_mine_pos:  [0, 0, 0],
            sb_shape_owner: 0
        }
    }

    #[export]
    fn _ready(&mut self, mut owner: Spatial) {
    }
}
