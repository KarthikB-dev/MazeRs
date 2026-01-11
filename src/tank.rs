use crate::GridPosition;

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
