use ggez::{graphics, Context, GameResult};

const BUTTON_WIDTH: f32 = 600.0;
const BUTTON_HEIGHT: f32 = 100.0;
const BUTTON_SPACING: f32 = 40.0;

#[derive(PartialEq, Clone)]
pub enum MenuAction {
    None,
    Host,
    Join,
    Back,
    Quit,
    CopyToClipboard,
    PasteFromClipboard,
}

#[derive(PartialEq, Clone)]
enum MenuMode {
    Main,
    JoinWaiting,
    HostWaiting(String), // Contains the host code
}

pub struct MenuState {
    mode: MenuMode,
    buttons: Vec<Button>,
    button_width: f32,
    button_height: f32,
    button_spacing: f32,
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
                .color(bg_color),
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
                    border_width,
                ))
                .color(border_color),
        );

        // Bottom border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x,
                    self.rect.y + self.rect.h - border_width,
                    self.rect.w,
                    border_width,
                ))
                .color(border_color),
        );

        // Left border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x,
                    self.rect.y,
                    border_width,
                    self.rect.h,
                ))
                .color(border_color),
        );

        // Right border
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    self.rect.x + self.rect.w - border_width,
                    self.rect.y,
                    border_width,
                    self.rect.h,
                ))
                .color(border_color),
        );

        Ok(())
    }

    fn draw_text(&self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
        let mut text = graphics::Text::new(&self.text);
        text.set_scale(40.0);
        text.set_font("nerd");

        let text_dims = text.measure(ctx)?;
        let text_x = self.rect.x + (self.rect.w - text_dims.x) / 2.0;
        let text_y = self.rect.y + (self.rect.h - text_dims.y) / 2.0;

        let text_color = [1.0, 1.0, 1.0, 1.0];

        canvas.draw(
            &text,
            graphics::DrawParam::new()
                .dest([text_x, text_y])
                .color(text_color),
        );

        Ok(())
    }
}

impl MenuState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut state = Self {
            mode: MenuMode::Main,
            buttons: Vec::new(),
            button_width: BUTTON_WIDTH,
            button_height: BUTTON_HEIGHT,
            button_spacing: BUTTON_SPACING,
        };
        state.buttons = state.create_main_buttons(screen_width, screen_height);
        state
    }

    fn create_main_buttons(&self, screen_width: f32, screen_height: f32) -> Vec<Button> {
        let start_x = (screen_width - self.button_width) / 2.0;
        let start_y = screen_height / 2.0 - self.button_height;

        vec![
            Button::new(
                start_x,
                start_y,
                self.button_width,
                self.button_height,
                "Host Game".to_string(),
                MenuAction::Host,
            ),
            Button::new(
                start_x,
                start_y + self.button_height + self.button_spacing,
                self.button_width,
                self.button_height,
                "Join Game".to_string(),
                MenuAction::Join,
            ),
            Button::new(
                start_x,
                start_y + 2.0 * (self.button_height + self.button_spacing),
                self.button_width,
                self.button_height,
                "Quit".to_string(),
                MenuAction::Quit,
            ),
        ]
    }

    fn create_host_waiting_buttons(&self, screen_width: f32) -> Vec<Button> {
        let start_x = (screen_width - self.button_width) / 2.0;
        let start_y = 500.0;

        vec![
            Button::new(
                start_x,
                start_y,
                self.button_width,
                self.button_height,
                "Copy to Clipboard".to_string(),
                MenuAction::CopyToClipboard,
            ),
            Button::new(
                start_x,
                start_y + self.button_height + self.button_spacing,
                self.button_width,
                self.button_height,
                "Back".to_string(),
                MenuAction::Back,
            ),
        ]
    }

    fn create_join_waiting_buttons(&self, screen_width: f32) -> Vec<Button> {
        let start_x = (screen_width - self.button_width) / 2.0;
        let start_y = 400.0;

        vec![
            Button::new(
                start_x,
                start_y,
                self.button_width,
                self.button_height,
                "Paste from Clipboard".to_string(),
                MenuAction::PasteFromClipboard,
            ),
            Button::new(
                start_x,
                start_y + self.button_height + self.button_spacing,
                self.button_width,
                self.button_height,
                "Back".to_string(),
                MenuAction::Back,
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
        let title_text = match &self.mode {
            MenuMode::Main => "Tank Maze",
            MenuMode::JoinWaiting => "Join Game",
            MenuMode::HostWaiting(_) => "Waiting for Player",
        };

        let mut title = graphics::Text::new(title_text);
        title.set_scale(48.0);
        title.set_font("nerd");
        let title_dims = title.measure(ctx)?;
        let title_x = (ctx.gfx.drawable_size().0 - title_dims.x) / 2.0;
        let title_y = 100.0;

        canvas.draw(
            &title,
            graphics::DrawParam::new()
                .dest([title_x, title_y])
                .color([1.0, 1.0, 0.0, 1.0]), // Yellow title
        );

        for button in &self.buttons {
            button.draw(canvas)?;
            button.draw_text(ctx, canvas)?;
        }

        match &self.mode {
            MenuMode::HostWaiting(code) => {
                // Split the base64 code into chunks for better display
                let chunks: Vec<&str> = code
                    .as_bytes()
                    .chunks(80)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
                    .collect();
                let formatted_code = format!("Host Code:\n{}", chunks.join("\n"));

                let mut code_text = graphics::Text::new(&formatted_code);
                code_text.set_scale(40.0);
                code_text.set_font("nerd");
                let code_dims = code_text.measure(ctx)?;
                let code_x = (ctx.gfx.drawable_size().0 - code_dims.x) / 2.0;
                let code_y = 280.0;

                canvas.draw(
                    &code_text,
                    graphics::DrawParam::new()
                        .dest([code_x, code_y])
                        .color([1.0, 1.0, 1.0, 1.0]),
                );
            }
            MenuMode::JoinWaiting => {
                let mut instruction_text =
                    graphics::Text::new("Paste the host code from clipboard\nto connect to a game");
                instruction_text.set_scale(40.0);
                instruction_text.set_font("nerd");
                let instruction_dims = instruction_text.measure(ctx)?;
                let instruction_x = (ctx.gfx.drawable_size().0 - instruction_dims.x) / 2.0;
                let instruction_y = 300.0;

                canvas.draw(
                    &instruction_text,
                    graphics::DrawParam::new()
                        .dest([instruction_x, instruction_y])
                        .color([0.8, 0.8, 0.8, 1.0]),
                );
            }
            MenuMode::Main => {}
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
    pub fn set_host_waiting(&mut self, code: String, screen_width: f32) {
        self.mode = MenuMode::HostWaiting(code);
        self.buttons = self.create_host_waiting_buttons(screen_width);
    }

    pub fn set_join_waiting(&mut self, screen_width: f32) {
        self.mode = MenuMode::JoinWaiting;
        self.buttons = self.create_join_waiting_buttons(screen_width);
    }

    pub fn set_main_mode(&mut self, screen_width: f32, screen_height: f32) {
        self.mode = MenuMode::Main;
        self.buttons = self.create_main_buttons(screen_width, screen_height);
    }

    pub fn get_host_code(&self) -> Option<&String> {
        match &self.mode {
            MenuMode::HostWaiting(code) => Some(code),
            _ => None,
        }
    }
}
