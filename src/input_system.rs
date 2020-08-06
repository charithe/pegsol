use crate::components::*;
use crate::constants::{PADDING, TILE_SIZE};
use crate::resources::*;
use specs::{Entities, Join, ReadStorage, System, Write, WriteExpect, WriteStorage};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Slot>,
        WriteExpect<'a, GameState>,
        Write<'a, InputQueue>,
        WriteStorage<'a, Selected>,
        WriteStorage<'a, Highlighted>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, slots, mut game_state, mut input_queue, mut selected, mut highlighted) =
            data;

        if game_state.status == GameStatus::Completed {
            return;
        }

        let (entity, slot, _is_highlighted) =
            (&*entities, &slots, &highlighted).join().nth(0).unwrap();

        let event = input_queue.events.pop_front();
        match event {
            Some(InputEvent::MouseClick { x, y }) => {
                for (e, s) in (&*entities, &slots).join() {
                    let sx = (s.x as f32 * TILE_SIZE) + PADDING;
                    let sy = (s.y as f32 * TILE_SIZE) + PADDING;

                    if x >= sx && x <= sx + TILE_SIZE {
                        if y >= sy && y <= sy + TILE_SIZE {
                            if let Some(prev) =
                                (&*entities, &selected).join().map(|(en, _)| en).nth(0)
                            {
                                game_state.moves.push_back(Move { prev, curr: e });
                                selected.remove(prev);
                            }

                            selected
                                .insert(e, Selected)
                                .expect("failed to mark entity as selected");

                            return;
                        }
                    }
                }
            }
            Some(InputEvent::Enter) => {
                if let Some(prev) = (&*entities, &selected).join().map(|(e, _)| e).nth(0) {
                    game_state.moves.push_back(Move { prev, curr: entity });
                    selected.remove(prev);
                }

                selected
                    .insert(entity, Selected)
                    .expect("failed to mark entity as selected");
            }
            Some(InputEvent::Up) => {
                if let Some(e) = game_state.board.entity_above(slot.x, slot.y) {
                    highlighted
                        .insert(e, Highlighted)
                        .expect("failed to mark entity as highlighted");
                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Down) => {
                if let Some(e) = game_state.board.entity_below(slot.x, slot.y) {
                    highlighted
                        .insert(e, Highlighted)
                        .expect("failed to mark entity as highlighted");
                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Left) => {
                if let Some(e) = game_state.board.entity_to_left(slot.x, slot.y) {
                    highlighted
                        .insert(e, Highlighted)
                        .expect("failed to mark entity as highlighted");
                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Right) => {
                if let Some(e) = game_state.board.entity_to_right(slot.x, slot.y) {
                    highlighted
                        .insert(e, Highlighted)
                        .expect("failed to mark entity as highlighted");
                    highlighted.remove(entity);
                }
            }
            _ => {}
        }
    }
}
