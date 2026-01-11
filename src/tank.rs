use crate::GridPosition;
use crate::assets::GameAssets;
use ggez::{graphics, mint::Vector2, graphics::Canvas};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Tank {
    pos: GridPosition,
    direction: Direction,
    current_frame: usize,
    animation_timer: f32,
    animation_speed: f32, // Frames per second
}

impl Tank {
    pub fn new(pos: GridPosition) -> Self {
        Self {
            pos,
            direction: Direction::Right, // Default direction
            current_frame: 0,
            animation_timer: 0.0,
            animation_speed: 8.0, // Adjust as needed
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, assets: &GameAssets) {
        let current_body_frame = self.current_frame();
        let body_image = &assets.tank.body_sprites[current_body_frame];
        let tank_pos_rect: graphics::Rect = self.pos().into();
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
                .dest(tank_pos_rect.point())
                .rotation(rotation_angle)
                .scale(Vector2 {
                    x: tank_pos_rect.w / body_image.width() as f32,
                    y: tank_pos_rect.h / body_image.height() as f32,
                })
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );

        // Draw turret
        canvas.draw(
            &assets.tank.turret_sprite,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.point())
                .rotation(rotation_angle)
                .scale(Vector2 {
                    x: tank_pos_rect.w / assets.tank.turret_sprite.width() as f32,
                    y: tank_pos_rect.h / assets.tank.turret_sprite.height() as f32,
                })
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );
    }

    pub fn pos(&self) -> GridPosition {
        self.pos
    }

    pub fn set_pos(&mut self, pos: GridPosition) {
        self.pos = pos;
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
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
