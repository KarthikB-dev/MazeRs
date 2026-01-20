use crate::assets::TankAssets;
use crate::game_types::Direction;
use crate::map::MapPos;
use ggez::{graphics, graphics::Canvas, mint::Vector2};

pub struct Tank {
    pos: MapPos,
    direction: Direction,
    current_frame: usize,
    animation_timer: f32,
    animation_speed: f32, // Frames per second
}

impl Tank {
    pub fn new(pos: MapPos) -> Self {
        Self {
            pos,
            direction: Direction::Right, // Default direction
            current_frame: 0,
            animation_timer: 0.0,
            animation_speed: 8.0, // Adjust as needed
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, assets: &TankAssets, cell_size: f32) {
        let current_body_frame = self.current_frame();
        let body_image = &assets.body_sprites[current_body_frame];
        let tank_pos_rect = graphics::Rect::new(
            self.pos().x as f32 * cell_size as f32,
            self.pos().y as f32 * cell_size as f32,
            cell_size as f32,
            cell_size as f32,
        );
        let size = Vector2 {
            x: tank_pos_rect.w * 1.5 / body_image.width() as f32,
            y: tank_pos_rect.h * 0.8 / body_image.height() as f32,
        };
        let rotation_angle = match self.direction() {
            Direction::Up => 0.0,
            Direction::Right => std::f32::consts::PI / 2.0,
            Direction::Down => std::f32::consts::PI,
            Direction::Left => 3.0 * std::f32::consts::PI / 2.0,
        };

        // Draw tank body
        canvas.draw(
            body_image,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.center())
                .rotation(rotation_angle)
                .scale(size)
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );

        // Draw turret
        canvas.draw(
            &assets.turret_sprite,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.center())
                .rotation(rotation_angle)
                .scale(size)
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );
    }

    pub fn pos(&self) -> MapPos {
        self.pos
    }

    pub fn set_pos(&mut self, pos: MapPos) {
        self.pos = pos;
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn update_animation(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= 1.0 / self.animation_speed {
            self.current_frame = (self.current_frame + 1) % 6; // Assuming 6 frames for tank body
            self.animation_timer = 0.0;
        }
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}
