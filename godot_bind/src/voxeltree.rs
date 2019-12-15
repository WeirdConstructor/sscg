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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Node<C: VoxelColor> {
    pub voxel: Option<Voxel<C>>,
    pub pos:   Pos,
    pub empty: bool,
}

impl<C> Node<C> where C: VoxelColor {
    pub fn new() -> Self {
        Self {
            voxel:  None,
            pos:    Pos::default(),
            empty:  false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Octree<C: VoxelColor> {
    nodes: std::vec::Vec<Node<C>>,
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
        let mut alloc = 0;
        while size > 0 {
            alloc += size * size * size;
//            println!("SIZ {} {}", size, alloc);
            size = size >> 1;
        }
        let mut nodes = std::vec::Vec::new();
        nodes.resize(alloc, Node::default());
        Self {
            nodes,
            nodes_size: vol.size >> 1,
            vol,
        }
    }

    pub fn draw<F>(&self, f: &mut F) where F: FnMut(usize, &Pos, Voxel<C>) -> () {
        self.draw_level(
            1,
            self.vol.size,
            Pos { x: 0, y: 0, z: 0 },
            Pos { x: 0, y: 0, z: 0 },
            f);
    }

    pub fn draw_level<F>(&self, level: usize, size: usize, tree_pos: Pos, top_left: Pos, f: &mut F)
        where F: FnMut(usize, &Pos, Voxel<C>) -> ()
    {
//        println!("DRAW lvl={}, size={}, tp={:?}, tl={:?}",
//                 level, size, tree_pos, top_left);
        if level == self.vol.size {
//            for z in 0..2 {
//                for y in 0..2 {
//                    for x in 0..2 {
//                        let npos = top_left.offs(x, y, z);
                        f(1, &top_left, *self.vol.at(top_left));
//                    }
//                }
//            }
        } else {
            let n = *self.node_at(level, tree_pos);
            if n.empty { return; }
            if let Some(v) = n.voxel {
                f(size, &top_left, v);

            } else {
                for z in 0..2 {
                    for y in 0..2 {
                        for x in 0..2 {
                            self.draw_level(
                                level << 1,
                                size >> 1,
                                tree_pos.offs(x, y, z),
                                top_left.offs(
                                    x * (size >> 1) as pint,
                                    y * (size >> 1) as pint,
                                    z * (size >> 1) as pint),
                                f);
                        }
                    }//
                }
            }
        }
    }

    pub fn set(&mut self, x: pint, y: pint, z: pint, v: Voxel<C>) {
        self.vol.set(x, y, z, v);
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
//    fn init_level(&mut self, pow: usize, offs_x: usize, offs_y: usize, offs_z: usize) {
//        let mut size = self.vol.size;
//
//        if pow < size {
//            self.init_level(pow << 1, 0, 0, 0);
//            self.init_level(pow << 1, 1, 0, 0);
//            self.init_level(pow << 1, 0, 1, 0);
//            self.init_level(pow << 1, 1, 1, 0);
//            self.init_level(pow << 1, 0, 0, 1);
//            self.init_level(pow << 1, 1, 0, 1);
//            self.init_level(pow << 1, 0, 1, 1);
//            self.init_level(pow << 1, 1, 1, 1);
//        }
//    }
//
    pub fn recompute(&mut self) -> Node<C> {
        self.compute_node(1, Pos { x: 0, y: 0, z: 0 }, Pos { x: 0, y: 0, z: 0 })
    }

    fn compute_node(&mut self, level: usize, tree_pos: Pos, top_left: Pos) -> Node<C> {
        let lvl_size = level << 1;
        let mut ret = if lvl_size == self.vol.size {
            self.compute_voxel_node(top_left)

        } else {
            let mut n : Node<C> = Node::default();
            let mut faces : u8 = 0x0;
            let mut color : C = C::default();

            let mut first       = true;
            let mut equal_color = true;
            let mut all_empty   = true;
            for z in 0..2 {
                for y in 0..2 {
                    for x in 0..2 {
                        let n = self.compute_node(
                            level << 1,
                            tree_pos.offs(x, y, z),
                            top_left.offs(
                                x * lvl_size as pint,
                                y * lvl_size as pint,
                                z * lvl_size as pint));

                        if !n.empty { all_empty = false; }
                        if let Some(v) = n.voxel {
                            if first { color = v.color; first = false; }
                            else if color != v.color { equal_color = false; }

                            faces |= xyz2facemask(x, y, z) & v.faces;
//                            println!("RET: {:?} : {}", n, faces);
                        } else {
                            equal_color = false;
                        }
                    }
                }
            }
//            dbg!(all_empty, equal_color, color);

            let mut n = Node::default();
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

            n
        };

        ret.pos = tree_pos;

        *self.node(level, tree_pos) = ret;

        ret
    }

    fn node_at(&self, level: usize, pos: Pos) -> &Node<C> {
        let offs = (level - 1) * (level - 1) * (level - 1);

        &self.nodes[
            offs
            + pos.z as usize * level * level
            + pos.y as usize * level
            + pos.x as usize]
    }

    fn node(&mut self, level: usize, pos: Pos) -> &mut Node<C> {
        // With 4x4x4 voxels, the first level is 1x1x1,
        // 2nd level is: 2x2x2 = 8
        let offs = (level - 1) * (level - 1) * (level - 1);

//        println!("ACCESS {},{},{} => {}@{}", pos.x, pos.y, pos.z, 
//            pos.z * level * level
//            + pos.y * level
//            + pos.x, offs);
        &mut self.nodes[
            offs
            + pos.z as usize * level * level
            + pos.y as usize * level
            + pos.x as usize]
//        println!("ANOD lvl={}, pos={:?} => {:?}", level, pos, self.nodes);
    }

    fn compute_voxel_node(&mut self, top_left: Pos) -> Node<C> {
        let mut faces : u8 = 0x0;
        let mut color : C  = C::default();

        let size = self.vol.size;

        let mut first       = true;
        let mut equal_color = true;
        let mut all_empty   = true;

        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let vox = self.vol.get(top_left.offs(x, y, z));

                    if first { color = vox.color; first = false; }
                    else if color != vox.color { equal_color = false; }
                    if color != C::default() { all_empty = false; }

                    faces |= xyz2facemask(x, y, z) & vox.faces;
//                    eprintln!("{},{},{} :: {:?} :: {:x} {:x}",
//                              x, y, z, top_left, vox.faces, faces);
                }
            }
        }

        let mut n = Node::default();
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

        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_n1_filled() {
        let v : Vol<u8> = Vol::new(2);
        let mut t : Octree<u8> = Octree::new(8, v);
        t.fill(0, 0, 0, 2, 2, 2, 10.into());
        let n = t.recompute();

        assert_eq!(n.voxel.unwrap().color, 10);
        assert_eq!(n.voxel.unwrap().faces, 0x3F);
        assert_eq!(t.nodes[0].voxel.unwrap().color, 10);
        assert_eq!(t.nodes[0].empty, false);

        let mut first : Option<(usize, Pos, Voxel<u8>)> = None;
        let mut last : Option<(usize, Pos, Voxel<u8>)> = None;
        t.draw(&mut |lvl, pos, v| {
            if first.is_none() { first = Some((lvl, *pos, v)); }
            last = Some((lvl, *pos, v));
        });
        let first = first.unwrap();
        assert_eq!((first.0, first.1.x, first.1.y, first.1.z), (2, 0, 0, 0));
        assert_eq!(first.2.color, 10);
        assert_eq!(first.2.faces, 0x3f); // all faces

        let last = last.unwrap();
        assert_eq!((last.0, last.1.x, last.1.y, last.1.z), (2, 0, 0, 0));
        assert_eq!(last.2.color, 10);
        assert_eq!(last.2.faces, 0x3f); // all faces
    }

    #[test]
    fn check_n1_partial_minus_1() {
        let v : Vol<u8> = Vol::new(2);
        let mut t : Octree<u8> = Octree::new(8, v);
        t.fill(0, 0, 0, 2, 2, 2, 10.into());
        t.set(1, 1, 1, 12.into());
        let n = t.recompute();

        assert_eq!(n.voxel, None);
        assert_eq!(t.nodes[0].empty, false);
        assert_eq!(t.nodes[0].voxel, None);
    }

    #[test]
    fn check_n2_filled() {
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
    fn check_n2_broken() {
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
}
