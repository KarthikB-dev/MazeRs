use ggez::{graphics, Context, GameResult};

#[derive(PartialEq, Clone)]
pub enum MultiplayerAction {
    None,
    Host,
    Join,
    Back,
    CopyCode,
    PasteCode,
}

#[derive(PartialEq, Clone)]
pub enum MultiplayerState {
    ModeSelect,
    HostWaiting(String), // Contains the host code
    ClientInput(String), // Contains the current input
    Connecting,
}

pub struct MultiplayerMenu {
    state: MultiplayerState,
    buttons: Vec<Button>,
    input_box: Option<InputBox>,
    copy_button: Option<Button>,
    paste_button: Option<Button>,
    connect_button: Option<Button>,
    error_message: Option<String>,
}

struct Button {
    rect: graphics::Rect,
    text: String,
    action: MultiplayerAction,
    hovered: bool,
}

struct InputBox {
    rect: graphics::Rect,
    text: String,
    active: bool,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: String, action: MultiplayerAction) -> Self {
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

        // Border
        let border_color = [0.6, 0.6, 0.6, 1.0];
        let border_width = 2.0;

        // Draw borders (simplified)
        for (x, y, w, h) in [
            (self.rect.x, self.rect.y, self.rect.w, border_width),
            (self.rect.x, self.rect.y + self.rect.h - border_width, self.rect.w, border_width),
            (self.rect.x, self.rect.y, border_width, self.rect.h),
            (self.rect.x + self.rect.w - border_width, self.rect.y, border_width, self.rect.h),
        ] {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new(x, y, w, h))
                    .color(border_color)
            );
        }

        Ok(())
    }

    fn draw_text(&self, ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
        let mut text = graphics::Text::new(&self.text);
        text.set_scale(20.0);

        let text_dims = text.measure(ctx)?;
        let text_x = self.rect.x + (self.rect.w - text_dims.x) / 2.0;
        let text_y = self.rect.y + (self.rect.h - text_dims.y) / 2.0;

        canvas.draw(
            &text,
            graphics::DrawParam::new()
                .dest([text_x, text_y])
                .color([1.0, 1.0, 1.0, 1.0])
        );

        Ok(())
    }
}

impl InputBox {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: graphics::Rect::new(x, y, width, height),
            text: String::new(),
            active: true,
        }
    }

    fn draw(&self, _ctx: &Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Box background
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.rect)
                .color([0.2, 0.2, 0.2, 1.0])
        );

        // Border
        let border_width = 2.0;
        let border_color = if self.active {
            [0.0, 0.8, 1.0, 1.0] // Cyan when active
        } else {
            [0.6, 0.6, 0.6, 1.0]
        };

        for (x, y, w, h) in [
            (self.rect.x, self.rect.y, self.rect.w, border_width),
            (self.rect.x, self.rect.y + self.rect.h - border_width, self.rect.w, border_width),
            (self.rect.x, self.rect.y, border_width, self.rect.h),
            (self.rect.x + self.rect.w - border_width, self.rect.y, border_width, self.rect.h),
        ] {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new(x, y, w, h))
                    .color(border_color)
            );
        }

        // Text
        let display_text = if self.text.is_empty() {
            "Paste host code here..."
        } else {
            &self.text
        };

        let mut text = graphics::Text::new(display_text);
        text.set_scale(16.0);

        canvas.draw(
            &text,
            graphics::DrawParam::new()
                .dest([self.rect.x + 10.0, self.rect.y + 10.0])
                .color([0.8, 0.8, 0.8, 1.0])
        );

        Ok(())
    }
}

impl MultiplayerMenu {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let button_width = 200.0;
        let button_height = 60.0;
        let button_spacing = 20.0;

        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height / 2.0 - button_height - button_spacing / 2.0;

        let buttons = vec![
            Button::new(
                start_x,
                start_y,
                button_width,
                button_height,
                "Host Game".to_string(),
                MultiplayerAction::Host
            ),
            Button::new(
                start_x,
                start_y + button_height + button_spacing,
                button_width,
                button_height,
                "Join Game".to_string(),
                MultiplayerAction::Join
            ),
            Button::new(
                start_x,
                start_y + 2.0 * (button_height + button_spacing),
                button_width,
                button_height,
                "Back".to_string(),
                MultiplayerAction::Back
            ),
        ];

