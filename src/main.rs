use ggez::GameResult;
mod gamestate;
mod maze;
mod tank;
mod sidebar;
mod network;

use gamestate::GameState;
use crate::network::start_network;

pub const GRID_SIZE: (i16, i16) = (15, 10);
const CELL_SIZE: i16 = 45;
pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = GRID_SIZE.0 as f32 * CELL_SIZE as f32;
pub const MAP_HEIGHT: f32 = GRID_SIZE.1 as f32 * CELL_SIZE as f32;
const SCREEN_SIZE: (f32, f32) = (MAP_WIDTH + SIDEBAR_WIDTH, MAP_HEIGHT);
pub const TURN_INSTRUCTIONS: usize = 10;
pub const DESIRED_FPS: u32 = 60;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition { pub x: i16, pub y: i16 }
impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self { Self { x, y } }
    pub fn moved(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }
}
impl From<GridPosition> for ggez::graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        ggez::graphics::Rect::new_i32(pos.x as i32 * CELL_SIZE as i32, pos.y as i32 * CELL_SIZE as i32, CELL_SIZE as i32, CELL_SIZE as i32)
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Direction { Up, Down, Left, Right }
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Instruction { Move(Direction), Noop, Interact }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase { Plan, Waiting, Execution }

#[tokio::main]
async fn main() -> GameResult {
    let network_manager = match start_network().await {
        Ok(nm) => nm,
        Err(e) => {
            eprintln!("Network setup failed: {:?}", e);
            return Ok(());
        }
    };

    let (ctx, event_loop) = ggez::ContextBuilder::new("tank_maze", "you")
        .window_setup(ggez::conf::WindowSetup::default().title("Tank Maze P2P"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new(network_manager);
    ggez::event::run(ctx, event_loop, state)
}
