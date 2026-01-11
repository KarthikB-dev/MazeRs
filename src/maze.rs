use crate::{GRID_SIZE};

const WALL_TOP:    u8 = 0b1000_0000;
const WALL_RIGHT:  u8 = 0b0100_0000;
const WALL_BOTTOM: u8 = 0b0010_0000;
const WALL_LEFT:   u8 = 0b0001_0000;

const WALL_MASK: u8 = 0b1111_0000;
const TYPE_MASK: u8 = 0b0000_1111;

// ==========================
// Tiles
// ==========================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum TileType {
    Empty  = 0x0,
    Goal   = 0x1,
    Button = 0x2,
    Flag   = 0x3,
}

impl TileType {
    fn from_u8(v: u8) -> Self {
        match v & TYPE_MASK {
            0x1 => TileType::Goal,
            0x2 => TileType::Button,
            0x3 => TileType::Flag,
            _   => TileType::Empty,
        }
    }
}

#[derive(Clone, Copy)]
struct Tile(u8);

impl Tile {
    const fn new(walls: u8, tile_type: TileType) -> Self {
        Tile(walls | tile_type as u8)
    }

    const fn empty() -> Self {
        Tile(0)
    }

    fn has_wall_top(self) -> bool {
        self.0 & WALL_TOP != 0
    }

    fn has_wall_right(self) -> bool {
        self.0 & WALL_RIGHT != 0
    }

    fn has_wall_bottom(self) -> bool {
        self.0 & WALL_BOTTOM != 0
    }

    fn has_wall_left(self) -> bool {
        self.0 & WALL_LEFT != 0
    }

    fn tile_type(self) -> TileType {
        TileType::from_u8(self.0)
    }
}

// ==========================
// Map
// ==========================
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.width + x]
    }
}

pub fn hardcoded_map() -> Map {
    let width = GRID_SIZE.0 as usize;
    let height = GRID_SIZE.1 as usize;

    let walls = vec![
        WALL_TOP | WALL_LEFT, WALL_TOP, WALL_TOP, WALL_TOP, WALL_TOP | WALL_RIGHT,

        WALL_LEFT, 0, 0, 0, WALL_RIGHT,

        WALL_LEFT, 0, 0, 0, WALL_RIGHT,

        WALL_LEFT, 0, 0, 0, WALL_RIGHT,

        WALL_BOTTOM | WALL_LEFT, WALL_BOTTOM, WALL_BOTTOM, WALL_BOTTOM, WALL_BOTTOM | WALL_RIGHT,
    ];

    let types = vec![
        0x0, 0x3, 0x0, 0x0, 0x1,

        0x0, 0x0, 0x0, 0x0, 0x0,

        0x0, 0x2, 0x0, 0x2, 0x0,

        0x0, 0x0, 0x0, 0x0, 0x0,

        0x0, 0x0, 0x0, 0x0, 0x0
    ];

    assert_eq!(walls.len(), width * height, "walls length mismatch");
    assert_eq!(types.len(), width * height, "types length mismatch");

    let tiles = walls
        .into_iter()
        .zip(types.into_iter())
        .map(|(w, t)| Tile::new(w, TileType::from_u8(t)))
        .collect();

    Map { width, height, tiles }
}
