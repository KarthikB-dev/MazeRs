use crate::{GridPosition, GRID_SIZE};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
}

pub struct Maze {
    tiles: Vec<Vec<Tile>>,
    pub p1_start: GridPosition,
    pub p1_goal: GridPosition,
    pub p2_start: GridPosition,
    pub p2_goal: GridPosition,
}

impl Maze {
    pub fn new() -> Self {
        let mut tiles = vec![vec![Tile::Empty; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize];

        // Example walls
        tiles[3][5] = Tile::Wall;
        tiles[4][5] = Tile::Wall;
        tiles[5][5] = Tile::Wall;
        tiles[3][8] = Tile::Wall;
        tiles[4][8] = Tile::Wall;

        let p1_start = GridPosition::new(0, 0);
        let p1_goal = GridPosition::new(GRID_SIZE.0 - 1, GRID_SIZE.1 - 1);
        let p2_start = GridPosition::new(GRID_SIZE.0 - 1, 0); // Top right instead
        let p2_goal = GridPosition::new(0, GRID_SIZE.1 - 1);

        Self { 
            tiles, 
            p1_start, 
            p1_goal,
            p2_start,
            p2_goal,
        }
    }

    pub fn tile_at(&self, pos: GridPosition) -> Option<Tile> {
        if pos.x < 0 || pos.y < 0 || pos.x >= GRID_SIZE.0 || pos.y >= GRID_SIZE.1 {
            return None;
        }
        Some(self.tiles[pos.y as usize][pos.x as usize])
    }
}
