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

#[derive(Debug, Clone)]
struct Vol<C> where C: VoxelColor {
    size: usize,
    data: std::vec::Vec<Voxel<C>>,
}

impl<C> Vol<C> where C: VoxelColor {
    pub fn get(&mut self, pos: Pos) -> &Voxel<C> {
        let mut faces: u8 = 0x0;

        let clr_def = C::default();
        let size    = self.size;

        if pos.x == 0    { faces |= 0x08; }
        if pos.x == size { faces |= 0x10; }
        if pos.x > 0 && pos.x < size {
            let clr1 = self.data[pos.z * size * size + pos.y * size + pos.x - 1].color;
            let clr2 = self.data[pos.z * size * size + pos.y * size + pos.x + 1].color;
            if clr1 == clr_def { faces |= 0x08; }
            if clr2 == clr_def { faces |= 0x10; }
        }

        if pos.y == 0         { faces |= 0x20; }
        else if pos.y == size { faces |= 0x02; }
        if pos.y > 0 && pos.y < size {
            let clr1 = self.data[pos.z * size * size + (pos.y - 1) * size].color;
            let clr2 = self.data[pos.z * size * size + (pos.y + 1) * size].color;
            if clr1 == clr_def { faces |= 0x20; }
            if clr2 == clr_def { faces |= 0x02; }
        }

        if pos.z == 0         { faces |= 0x01; }
        else if pos.z == size { faces |= 0x04; }
        if pos.z > 0 && pos.z < size {
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
}

impl<C> Node<C> where C: VoxelColor {
    pub fn new() -> Self {
        Self {
            voxel: None,
            pos: Pos::default(),
        }
    }
}

struct Tree<C: VoxelColor> {
    nodes: std::vec::Vec<Node<C>>,
    vol:   Vol<C>,
}

impl<C> Tree<C> where C: VoxelColor {
    fn new(node_count: usize, vol: Vol<C>) -> Self {
        let mut nodes = std::vec::Vec::new();
        nodes.resize(node_count, Node::default());
        Self {
            nodes,
            vol,
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
    fn compute_node(&mut self, level: usize, top_left: Pos) -> Node<C> {
        let mut n = Node::default();
        let lvl_size = level << 1;
        if lvl_size == self.vol.size {
        } else {
            let mut faces : u8 = 0x0;
            let mut color : Option<C> = None;

            let n0 =
                self.compute_node(level << 1, Pos {
                    x: top_left.x,
                    y: top_left.y,
                    z: top_left.z,
                });
            // merge appropriate faces (no inner faces)

            let n1 =
                self.compute_node(level << 1, Pos {
                    x: top_left.x + lvl_size,
                    y: top_left.y,
                    z: top_left.z,
                });

            // if we all have different color, assign no color
        }

        n
    }

    fn compute_voxel_node(&mut self, top_left: Pos) -> Node<C> {
        let mut faces : u8 = 0x0;
        let mut color : C  = C::default();

        let size = self.vol.size;

        let mut first       = true;
        let mut equal_color = true;

        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let vox = self.vol.get(Pos {
                        x: top_left.x + x,
                        y: top_left.y + y,
                        z: top_left.z + z
                    });

                    if first { color = vox.color; }
                    else if color != vox.color { equal_color = false; }

                    // TODO: Merge only appropriate faces!

                    faces |= vox.faces;
                }
            }
        }

        let mut n = Node::default();
//        if equal_color {
//            n.voxel.faces = faces;
//        }
        n
    }
}
