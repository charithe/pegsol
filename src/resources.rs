use crate::entities::Board;
use ggez::graphics::{Font, Image};
use ggez::Context;
use specs::Entity;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

#[derive(Debug)]
pub enum InputEvent {
    MouseClick { x: f32, y: f32 },
    Up,
    Down,
    Left,
    Right,
    Enter,
}

#[derive(Default)]
pub struct InputQueue {
    pub events: VecDeque<InputEvent>,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    OnGoing,
    Completed,
}

pub struct Move {
    pub prev: Entity,
    pub curr: Entity,
}

pub struct GameState {
    pub board: Board,
    pub status: GameStatus,
    pub moves: VecDeque<Move>,
    pub move_count: u32,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            status: GameStatus::OnGoing,
            moves: VecDeque::new(),
            move_count: 0,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpriteType {
    OccupiedSelectedHighlighted,
    OccupiedSelected,
    OccupiedHighlighted,
    OccupiedNormal,
    UnoccupiedSelectedHighlighted,
    UnoccupiedSelected,
    UnoccupiedHighlighted,
    UnoccupiedNormal,
}

impl SpriteType {
    fn image_name(&self) -> &str {
        match self {
            SpriteType::OccupiedSelectedHighlighted => "slot_occupied_selected_highlighted.png",
            SpriteType::OccupiedSelected => "slot_occupied_selected.png",
            SpriteType::OccupiedHighlighted => "slot_occupied_highlighted.png",
            SpriteType::OccupiedNormal => "slot_occupied_normal.png",
            SpriteType::UnoccupiedSelectedHighlighted => "slot_unoccupied_selected_highlighted.png",
            SpriteType::UnoccupiedSelected => "slot_unoccupied_selected.png",
            SpriteType::UnoccupiedHighlighted => "slot_unoccupied_highlighted.png",
            SpriteType::UnoccupiedNormal => "slot_unoccupied_normal.png",
        }
    }
}

pub struct ResourceCache {
    sprites: HashMap<SpriteType, Image>,
    font: Font,
    header: Image,
}

impl ResourceCache {
    pub fn new(context: &mut Context) -> Self {
        let mut sprites: HashMap<SpriteType, Image> = HashMap::new();
        for s in [
            SpriteType::OccupiedSelectedHighlighted,
            SpriteType::OccupiedSelected,
            SpriteType::OccupiedHighlighted,
            SpriteType::OccupiedNormal,
            SpriteType::UnoccupiedSelectedHighlighted,
            SpriteType::UnoccupiedSelected,
            SpriteType::UnoccupiedHighlighted,
            SpriteType::UnoccupiedNormal,
        ]
        .iter()
        {
            let image_path = Path::new("/images").join(s.image_name());
            sprites.insert(
                *s,
                Image::new(context, image_path).expect("unable to load image"),
            );
        }

        let header =
            Image::new(context, Path::new("/images/header.png")).expect("unable to load header");

        let font =
            Font::new(context, Path::new("/fonts/Roboto-Bold.ttf")).expect("unable to load font");

        Self {
            sprites,
            font,
            header,
        }
    }

    pub fn sprite(&self, s: SpriteType) -> Image {
        self.sprites.get(&s).unwrap().clone()
    }

    pub fn font(&self) -> Font {
        self.font.clone()
    }

    pub fn header(&self) -> Image {
        self.header.clone()
    }
}
