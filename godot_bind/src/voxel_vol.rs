//use crate::state::SSCG;
//use wlambda::VVal;
//#[macro_use]
//use crate::state::*;
//use crate::util::{variant2vval, vval2variant, c2c};

use gdnative::*;
use euclid::{vec2, vec3};
use std::rc::Rc;

#[derive(NativeClass)]
#[inherit(gdnative::MeshInstance)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct VoxVolume {
}

enum Face {
    Front,  // x,       y,      z - 1
    Top,    // x,       y + 1,  z
    Back,   // x,       y,      z + 1
    Left,   // x - 1,   y,      z
    Right,  // x + 1,   y,      z
    Bottom, // x,       y - 1,  z
}

const UV_TEXTURE_ATLAS_WIDTH : usize = 8;

const CUBE_VERTICES : [[f32; 3]; 8] = [
  [ 0., 0., 0. ],
  [ 0., 1., 0. ],
  [ 1., 1., 0. ],
  [ 1., 0., 0. ],

  [ 0., 0., 1. ],
  [ 0., 1., 1. ],
  [ 1., 1., 1. ],
  [ 1., 0., 1. ],
];

/// Indices into the CUBE_VERTICES constant:
const FACE_TRIANGLE_VERTEX_IDX : [[usize; 6]; 6] = [
  [0, 1, 2,  2, 3, 0],
  [1, 5, 6,  6, 2, 1],
  [7, 6, 5,  5, 4, 7],
  [4, 5, 1,  1, 0, 4],
  [3, 2, 6,  6, 7, 3],
  [3, 7, 4,  4, 0, 3],
];

const FACE_TRIANGLE_VERTEX_UV : [[f32; 2]; 8] = [
    [0., 0.],
    [0., 1.],
    [1., 1.],
    [1., 0.],
    [0., 0.],
    [0., 1.],
    [1., 1.],
    [1., 0.],
];

impl Face {
    fn render_to(&self, texture_index: usize, offs: Vector3, scale: f32, sf: &mut SurfaceTool, collision_tris: &mut Vector3Array) {
        let tris = match self {
            Face::Front  => &FACE_TRIANGLE_VERTEX_IDX[0],
            Face::Top    => &FACE_TRIANGLE_VERTEX_IDX[1],
            Face::Back   => &FACE_TRIANGLE_VERTEX_IDX[2],
            Face::Left   => &FACE_TRIANGLE_VERTEX_IDX[3],
            Face::Right  => &FACE_TRIANGLE_VERTEX_IDX[4],
            Face::Bottom => &FACE_TRIANGLE_VERTEX_IDX[5],
        };

        let u_offs = (texture_index % UV_TEXTURE_ATLAS_WIDTH) as f32;
        let v_offs = (texture_index / UV_TEXTURE_ATLAS_WIDTH) as f32;

        for idx in tris.iter().rev() {
            sf.add_uv(vec2(
                FACE_TRIANGLE_VERTEX_UV[*idx][0] * u_offs,
                FACE_TRIANGLE_VERTEX_UV[*idx][1] * v_offs));
            let v = vec3(
                (CUBE_VERTICES[*idx][0] + offs.x) * scale,
                (CUBE_VERTICES[*idx][1] + offs.y) * scale,
                (CUBE_VERTICES[*idx][2] + offs.z) * scale);
            collision_tris.push(&v);
            sf.add_vertex(v);
        }
    }
}

//int VOX_DIR_OFFS[6][3] = {
//    { 0,  0, -1},
//    { 0,  1,  0},
//    { 0,  0,  1},
//    {-1,  0,  0},
//    { 1,  0,  0},
//    { 0, -1,  0},
//};
//
//float NORMAL_CUBE_FACE_LIGHT = 0.75;
//float CUBE_FACE_LIGHT_ADD[6] = {
//    0.2f,
//    0.25f,
//    0.0f,
//    0.15f,
//    0.1f,
//    0.05f
//};
//---------------------------------------------------------------------------

struct Volume {
    size:       usize,
    data:       std::vec::Vec<(u8,u8)>,
}

impl Volume {
    fn new() -> Self {
        Self {
            size: 30,
            data: vec![],
        }
    }

    fn render_to(&self, sf: &mut SurfaceTool) {
        let mut va = Vector3Array::new();

        for y in 0..self.size {
            for z in 0..self.size {
                for x in 0..self.size {
                    let is_border =
                           y == 0 || y == (self.size - 1)
                        || x == 0 || x == (self.size - 1)
                        || y == 0 || y == (self.size - 1);
                    if !is_border {
                    }

                    let v = vec3(y as f32, z as f32, x as f32);
                    Face::Front. render_to(0, v, 0.2, sf, &mut va);
                    Face::Back.  render_to(0, v, 0.2, sf, &mut va);
                    Face::Top.   render_to(0, v, 0.2, sf, &mut va);
                    Face::Bottom.render_to(0, v, 0.2, sf, &mut va);
                    Face::Left.  render_to(0, v, 0.2, sf, &mut va);
                    Face::Right. render_to(0, v, 0.2, sf, &mut va);
                }
            }
        }
    }
}

#[methods]
impl VoxVolume {
    fn _init(owner: MeshInstance) -> Self {
        Self { }
    }
    #[export]
    fn _ready(&mut self, mut owner: MeshInstance) {
        let mut sf = SurfaceTool::new();
        let vol = Volume::new();

        sf.begin(Mesh::PRIMITIVE_TRIANGLES);

        println!("RENDER TO:");
        vol.render_to(&mut sf);
        println!("GEN NORMALS");
        sf.generate_normals(false);
        println!("COMMIT");
        let mesh = sf.commit(None, 97792).unwrap();
        println!("COMMITTED!");

        unsafe {
            owner.set_mesh(mesh.cast::<Mesh>());
            owner.show();
        }
    }
    #[export]
    fn _process(&mut self, mut owner: MeshInstance, delta: f64) {
        let rot_speed = (10.0_f64).to_radians();
        unsafe {
            owner.rotate_x(rot_speed * delta);
            owner.rotate_z(rot_speed * 0.5 * delta);
        }
    }
}