        Self {
            state: MultiplayerState::ModeSelect,
            buttons,
            input_box: None,
            copy_button: None,
            paste_button: None,
            connect_button: None,
            error_message: None,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos = ctx.mouse.position();

        for button in &mut self.buttons {
            button.update_hover(mouse_pos.x, mouse_pos.y);
        }

        if let Some(ref mut copy_btn) = self.copy_button {
            copy_btn.update_hover(mouse_pos.x, mouse_pos.y);
        }

        if let Some(ref mut paste_btn) = self.paste_button {
            paste_btn.update_hover(mouse_pos.x, mouse_pos.y);
        }

        if let Some(ref mut connect_btn) = self.connect_button {
            connect_btn.update_hover(mouse_pos.x, mouse_pos.y);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw title
        let mut title = graphics::Text::new("Multiplayer Setup");
        title.set_scale(48.0);
        let title_dims = title.measure(ctx)?;
        let title_x = (ctx.gfx.drawable_size().0 - title_dims.x) / 2.0;
        let title_y = 80.0;

        canvas.draw(
            &title,
            graphics::DrawParam::new()
                .dest([title_x, title_y])
                .color([1.0, 1.0, 0.0, 1.0])
        );

        match &self.state {
            MultiplayerState::ModeSelect => {
                // Draw buttons
                for button in &self.buttons {
                    button.draw(canvas)?;
                    button.draw_text(ctx, canvas)?;
                }
            }
            MultiplayerState::HostWaiting(code) => {
                // Show waiting message and host code
                let mut msg = graphics::Text::new("Waiting for player to join...");
                msg.set_scale(24.0);
                let msg_dims = msg.measure(ctx)?;
                canvas.draw(
                    &msg,
                    graphics::DrawParam::new()
                        .dest([(ctx.gfx.drawable_size().0 - msg_dims.x) / 2.0, 180.0])
                        .color([1.0, 1.0, 1.0, 1.0])
                );

                let mut code_label = graphics::Text::new("Share this code with your friend:");
                code_label.set_scale(18.0);
                let label_dims = code_label.measure(ctx)?;
                canvas.draw(
                    &code_label,
                    graphics::DrawParam::new()
                        .dest([(ctx.gfx.drawable_size().0 - label_dims.x) / 2.0, 240.0])
                        .color([0.8, 0.8, 0.8, 1.0])
                );

                // Code box - larger and scrollable
                let box_x = 50.0;
                let box_y = 280.0;
                let box_width = ctx.gfx.drawable_size().0 - 100.0;
                let box_height = 120.0;
                
                let code_box = graphics::Rect::new(box_x, box_y, box_width, box_height);
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(code_box)
                        .color([0.2, 0.2, 0.2, 1.0])
                );

                // Border for code box
                let border_color = [0.0, 0.8, 0.0, 1.0];
                let border_width = 2.0;
                for (x, y, w, h) in [
                    (box_x, box_y, box_width, border_width),
                    (box_x, box_y + box_height - border_width, box_width, border_width),
                    (box_x, box_y, border_width, box_height),
                    (box_x + box_width - border_width, box_y, border_width, box_height),
                ] {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new(x, y, w, h))
                            .color(border_color)
                    );
                }

                // Wrap the code text to fit in the box
                let mut code_text = graphics::Text::new("");
                code_text.set_scale(12.0);
                
                let padding = 10.0;
                
                // Split code into chunks that fit
                let chars_per_line = 80; // Approximate based on font size
                let mut wrapped_text = String::new();
                for (i, chunk) in code.as_bytes().chunks(chars_per_line).enumerate() {
                    if i > 0 {
                        wrapped_text.push('\n');
                    }
                    wrapped_text.push_str(&String::from_utf8_lossy(chunk));
                }
                
                code_text = graphics::Text::new(&wrapped_text);
                code_text.set_scale(12.0);
                
                canvas.draw(
                    &code_text,
                    graphics::DrawParam::new()
                        .dest([box_x + padding, box_y + padding])
                        .color([0.0, 1.0, 0.0, 1.0])
                );

                // Copy button
                if let Some(ref copy_btn) = self.copy_button {
                    copy_btn.draw(canvas)?;
                    copy_btn.draw_text(ctx, canvas)?;
                }
            }
            MultiplayerState::ClientInput(_) => {
                // Show input box
                let mut msg = graphics::Text::new("Enter host code:");
                msg.set_scale(24.0);
                let msg_dims = msg.measure(ctx)?;
                canvas.draw(
                    &msg,
                    graphics::DrawParam::new()
                        .dest([(ctx.gfx.drawable_size().0 - msg_dims.x) / 2.0, 200.0])
                        .color([1.0, 1.0, 1.0, 1.0])
                );

                if let Some(input_box) = &self.input_box {
                    input_box.draw(ctx, canvas)?;
                }

                // Draw paste button
                if let Some(ref paste_btn) = self.paste_button {
                    paste_btn.draw(canvas)?;
                    paste_btn.draw_text(ctx, canvas)?;
                }

                // Draw connect button
                if let Some(ref connect_btn) = self.connect_button {
                    connect_btn.draw(canvas)?;
                    connect_btn.draw_text(ctx, canvas)?;
                }

                // Show error message if any
                if let Some(ref error_msg) = self.error_message {
                    let mut error_text = graphics::Text::new(error_msg);
                    error_text.set_scale(16.0);
                    let error_dims = error_text.measure(ctx)?;
                    canvas.draw(
                        &error_text,
                        graphics::DrawParam::new()
                            .dest([(ctx.gfx.drawable_size().0 - error_dims.x) / 2.0, 500.0])
                            .color([1.0, 0.3, 0.3, 1.0]) // Red color for errors
                    );
                }
            }
            MultiplayerState::Connecting => {
                let mut msg = graphics::Text::new("Connecting...");
                msg.set_scale(36.0);
                let msg_dims = msg.measure(ctx)?;
                canvas.draw(
                    &msg,
                    graphics::DrawParam::new()
                        .dest([
                            (ctx.gfx.drawable_size().0 - msg_dims.x) / 2.0,
                            (ctx.gfx.drawable_size().1 - msg_dims.y) / 2.0
                        ])
                        .color([1.0, 1.0, 0.0, 1.0])
                );
            }
        }

        Ok(())
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> MultiplayerAction {
        match &self.state {
            MultiplayerState::ModeSelect => {
                for button in &self.buttons {
                    if button.contains_point(x, y) {
                        return button.action.clone();
                    }
                }
            }
            MultiplayerState::HostWaiting(_) => {
                // Check copy button
                if let Some(ref copy_btn) = self.copy_button {
                    if copy_btn.contains_point(x, y) {
                        return MultiplayerAction::CopyCode;
                    }
                }
            }
            MultiplayerState::ClientInput(_) => {
                // Check paste button
                if let Some(ref paste_btn) = self.paste_button {
                    if paste_btn.contains_point(x, y) {
                        return MultiplayerAction::PasteCode;
                    }
                }
                
                // Check connect button
                if let Some(ref connect_btn) = self.connect_button {
                    if connect_btn.contains_point(x, y) {
                        return MultiplayerAction::Join;
                    }
                }
            }
            _ => {}
        }
        MultiplayerAction::None
    }

    pub fn handle_text_input(&mut self, text: &str) {
        if let MultiplayerState::ClientInput(ref mut input) = self.state {
            input.push_str(text);
            if let Some(ref mut input_box) = self.input_box {
                input_box.text = input.clone();
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        if let MultiplayerState::ClientInput(ref mut input) = self.state {
            input.pop();
            if let Some(ref mut input_box) = self.input_box {
                input_box.text = input.clone();
            }
        }
    }

    pub fn set_state(&mut self, state: MultiplayerState) {
        match &state {
            MultiplayerState::ClientInput(_) => {
                // Create input box, paste button, and connect button
                self.input_box = Some(InputBox::new(100.0, 280.0, 600.0, 50.0));
                
                // Paste button positioned below the input box
                self.paste_button = Some(Button::new(
                    100.0,
                    345.0,
                    120.0,
                    35.0,
                    "Paste".to_string(),
                    MultiplayerAction::PasteCode
                ));
                
                // Connect button centered at bottom
                let screen_width = 800.0; // Approximate
                let button_width = 200.0;
                let button_x = (screen_width - button_width) / 2.0;
                
                self.connect_button = Some(Button::new(
                    button_x,
                    420.0,
                    button_width,
                    60.0,
                    "Connect".to_string(),
                    MultiplayerAction::Join
                ));
                
                self.copy_button = None;
                self.error_message = None;
            }
            MultiplayerState::HostWaiting(_) => {
                // Create copy button
                let screen_width = 800.0; // Approximate
                let button_width = 150.0;
                let button_x = (screen_width - button_width) / 2.0;
                
                self.copy_button = Some(Button::new(
                    button_x,
                    420.0,
                    button_width,
                    50.0,
                    "Copy Code".to_string(),
                    MultiplayerAction::CopyCode
                ));
                
                self.input_box = None;
                self.paste_button = None;
                self.connect_button = None;
                self.error_message = None;
            }
            _ => {
                self.input_box = None;
                self.copy_button = None;
                self.paste_button = None;
                self.connect_button = None;
                self.error_message = None;
            }
        }
        self.state = state;
    }

    pub fn get_state(&self) -> &MultiplayerState {
        &self.state
    }

    pub fn get_input_text(&self) -> Option<String> {
        if let MultiplayerState::ClientInput(ref text) = self.state {
            Some(text.clone())
        } else {
            None
        }
    }

    pub fn get_host_code(&self) -> Option<String> {
        if let MultiplayerState::HostWaiting(ref code) = self.state {
            Some(code.clone())
        } else {
            None
        }
    }

    pub fn set_input_text(&mut self, text: String) {
        if let MultiplayerState::ClientInput(ref mut input) = self.state {
            *input = text.clone();
            if let Some(ref mut input_box) = self.input_box {
                input_box.text = text;
            }
        }
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}