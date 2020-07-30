use crate::components::*;
use specs::{world::Builder, world::Entity, World, WorldExt};

const LAYOUT: [[char; 7]; 7] = [
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'O', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
];

pub struct Board {
    pub board: [[Option<Entity>; 7]; 7],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[None; 7]; 7],
        }
    }
}

pub fn create_board(world: &mut World) -> Board {
    let mut board = Board::default();

    for (y, row) in LAYOUT.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                'X' => {
                    let entity = create_occupied_slot(world, x, y);
                    board.board[y][x] = Some(entity);
                }
                'O' => {
                    let entity = create_empty_slot(world, x, y);
                    board.board[y][x] = Some(entity);
                }
                _ => (),
            }
        }
    }

    board
}

fn create_occupied_slot(world: &mut World, x: usize, y: usize) -> Entity {
    world
        .create_entity()
        .with(Slot { x, y })
        .with(Occupied)
        .build()
}

fn create_empty_slot(world: &mut World, x: usize, y: usize) -> Entity {
    world
        .create_entity()
        .with(Slot { x, y })
        .with(Highlighted)
        .build()
}
