use ggez::{event, Context, GameResult, input::keyboard};
use crate::menu::{MenuState, MenuAction};
use crate::multiplayer_menu::{MultiplayerMenu, MultiplayerAction, MultiplayerState};
use crate::gamestate::GameState;
use crate::map::GRID_SIZE;
use crate::network::{NetworkManager};
use tokio::runtime::Runtime;
use arboard::Clipboard;

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
    Multiplayer(MultiplayerMenu),
    Game(GameState),
}

pub enum PendingAction {
    None,
    StartMultiplayer,
    HostGame,
    JoinGame(String),
}

enum ConnectionState {
    Idle,
    HostingGettingCode(tokio::sync::oneshot::Receiver<Result<(iroh::net::Endpoint, String), String>>),
    HostingWaitingForClient(tokio::sync::oneshot::Receiver<Result<NetworkManager, String>>),
    Joining(tokio::sync::oneshot::Receiver<Result<NetworkManager, String>>),
}

pub struct ScreenManager {
    current_screen: Screen,
    pending_action: PendingAction,
    screen_width: f32,
    screen_height: f32,
    runtime: Runtime,
    connection_state: ConnectionState,
    clipboard: Option<Clipboard>,
}

impl ScreenManager {
    pub fn new(_ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| ggez::GameError::CustomError(format!("Failed to create runtime: {}", e)))?;
        
        // Try to initialize clipboard once at startup
        let clipboard = match Clipboard::new() {
            Ok(cb) => Some(cb),
            Err(e) => {
                eprintln!("Warning: Failed to initialize clipboard at startup: {}", e);
                None
            }
        };

        Ok(Self {
            current_screen: Screen::Menu(MenuState::new(screen_width, screen_height)),
            pending_action: PendingAction::None,
            screen_width,
            screen_height,
            runtime,
            connection_state: ConnectionState::Idle,
            clipboard,
        })
    }

    fn transition_to_multiplayer(&mut self) {
        self.current_screen = Screen::Multiplayer(MultiplayerMenu::new(self.screen_width, self.screen_height));
    }

    fn transition_to_menu(&mut self) {
        self.current_screen = Screen::Menu(MenuState::new(self.screen_width, self.screen_height));
    }

    fn transition_to_game(&mut self, ctx: &mut Context, network: NetworkManager) -> GameResult {
        match GameState::new_multiplayer(ctx, network) {
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

    fn start_hosting_step1(&mut self) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        // Step 1: Get the endpoint and code
        self.runtime.spawn(async move {
            let result = crate::network::start_hosting().await
                .map_err(|e| format!("{}", e));
            let _ = tx.send(result);
        });
        
        self.connection_state = ConnectionState::HostingGettingCode(rx);
    }

    fn start_hosting_step2(&mut self, endpoint: iroh::net::Endpoint) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        // Step 2: Wait for client to connect
        self.runtime.spawn(async move {
            let result = crate::network::wait_for_client(endpoint).await
                .map_err(|e| format!("{}", e));
            let _ = tx.send(result);
        });
        
        self.connection_state = ConnectionState::HostingWaitingForClient(rx);
    }

    fn start_joining_background(&mut self, code: String) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        self.runtime.spawn(async move {
            let result = crate::network::connect_as_client(code).await
                .map_err(|e| format!("{}", e));
            let _ = tx.send(result);
        });
        
        self.connection_state = ConnectionState::Joining(rx);
    }
}

