use crate::game_types::Direction;
use ggez::graphics::{self, Canvas, Color, Rect};
use rand::Rng;

const WALL_TOP:    u8 = 0b1000_0000;
const WALL_RIGHT:  u8 = 0b0100_0000;
const WALL_BOTTOM: u8 = 0b0010_0000;
const WALL_LEFT:   u8 = 0b0001_0000;

const TYPE_MASK: u8 = 0b0000_1111;

// ==========================
// Tiles
// ==========================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TileType {
    Empty  = 0x0,
    Button = 0x1,
    LocalFlag = 0x2,
    RemoteFlag = 0x3,
}

impl TileType {
    fn from_u8(v: u8) -> Self {
        match v & TYPE_MASK {
            0x1 => TileType::Button,
            0x2 => TileType::LocalFlag,
            0x3 => TileType::RemoteFlag,
            _   => TileType::Empty,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile(u8);

impl Tile {
    const fn new(walls: u8, tile_type: TileType) -> Self {
        Tile(walls | tile_type as u8)
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

    pub fn draw(self, canvas: &mut Canvas, x: u16, y: u16, cell_size: f32) {
        let tile_type = self.tile_type();
        let tile_color = match tile_type {
            TileType::Empty      => Color::from_rgb(255, 255, 255),
            TileType::Button     => Color::from_rgb(0, 0, 255),
            TileType::LocalFlag  => Color::from_rgb(0, 255, 0),
            TileType::RemoteFlag => Color::from_rgb(255, 255, 0),
        };

        let tile_rect = Rect::new(
            x as f32 * cell_size,
            y as f32 * cell_size,
            cell_size,
            cell_size,
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(tile_rect)
                .color(tile_color),
        );

        let wall_width = 5.0;
        let wall_color = Color::BLACK;

        if self.has_wall_top() {
            let wall_rect = Rect::new(
                x as f32 * cell_size,
                y as f32 * cell_size,
                cell_size,
                wall_width,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(wall_rect)
                    .color(wall_color),
            );
        }

        if self.has_wall_bottom() {
            let wall_rect = Rect::new(
                x as f32 * cell_size,
                y as f32 * cell_size + cell_size - wall_width,
                cell_size,
                wall_width,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(wall_rect)
                    .color(wall_color),
            );
        }

        if self.has_wall_left() {
            let wall_rect = Rect::new(
                x as f32 * cell_size,
                y as f32 * cell_size,
                wall_width,
                cell_size,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(wall_rect)
                    .color(wall_color),
            );
        }

        if self.has_wall_right() {
            let wall_rect = Rect::new(
                x as f32 * cell_size + cell_size - wall_width,
                y as f32 * cell_size,
                wall_width,
                cell_size,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(wall_rect)
                    .color(wall_color),
            );
        }
    }
}

// ==========================
// Map
// ==========================
#[derive(Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(grid_size: (usize, usize), walls: Vec<u8>, types: Vec<u8>) -> Map {
        let width = grid_size.0;
        let height = grid_size.1;

        assert_eq!(walls.len(), width * height, "walls length mismatch");
        assert_eq!(types.len(), width * height, "types length mismatch");

        let mut tiles = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let i = y * width + x;
                row.push(Tile::new(
                    walls[i],
                    TileType::from_u8(types[i]),
                ));
            }
            tiles.push(row);
        }

        Map { width, height, tiles }
    }

    pub fn get(&self, pos: MapPos) -> Option<Tile> {
        self.tiles
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .copied()
    }
}

// ==========================
// Tiles
// ==========================
#[derive(Clone, Copy)]
pub struct MapPos {
    pub x: u16,
    pub y: u16,
}

impl MapPos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn moved(self, map: &Map, dir: Direction) -> Self {
        match dir {
            Direction::Up => {
                if self.y == 0 || map.get(self).unwrap().has_wall_top() {
                    Self::new(self.x, self.y)
                } else {
                    Self::new(self.x, self.y - 1)
                }
            },
            Direction::Down  => {
                if self.y == map.height as u16 - 1 || map.get(self).unwrap().has_wall_bottom() {
                    Self::new(self.x, self.y)
                } else {
                    Self::new(self.x, self.y + 1)
                }
            }
            Direction::Left  => {
                if self.x == 0 || map.get(self).unwrap().has_wall_left() {
                    Self::new(self.x, self.y)
                } else {
                    Self::new(self.x - 1, self.y)
                }
            }
            Direction::Right => {
                if self.x == map.width as u16 - 1 || map.get(self).unwrap().has_wall_right() {
                    Self::new(self.x, self.y)
                } else {
                    Self::new(self.x + 1, self.y)
                }
            }
        }
    }
}

// ==========================
// Tiles
// ==========================
pub fn generate_random_map(width: usize, height: usize) -> Map {
    let mut rng = rand::rng();

    let mut walls = vec![0u8; width * height];
    let mut types = vec![0u8; width * height];

    // Helper to get index
    let idx = |x: usize, y: usize| -> usize { y * width + x };

    // Set outer walls
    for x in 0..width {
        walls[idx(x, 0)]        |= WALL_TOP;
        walls[idx(x, height-1)] |= WALL_BOTTOM;
    }
    for y in 0..height {
        walls[idx(0, y)]        |= WALL_LEFT;
        walls[idx(width-1, y)]  |= WALL_RIGHT;
    }

    // Random internal walls
    for y in 0..height {
        for x in 0..width {
            if (x < 3 && y < 3) || (x > width - 3 && y > height - 3) {
                continue;
            }
            let mut cell = 0u8;
            if rng.random_bool(0.15) { cell |= WALL_TOP;    if y > 0        { walls[idx(x, y-1)] |= WALL_BOTTOM; } }
            if rng.random_bool(0.15) { cell |= WALL_BOTTOM; if y < height-1 { walls[idx(x, y+1)] |= WALL_TOP; } }
            if rng.random_bool(0.15) { cell |= WALL_LEFT;   if x > 0        { walls[idx(x-1, y)] |= WALL_RIGHT; } }
            if rng.random_bool(0.15) { cell |= WALL_RIGHT;  if x < width-1  { walls[idx(x+1, y)] |= WALL_LEFT; } }
            walls[idx(x, y)] |= cell;
        }
    }

    // Randomly place special tiles
    let place_tiles = |tile: TileType, count: usize, types: &mut Vec<u8>| {
        let mut placed = 0;
        let mut rng = rand::rng();
        while placed < count {
            let x = rng.random_range(1..width-1);
            let y = rng.random_range(1..height-1);
            let i = idx(x, y);
            if types[i] == 0 {
                types[i] = tile as u8;
                placed += 1;
            }
        }
    };
    place_tiles(TileType::Button, 8, &mut types);
    types[0] = TileType::RemoteFlag as u8;
    types[width * height - 1] = TileType::LocalFlag as u8;

    Map::new((width, height), walls, types)
}
