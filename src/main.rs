use crate::resources::{
    AssetStore, GameEventQueue, GameState, InputEvent, InputEventQueue, MoveQueue,
};
use ggez;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod resources;
mod systems;

fn main() -> GameResult {
    // create new World and register components and resources
    let mut world = World::new();
    components::register_components(&mut world);

    let board = entities::create_board(&mut world);
    world.insert(GameState::new(board));
    world.insert(InputEventQueue::default());
    world.insert(GameEventQueue::default());
    world.insert(MoveQueue::default());
    world.insert(AssetStore::default());

    // create context
    let context_builder = ggez::ContextBuilder::new("pegsol", "Charith Ellawala")
        .window_setup(conf::WindowSetup::default().title("Peg Solitaire"))
        .window_mode(conf::WindowMode::default().dimensions(700.0, 800.0))
        .add_resource_path(path::PathBuf::from("./assets"));

    let (context, event_loop) = &mut context_builder.build()?;

    // load assets into memory
    resources::load_assets(&mut world, context);

    // start the event loop
    let game = &mut Game { world };
    event::run(context, event_loop, game)
}

struct Game {
    world: World,
}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut is = systems::input::InputSystem;
        is.run_now(&self.world);

        let mut gs = systems::gameplay::GamePlaySystem;
        gs.run_now(&self.world);

        let mut ss = systems::sound::SoundSystem;
        ss.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = systems::rendering::RenderingSystem { context };
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
            let mut input_queue = self.world.write_resource::<InputEventQueue>();
            input_queue.enqueue(evt);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let mut input_queue = self.world.write_resource::<InputEventQueue>();
            input_queue.enqueue(InputEvent::MouseClick { x, y });
        }
    }
}
