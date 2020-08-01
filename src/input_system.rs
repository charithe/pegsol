use crate::components::*;
use crate::constants::TILE_SIZE;
use crate::resources::*;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, Write, WriteStorage};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, GameState>,
        Write<'a, InputQueue>,
        ReadStorage<'a, Slot>,
        WriteStorage<'a, Selected>,
        WriteStorage<'a, Highlighted>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, game_state, mut input_queue, slots, mut selected, mut highlighted) = data;

        let (entity, slot, _is_highlighted) =
            (&*entities, &slots, &highlighted).join().nth(0).unwrap();

        let event = input_queue.events.pop_front();
        match event {
            Some(InputEvent::MouseClick { x, y }) => {
                for (e, s) in (&*entities, &slots).join() {
                    let sx = s.x as f32 * TILE_SIZE;
                    let sy = s.y as f32 * TILE_SIZE;

                    if x >= sx && x <= sx + TILE_SIZE {
                        if y >= sy && y <= sy + TILE_SIZE {
                            if let Some(en) =
                                (&*entities, &selected).join().map(|(en, _)| en).nth(0)
                            {
                                selected.remove(en);
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
                if let Some(e) = (&*entities, &selected).join().map(|(e, _)| e).nth(0) {
                    selected.remove(e);
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
