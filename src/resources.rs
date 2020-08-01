use crate::entities::Board;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum InputEvent {
    MouseClick { x: f32, y: f32 },
    Up,
    Down,
    Left,
    Right,
    Enter,
}

#[derive(Default)]
pub struct InputQueue {
    pub events: VecDeque<InputEvent>,
}

#[derive(Debug)]
pub enum GameStatus {
    OnGoing,
    Completed(u8),
}

pub struct GameState {
    pub board: Board,
    pub status: GameStatus,
    pub move_count: u32,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            status: GameStatus::OnGoing,
            move_count: 0,
        }
    }
}
