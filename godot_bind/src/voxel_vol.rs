//use crate::state::SSCG;
//use wlambda::VVal;
//#[macro_use]
//use crate::state::*;
//use crate::util::{variant2vval, vval2variant, c2c};

use gdnative::*;
use euclid::{vec2, vec3};
use std::rc::Rc;

#[derive(NativeClass)]
#[inherit(gdnative::MultiMeshInstance)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct InstVoxVolume {
}

#[methods]
impl InstVoxVolume {
    fn _init(owner: MultiMeshInstance) -> Self {
        Self { }
    }
    #[export]
    fn _ready(&mut self, mut owner: MultiMeshInstance) {
        let mut am = ArrayMesh::new();
        let vol = Volume::new(vec3(0., 0., 0.), 1);
        vol.render_to_am(&mut am);
        println!("IVV COMMITTED!");
        unsafe {
//            let mut mm = MultiMesh::new();
////            let mut mm = owner.get_multimesh().unwrap();
//            mm.set_mesh(am.cast::<Mesh>());
////            mm.set_instance_count(0);
//
//            let mut t = owner.get_transform();
//            println!("TOTO {:?}", t);
////            mm.transform(t);
////            t.origin.x -= 20.;
////            t.origin.y -= 10.;
////            mm.set_instance_transform(0, t);
//
////            let mut t = owner.get_transform();
////            t.origin.x -= 10.;
////            mm.set_instance_transform(0, t);
////            mm.set_instance_transform(1, t);
////
//            mm.set_transform_format(1);
//            mm.set_instance_count(3);
//            let mut t = owner.get_transform();
//            mm.set_instance_transform(0, t);
//            owner.set_multimesh(Some(mm));

            let mut mm = owner.get_multimesh().unwrap();
            mm.set_mesh(am.cast::<Mesh>());
            let c = 300_000;
            mm.set_instance_count(c);
            mm.set_instance_transform(0, owner.get_transform());
            let mut tt = owner.get_transform();
//            tt.origin.y += 2.0;
//            mm.set_instance_transform(1, tt);
            tt.origin.z += 0.;
            for i in 0..c {
                tt.origin.y += 1.0;
                mm.set_instance_transform(i, tt);
            }
            println!("IVV DONE!");
            owner.rotate_x((90.0_f64).to_radians());
            owner.rotate_z((90.0_f64).to_radians());
            owner.show();
        }
    }
    #[export]
    fn _process(&mut self, mut owner: MultiMeshInstance, delta: f64) {
        let rot_speed = (10.0_f64).to_radians();
        unsafe {
//            owner.rotate_x(rot_speed * 0.1 * delta);
//            owner.rotate_z(rot_speed * 0.1 * delta);
        }
    }
}

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
const FACE_TRIANGLE_VERTEX_IDX : [[usize; 10]; 6] = [
// -  [0, 1, 2,  2, 3, 0],
// -  [1, 5, 6,  6, 2, 1],
// -  [7, 6, 5,  5, 4, 7],
// -  [4, 5, 1,  1, 0, 4],
// -  [3, 2, 6,  6, 7, 3],
// -  [3, 7, 4,  4, 0, 3],

// Cube Vertex Idx | relative indexes of those:
   [0, 1, 2, 3,      2, 1, 0,  0, 3, 2, ],
   [1, 2, 5, 6,      3, 2, 0,  0, 1, 3, ],
   [4, 5, 6, 7,      1, 2, 3,  3, 0, 1, ],
   [0, 1, 4, 5,      1, 3, 2,  2, 0, 1, ],
   [2, 3, 6, 7,      2, 0, 1,  1, 3, 2, ],
   [0, 3, 4, 7,      2, 3, 1,  1, 0, 2, ],
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
    fn render_to_arr(&self,
                     idxlen: &mut usize,
                     vtxlen: &mut usize,
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

        for i in 0..4 {
            let idx = tris[i];
            uvs.set(*vtxlen as i32, &vec2(
                FACE_TRIANGLE_VERTEX_UV[idx][0] * u_offs,
                FACE_TRIANGLE_VERTEX_UV[idx][1] * v_offs));
            let v = vec3(
                (CUBE_VERTICES[idx][0] + offs.x) * scale,
                (CUBE_VERTICES[idx][1] + offs.y) * scale,
                (CUBE_VERTICES[idx][2] + offs.z) * scale);
//            collision_tris.set(*vtxlen as i32, &v);
            verts.set(*vtxlen as i32, &v);
            normals.set(*vtxlen as i32, &vec3(normal[0], normal[1], normal[2]));
            *vtxlen += 1;
        }

        for i in 4..10 {
            let idx = tris[i];
            indices.set(*idxlen as i32, *vtxlen as i32 - (4 - idx as i32));
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

const VOLUME_CHUNK_SIZE : usize = 24;

impl Volume {
    fn new(offs: Vector3, size: usize) -> Self {
        Self {
            offs,
            size,
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
        let (idxlen, vtxlen) = self.render_to_arr(&mut verts, &mut uvs, &mut normals, &mut indices);
        println!("DONE RENDER");
        verts  .resize(vtxlen as i32);
        uvs    .resize(vtxlen as i32);
        normals.resize(vtxlen as i32);
        indices.resize(idxlen as i32);

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
                     indices: &mut Int32Array) -> (usize, usize)
    {
        let mut va = Vector3Array::new();
        let mut idxlen = 0;
        let mut vtxlen = 0;

        for y in 0..self.size {
            for z in 0..self.size {
                for x in 0..self.size {
                    let is_border =
                           y == 0 || y == (self.size - 1)
                        || x == 0 || x == (self.size - 1)
                        || y == 0 || y == (self.size - 1);
                    if !is_border {
                    }

                    if y % 2 == 0 || x % 5 == 0 || z % 10 == 0 {
//                        continue;
                    }

                    let mut v = vec3(y as f32, z as f32, x as f32);
                    v += self.offs;
                    Face::Front. render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                    Face::Back.  render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                    Face::Top.   render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                    Face::Bottom.render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                    Face::Left.  render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                    Face::Right. render_to_arr(&mut idxlen, &mut vtxlen, 0, v, 1.0, verts, uvs, normals, indices, &mut va);
                }
            }
        }

        (idxlen, vtxlen)
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

        let vol = Volume::new(vec3(0., 0., 0.), 16);
//        for x in 0..1 {
//            for y in 0..1 {
//                for z in 0..1 {
//                    println!("RENDER TO {}{}{}", x, y, z);
//                    let vol = Volume::new(vec3(
//                        x as f32 * VOLUME_CHUNK_SIZE as f32,
//                        y as f32 * VOLUME_CHUNK_SIZE as f32,
//                        z as f32 * VOLUME_CHUNK_SIZE as f32),
//                        VOLUME_CHUNK_SIZE);
                    vol.render_to_am(&mut am);
//                }
//            }
//        }
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
//            owner.show();
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
