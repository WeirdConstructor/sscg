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

const CUBE_NORMALS : [[f32; 3]; 6] = [
  [  0.,  0., -1. ],
  [  0.,  1.,  0. ],
  [  0.,  0.,  1. ],
  [ -1.,  0.,  0. ],
  [  1.,  0.,  0. ],
  [  0., -1.,  0. ],
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
    fn render_to_arr(&self, idxlen: &mut usize,
                     texture_index: usize,
                     offs: Vector3,
                     scale: f32,
                     verts: &mut Vector3Array,
                     uvs: &mut Vector2Array,
                     normals: &mut Vector3Array,
                     indices: &mut Int32Array,
                     collision_tris: &mut Vector3Array) {

        let tris = match self {
            Face::Front  => &FACE_TRIANGLE_VERTEX_IDX[0],
            Face::Top    => &FACE_TRIANGLE_VERTEX_IDX[1],
            Face::Back   => &FACE_TRIANGLE_VERTEX_IDX[2],
            Face::Left   => &FACE_TRIANGLE_VERTEX_IDX[3],
            Face::Right  => &FACE_TRIANGLE_VERTEX_IDX[4],
            Face::Bottom => &FACE_TRIANGLE_VERTEX_IDX[5],
        };

        let normal = match self {
            Face::Front  => &CUBE_NORMALS[0],
            Face::Top    => &CUBE_NORMALS[1],
            Face::Back   => &CUBE_NORMALS[2],
            Face::Left   => &CUBE_NORMALS[3],
            Face::Right  => &CUBE_NORMALS[4],
            Face::Bottom => &CUBE_NORMALS[5],
        };

        let u_offs = (texture_index % UV_TEXTURE_ATLAS_WIDTH) as f32;
        let v_offs = (texture_index / UV_TEXTURE_ATLAS_WIDTH) as f32;

        for (i, idx) in tris.iter().rev().enumerate() {
            uvs.set(*idxlen as i32, &vec2(
                FACE_TRIANGLE_VERTEX_UV[*idx][0] * u_offs,
                FACE_TRIANGLE_VERTEX_UV[*idx][1] * v_offs));
            let v = vec3(
                (CUBE_VERTICES[*idx][0] + offs.x) * scale,
                (CUBE_VERTICES[*idx][1] + offs.y) * scale,
                (CUBE_VERTICES[*idx][2] + offs.z) * scale);
//            collision_tris.set(*idxlen as i32, &v);
            verts.set(*idxlen as i32, &v);
            normals.set(*idxlen as i32, &vec3(normal[0], normal[1], normal[2]));
            indices.set(*idxlen as i32, *idxlen as i32);
            *idxlen += 1;
        }
    }

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
    offs:       Vector3,
    size:       usize,
    data:       std::vec::Vec<(u8,u8)>,
}

const VOLUME_CHUNK_SIZE : usize = 32;

impl Volume {
    fn new(offs: Vector3) -> Self {
        Self {
            offs,
            size: VOLUME_CHUNK_SIZE,
            data: vec![],
        }
    }

    fn render_to_am(&self, am: &mut ArrayMesh) {
        let mut verts   = Vector3Array::new();
        let mut uvs     = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut indices = Int32Array::new();

        verts  .resize(6 * 6 * (VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE) as i32);
        uvs    .resize(6 * 6 * (VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE) as i32);
        normals.resize(6 * 6 * (VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE) as i32);
        indices.resize(6 * 6 * (VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE * VOLUME_CHUNK_SIZE) as i32);

        println!("RENDER TO:");
        let len = self.render_to_arr(&mut verts, &mut uvs, &mut normals, &mut indices);
        println!("DONE RENDER");
        verts.resize(len as i32);
        uvs.resize(len as i32);
        normals.resize(len as i32);
        indices.resize(len as i32);

        let mut arr = VariantArray::new();
        arr.push(&Variant::from_vector3_array(&verts));
        arr.push(&Variant::from_vector3_array(&normals));
        arr.push(&Variant::new()); // tangent
        arr.push(&Variant::new()); // color
        arr.push(&Variant::from_vector2_array(&uvs));
        arr.push(&Variant::new()); // uv2
        arr.push(&Variant::new()); // bones
        arr.push(&Variant::new()); // weights
        arr.push(&Variant::from_int32_array(&indices));

        am.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arr, VariantArray::new(), 97280);
    }

    fn render_to_arr(&self,
                     verts: &mut Vector3Array,
                     uvs: &mut Vector2Array,
                     normals: &mut Vector3Array,
                     indices: &mut Int32Array) -> usize
    {
        let mut va = Vector3Array::new();
        let mut len = 0;

        for y in 0..self.size {
            for z in 0..self.size {
                for x in 0..self.size {
                    let is_border =
                           y == 0 || y == (self.size - 1)
                        || x == 0 || x == (self.size - 1)
                        || y == 0 || y == (self.size - 1);
                    if !is_border {
                    }

                    let mut v = vec3(y as f32, z as f32, x as f32);
                    v += self.offs;
                    Face::Front. render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                    Face::Back.  render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                    Face::Top.   render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                    Face::Bottom.render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                    Face::Left.  render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                    Face::Right. render_to_arr(&mut len, 0, v, 0.2, verts, uvs, normals, indices, &mut va);
                }
            }
        }
        len
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

                    let mut v = vec3(y as f32, z as f32, x as f32);
                    v += self.offs;
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
//        let mut sf = SurfaceTool::new();
//        sf.begin(Mesh::PRIMITIVE_TRIANGLES);
        let mut am      = ArrayMesh::new();

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    println!("RENDER TO {}{}{}", x, y, z);
                    let vol = Volume::new(vec3(
                        x as f32 * VOLUME_CHUNK_SIZE as f32,
                        y as f32 * VOLUME_CHUNK_SIZE as f32,
                        z as f32 * VOLUME_CHUNK_SIZE as f32));
                    vol.render_to_am(&mut am);
                }
            }
        }
//        println!("RENDER TO:");
//        vol.render_to_am(&mut am);
//        println!("RENDER TO:");
//        vol2.render_to_am(&mut am);
//        vol3.render_to_am(&mut am);
//        vol4.render_to_am(&mut am);
//        println!("GEN NORMALS");
//        sf.generate_normals(false);
//        println!("COMMIT");
//        let mesh = sf.commit(None, 97792).unwrap();

//        am.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arr, VariantArray::new(), 97280);
        println!("COMMITTED!");
        unsafe {
            owner.set_mesh(am.cast::<Mesh>());
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
