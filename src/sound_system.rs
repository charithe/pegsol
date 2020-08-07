use crate::assets::*;
use crate::resources::*;
use specs::{System, Write};

pub struct SoundSystem;

impl<'a> System<'a> for SoundSystem {
    type SystemData = (Write<'a, AssetStore>, Write<'a, GameEventQueue>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut asset_store, mut game_event_queue) = data;
        for event in game_event_queue.events.drain(..) {
            match event {
                GameEvent::CorrectMove => asset_store.play_sound(SoundType::CorrectMove),
                GameEvent::IncorrectMove => asset_store.play_sound(SoundType::IncorrectMove),
                GameEvent::GameOver => asset_store.play_sound(SoundType::GameOver),
            }
        }
    }
}