impl event::EventHandler for ScreenManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Check for completed connections
        match &mut self.connection_state {
            ConnectionState::HostingGettingCode(rx) => {
                match rx.try_recv() {
                    Ok(Ok((endpoint, host_code))) => {
                        println!("Host code generated: {}", host_code);
                        
                        // Show the host code to the user
                        if let Screen::Multiplayer(ref mut menu) = self.current_screen {
                            menu.set_state(MultiplayerState::HostWaiting(host_code));
                        }
                        
                        // Now wait for client in background
                        self.start_hosting_step2(endpoint);
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
            ConnectionState::Joining(rx) => {
                match rx.try_recv() {
                    Ok(Ok(network_manager)) => {
                        println!("Connected to host!");
                        self.connection_state = ConnectionState::Idle;
                        self.transition_to_game(ctx, network_manager)?;
                    }
                    Ok(Err(e)) => {
                        eprintln!("Join failed: {}", e);
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
            ConnectionState::Idle => {}
        }

        // Handle pending actions
        match std::mem::replace(&mut self.pending_action, PendingAction::None) {
            PendingAction::StartMultiplayer => {
                self.transition_to_multiplayer();
            }
            PendingAction::HostGame => {
                self.start_hosting_step1();
                if let Screen::Multiplayer(ref mut menu) = self.current_screen {
                    menu.set_state(MultiplayerState::Connecting);
                }
            }
            PendingAction::JoinGame(code) => {
                self.start_joining_background(code);
                if let Screen::Multiplayer(ref mut menu) = self.current_screen {
                    menu.set_state(MultiplayerState::Connecting);
                }
            }
            PendingAction::None => {}
        }

        match &mut self.current_screen {
            Screen::Menu(menu) => menu.update(ctx),
            Screen::Multiplayer(menu) => menu.update(ctx),
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
            Screen::Multiplayer(menu) => {
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
                        self.pending_action = PendingAction::StartMultiplayer;
                    }
                    MenuAction::Quit => {
                        ctx.request_quit();
                    }
                    MenuAction::None => {}
                }
                Ok(())
            }
            Screen::Multiplayer(menu) => {
                let action = menu.handle_click(x, y);

                match action {
                    MultiplayerAction::Host => {
                        self.pending_action = PendingAction::HostGame;
                    }
                    MultiplayerAction::Join => {
                        if let Some(code) = menu.get_input_text() {
                            if !code.is_empty() {
                                self.pending_action = PendingAction::JoinGame(code);
                            }
                        } else {
                            menu.set_state(MultiplayerState::ClientInput(String::new()));
                        }
                    }
                    MultiplayerAction::CopyCode => {
                        // Fix: Inline the lazy initialization and access self.clipboard directly
                        // to avoid "cannot borrow *self as mutable more than once" error.
                        
                        if self.clipboard.is_none() {
                             match Clipboard::new() {
                                Ok(cb) => self.clipboard = Some(cb),
                                Err(e) => eprintln!("Failed to initialize clipboard: {}", e),
                            }
                        }

                        if let Some(code) = menu.get_host_code() {
                            if let Some(ref mut clipboard) = self.clipboard {
                                if let Err(e) = clipboard.set_text(&code) {
                                    eprintln!("Failed to copy to clipboard: {}", e);
                                } else {
                                    println!("Host code copied to clipboard!");
                                }
                            } else {
                                eprintln!("Clipboard unavailable");
                            }
                        }
                    }
                    MultiplayerAction::PasteCode => {
                        // Fix: Same direct access strategy here
                        
                        if self.clipboard.is_none() {
                             match Clipboard::new() {
                                Ok(cb) => self.clipboard = Some(cb),
                                Err(e) => eprintln!("Failed to initialize clipboard: {}", e),
                            }
                        }

                        if let Some(ref mut clipboard) = self.clipboard {
                            match clipboard.get_text() {
                                Ok(text) => {
                                    let trimmed = text.trim().to_string();
                                    if !trimmed.is_empty() {
                                        println!("Pasted code from clipboard!");
                                        menu.set_input_text(trimmed);
                                    } else {
                                        eprintln!("Clipboard is empty");
                                        menu.set_error("Clipboard is empty!".to_string());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read from clipboard: {}", e);
                                    menu.set_error("Failed to read clipboard".to_string());
                                }
                            }
                        } else {
                            eprintln!("Clipboard unavailable");
                            menu.set_error("Clipboard unavailable".to_string());
                        }
                    }
                    MultiplayerAction::Back => {
                        self.transition_to_menu();
                    }
                    MultiplayerAction::None => {}
                }
                Ok(())
            }
            Screen::Game(game) => game.mouse_button_down_event(ctx, button, x, y),
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        if let Screen::Multiplayer(ref mut menu) = self.current_screen {
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
        if let Screen::Multiplayer(ref mut menu) = self.current_screen {
            if let Some(keycode) = input.keycode {
                if keycode == keyboard::KeyCode::Back {
                    menu.handle_backspace();
                }
            }
        }
        Ok(())
    }
}