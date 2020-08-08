use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{Font, Image};
use ggez::Context;
use specs::{World, WorldExt};
use std::collections::HashMap;
use std::path::Path;

pub fn load_assets(world: &mut World, context: &mut Context) {
    let mut asset_store = world.write_resource::<AssetStore>();
    asset_store.load(context);
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageType {
    OccupiedSelectedHighlighted,
    OccupiedSelected,
    OccupiedHighlighted,
    OccupiedNormal,
    UnoccupiedSelectedHighlighted,
    UnoccupiedSelected,
    UnoccupiedHighlighted,
    UnoccupiedNormal,
    Header,
}

impl ImageType {
    fn image_name(&self) -> &str {
        match self {
            ImageType::OccupiedSelectedHighlighted => "slot_occupied_selected_highlighted.png",
            ImageType::OccupiedSelected => "slot_occupied_selected.png",
            ImageType::OccupiedHighlighted => "slot_occupied_highlighted.png",
            ImageType::OccupiedNormal => "slot_occupied_normal.png",
            ImageType::UnoccupiedSelectedHighlighted => "slot_unoccupied_selected_highlighted.png",
            ImageType::UnoccupiedSelected => "slot_unoccupied_selected.png",
            ImageType::UnoccupiedHighlighted => "slot_unoccupied_highlighted.png",
            ImageType::UnoccupiedNormal => "slot_unoccupied_normal.png",
            ImageType::Header => "header.png",
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SoundType {
    CorrectMove,
    IncorrectMove,
    GameOver,
}

impl SoundType {
    fn sound_name(&self) -> &str {
        match self {
            SoundType::CorrectMove => "correct_move.ogg",
            SoundType::IncorrectMove => "incorrect_move.ogg",
            SoundType::GameOver => "game_over.ogg",
        }
    }
}

pub struct AssetStore {
    images: HashMap<ImageType, Image>,
    sounds: HashMap<SoundType, audio::Source>,
    font: Font,
}

impl Default for AssetStore {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
            sounds: HashMap::new(),
            font: Font::default(),
        }
    }
}

impl AssetStore {
    fn load(&mut self, context: &mut Context) {
        for s in [
            ImageType::OccupiedSelectedHighlighted,
            ImageType::OccupiedSelected,
            ImageType::OccupiedHighlighted,
            ImageType::OccupiedNormal,
            ImageType::UnoccupiedSelectedHighlighted,
            ImageType::UnoccupiedSelected,
            ImageType::UnoccupiedHighlighted,
            ImageType::UnoccupiedNormal,
            ImageType::Header,
        ]
        .iter()
        {
            let image_path = Path::new("/images").join(s.image_name());
            self.images.insert(
                *s,
                Image::new(context, image_path).expect("unable to load image"),
            );
        }

        for a in [
            SoundType::CorrectMove,
            SoundType::IncorrectMove,
            SoundType::GameOver,
        ]
        .iter()
        {
            let sound_path = Path::new("/sounds").join(a.sound_name());
            self.sounds.insert(
                *a,
                audio::Source::new(context, sound_path).expect("unable to load sound"),
            );
        }

        self.font =
            Font::new(context, Path::new("/fonts/Roboto-Bold.ttf")).expect("unable to load font");
    }

    pub fn image(&self, s: ImageType) -> Image {
        self.images.get(&s).unwrap().clone()
    }

    pub fn font(&self) -> Font {
        self.font.clone()
    }

    pub fn play_sound(&mut self, s: SoundType) {
        let _ = self
            .sounds
            .get_mut(&s)
            .expect("sound not found")
            .play_detached();
    }
}
