use ggez::{
    graphics,
    Context, GameResult,
};

#[derive(PartialEq, Clone)]
pub enum MenuAction {
    None,
    StartGame,
    Host,
    Join, 
    Back,
    Quit,
}

#[derive(PartialEq, Clone)]
enum MenuMode {
    Main,
    Multiplayer,
    HostWaiting(String), // Contains the host code
}

pub struct MenuState {
    mode: MenuMode,
    buttons: Vec<Button>,
}

struct Button {
    rect: graphics::Rect,
    text: String,
    action: MenuAction,
    hovered: bool,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: String, action: MenuAction) -> Self {
        Self {
            rect: graphics::Rect::new(x, y, width, height),
            text,
            action,
            hovered: false,
        }
    }

    fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains([x, y])
    }

    fn update_hover(&mut self, mouse_x: f32, mouse_y: f32) {
        self.hovered = self.contains_point(mouse_x, mouse_y);
    }

    fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        // Button background
        let bg_color = if self.hovered {
            [0.4, 0.4, 0.4, 1.0]
        } else {
            [0.3, 0.3, 0.3, 1.0]
        };

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.rect)
                .color(bg_color)
        );

        // Button border
        let border_color = [0.6, 0.6, 0.6, 1.0];
        let border_width = 2.0;

        // Top border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x,
                    self.rect.y,
                    self.rect.w,
                    border_width
                ))
                .color(border_color)
        );

        // Bottom border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x,
                    self.rect.y + self.rect.h - border_width,
                    self.rect.w,
                    border_width
                ))
                .color(border_color)
        );

        // Left border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x,
                    self.rect.y,
                    border_width,
                    self.rect.h
                ))
                .color(border_color)
        );

        // Right border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x + self.rect.w - border_width,
                    self.rect.y,
                    border_width,
                    self.rect.h
                ))
                .color(border_color)
        );

        Ok(())
    }

    fn draw_text(&self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
        let mut text = graphics::Text::new(&self.text);
        text.set_scale(24.0);

        let text_dims = text.measure(ctx)?;
        let text_x = self.rect.x + (self.rect.w - text_dims.x) / 2.0;
        let text_y = self.rect.y + (self.rect.h - text_dims.y) / 2.0;

        let text_color = [1.0, 1.0, 1.0, 1.0];

        canvas.draw(
            &text,
            graphics::DrawParam::new()
                .dest([text_x, text_y])
                .color(text_color)
        );

        Ok(())
    }
}

impl MenuState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let buttons = Self::create_main_buttons(screen_width, screen_height);
        Self { 
            mode: MenuMode::Main,
            buttons,
        }
    }

    fn create_main_buttons(screen_width: f32, screen_height: f32) -> Vec<Button> {
        let button_width = 200.0;
        let button_height = 60.0;
        let button_spacing = 20.0;

        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height / 2.0 - button_height - button_spacing / 2.0;

        vec![
            Button::new(
                start_x,
                start_y,
                button_width,
                button_height,
                "Start Game".to_string(),
                MenuAction::StartGame
            ),
            Button::new(
                start_x,
                start_y + button_height + button_spacing,
                button_width,
                button_height,
                "Quit".to_string(),
                MenuAction::Quit
            ),
        ]
    }

    fn create_multiplayer_buttons(screen_width: f32, screen_height: f32) -> Vec<Button> {
        let button_width = 200.0;
        let button_height = 60.0;
        let button_spacing = 20.0;

        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height / 2.0 - button_height - button_spacing;

        vec![
            Button::new(
                start_x,
                start_y,
                button_width,
                button_height,
                "Host Game".to_string(),
                MenuAction::Host
            ),
            Button::new(
                start_x,
                start_y + button_height + button_spacing,
                button_width,
                button_height,
                "Join Game".to_string(),
                MenuAction::Join
            ),
            Button::new(
                start_x,
                start_y + 2.0 * (button_height + button_spacing),
                button_width,
                button_height,
                "Back".to_string(),
                MenuAction::Back
            ),
        ]
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos = ctx.mouse.position();

        for button in &mut self.buttons {
            button.update_hover(mouse_pos.x, mouse_pos.y);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw title
        let title_text = match &self.mode {
            MenuMode::Main => "Tank Maze P2P",
            MenuMode::Multiplayer => "Multiplayer Setup",
            MenuMode::HostWaiting(_) => "Waiting for Player",
        };

        let mut title = graphics::Text::new(title_text);
        title.set_scale(48.0);
        let title_dims = title.measure(ctx)?;
        let title_x = (ctx.gfx.drawable_size().0 - title_dims.x) / 2.0;
        let title_y = 100.0;

        canvas.draw(
            &title,
            graphics::DrawParam::new()
                .dest([title_x, title_y])
                .color([1.0, 1.0, 0.0, 1.0]) // Yellow title
        );

        // Draw mode-specific content
        match &self.mode {
            MenuMode::Main | MenuMode::Multiplayer => {
                // Draw buttons
                for button in &self.buttons {
                    button.draw(canvas)?;
                    button.draw_text(ctx, canvas)?;
                }
            }
            MenuMode::HostWaiting(code) => {
                let mut code_text = graphics::Text::new(&format!("Host Code:\n{}", code));
                code_text.set_scale(24.0);
                let code_dims = code_text.measure(ctx)?;
                let code_x = (ctx.gfx.drawable_size().0 - code_dims.x) / 2.0;
                let code_y = 300.0;

                canvas.draw(
                    &code_text,
                    graphics::DrawParam::new()
                        .dest([code_x, code_y])
                        .color([1.0, 1.0, 1.0, 1.0])
                );

                let mut wait_text = graphics::Text::new("Waiting for client to connect...");
                wait_text.set_scale(20.0);
                let wait_dims = wait_text.measure(ctx)?;
                let wait_x = (ctx.gfx.drawable_size().0 - wait_dims.x) / 2.0;
                let wait_y = 400.0;

                canvas.draw(
                    &wait_text,
                    graphics::DrawParam::new()
                        .dest([wait_x, wait_y])
                        .color([0.8, 0.8, 0.8, 1.0])
                );
            }
        }

        Ok(())
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> MenuAction {
        for button in &mut self.buttons {
            if button.contains_point(x, y) {
                return button.action.clone();
            }
        }
        MenuAction::None
    }

    // Methods to handle state transitions (called from main.rs)
    pub fn set_host_waiting(&mut self, code: String) {
        self.mode = MenuMode::HostWaiting(code);
        self.buttons.clear(); // Simple: just show the code, no buttons needed
    }


    pub fn set_multiplayer_mode(&mut self, screen_width: f32, screen_height: f32) {
        self.mode = MenuMode::Multiplayer;
        self.buttons = Self::create_multiplayer_buttons(screen_width, screen_height);
    }

    pub fn set_main_mode(&mut self, screen_width: f32, screen_height: f32) {
        self.mode = MenuMode::Main;
        self.buttons = Self::create_main_buttons(screen_width, screen_height);
    }


}
