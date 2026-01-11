use ggez::GameResult;
use std::{env, path};

mod assets;
mod game_types;
mod gamestate;
mod map;
mod tank;
mod sidebar;
mod menu;
mod screen;
mod network;

use screen::{ScreenManager, SCREEN_SIZE};
use gamestate::GameState;


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
        .build()?

    let screen_manager = ScreenManager::new(&mut ctx, SCREEN_SIZE.0, SCREEN_SIZE.1)?;
    ggez::event::run(ctx, event_loop, screen_manager)
    let state = GameState::new(network_manager);
    ggez::event::run(ctx, event_loop, screen_manager)
}
