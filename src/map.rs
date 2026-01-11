use crate::game_types::Direction;
use ggez::graphics::{self, Canvas, Color, Rect};

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
pub enum TileType {
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
            TileType::Empty  => Color::from_rgb(255, 255, 255),
            TileType::Goal   => Color::from_rgb(0, 255, 0),
            TileType::Button => Color::from_rgb(0, 0, 255),
            TileType::Flag   => Color::from_rgb(255, 255, 0),
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

        let wall_width = 2.0;
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
#[derive(Clone, Copy)]
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

const T: u8 = 0b1000_0000;
const R: u8 = 0b0100_0000;
const B: u8 = 0b0010_0000;
const L: u8 = 0b0001_0000;
pub fn get_map_list() -> [Map; 2] {
    [
        Map::new(
            (5, 5),
            vec![
                T | L, T, T, T, T | R,

                L, 0, 0, 0, R,

                L, 0, B, 0, R,

                L, 0, T, 0, R,

                L | B, B, B, B, B | R,
            ],
            vec![
                0x0, 0x3, 0x0, 0x0, 0x1,

                0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x2, 0x0, 0x2, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0
            ]
        ),

        Map::new(
            (25, 25),
            vec![
                T | L, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T | R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, R,

                B | L, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B | R
            ],
            vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,

                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
            ]
        )
    ]
}
