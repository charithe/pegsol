use crate::components::*;
use ggez::graphics;
use ggez::graphics::{DrawParam, Image};
use ggez::nalgebra as na;
use ggez::Context;
use specs::{Join, ReadStorage, System};
use std::path::Path;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
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

        for (slot, occ, sel, hi) in (
            &slots,
            (&occupied).maybe(),
            (&selected).maybe(),
            (&highlighted).maybe(),
        )
            .join()
        {
            let image_file = match (occ, sel, hi) {
                (Some(_), Some(_), _) => "slot_occupied_selected.png",
                (Some(_), None, Some(_)) => "slot_occupied_highlighted.png",
                (Some(_), None, None) => "slot_occupied_normal.png",
                (None, Some(_), _) => "slot_unoccupied_selected.png",
                (None, None, Some(_)) => "slot_unoccupied_highlighted.png",
                _ => "slot_unoccupied_normal.png",
            };

            let image_path = Path::new("/images").join(image_file);
            let image = Image::new(self.context, image_path).expect("unable to load image");
            let x = slot.x as f32 * image.width() as f32;
            let y = slot.y as f32 * image.height() as f32;
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));

            graphics::draw(self.context, &image, draw_params).expect("render failed");
        }

        graphics::present(self.context).expect("present failed");
    }
}
