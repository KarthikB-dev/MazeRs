// ==========================
// Core Game Types
// ==========================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Plan,
    Execution,
    Waiting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Move(Direction),
    Noop,
    Interact,
}
