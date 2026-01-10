use ggez::{
    event,
    graphics,
    input::keyboard::KeyInput,
    Context, GameResult,
};
use winit::keyboard::{Key, NamedKey};

use std::collections::VecDeque;

// ==========================
// Constants
// ==========================
const GRID_SIZE: (i16, i16) = (15, 10);
const CELL_SIZE: i16 = 32;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * CELL_SIZE as f32,
    GRID_SIZE.1 as f32 * CELL_SIZE as f32,
);

const TURN_INSTRUCTIONS: usize = 10;
const DESIRED_FPS: u32 = 10;

// ==========================
// Grid Position
// ==========================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn moved(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * CELL_SIZE as i32,
            pos.y as i32 * CELL_SIZE as i32,
            CELL_SIZE as i32,
            CELL_SIZE as i32,
        )
    }
}

// ==========================
// Directions & Instructions
// ==========================
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Move(Direction),
    Noop,
    Interact,
}

impl Instruction {
    fn from_key(key: &Key) -> Option<Self> {
        match key {
            Key::Named(NamedKey::ArrowUp) => Some(Self::Move(Direction::Up)),
            Key::Named(NamedKey::ArrowDown) => Some(Self::Move(Direction::Down)),
            Key::Named(NamedKey::ArrowLeft) => Some(Self::Move(Direction::Left)),
            Key::Named(NamedKey::ArrowRight) => Some(Self::Move(Direction::Right)),
            Key::Character(c) if c == "e" => Some(Self::Interact),
            Key::Character(c) if c == " " => Some(Self::Noop),
            _ => None,
        }
    }
}

// ==========================
// Maze
// ==========================
#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Goal,
    Button,
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn new() -> Self {
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

    fn tile_at(&self, pos: GridPosition) -> Option<Tile> {
        if pos.x < 0 || pos.y < 0 || pos.x >= GRID_SIZE.0 || pos.y >= GRID_SIZE.1 {
            return None;
        }
        Some(self.tiles[pos.y as usize][pos.x as usize])
    }
}

// ==========================
// Tank
// ==========================
struct Tank {
    pos: GridPosition,
}

impl Tank {
    fn new(pos: GridPosition) -> Self {
        Self { pos }
    }
}

// ==========================
// Game State
// ==========================
struct GameState {
    maze: Maze,
    tank: Tank,
    instruction_queue: VecDeque<Instruction>,
    game_won: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            maze: Maze::new(),
            tank: Tank::new(GridPosition::new(0, 0)),
            instruction_queue: VecDeque::new(),
            game_won: false,
        }
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Move(dir) => {
                let new_pos = self.tank.pos.moved(dir);
                if let Some(tile) = self.maze.tile_at(new_pos) {
                    if !matches!(tile, Tile::Wall) {
                        self.tank.pos = new_pos;
                    }
                }
            }
            Instruction::Interact => {
                if let Some(Tile::Goal) = self.maze.tile_at(self.tank.pos) {
                    self.game_won = true;
                }
                // Buttons do nothing yet
            }
            Instruction::Noop => {}
        }
    }
}

// ==========================
// EventHandler
// ==========================
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if !self.game_won {
                if let Some(instr) = self.instruction_queue.pop_front() {
                    self.execute_instruction(instr);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

        // Draw maze
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                let tile = self.maze.tiles[y as usize][x as usize];
                let color = match tile {
                    Tile::Empty => [0.2, 0.2, 0.2, 1.0],
                    Tile::Wall => [0.0, 0.0, 0.0, 1.0],
                    Tile::Goal => [0.0, 1.0, 0.0, 1.0],
                    Tile::Button => [0.0, 0.0, 1.0, 1.0],
                };
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(GridPosition::new(x, y).into())
                        .color(color),
                );
            }
        }

        // Draw tank
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.tank.pos.into())
                .color([1.0, 0.0, 0.0, 1.0]),
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _: bool) -> GameResult {
        if self.instruction_queue.len() < TURN_INSTRUCTIONS {
            if let Some(instr) = Instruction::from_key(&input.event.logical_key) {
                self.instruction_queue.push_back(instr);
            }
        }
        Ok(())
    }
}

// ==========================
// Main
// ==========================

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("tank_maze", "you")
        .window_setup(ggez::conf::WindowSetup::default().title("Tank Maze"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1),
        )
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}

