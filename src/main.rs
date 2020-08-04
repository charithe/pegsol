use crate::resources::{GameState, InputEvent, InputQueue, SpriteCache};
use ggez;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod gameplay_system;
mod input_system;
mod rendering_system;
mod resources;

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);

    let board = entities::create_board(&mut world);
    world.insert(GameState::new(board));
    world.insert(InputQueue::default());

    let context_builder = ggez::ContextBuilder::new("rust_pegsol", "pegsol")
        .window_setup(conf::WindowSetup::default().title("Peg Solitaire"))
        .window_mode(conf::WindowMode::default().dimensions(900.0, 900.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    let pegsol = &mut PegSol {
        world,
        sprite_cache: None,
    };
    event::run(context, event_loop, pegsol)
}

struct PegSol {
    world: World,
    sprite_cache: Option<SpriteCache>,
}

impl event::EventHandler for PegSol {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut is = input_system::InputSystem;
        is.run_now(&self.world);

        let mut gs = gameplay_system::GamePlaySystem;
        gs.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        if self.sprite_cache.is_none() {
            self.sprite_cache = Some(SpriteCache::new(context));
        }

        let sprite_cache = self.sprite_cache.as_ref().unwrap();
        let mut rs = rendering_system::RenderingSystem {
            context,
            sprite_cache,
        };
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
