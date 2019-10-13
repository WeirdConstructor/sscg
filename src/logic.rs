struct UnitTypeDefinition {
    base_price: i32,
    color:      (u8, u8, u8),
}

struct UnitTypes {
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

struct EngineDefinition {
    power:          i32,
    fuel_capacity:  u32,
    fuel:           u32,
}

struct Cargo {
    orientation: u8,
    x: usize,
    y: usize,
    data: Block,
}

struct Module {
    size:       usize,
    topology:   std::vec::Vec<bool>,
    cargo:      std::vec::Vec<Cargo>,
}

struct Ship {
    pos:             (i32, i32),
    engines:         std::vec::Vec<EngineDefinition>,
    size:            usize,
    module_topology: std::vec::Vec<Option<Module>>,
}

struct Robot {
    pos:                (i32, i32),
    path:               std::vec::Vec<(i32, i32)>,
    path_pos:           usize,
    /// If their charge runs out, they loose their cargo and move very slowly back to the ship
    charge:             usize,
    charge_capacity:    usize,
    cargo:              Option<Block>,
}

struct UnitSource {
    unit_count:         usize,
    block_unit_type:    usize,
}

struct TerrainTypeDefinition {
    blocked: bool,
    typ:     usize,
    color:   (u8, u8, u8),
}

struct TerrainTypes {
    typs: std::vec::Vec<TerrainTypeDefinition>,
}

type TerrainType = usize;

struct Cell {
    src:        Option<UnitSource>,
    terrain:    TerrainType,
}

struct Map {
    pos:        (i32, i32),
    width:      usize,
    height:     usize,
    data:       std::vec::Vec<Cell>,
}

struct SpaceStation {
    pos:        (i32, i32),
}

struct Starmap {
    stations:   std::vec::Vec<SpaceStation>,
    width:      usize,
    height:     usize,
}

struct Game {
    units:      UnitTypes,
    terrains:   TerrainTypes,
    ship:       Ship,
    map:        Starmap,
    planets:    std::vec::Vec<Map>,
    credits:    i32,
}
