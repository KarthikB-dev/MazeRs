use ggez::{
    graphics,
    Context, GameResult,
};

#[derive(PartialEq, Clone)]
pub enum MenuAction {
    None,
    StartGame,
    Quit,
}

pub struct MenuState {
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
        ];

        Self { buttons }
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
        let mut title = graphics::Text::new("Tank Maze P2P");
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

        // Draw buttons
        for button in &self.buttons {
            button.draw(canvas)?;
            button.draw_text(ctx, canvas)?;
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

    pub fn handle_text_input(&mut self, _text: &str) {
        // Placeholder for text input handling
    }

    pub fn handle_backspace(&mut self) {
        // Placeholder for backspace handling
    }
}
