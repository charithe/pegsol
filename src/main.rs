use crate::assets::AssetStore;
use crate::resources::{GameEventQueue, GameState, InputEvent, InputQueue};
use ggez;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod assets;
mod components;
mod constants;
mod entities;
mod gameplay_system;
mod input_system;
mod rendering_system;
mod resources;
mod sound_system;

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);

    let board = entities::create_board(&mut world);
    world.insert(GameState::new(board));
    world.insert(InputQueue::default());
    world.insert(GameEventQueue::default());
    world.insert(AssetStore::default());

    let context_builder = ggez::ContextBuilder::new("rust_pegsol", "pegsol")
        .window_setup(conf::WindowSetup::default().title("Peg Solitaire"))
        .window_mode(conf::WindowMode::default().dimensions(700.0, 800.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    assets::load_assets(&mut world, context);

    let pegsol = &mut PegSol { world };
    event::run(context, event_loop, pegsol)
}

struct PegSol {
    world: World,
}

impl event::EventHandler for PegSol {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut is = input_system::InputSystem;
        is.run_now(&self.world);

        let mut gs = gameplay_system::GamePlaySystem;
        gs.run_now(&self.world);

        let mut ss = sound_system::SoundSystem;
        ss.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = rendering_system::RenderingSystem { context };
        rs.run_now(&self.world);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        key_code: KeyCode,
        _key_mod: KeyMods,
        _repeat: bool,
    ) {
        let event = match key_code {
            KeyCode::Up | KeyCode::W => Some(InputEvent::Up),
            KeyCode::Down | KeyCode::S => Some(InputEvent::Down),
            KeyCode::Left | KeyCode::A => Some(InputEvent::Left),
            KeyCode::Right | KeyCode::D => Some(InputEvent::Right),
            KeyCode::Return => Some(InputEvent::Enter),
            KeyCode::Escape => {
                context.continuing = false;
                return;
            }
            _ => None,
        };

        if let Some(evt) = event {
            let mut input_queue = self.world.write_resource::<InputQueue>();
            input_queue.events.push_back(evt);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let mut input_queue = self.world.write_resource::<InputQueue>();
            input_queue
                .events
                .push_back(InputEvent::MouseClick { x, y });
        }
    }
}
