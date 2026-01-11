use ggez::GameResult;
use winit::keyboard::{Key, NamedKey};
use std::{env, path};
use crate::map::GRID_SIZE;

mod assets;
mod gamestate;
mod map;
mod tank;
mod sidebar;

use gamestate::GameState;
use tank::Direction;

// ==========================
// Constants
// ==========================
const CELL_SIZE: u16 = 45;

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = GRID_SIZE.0 as f32 * CELL_SIZE as f32;
pub const MAP_HEIGHT: f32 = GRID_SIZE.1 as f32 * CELL_SIZE as f32;

const SCREEN_SIZE: (f32, f32) = (
    MAP_WIDTH + SIDEBAR_WIDTH,
    MAP_HEIGHT,
);

pub const TURN_INSTRUCTIONS: usize = 10;

// ==========================
// Instructions
// ==========================

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
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("tank_maze", "newline")
        .window_setup(ggez::conf::WindowSetup::default().title("Tank Maze"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = GameState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
