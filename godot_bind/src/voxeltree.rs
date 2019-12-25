pub trait VoxelColor: PartialEq + Sized + Copy + std::fmt::Debug + Default {}
impl<T: PartialEq + Sized + Copy + std::fmt::Debug + Default> VoxelColor for T {}

type pint = u16;

pub const F_NONE   : u8 = 0x00;
pub const F_FRONT  : u8 = 0x01;
pub const F_TOP    : u8 = 0x02;
pub const F_BACK   : u8 = 0x04;
pub const F_LEFT   : u8 = 0x08;
pub const F_RIGHT  : u8 = 0x10;
pub const F_BOTTOM : u8 = 0x20;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Voxel<C> where C: VoxelColor {
    pub color: C,
    /// Bits:
    /// Front,  // x,       y,      z - 1
    /// Top,    // x,       y - 1,  z
    /// Back,   // x,       y,      z + 1
    /// Left,   // x - 1,   y,      z
    /// Right,  // x + 1,   y,      z
    /// Bottom, // x,       y + 1,  z
    /// - 0x01     - Front
    /// - 0x02     - Top
    /// - 0x04     - Back
    /// - 0x08     - Left
    /// - 0x10     - Right
    /// - 0x20     - Bottom
    pub faces: u8,
}

impl<C> Voxel<C> where C: VoxelColor {
}

fn xyz2facemask(x: pint, y: pint, z: pint) -> u8 {
    let mut mask : u8 = F_NONE;
    if x == 0 { mask |= F_LEFT;   }
    else      { mask |= F_RIGHT;  }
    if y == 0 { mask |= F_TOP;    }
    else      { mask |= F_BOTTOM; }
    if z == 0 { mask |= F_FRONT;  }
    else      { mask |= F_BACK;   }
    mask = 0xFF;
    mask
}

