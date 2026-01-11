use ggez::GameResult;
use std::{env, path};
use crate::map::GRID_SIZE;

mod assets;
mod gamestate;
mod map;
mod tank;
mod sidebar;
mod menu;
mod screen;

use tank::Direction;
use screen::{ScreenManager, SCREEN_SIZE};

// ==========================
// Constants
// ==========================
const CELL_SIZE: u16 = 180;

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = GRID_SIZE.0 as f32 * CELL_SIZE as f32;
pub const MAP_HEIGHT: f32 = GRID_SIZE.1 as f32 * CELL_SIZE as f32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Plan,
    Execution,
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

    let screen_manager = ScreenManager::new(&mut ctx, SCREEN_SIZE.0, SCREEN_SIZE.1)?;
    ggez::event::run(ctx, event_loop, screen_manager)
}
