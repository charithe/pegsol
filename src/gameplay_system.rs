use crate::components::*;
use crate::resources::*;
use specs::{ReadStorage, System, WriteExpect, WriteStorage};

pub struct GamePlaySystem;

impl<'a> System<'a> for GamePlaySystem {
    type SystemData = (
        ReadStorage<'a, Slot>,
        WriteExpect<'a, GameState>,
        WriteStorage<'a, Occupied>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (slots, mut game_state, mut occupied) = data;
        if let Some(m) = game_state.moves.pop_front() {
            let prev_occupied = occupied.get(m.prev).is_some();
            let curr_unoccupied = occupied.get(m.curr).is_none();
            if prev_occupied && curr_unoccupied {
                let prev_slot = slots.get(m.prev).expect("previous slot not found");
                let curr_slot = slots.get(m.curr).expect("current slot not found");

                let dist_x = prev_slot.x as i32 - curr_slot.x as i32;
                let dist_y = prev_slot.y as i32 - curr_slot.y as i32;
                let slot_to_remove = match (dist_x, dist_y) {
                    (0, 2) => game_state.board.entity_at(prev_slot.x, prev_slot.y - 1),
                    (0, -2) => game_state.board.entity_at(prev_slot.x, prev_slot.y + 1),
                    (2, 0) => game_state.board.entity_at(prev_slot.x - 1, prev_slot.y),
                    (-2, 0) => game_state.board.entity_at(prev_slot.x + 1, prev_slot.y),
                    _ => None,
                };

                if let Some(sr) = slot_to_remove {
                    if let Some(_) = occupied.get(sr) {
                        occupied.remove(m.prev);
                        occupied.remove(sr);
                        occupied
                            .insert(m.curr, Occupied)
                            .expect("failed to mark entity as occupied");
                        game_state.move_count = game_state.move_count + 1;
                    }
                }
            }
        }
    }
}
