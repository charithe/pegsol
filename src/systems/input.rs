use crate::components::*;
use crate::constants::{PADDING_LEFT, PADDING_TOP, TILE_SIZE};
use crate::resources::*;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Slot>,
        ReadExpect<'a, GameState>,
        WriteExpect<'a, InputEventQueue>,
        WriteExpect<'a, MoveQueue>,
        WriteStorage<'a, Selected>,
        WriteStorage<'a, Highlighted>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            slots,
            game_state,
            mut input_queue,
            mut move_queue,
            mut selected,
            mut highlighted,
        ) = data;

        if let Some(event) = input_queue.dequeue() {
            let (entity, slot, _is_highlighted) =
                (&*entities, &slots, &highlighted).join().nth(0).unwrap();

            match event {
                InputEvent::MouseClick { x, y } => {
                    for (e, s) in (&*entities, &slots).join() {
                        let sx = (s.x as f32 * TILE_SIZE) + PADDING_LEFT;
                        let sy = (s.y as f32 * TILE_SIZE) + PADDING_TOP;

                        if x >= sx && x <= sx + TILE_SIZE {
                            if y >= sy && y <= sy + TILE_SIZE {
                                if let Some(prev) =
                                    (&*entities, &selected).join().map(|(en, _)| en).nth(0)
                                {
                                    move_queue.enqueue(Move { prev, curr: e });
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
                InputEvent::Enter => {
                    if let Some(prev) = (&*entities, &selected).join().map(|(e, _)| e).nth(0) {
                        move_queue.enqueue(Move { prev, curr: entity });
                        selected.remove(prev);
                    }

                    selected
                        .insert(entity, Selected)
                        .expect("failed to mark entity as selected");
                }
                InputEvent::Up => {
                    if let Some(e) = game_state.board.entity_above(slot.x, slot.y) {
                        highlighted
                            .insert(e, Highlighted)
                            .expect("failed to mark entity as highlighted");
                        highlighted.remove(entity);
                    }
                }
                InputEvent::Down => {
                    if let Some(e) = game_state.board.entity_below(slot.x, slot.y) {
                        highlighted
                            .insert(e, Highlighted)
                            .expect("failed to mark entity as highlighted");
                        highlighted.remove(entity);
                    }
                }
                InputEvent::Left => {
                    if let Some(e) = game_state.board.entity_to_left(slot.x, slot.y) {
                        highlighted
                            .insert(e, Highlighted)
                            .expect("failed to mark entity as highlighted");
                        highlighted.remove(entity);
                    }
                }
                InputEvent::Right => {
                    if let Some(e) = game_state.board.entity_to_right(slot.x, slot.y) {
                        highlighted
                            .insert(e, Highlighted)
                            .expect("failed to mark entity as highlighted");
                        highlighted.remove(entity);
                    }
                }
            }
        }
    }
}
