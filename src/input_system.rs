use crate::components::*;
use crate::entities::Board;
use crate::resources::*;
use ggez::Context;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Board>,
        Write<'a, InputQueue>,
        ReadStorage<'a, Slot>,
        WriteStorage<'a, Selected>,
        WriteStorage<'a, Highlighted>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, board, mut input_queue, slots, mut selected, mut highlighted) = data;

        let (entity, slot, is_highlighted) =
            (&*entities, &slots, &highlighted).join().nth(0).unwrap();

        let event = input_queue.events.pop_front();
        match event {
            Some(InputEvent::MouseClick { x, y }) => {
                for (e, s) in (&*entities, &slots).join() {
                    let sx = s.x as f32 * 128.0;
                    let sy = s.y as f32 * 128.0;

                    if x >= sx && x <= sx + 128.0 {
                        if y >= sy && y <= sy + 128.0 {
                            if let Some(en) =
                                (&*entities, &selected).join().map(|(en, _)| en).nth(0)
                            {
                                selected.remove(en);
                            }

                            selected.insert(e, Selected);

                            return;
                        }
                    }
                }
            }
            Some(InputEvent::Enter) => {
                if let Some(e) = (&*entities, &selected).join().map(|(e, _)| e).nth(0) {
                    selected.remove(e);
                }

                selected.insert(entity, Selected);
            }
            Some(InputEvent::Up) => {
                if slot.y > 0 {
                    if let Some(e) = board.board[slot.y - 1][slot.x] {
                        highlighted.insert(e, Highlighted);
                    }

                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Down) => {
                if slot.y < 6 {
                    if let Some(e) = board.board[slot.y + 1][slot.x] {
                        highlighted.insert(e, Highlighted);
                    }

                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Left) => {
                if slot.x > 0 {
                    if let Some(e) = board.board[slot.y][slot.x - 1] {
                        highlighted.insert(e, Highlighted);
                    }

                    highlighted.remove(entity);
                }
            }
            Some(InputEvent::Right) => {
                if slot.x < 6 {
                    if let Some(e) = board.board[slot.y][slot.x + 1] {
                        highlighted.insert(e, Highlighted);
                    }

                    highlighted.remove(entity);
                }
            }
            _ => {}
        }
    }
}
