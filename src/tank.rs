use crate::screen::CELL_SIZE;
use crate::map::MapPos;
use crate::assets::GameAssets;
use crate::game_types::Direction;
use ggez::{graphics, mint::Vector2, graphics::Canvas};

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

    pub fn draw(&self, canvas: &mut Canvas, assets: &GameAssets) {
        let current_body_frame = self.current_frame();
        let body_image = &assets.tank.body_sprites[current_body_frame];
        let tank_pos_rect = graphics::Rect::new(
            self.pos().x as f32 * CELL_SIZE as f32,
            self.pos().y as f32 * CELL_SIZE as f32,
            CELL_SIZE as f32,
            CELL_SIZE as f32,
        );
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
                .scale(Vector2 { x: tank_pos_rect.w / body_image.width() as f32,
                    y: tank_pos_rect.h / body_image.height() as f32,
                })
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );

        // Draw turret
        canvas.draw(
            &assets.tank.turret_sprite,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.center())
                .rotation(rotation_angle)
                .scale(Vector2 {
                    x: tank_pos_rect.w / assets.tank.turret_sprite.width() as f32,
                    y: tank_pos_rect.h / assets.tank.turret_sprite.height() as f32,
                })
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
