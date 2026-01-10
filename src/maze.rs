use crate::{GridPosition, GRID_SIZE};

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Goal,
    Button,
}

pub struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn new() -> Self {
        let mut tiles = vec![vec![Tile::Empty; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize];

        // Example walls
        tiles[3][5] = Tile::Wall;
        tiles[4][5] = Tile::Wall;

        // Goal (bottom-right)
        tiles[(GRID_SIZE.1 - 1) as usize][(GRID_SIZE.0 - 1) as usize] = Tile::Goal;

        // Button
        tiles[2][2] = Tile::Button;

        Self { tiles }
    }

    pub fn tile_at(&self, pos: GridPosition) -> Option<Tile> {
        if pos.x < 0 || pos.y < 0 || pos.x >= GRID_SIZE.0 || pos.y >= GRID_SIZE.1 {
            return None;
        }
        Some(self.tiles[pos.y as usize][pos.x as usize])
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }
}