

struct UnitTypeDefinition {
    base_price: i32,
    color:      (u8, u8, u8),
}

struct UnitTypeCollection {
    types: std::vec::Vec<UnitTypeDefinition>,
}

type UnitType = usize;

struct Block {
    size: usize,
    topology: std::vec::Vec<UnitType>,
}

impl Block {
    fn new(size: usize) -> Self {
        Block {
            size,
            topology: vec![0; size],
        }
    }
}
