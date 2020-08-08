use crate::entities::Board;
use specs::Entity;
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
pub enum GameEvent {
    CorrectMove,
    IncorrectMove,
    GameOver,
}

#[derive(Default)]
pub struct GameEventQueue {
    pub events: Vec<GameEvent>,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    OnGoing,
    Completed,
}

pub struct Move {
    pub prev: Entity,
    pub curr: Entity,
}

pub struct GameState {
    pub board: Board,
    pub status: GameStatus,
    pub moves: VecDeque<Move>,
    pub move_count: u32,
    pub peg_count: u32,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            status: GameStatus::OnGoing,
            moves: VecDeque::new(),
            move_count: 0,
            peg_count: 32,
        }
    }
}
