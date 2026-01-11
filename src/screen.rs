use ggez::{event, Context, GameResult};
use crate::menu::{MenuState, MenuAction};
use crate::gamestate::GameState;
use crate::network::{NetworkManager, start_hosting, wait_for_client};
use tokio::runtime::Runtime;

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const MAP_WIDTH: f32 = 1250.0;
pub const MAP_HEIGHT: f32 = 1250.0;

pub const SCREEN_SIZE: (f32, f32) = (
    MAP_WIDTH + SIDEBAR_WIDTH,
    MAP_HEIGHT,
);

pub enum Screen {
    Menu(MenuState),
    Game(GameState),
}

enum ConnectionState {
    Idle,
    HostingGettingCode(tokio::sync::oneshot::Receiver<Result<(iroh::Endpoint, String), String>>),
    HostingWaitingForClient(tokio::sync::oneshot::Receiver<Result<NetworkManager, String>>),
}

pub struct ScreenManager {
    current_screen: Screen,
    screen_width: f32,
    screen_height: f32,
    runtime: Runtime,
    connection_state: ConnectionState,
}

impl ScreenManager {
    pub fn new(_ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| ggez::GameError::CustomError(format!("Failed to create runtime: {}", e)))?;

        Ok(Self {
            current_screen: Screen::Menu(MenuState::new(screen_width, screen_height)),
            screen_width,
            screen_height,
            runtime,
            connection_state: ConnectionState::Idle,
        })
    }


    fn transition_to_game(&mut self, ctx: &mut Context, network: NetworkManager) -> GameResult {
        match GameState::new(ctx, network) {
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

    fn transition_to_multiplayer(&mut self) {
        if let Screen::Menu(ref mut menu) = self.current_screen {
            menu.set_multiplayer_mode(self.screen_width, self.screen_height);
        }
    }

    fn transition_to_main(&mut self) {
        if let Screen::Menu(ref mut menu) = self.current_screen {
            menu.set_main_mode(self.screen_width, self.screen_height);
        }
    }

    fn start_hosting(&mut self) {
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Start hosting in background
        self.runtime.spawn(async move {
            let result = start_hosting().await
                .map_err(|e| format!("{}", e));
            let _ = tx.send(result);
        });

        self.connection_state = ConnectionState::HostingGettingCode(rx);
    }

    fn wait_for_client(&mut self, endpoint: iroh::Endpoint) {
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Wait for client in background
        self.runtime.spawn(async move {
            let result = wait_for_client(endpoint).await
                .map_err(|e| format!("{}", e));
            let _ = tx.send(result);
        });

        self.connection_state = ConnectionState::HostingWaitingForClient(rx);
    }

}

impl event::EventHandler for ScreenManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Handle connection state
        match &mut self.connection_state {
            ConnectionState::HostingGettingCode(rx) => {
                match rx.try_recv() {
                    Ok(Ok((endpoint, host_code))) => {
                        println!("Host code generated: {}", host_code);

                        // Show the host code to the user
                        if let Screen::Menu(ref mut menu) = self.current_screen {
                            menu.set_host_waiting(host_code);
                        }

                        // Now wait for client in background
                        self.wait_for_client(endpoint);
                    }
                    Ok(Err(e)) => {
                        eprintln!("Failed to start hosting: {}", e);
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_multiplayer();
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {
                        // Still waiting...
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                        eprintln!("Connection channel closed unexpectedly");
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_multiplayer();
                    }
                }
            }
            ConnectionState::HostingWaitingForClient(rx) => {
                match rx.try_recv() {
                    Ok(Ok(network_manager)) => {
                        println!("Client connected!");
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_game(ctx, network_manager)?;
                    }
                    Ok(Err(e)) => {
                        eprintln!("Client connection failed: {}", e);
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_multiplayer();
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {
                        // Still waiting for client...
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                        eprintln!("Connection channel closed unexpectedly");
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_multiplayer();
                    }
                }
            }
            ConnectionState::Idle => {}
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
                        self.transition_to_multiplayer();
                    }
                    MenuAction::Host => {
                        self.start_hosting();
                    }
                    MenuAction::Join => {
                        // For now, show a message that join needs to be implemented
                        // In a real game, this could use system clipboard or a dialog
                        eprintln!("Join functionality needs host code input - not yet implemented");
                    }
                    MenuAction::Back => {
                        self.transition_to_main();
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

}
