trait VoxelColor: PartialEq + Sized + Copy + Default {}
impl<T: PartialEq + Sized + Copy + Default> VoxelColor for T {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Voxel<C> where C: VoxelColor {
    color: C,
    /// Bits:
    /// Front,  // x,       y,      z - 1
    /// Top,    // x,       y + 1,  z
    /// Back,   // x,       y,      z + 1
    /// Left,   // x - 1,   y,      z
    /// Right,  // x + 1,   y,      z
    /// Bottom, // x,       y - 1,  z
    /// - 0x01     - Front
    /// - 0x02     - Top
    /// - 0x04     - Back
    /// - 0x08     - Left
    /// - 0x10     - Right
    /// - 0x20     - Bottom
    faces: u8,
}

impl<C> Voxel<C> where C: VoxelColor {
}

fn xyz2facemask(x: usize, y: usize, z: usize) -> u8 {
    let mut mask : u8 = 0x00;
    if x == 0 { mask |= 0x08; }
    else      { mask |= 0x10; }
    if y == 0 { mask |= 0x20; }
    else      { mask |= 0x02; }
    if z == 0 { mask |= 0x01; }
    else      { mask |= 0x04; }
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
struct Vol<C> where C: VoxelColor {
    size: usize,
    data: std::vec::Vec<Voxel<C>>,
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

    pub fn set(&mut self, x: usize, y: usize, z: usize, v: Voxel<C>) {
        self.data[z * self.size * self.size + y * self.size + x] = v;
    }

    pub fn at(&self, pos: Pos) -> &Voxel<C> {
        &self.data[pos.z * self.size * self.size + pos.y * self.size + pos.x - 1]
    }

    pub fn get(&mut self, pos: Pos) -> &Voxel<C> {
        let mut faces: u8 = 0x0;

        let clr_def = C::default();
        let size    = self.size;
        let last    = self.size - 1;

        if pos.x == 0    { faces |= 0x08; }
        if pos.x == last { faces |= 0x10; }
        if pos.x > 0 && pos.x < last {
            let clr1 = self.data[pos.z * size * size + pos.y * size + pos.x - 1].color;
            let clr2 = self.data[pos.z * size * size + pos.y * size + pos.x + 1].color;
            if clr1 == clr_def { faces |= 0x08; }
            if clr2 == clr_def { faces |= 0x10; }
        }

        if pos.y == 0         { faces |= 0x20; }
        else if pos.y == last { faces |= 0x02; }
        if pos.y > 0 && pos.y < last {
            let clr1 = self.data[pos.z * size * size + (pos.y - 1) * size].color;
            let clr2 = self.data[pos.z * size * size + (pos.y + 1) * size].color;
            if clr1 == clr_def { faces |= 0x20; }
            if clr2 == clr_def { faces |= 0x02; }
        }

        if pos.z == 0         { faces |= 0x01; }
        else if pos.z == last { faces |= 0x04; }
        if pos.z > 0 && pos.z < last {
            let clr1 = self.data[(pos.z - 1) * size * size + pos.y * size + pos.x].color;
            let clr2 = self.data[(pos.z + 1) * size * size + pos.y * size + pos.x].color;
            if clr1 == clr_def { faces |= 0x01; }
            if clr2 == clr_def { faces |= 0x04; }
        }

        let vox = &mut self.data[pos.z * size * size + pos.y * size + pos.x];
        vox.faces = faces;
        vox
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Node<C: VoxelColor> {
    voxel: Option<Voxel<C>>,
    pos:   Pos,
    empty: bool,
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
struct Tree<C: VoxelColor> {
    nodes: std::vec::Vec<Node<C>>,
    nodes_size: usize,
    vol:   Vol<C>,
}

impl<C> Tree<C> where C: VoxelColor {
    fn new(node_count: usize, vol: Vol<C>) -> Self {
        let mut size = vol.size >> 1;
        let mut alloc = 0;
        while size > 0 {
            alloc += size;
            size = size >> 1;
        }
        let mut nodes = std::vec::Vec::new();
        nodes.resize(size, Node::default());
        Self {
            nodes,
            nodes_size: vol.size >> 1,
            vol,
        }
    }

    pub fn draw<F>(&self, f: &mut F) where F: FnMut(usize, usize, usize, usize, Voxel<C>) -> () {
        self.draw_level(1, Pos { x: 0, y: 0, z: 0 }, f);
    }

    pub fn draw_level<F>(&self, level: usize, top_left: Pos, f: &mut F)
        where F: FnMut(usize, usize, usize, usize, Voxel<C>) -> ()
    {
        let lvl_size = level << 1;
        if lvl_size == self.vol.size {
            for z in 0..2 {
                for y in 0..2 {
                    for x in 0..2 {
                        f(lvl_size, top_left.x + x, top_left.y + y, top_left.z + z,
                          *self.vol.at(Pos {
                               x: top_left.x + x,
                               y: top_left.y + y,
                               z: top_left.z + z
                           }));
                    }
                }
            }
        } else {
            let n =
                self.nodes[
                    top_left.z * (self.vol.size >> 1) * (self.vol.size >> 1)
                    + top_left.y * (self.vol.size >> 1)
                    + top_left.x];
            if n.empty { return; }
            if let Some(v) = n.voxel {
                f(lvl_size, top_left.x, top_left.y, top_left.z, v);

            } else {
                for z in 0..2 {
                    for y in 0..2 {
                        for x in 0..2 {
                            self.draw_level(
                                level << 1, 
                                Pos {
                                    x: top_left.x + x,
                                    y: top_left.y * lvl_size + y,
                                    z: top_left.z * lvl_size * lvl_size + z,
                                },
                                f);
                        }
                    }
                }
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, v: Voxel<C>) {
        self.vol.set(x, y, z, v);
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
    fn compute_node(&mut self, level: usize, top_left: Pos) -> Node<C> {
        let lvl_size = level << 1;
        let ret = if lvl_size == self.vol.size {
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
                        let n = self.compute_node(level << 1, Pos {
                            x: top_left.x + x,
                            y: top_left.y * lvl_size + y,
                            z: top_left.z * lvl_size * lvl_size + z,
                        });

                        if !n.empty { all_empty = false; }
                        if let Some(v) = n.voxel {
                            if first { color = v.color; first = false; }
                            else if color != v.color { equal_color = false; }

                            faces |= xyz2facemask(x, y, z) & v.faces;
                        } else {
                            equal_color = false;
                        }
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
        };

        self.nodes[
            top_left.z * (self.vol.size >> 1) * (self.vol.size >> 1)
            + top_left.y * (self.vol.size >> 1)
            + top_left.x] = ret;

        ret
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
                    eprintln!("{},{},{} :: {:?}", x, y, z, top_left);
                    let vox = self.vol.get(Pos {
                        x: top_left.x + x,
                        y: top_left.y + y,
                        z: top_left.z + z
                    });

                    if first { color = vox.color; first = false; }
                    else if color != vox.color { equal_color = false; }
                    if color != C::default() { all_empty = false; }

                    faces |= xyz2facemask(x, y, z) & vox.faces;
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
        let mut t : Tree<u8> = Tree::new(8, v);
        t.set(0, 0, 0, 10.into());
        t.set(0, 1, 0, 10.into());
        t.set(1, 0, 0, 10.into());
        t.set(1, 1, 0, 10.into());
        t.set(0, 0, 1, 10.into());
        t.set(0, 1, 1, 10.into());
        t.set(1, 0, 1, 10.into());
        t.set(1, 1, 1, 10.into());
        let n = t.compute_node(1, Pos { x: 0, y: 0, z: 0 });

        assert_eq!(n.voxel.unwrap().color, 10);
        assert_eq!(n.voxel.unwrap().faces, 0x3F);
    }

    #[test]
    fn check_n1_partial_minus_1() {
        let v : Vol<u8> = Vol::new(2);
        let mut t : Tree<u8> = Tree::new(8, v);
        t.set(0, 0, 0, 10.into());
        t.set(0, 1, 0, 10.into());
        t.set(1, 0, 0, 10.into());
        t.set(1, 1, 0, 10.into());
        t.set(0, 0, 1, 10.into());
        t.set(0, 1, 1, 10.into());
        t.set(1, 0, 1, 10.into());
        t.set(1, 1, 1, 12.into());
        let n = t.compute_node(1, Pos { x: 0, y: 0, z: 0 });

        assert_eq!(n.voxel, None);
    }
}