impl Into<Voxel<u8>> for u8 {
    fn into(self) -> Voxel<u8> {
        Voxel {
            color: self,
            faces: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vol<C> where C: VoxelColor {
    pub size: usize,
    pub data: std::vec::Vec<Voxel<C>>,
}

impl<C> Vol<C> where C: VoxelColor {
    pub fn new(size: usize) -> Self {
        let mut data = std::vec::Vec::new();
        data.resize(size * size * size, Voxel::default());
        Self {
            size,
            data,
        }
    }

    pub fn set(&mut self, x: pint, y: pint, z: pint, v: Voxel<C>) {
        self.data[z as usize * self.size * self.size + y as usize * self.size + x as usize] = v;
    }

    pub fn at(&self, pos: Pos) -> &Voxel<C> {
        &self.data[pos.z as usize * self.size * self.size + pos.y as usize * self.size + pos.x as usize]
    }

    pub fn get(&mut self, pos: Pos) -> &Voxel<C> {
        let mut faces: u8 = 0x0;

        let clr_def = C::default();
        let size        = self.size;
        let last : pint = (self.size - 1) as pint;

        if pos.x == 0    { faces |= F_LEFT; }
        if pos.x == last { faces |= F_RIGHT; }
        if pos.x > 0 {
            let clr1 = self.data[pos.z as usize * size * size + pos.y as usize * size + pos.x as usize - 1].color;
            if clr1 == clr_def { faces |= 0x08; }
        }
        if pos.x < last {
            let clr1 = self.data[pos.z as usize * size * size + pos.y as usize * size + pos.x as usize + 1].color;
            if clr1 == clr_def { faces |= 0x10; }
        }

        if pos.y == 0         { faces |= F_TOP; }
        else if pos.y == last { faces |= F_BOTTOM; }
        if pos.y > 0 {
            let clr1 = self.data[pos.z as usize * size * size + (pos.y as usize - 1) * size + pos.x as usize].color;
            if clr1 == clr_def { faces |= F_TOP; }
        }
        if pos.y < last {
            let clr2 = self.data[pos.z as usize * size * size + (pos.y as usize + 1) * size + pos.x as usize].color;
            if clr2 == clr_def { faces |= F_BOTTOM; }
        }

        if pos.z == 0         { faces |= F_FRONT; }
        else if pos.z == last { faces |= F_BACK; }
        if pos.z > 0 {
            let clr1 = self.data[(pos.z as usize - 1) * size * size + pos.y as usize * size + pos.x as usize].color;
            if clr1 == clr_def { faces |= F_FRONT; }
        }
        if pos.z < last {
            let clr2 = self.data[(pos.z as usize + 1) * size * size + pos.y as usize * size + pos.x as usize].color;
            if clr2 == clr_def { faces |= F_BACK; }
        }

        let vox = &mut self.data[
                  pos.z as usize * size * size
                + pos.y as usize * size
                + pos.x as usize];
        vox.faces = faces;
        vox
    }

    pub fn fill(&mut self, x: pint, y: pint, z: pint,
                w: pint, h: pint, d: pint, v: Voxel<C>)
    {
        for z in z..(z + d) {
            for y in y..(y + h) {
                for x in x..(x + w) {
                    self.set(x, y, z, v);
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Pos {
    pub x: pint,
    pub y: pint,
    pub z: pint,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt;
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


impl Pos {
    pub fn new(x: pint, y: pint, z: pint) -> Self {
        Self { x, y, z }
    }

    pub fn offs(&self, xo: pint, yo: pint, zo: pint) -> Self {
        Self { x: self.x + xo, y: self.y + yo, z: self.z + zo }
    }

    pub fn mul(&self, m: pint) -> Self {
        Self { x: self.x * m, y: self.y * m, z: self.z * m }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct TreePos {
    pub offs:  usize,
    pub level: usize,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl std::fmt::Debug for TreePos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt;
        write!(f, "[lvl={},offs={}]+({}, {}, {})=>{}",
                  self.level, self.offs,
                  self.x, self.y, self.z, self.idx())
    }
}

impl TreePos {
    pub fn new() -> Self {
        Self { offs: 0, level: 0, x: 0, y: 0, z: 0 }
    }

    pub fn lvl_offs(&self, xo: u8, yo: u8, zo: u8) -> Self {
        let add_offs =
            if self.level == 0 { 1 }
            else { 8_usize.pow(self.level as u32) };
        Self {
            offs:  self.offs + add_offs,
            level: self.level + 1,
            x: (self.x << 1) + xo,
            y: (self.y << 1) + yo,
            z: (self.z << 1) + zo,
        }
    }

    pub fn idx(&self) -> usize {
        if self.level == 0 { return 0; }
        let edge_size = 2_usize.pow(self.level as u32);
        self.offs
        + self.z as usize * edge_size * edge_size
        + self.y as usize * edge_size
        + self.x as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OctNode<C: VoxelColor> {
    pub voxel: Option<Voxel<C>>,
    pub pos:   Pos,
    pub empty: bool,
    pub tree_pos: TreePos,
}

impl<C> OctNode<C> where C: VoxelColor {
    pub fn new() -> Self {
        Self {
            voxel:  None,
            pos:    Pos::default(),
            empty:  true,
            tree_pos: TreePos::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Octree<C: VoxelColor> {
    nodes: std::vec::Vec<OctNode<C>>,
    nodes_size: usize,
    pub vol: Vol<C>,
}

impl<C> Octree<C> where C: VoxelColor {
    pub fn new_from_size(size: usize) -> Self {
        let v : Vol<C> = Vol::new(size);
        Octree::new(size * size * size, v)
    }

    pub fn new(node_count: usize, vol: Vol<C>) -> Self {
        let mut size = vol.size >> 1;
        let mut alloc = 1;
        let mut subtree_size : usize = 2;
        while size > 1 {
            alloc += subtree_size.pow(3);
            subtree_size *= 2;
//            println!("SIZ {} {}", size, alloc);
            size = size >> 1;
        }
        alloc += 1; // For the root node
        let mut nodes = std::vec::Vec::new();
        nodes.resize(alloc, OctNode::default());
        Self {
            nodes,
            nodes_size: vol.size >> 1,
            vol,
        }
    }

    pub fn draw<F>(&self, f: &mut F) where F: FnMut(usize, &Pos, Voxel<C>) -> () {
        self.draw_level(TreePos::new(), self.vol.size, Pos { x: 0, y: 0, z: 0 }, f);
    }

    pub fn draw_level<F>(&self, tp: TreePos, size: usize, top_left: Pos, f: &mut F)
        where F: FnMut(usize, &Pos, Voxel<C>) -> ()
    {
        if size == 1 {
            let v = self.vol.at(top_left);
            if v.color != C::default() {
                f(1, &top_left, *v);
            }
            return
        }

        let n = self.nodes[tp.idx()];
        if n.empty { return; }

        if let Some(v) = n.voxel {
            f(size, &top_left, v);
            return;
        }

        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    self.draw_level(
                        tp.lvl_offs(x, y, z),
                        size >> 1,
                        top_left.offs(
                            (x as usize * (size >> 1)) as pint,
                            (y as usize * (size >> 1)) as pint,
                            (z as usize * (size >> 1)) as pint),
                        f);
                }
            }
        }
    }

    pub fn get_inv_y(&self, x: pint, y: pint, z: pint) -> Voxel<C> {
        self.get(x, (self.vol.size - 1) as u16 - y, z)
    }

    pub fn get(&self, x: pint, y: pint, z: pint) -> Voxel<C> {
        *self.vol.at(Pos::new(x, y, z))
    }

    pub fn set_inv_y(&mut self, x: pint, y: pint, z: pint, v: Voxel<C>) {
        self.vol.set(x, (self.vol.size - 1) as u16 - y, z, v);
    }

    pub fn set(&mut self, x: pint, y: pint, z: pint, v: Voxel<C>) {
        self.vol.set(x, y, z, v);
    }

    pub fn fill(&mut self, x: pint, y: pint, z: pint,
                w: pint, h: pint, d: pint, v: Voxel<C>)
    {
        self.vol.fill(x, y, z, w, h, d, v);
    }

    pub fn recompute(&mut self) -> OctNode<C> {
        let n = self.compute_node(TreePos::new(), self.vol.size, Pos { x: 0, y: 0, z: 0 });
        n
    }

    fn compute_node(&mut self, tp: TreePos, size: usize, top_left: Pos) -> OctNode<C> {
        if size == 1 {
            let v = self.vol.get(top_left);
            let mut n = OctNode::default();
            if v.color == C::default() {
                n.empty = true;
                n.pos = top_left;
            } else {
                n.voxel = Some(*v);
                n.empty = false;
                n.pos = top_left;
            }
//            dbg!(nidx, size, top_left, n.voxel, n.empty);
            return n;
        }

        let mut n : OctNode<C> = OctNode::default();
        let mut faces : u8 = 0x0;
        let mut color : C = C::default();

        let mut first       = true;
        let mut equal_color = true;
        let mut all_empty   = true;
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let n = self.compute_node(
                        tp.lvl_offs(x, y, z),
                        size >> 1,
                        top_left.offs(
                            (x as usize * (size >> 1)) as pint,
                            (y as usize * (size >> 1)) as pint,
                            (z as usize * (size >> 1)) as pint));

                    if !n.empty { all_empty = false; }
                    if let Some(v) = n.voxel {
                        if first { color = v.color; first = false; }
                        else if color != v.color { equal_color = false; }

                        faces |= v.faces;
                    } else {
                        equal_color = false;
                    }
                }
            }
        }

        let mut n = OctNode::default();
        n.pos      = top_left;
        n.tree_pos = tp;

        if !all_empty && equal_color {
            let mut v = Voxel::default();
            v.color = color;
            v.faces = faces;

            n.empty = false;
            n.voxel = Some(v);

        } else if !all_empty {
            n.empty       = false;
            n.voxel       = None;

        } else {
            n.empty       = true;
            n.voxel       = None;
        }

//        dbg!(tp, size, top_left, n.voxel, n.empty);
        if size > 1 {
            self.nodes[tp.idx()] = n;
        }

        n
    }

    fn node_at(&self, offs: usize,  x: usize, y: usize, z: usize) -> &OctNode<C> {
        &self.nodes[offs + (x * 2 * 2) + (y * 2) + x]
    }

    fn node(&mut self, offs: usize, x: usize, y: usize, z: usize) -> &mut OctNode<C> {
        &mut self.nodes[offs + (x * 2 * 2) + (y * 2) + x]
//        println!("ANOD lvl={}, pos={:?} => {:?}", level, pos, self.nodes);
    }

//    fn compute_voxel_node(&mut self, top_left: Pos) -> OctNode<C> {
//        let mut faces : u8 = 0x0;
//        let mut color : C  = C::default();
//
//        let size = self.vol.size;
//
//        let mut first       = true;
//        let mut equal_color = true;
//        let mut all_empty   = true;
//
//        for z in 0..2 {
//            for y in 0..2 {
//                for x in 0..2 {
//                    let vox = self.vol.get(top_left.offs(x, y, z));
//
//                    if first { color = vox.color; first = false; }
//                    else if color != vox.color { equal_color = false; }
//                    if color != C::default() { all_empty = false; }
//
//                    faces |= xyz2facemask(x, y, z) & vox.faces;
//                    eprintln!("{},{},{} :: {:?} :: {:x} {:x}",
//                              x, y, z, top_left, vox.faces, faces);
//                }
//            }
//        }
//
//        let mut n = OctNode::default();
//        if !all_empty && equal_color {
//            let mut v = Voxel::default();
//            v.color = color;
//            v.faces = faces;
//
//            n.empty = false;
//            n.voxel = Some(v);
//
//        } else if !all_empty {
//            n.empty       = false;
//            n.voxel       = None;
//
//        } else {
//            n.empty       = true;
//            n.voxel       = None;
//        }
//
//        n
//    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_octree_n1_filled() {
        let mut t : Octree<u8> = Octree::new_from_size(2);
        t.fill(0, 0, 0, 2, 2, 2, 10.into());
        let n = t.recompute();

        let mut log = vec![];
        t.draw(&mut |size, pos, v| {
            log.push((size, (pos.x, pos.y, pos.z), v.color, v.faces));
        });

        assert_eq!(log[0], (2, (0, 0, 0), 10, 63));
    }

    #[test]
    fn check_octree_n1_partial_minus_1() {
        let mut t : Octree<u8> = Octree::new_from_size(4);
        t.fill(0, 0, 0, 4, 4, 4, 10.into());
        t.set(0, 0, 0, 12.into());
        let n = t.recompute();

        let mut log = vec![];
        t.draw(&mut |size, pos, v| {
            log.push((size, (pos.x, pos.y, pos.z), v.color, v.faces));
        });
        for l in log.iter() { println!("E {:?} {:x}", l, l.3); }
        for (i, n) in t.nodes.iter().enumerate() {
            println!("AT {}: {:?}", i, n);
        }

        assert_eq!(log[0], (1, (0, 0, 0), 12, F_FRONT | F_LEFT | F_TOP));
        assert_eq!(log[1], (1, (1, 0, 0), 10, F_FRONT | F_TOP));
        assert_eq!(log[2], (1, (0, 1, 0), 10, F_FRONT | F_LEFT));
        assert_eq!(log[3], (1, (1, 1, 0), 10, F_FRONT));
        assert_eq!(log[4], (1, (0, 0, 1), 10, F_LEFT | F_TOP));
        assert_eq!(log[5], (1, (1, 0, 1), 10, F_TOP));
        assert_eq!(log[6], (1, (0, 1, 1), 10, F_LEFT));
        assert_eq!(log[7], (1, (1, 1, 1), 10, 0x00));
        assert_eq!(log[8], (2, (2, 0, 0), 10, F_RIGHT | F_FRONT | F_TOP));
    }

    #[test]
    fn check_octree_n2_filled() {
        let v : Vol<u8> = Vol::new(4);
        let mut t : Octree<u8> = Octree::new(64, v);
        t.fill(0, 0, 0, 4, 4, 4, 10.into());
        let n = t.recompute();
//        println!("NODES: {:?}", t.nodes);

        assert_eq!(n.voxel.unwrap().color, 10);
        assert_eq!(n.voxel.unwrap().faces, 0x3F);
        assert_eq!(t.nodes[0].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[0].empty, false);

        let mut first : Option<(usize, Pos, Voxel<u8>)> = None;
        let mut last : Option<(usize, Pos, Voxel<u8>)> = None;
        t.draw(&mut |size, pos, v| {
            if first.is_none() { first = Some((size, *pos, v)); }
            last = Some((size, *pos, v));
        });

        let first = first.unwrap();
        assert_eq!((first.0, first.1.x, first.1.y, first.1.z), (4, 0, 0, 0));
        assert_eq!(first.2.color, 10);
        assert_eq!(first.2.faces, 0x3f); // all faces

        let last = last.unwrap();
        assert_eq!((last.0, last.1.x, last.1.y, last.1.z), (4, 0, 0, 0));
        assert_eq!(last.2.color, 10);
        assert_eq!(last.2.faces, 0x3f); // all faces
    }

    #[test]
    fn check_octree_n2_broken() {
        let v : Vol<u8> = Vol::new(4);
        let mut t : Octree<u8> = Octree::new(64, v);
        t.fill(0, 0, 0, 4, 4, 4, 10.into());
        t.set(3, 3, 3, 20.into());
        let n = t.recompute();
//        println!("NODES: {:?}", t.nodes);

        assert_eq!(n.voxel, None);
        // first layer:
        assert_eq!(t.nodes[1].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[1].empty, false);
        assert_eq!(t.nodes[2].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[2].empty, false);
        assert_eq!(t.nodes[3].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[3].empty, false);
        assert_eq!(t.nodes[4].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[4].empty, false);

        // second layer:
        assert_eq!(t.nodes[5].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[5].empty, false);
        assert_eq!(t.nodes[6].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[6].empty, false);
        assert_eq!(t.nodes[7].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[7].empty, false);
        assert_eq!(t.nodes[8].voxel, None);
        assert_eq!(t.nodes[8].empty, false);

        let mut first : Option<(usize, Pos, Voxel<u8>)> = None;
        let mut last : Option<(usize, Pos, Voxel<u8>)> = None;
        t.draw(&mut |size, pos, v| {
//            println!("DRAWING: size={}, pos={:?} v={:?}", size, pos, v);
            if first.is_none() { first = Some((size, *pos, v)); }
            last = Some((size, *pos, v));
        });

        let first = first.unwrap();
        assert_eq!((first.0, first.1.x, first.1.y, first.1.z), (2, 0, 0, 0));
        assert_eq!(first.2.color, 10);
        assert_eq!(first.2.faces, F_FRONT | F_LEFT | F_TOP);

        let last = last.unwrap();
        assert_eq!((last.0, last.1.x, last.1.y, last.1.z), (1, 3, 3, 3));
        assert_eq!(last.2.color, 20);
        assert_eq!(last.2.faces, F_BACK | F_RIGHT | F_BOTTOM);
    }

    #[test]
    fn check_n3_filled() {
        let v : Vol<u8> = Vol::new(8);
        let mut t : Octree<u8> = Octree::new(8 * 8 * 8, v);
        t.fill(0, 0, 0, 8, 8, 8, 11.into());
        let n = t.recompute();

        assert_eq!(n.voxel.unwrap().color, 11);
        assert_eq!(n.voxel.unwrap().faces, 0x3F);
        assert_eq!(t.nodes[0].voxel.unwrap().color, 11);
        assert_eq!(t.nodes[0].empty, false);

        let mut first : Option<(usize, Pos, Voxel<u8>)> = None;
        let mut last : Option<(usize, Pos, Voxel<u8>)> = None;
        t.draw(&mut |size, pos, v| {
            if first.is_none() { first = Some((size, *pos, v)); }
            last = Some((size, *pos, v));
        });

        let first = first.unwrap();
        assert_eq!((first.0, first.1.x, first.1.y, first.1.z), (8, 0, 0, 0));
        assert_eq!(first.2.color, 11);
        assert_eq!(first.2.faces, 0x3f); // all faces

        let last = last.unwrap();
        assert_eq!((last.0, last.1.x, last.1.y, last.1.z), (8, 0, 0, 0));
        assert_eq!(last.2.color, 11);
        assert_eq!(last.2.faces, 0x3f); // all faces
    }

    #[test]
    fn check_n256_filled() {
        let v : Vol<u8> = Vol::new(256);
        let mut t : Octree<u8> = Octree::new(256 * 256 * 256, v);
        let d = std::time::Instant::now();
        t.fill(0, 0, 0, 256, 256, 256, 11.into());
        eprintln!("TF={}",d.elapsed().as_millis());
        let n = t.recompute();

        eprintln!("T={}",d.elapsed().as_millis());

        assert_eq!(n.voxel.unwrap().color, 11);
        assert_eq!(n.voxel.unwrap().faces, 0x3F);
        assert_eq!(t.nodes[0].voxel.unwrap().color, 11);
        assert_eq!(t.nodes[0].empty, false);

        let mut first : Option<(usize, Pos, Voxel<u8>)> = None;
        let mut last : Option<(usize, Pos, Voxel<u8>)> = None;
        t.draw(&mut |size, pos, v| {
            if first.is_none() { first = Some((size, *pos, v)); }
            last = Some((size, *pos, v));
        });

        let first = first.unwrap();
        assert_eq!((first.0, first.1.x, first.1.y, first.1.z), (256, 0, 0, 0));
        assert_eq!(first.2.color, 11);
        assert_eq!(first.2.faces, 0x3f); // all faces

        let last = last.unwrap();
        assert_eq!((last.0, last.1.x, last.1.y, last.1.z), (256, 0, 0, 0));
        assert_eq!(last.2.color, 11);
        assert_eq!(last.2.faces, 0x3f); // all faces
    }

    #[test]
    fn check_size8_1() {
        let mut ot : Octree<u8> = Octree::new_from_size(8);
        ot.fill(0, 0, 0, 4, 4, 4, 1.into());
        ot.fill(4, 4, 4, 4, 4, 4, 2.into());
        ot.recompute();

        let mut log = vec![];
        ot.draw(&mut |size, pos, v| {
            log.push((size, (pos.x, pos.y, pos.z), v.color, v.faces));
        });
        for l in log.iter() { println!("E {:?} {:x}", l, l.3); }

        assert_eq!(log[0], (4, (0, 0, 0), 1, 63));
        assert_eq!(log[1], (4, (4, 4, 4), 2, 63));
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn check_smal() {
        let mut ot : Octree<u8> = Octree::new_from_size(4);
        ot.set(2, 2, 2, 1.into());
        let n = ot.recompute();
        let mut log = vec![];
        ot.draw(&mut |size, pos, v| {
            log.push((size, (pos.x, pos.y, pos.z), v.color, v.faces));
        });
//
//        for (i, n) in ot.nodes.iter().enumerate() {
//            println!("AT {}: {:?}", i, n);
//        }


        assert_eq!(log[0], (1, (2, 2, 2), 1, 63));
    }
}
