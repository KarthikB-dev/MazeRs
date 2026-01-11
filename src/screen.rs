use ggez::{event, Context, GameResult, input::keyboard};
use crate::menu::{MenuState, MenuAction};
use crate::gamestate::GameState;

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = 1250.0;
pub const MAP_HEIGHT: f32 = 1250.0;
pub const CELL_SIZE: u16 = 180;

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
    pending_game_state: Option<GameState>,
}

impl ScreenManager {
    pub fn new(_ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Self> {
        Ok(Self {
            current_screen: Screen::Menu(MenuState::new(screen_width, screen_height)),
            pending_game_state: None,
        })
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.pending_game_state = Some(game_state);
    }

    fn transition_to_game(&mut self) -> GameResult {
        if let Some(game_state) = self.pending_game_state.take() {
            self.current_screen = Screen::Game(game_state);
            Ok(())
        } else {
            Err(ggez::GameError::CustomError("No game state available".to_string()))
        }
    }
}

impl event::EventHandler for ScreenManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Handle pending game start
        if self.pending_game_state.is_some() {
            self.transition_to_game()?;
        }

        match &mut self.current_screen {
            Screen::Menu(menu) => menu.update(ctx),
            Screen::Game(game) => game.update(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.current_screen {
            Screen::Menu(menu) => {
                let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::from([0.1, 0.1, 0.1, 1.0]));
                menu.draw(ctx, &mut canvas)?;
                canvas.finish(ctx)?;
                Ok(())
            }
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
                let action = menu.handle_click(x, y);

                match action {
                    MenuAction::StartGame => {
                        // Game creation handled by main.rs
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

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        if let Screen::Menu(ref mut menu) = self.current_screen {
            if !character.is_control() {
                menu.handle_text_input(&character.to_string());
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Screen::Menu(ref mut menu) = self.current_screen {
            if let Some(keycode) = input.keycode {
                if keycode == keyboard::KeyCode::Back {
                    menu.handle_backspace();
                }
            }
        }
        Ok(())
    }
}