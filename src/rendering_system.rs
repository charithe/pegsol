use crate::components::*;
use crate::constants::TILE_SIZE;
use ggez::graphics::{spritebatch::SpriteBatch, Color, DrawParam, Image};
use ggez::{graphics, nalgebra as na, timer, Context};
use itertools::Itertools;
use specs::{Join, ReadStorage, System};
use std::path::Path;

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum SpriteType {
    OccupiedSelectedHighlighted,
    OccupiedSelected,
    OccupiedHighlighted,
    OccupiedNormal,
    UnoccupiedSelectedHighlighted,
    UnoccupiedSelected,
    UnoccupiedHighlighted,
    UnoccupiedNormal,
}

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> RenderingSystem<'a> {
    fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(Color::new(0.0, 1.0, 0.0, 1.0));
        let dimensions = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (slots, occupied, selected, highlighted) = data;

        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut sprite_groups: Vec<(SpriteType, DrawParam)> = Vec::new();

        for (slot, occ, sel, hi) in (
            &slots,
            (&occupied).maybe(),
            (&selected).maybe(),
            (&highlighted).maybe(),
        )
            .join()
        {
            let sprite_type = match (occ, sel, hi) {
                (Some(_), Some(_), Some(_)) => SpriteType::OccupiedSelectedHighlighted,
                (Some(_), Some(_), None) => SpriteType::OccupiedSelected,
                (Some(_), None, Some(_)) => SpriteType::OccupiedHighlighted,
                (Some(_), None, None) => SpriteType::OccupiedNormal,
                (None, Some(_), Some(_)) => SpriteType::UnoccupiedSelectedHighlighted,
                (None, Some(_), None) => SpriteType::UnoccupiedSelected,
                (None, None, Some(_)) => SpriteType::UnoccupiedHighlighted,
                _ => SpriteType::UnoccupiedNormal,
            };

            let x = slot.x as f32 * TILE_SIZE;
            let y = slot.y as f32 * TILE_SIZE;
            sprite_groups.push((sprite_type, DrawParam::new().dest(na::Point2::new(x, y))));
        }

        sprite_groups
            .iter()
            .group_by(|(t, _)| t)
            .into_iter()
            .for_each(|(t, params)| {
                let image_file = match t {
                    SpriteType::OccupiedSelectedHighlighted => {
                        "slot_occupied_selected_highlighted.png"
                    }
                    SpriteType::OccupiedSelected => "slot_occupied_selected.png",
                    SpriteType::OccupiedHighlighted => "slot_occupied_highlighted.png",
                    SpriteType::OccupiedNormal => "slot_occupied_normal.png",
                    SpriteType::UnoccupiedSelectedHighlighted => {
                        "slot_unoccupied_selected_highlighted.png"
                    }
                    SpriteType::UnoccupiedSelected => "slot_unoccupied_selected.png",
                    SpriteType::UnoccupiedHighlighted => "slot_unoccupied_highlighted.png",
                    SpriteType::UnoccupiedNormal => "slot_unoccupied_normal.png",
                };
                let image_path = Path::new("/images").join(image_file);
                let image = Image::new(self.context, image_path).expect("unable to load image");
                let mut sprite_batch = SpriteBatch::new(image);
                params.for_each(|(_, p)| {
                    sprite_batch.add(*p);
                });

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("failed to render");
            });

        let fps = format!("FPS: {:.0}", timer::fps(self.context));
        self.draw_text(&fps, 1.0, 1.0);

        graphics::present(self.context).expect("present failed");
    }
}
