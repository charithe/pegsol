use crate::entities::Board;
use ggez::graphics::Image;
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

pub struct SpriteCache(HashMap<SpriteType, Image>);

impl SpriteCache {
    pub fn new(context: &mut Context) -> Self {
        let mut m: HashMap<SpriteType, Image> = HashMap::new();
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
            m.insert(
                *s,
                Image::new(context, image_path).expect("unable to load image"),
            );
        }

        Self(m)
    }

    pub fn get(&self, s: SpriteType) -> Image {
        self.0.get(&s).unwrap().clone()
    }
}
