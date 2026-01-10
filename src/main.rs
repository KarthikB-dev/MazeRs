use ggez::{
    graphics,
    GameResult,
};
use winit::keyboard::{Key, NamedKey};

mod gamestate;
mod maze;
mod tank;
mod sidebar;

use gamestate::GameState;

// ==========================
// Constants
// ==========================
pub const GRID_SIZE: (i16, i16) = (15, 10);
const CELL_SIZE: i16 = 45;

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = GRID_SIZE.0 as f32 * CELL_SIZE as f32;
pub const MAP_HEIGHT: f32 = GRID_SIZE.1 as f32 * CELL_SIZE as f32;

const SCREEN_SIZE: (f32, f32) = (
    MAP_WIDTH + SIDEBAR_WIDTH,
    MAP_HEIGHT,
);

pub const TURN_INSTRUCTIONS: usize = 10;
pub const DESIRED_FPS: u32 = 10;

// ==========================
// Grid Position
// ==========================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn moved(self, dir: Direction) -> Self {
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Move(Direction),
    Noop,
    Interact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Plan,
    Execution,
}

impl Instruction {
    pub fn from_key(key: &Key) -> Option<Self> {
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
    ggez::event::run(ctx, event_loop, state)
}

