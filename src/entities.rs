use crate::components::*;
use crate::constants::{BOARD_LEN, LAYOUT};
use specs::{world::Builder, world::Entity, World, WorldExt};

pub struct Board([[Option<Entity>; BOARD_LEN]; BOARD_LEN]);

impl Default for Board {
    fn default() -> Self {
        Self([[None; BOARD_LEN]; BOARD_LEN])
    }
}

impl Board {
    pub fn add_entity(&mut self, x: usize, y: usize, entity: Entity) {
        self.0[y][x] = Some(entity);
    }

    pub fn entity_at(&self, x: usize, y: usize) -> Option<Entity> {
        if x >= BOARD_LEN || y >= BOARD_LEN {
            return None;
        }

        self.0[y][x]
    }

    pub fn entity_above(&self, x: usize, y: usize) -> Option<Entity> {
        if y == 0 {
            return None;
        }

        self.0[y - 1][x]
    }

    pub fn entity_below(&self, x: usize, y: usize) -> Option<Entity> {
        if y == BOARD_LEN - 1 {
            return None;
        }

        self.0[y + 1][x]
    }

    pub fn entity_to_left(&self, x: usize, y: usize) -> Option<Entity> {
        if x == 0 {
            return None;
        }

        self.0[y][x - 1]
    }

    pub fn entity_to_right(&self, x: usize, y: usize) -> Option<Entity> {
        if x == BOARD_LEN - 1 {
            return None;
        }

        self.0[y][x + 1]
    }
}

pub fn create_board(world: &mut World) -> Board {
    let mut board = Board::default();

    for (y, row) in LAYOUT.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                'X' => {
                    let entity = create_occupied_slot(world, x, y);
                    board.add_entity(x, y, entity);
                }
                'O' => {
                    let entity = create_empty_slot(world, x, y);
                    board.add_entity(x, y, entity);
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
