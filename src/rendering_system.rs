use crate::assets::*;
use crate::components::*;
use crate::constants::{PADDING_LEFT, PADDING_TOP, TILE_SIZE};
use crate::resources::*;
use ggez::graphics::{
    spritebatch::SpriteBatch, Align, Color, DrawParam, Scale, Text, TextFragment,
};
use ggez::{graphics, nalgebra as na, timer, Context};
use itertools::Itertools;
use specs::{Join, Read, ReadExpect, ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> RenderingSystem<'a> {
    fn draw_banner(&mut self, asset_store: &AssetStore, moves: u32, pegs: u32, game_over: bool) {
        graphics::draw(
            self.context,
            &asset_store.image(ImageType::Header),
            DrawParam::default().dest(na::Point2::new(0.0, 0.0)),
        )
        .expect("failed to render header");

        let mut moves_txt = Text::new(
            TextFragment::new(format!("Moves\n{:05}", moves))
                .font(asset_store.font())
                .scale(Scale::uniform(36.0)),
        );

        graphics::queue_text(
            self.context,
            moves_txt.set_bounds([150.0, 100.0], Align::Center),
            na::Point2::new(0.0, 20.0),
            Some(Color::new(1.0, 1.0, 1.0, 1.0)),
        );

        if game_over {
            let mut game_over_txt = Text::new(
                TextFragment::new("GAME OVER")
                    .font(asset_store.font())
                    .scale(Scale::uniform(48.0)),
            );

            graphics::queue_text(
                self.context,
                game_over_txt.set_bounds([400.0, 100.0], Align::Center),
                na::Point2::new(150.0, 20.0),
                Some(Color::new(1.0, 0.0, 0.0, 1.0)),
            );
        }

        let mut pegs_txt = Text::new(
            TextFragment::new(format!("Pegs\n{:04}", pegs))
                .font(asset_store.font())
                .scale(Scale::uniform(36.0)),
        );

        graphics::queue_text(
            self.context,
            pegs_txt.set_bounds([150.0, 100.0], Align::Center),
            na::Point2::new(550.0, 20.0),
            Some(Color::new(1.0, 1.0, 1.0, 1.0)),
        );

        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0)),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }

    fn draw_fps(&mut self, fps: f64) {
        let mut fps_txt =
            Text::new(TextFragment::new(format!("FPS: {:.0}", fps)).scale(Scale::uniform(14.0)));

        graphics::queue_text(
            self.context,
            fps_txt.set_bounds([100.0, 40.0], Align::Right),
            na::Point2::new(0.0, 0.0),
            Some(Color::new(0.0, 1.0, 0.0, 1.0)),
        );

        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(na::Point2::new(580.0, 780.0)),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Slot>,
        ReadStorage<'a, Occupied>,
        ReadStorage<'a, Selected>,
        ReadStorage<'a, Highlighted>,
        ReadExpect<'a, GameState>,
        Read<'a, AssetStore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (slots, occupied, selected, highlighted, game_state, asset_store) = data;

        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut sprite_groups: Vec<(ImageType, DrawParam)> = Vec::new();

        for (slot, occ, sel, hi) in (
            &slots,
            (&occupied).maybe(),
            (&selected).maybe(),
            (&highlighted).maybe(),
        )
            .join()
        {
            let sprite_type = match (occ, sel, hi) {
                (Some(_), Some(_), Some(_)) => ImageType::OccupiedSelectedHighlighted,
                (Some(_), Some(_), None) => ImageType::OccupiedSelected,
                (Some(_), None, Some(_)) => ImageType::OccupiedHighlighted,
                (Some(_), None, None) => ImageType::OccupiedNormal,
                (None, Some(_), Some(_)) => ImageType::UnoccupiedSelectedHighlighted,
                (None, Some(_), None) => ImageType::UnoccupiedSelected,
                (None, None, Some(_)) => ImageType::UnoccupiedHighlighted,
                _ => ImageType::UnoccupiedNormal,
            };

            let x = (slot.x as f32 * TILE_SIZE) + PADDING_LEFT;
            let y = (slot.y as f32 * TILE_SIZE) + PADDING_TOP;
            sprite_groups.push((sprite_type, DrawParam::new().dest(na::Point2::new(x, y))));
        }

        sprite_groups
            .iter()
            .group_by(|(t, _)| t)
            .into_iter()
            .for_each(|(t, params)| {
                let image = asset_store.image(*t);
                let mut sprite_batch = SpriteBatch::new(image);
                params.for_each(|(_, p)| {
                    sprite_batch.add(*p);
                });

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("failed to render");
            });

        self.draw_banner(
            &asset_store,
            game_state.move_count,
            game_state.peg_count,
            game_state.status == GameStatus::Completed,
        );

        self.draw_fps(timer::fps(self.context));

        graphics::present(self.context).expect("present failed");
    }
}
