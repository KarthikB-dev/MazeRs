use crate::GridPosition;

pub struct Tank {
    pos: GridPosition,
}

impl Tank {
    pub fn new(pos: GridPosition) -> Self {
        Self { pos }
    }

    pub fn pos(&self) -> GridPosition {
        self.pos
    }

    pub fn set_pos(&mut self, pos: GridPosition) {
        self.pos = pos;
    }
}