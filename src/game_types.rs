// ==========================
// Core Game Types
// ==========================

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Plan,
    Execution,
    Waiting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Instruction {
    Move(Direction),
    NoOp,
    Interact,
}
