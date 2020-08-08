use crate::constants::PEG_COUNT;
use crate::entities::Board;

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    OnGoing,
    Completed,
}

pub struct GameState {
    pub board: Board,
    pub status: GameStatus,
    pub move_count: u8,
    pub peg_count: u8,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            status: GameStatus::OnGoing,
            move_count: 0,
            peg_count: PEG_COUNT,
        }
    }
}
