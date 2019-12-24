//use crate::state::SSCG;
//use wlambda::VVal;
//#[macro_use]
//use crate::state::*;
//use crate::util::{variant2vval, vval2variant, c2c};

use gdnative::*;
use euclid::{vec2, vec3};
use std::rc::Rc;
use crate::voxeltree::*;

const VOLUME_CHUNK_SIZE : usize = 24;

struct ColorMap {
    colors: [[f32; 3]; 256],
}

impl ColorMap {
    fn new_8bit() -> Self {
        let mut colors = [[0.0; 3]; 256];
        for i in 0..256 {
            let r = (i as u8) >> 5 & 0x7;
            let g = (i as u8) >> 2 & 0x7;
            let b = (i as u8) & 0x3;
            colors[i] = [
                (0x7 as f32) / (r as f32),
                (0x7 as f32) / (g as f32),
                (0x3 as f32) / (b as f32),
            ];
        }
        Self { colors }
    }

    fn map(&self, c: u8) -> Color {
        let c = self.colors[c as usize];
        Color::rgb(c[0], c[1], c[2])
    }
}

fn render_octree_to_am(am: &mut ArrayMesh, cv: &mut ConcavePolygonShape, cm: &ColorMap, vt: &Octree<u8>) {
    let mut va      = Vector3Array::new();
    let mut verts   = Vector3Array::new();
    let mut uvs     = Vector2Array::new();
    let mut colors  = ColorArray::new();
    let mut normals = Vector3Array::new();
    let mut indices = Int32Array::new();

    let vol_size = vt.vol.size;
    verts  .resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);
    uvs    .resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);
    normals.resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);
    colors .resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);
    indices.resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);
    va     .resize(6 * 6 * (vol_size * vol_size * vol_size) as i32);

    let mut idxlen = 0;
    let mut vtxlen = 0;

    vt.draw(&mut |cube_size: usize, pos: &Pos, v: Voxel<u8>| {
        println!("RENDER CUBE: {}, {:?}, {:x}", cube_size, pos, v.faces);
        if v.color == 0 { return; }
        let vol_max_idx : u16 = vt.vol.size as u16 - cube_size as u16;

//        if !(  (pos.x == 0 && pos.y == 0 && pos.z == 0)
//            || (pos.x == 1 && pos.y == 0 && pos.z == 0)
//            || (pos.x == 0 && pos.y == 1 && pos.z == 0)
//            || (pos.x == 1 && pos.y == 1 && pos.z == 0)
//            || (pos.x == 0 && pos.y == 0 && pos.z == 1)
//            || (pos.x == 1 && pos.y == 0 && pos.z == 1)
//            ) { return; }

        let clr = cm.map(v.color);
        let mut p = vec3(
            pos.x as f32,
            (vol_max_idx - pos.y) as f32,
            pos.z as f32);
        if v.faces & F_FRONT > 0 {
            Face::Front. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
        if v.faces & F_TOP > 0 {
            Face::Top. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
        if v.faces & F_BACK > 0 {
            Face::Back. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
        if v.faces & F_LEFT > 0 {
            Face::Left. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
        if v.faces & F_RIGHT > 0 {
            Face::Right. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
        if v.faces & F_BOTTOM > 0 {
            Face::Bottom. render_to_arr(
                &mut idxlen, &mut vtxlen, clr, p, cube_size as f32, 1.0,
                &mut verts, &mut uvs, &mut colors, &mut normals, &mut indices, &mut va);
        }
    });

    verts  .resize(vtxlen as i32);
    uvs    .resize(vtxlen as i32);
    normals.resize(vtxlen as i32);
    colors .resize(vtxlen as i32);
    indices.resize(idxlen as i32);
    va     .resize(idxlen as i32);

    println!("VERTEXES={}", vtxlen);

    let mut arr = VariantArray::new();
    arr.push(&Variant::from_vector3_array(&verts));
    arr.push(&Variant::from_vector3_array(&normals));
    arr.push(&Variant::new()); // tangent
    arr.push(&Variant::from_color_array(&colors));
    arr.push(&Variant::from_vector2_array(&uvs));
    arr.push(&Variant::new()); // uv2
    arr.push(&Variant::new()); // bones
    arr.push(&Variant::new()); // weights
    arr.push(&Variant::from_int32_array(&indices));

    am.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arr, VariantArray::new(), 97280);
    cv.set_faces(va);
}



#[derive(NativeClass)]
#[inherit(gdnative::MeshInstance)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct InstVoxVolume {
    box_in_focus: bool,
}

#[methods]
impl InstVoxVolume {
    fn _init(owner: MeshInstance) -> Self {
        Self { box_in_focus: false }
    }

    #[export]
    fn _ready(&mut self, mut owner: MeshInstance) {
        let mut am = ArrayMesh::new();
        let mut ot : Octree<u8> = Octree::new_from_size(8);
        ot.fill(0, 0, 0, 4, 4, 4, 128.into());
        ot.fill(1, 1, 1, 2, 2, 2, 0.into());
        ot.fill(4, 4, 4, 4, 4, 4, 240.into());
        ot.set(0, 0, 0, 0.into());
        ot.set(1, 1, 0, 0.into());
        ot.set(0, 7, 0, 72.into());
        ot.set(0, 7, 7, 23.into());
//        ot.set(0, 0, 3, 0.into());
        ot.recompute();

        let cm = ColorMap::new_8bit();

        let mut cvshape = ConcavePolygonShape::new();

        render_octree_to_am(&mut am, &mut cvshape, &cm, &ot);

        println!("IVV COMMITTED!");
        unsafe {
            owner.set_mesh(am.cast::<Mesh>());

            let mut sb = StaticBody::new();
            let sb_obj = Object::from_sys(sb.cast::<Object>().unwrap().to_sys());
            let id = sb.create_shape_owner(Some(sb_obj));
            sb.shape_owner_add_shape(id, cvshape.cast::<Shape>());
            sb.set_name(GodotString::from("box_selector"));

//            owner.rotate_x((90.0_f64).to_radians());
//            owner.rotate_z((90.0_f64).to_radians());
            owner.add_child(sb.cast::<Node>(), false);
            owner.show();
        }
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
        }
    }

    #[export]
    fn looking_at_nothing(&mut self, mut owner: MeshInstance) {
        println!("NONE");
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
  [ 0., 0., 0. ], // 0
  [ 0., 1., 0. ], // 1
  [ 1., 1., 0. ], // 2
  [ 1., 0., 0. ], // 3

  [ 0., 0., 1. ], // 4
  [ 0., 1., 1. ], // 5
  [ 1., 1., 1. ], // 6
  [ 1., 0., 1. ], // 7
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
// Cube Vertex Idx | relative indexes of those:
   [0, 1, 2, 3,      2, 1, 0,  0, 3, 2, ],
   [1, 5, 6, 2,      2, 1, 0,  0, 3, 2, ],
   [4, 5, 6, 7,      1, 2, 3,  3, 0, 1, ],
   [0, 1, 5, 4,      1, 2, 3,  3, 0, 1, ],
   [3, 7, 6, 2,      2, 3, 0,  0, 1, 2, ],
   [0, 4, 7, 3,      1, 2, 3,  3, 0, 1, ],
];

//const FACE_TRIANGLE_VERTEX_UV : [[f32; 2]; 8] = [
const FACE_TRIANGLE_VERTEX_UV : [[f32; 2]; 4] = [
    [0., 0.],
    [0., 1.],
    [1., 1.],
    [1., 0.],
];

impl Face {
    fn render_to_arr(&self,
                     idxlen: &mut usize,
                     vtxlen: &mut usize,
                     color: Color,
                     offs: Vector3,
                     size: f32,
                     scale: f32,
                     verts: &mut Vector3Array,
                     uvs: &mut Vector2Array,
                     colors: &mut ColorArray,
                     normals: &mut Vector3Array,
                     indices: &mut Int32Array,
                     collision_tris: &mut Vector3Array) {

        let mut tris = match self {
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

        for i in 0..4 {
            let idx = tris[i];
            uvs.set(*vtxlen as i32, &vec2(
                FACE_TRIANGLE_VERTEX_UV[i][0],
                FACE_TRIANGLE_VERTEX_UV[i][1]));
            let v = vec3(
                (CUBE_VERTICES[idx][0] * size + offs.x) * scale,
                (CUBE_VERTICES[idx][1] * size + offs.y) * scale,
                (CUBE_VERTICES[idx][2] * size + offs.z) * scale);
            verts.set(*vtxlen as i32, &v);
            colors.set(*vtxlen as i32, &color);
            normals.set(*vtxlen as i32, &vec3(normal[0], normal[1], normal[2]));
            *vtxlen += 1;
        }

        for i in 4..10 {
            let idx = tris[i];
            let tri_vertex_index = *vtxlen as i32 - (4 - idx as i32);
            indices.set(*idxlen as i32, tri_vertex_index);
            collision_tris.set(*idxlen as i32, &verts.get(tri_vertex_index));
            *idxlen += 1;
        }
    }
}
