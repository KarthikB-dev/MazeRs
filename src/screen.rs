use ggez::{event, Context, GameResult};
use crate::menu::{MenuState, MenuAction};
use crate::gamestate::GameState;
use crate::map::GRID_SIZE;

pub const CELL_SIZE: u16 = 180;
pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = GRID_SIZE.0 as f32 * CELL_SIZE as f32;
pub const MAP_HEIGHT: f32 = GRID_SIZE.1 as f32 * CELL_SIZE as f32;

pub const SCREEN_SIZE: (f32, f32) = (
    MAP_WIDTH + SIDEBAR_WIDTH,
    MAP_HEIGHT,
);

pub enum Screen {
    Menu(MenuState),
    Game(GameState),
}

pub struct ScreenManager {
    current_screen: Screen,
    pending_game_start: bool,
}

impl ScreenManager {
    pub fn new(_ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Self> {
        Ok(Self {
            current_screen: Screen::Menu(MenuState::new(screen_width, screen_height)),
            pending_game_start: false,
        })
    }

    fn transition_to_game(&mut self, ctx: &mut Context) -> GameResult {
        match GameState::new(ctx) {
            Ok(game_state) => {
                self.current_screen = Screen::Game(game_state);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to create game state: {:?}", e);
                Err(e)
            }
        }
    }
}

impl event::EventHandler for ScreenManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Handle pending game start
        if self.pending_game_start {
            self.pending_game_start = false;
            self.transition_to_game(ctx)?;
        }

        match &mut self.current_screen {
            Screen::Menu(menu) => menu.update(ctx),
            Screen::Game(game) => game.update(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.current_screen {
            Screen::Menu(menu) => menu.draw(ctx),
            Screen::Game(game) => game.draw(ctx),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.current_screen {
            Screen::Menu(menu) => {
                let action = menu.handle_mouse_button_down(x, y);
                match action {
                    MenuAction::StartGame => {
                        self.pending_game_start = true;
                    }
                    MenuAction::Quit => {
                        ctx.request_quit();
                    }
                    MenuAction::None => {}
                }
                Ok(())
            }
            Screen::Game(game) => game.mouse_button_down_event(ctx, button, x, y),
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.current_screen {
            Screen::Menu(menu) => {
                menu.handle_mouse_button_up(x, y);
                Ok(())
            }
            Screen::Game(game) => game.mouse_button_up_event(ctx, button, x, y),
        }
    }
}
