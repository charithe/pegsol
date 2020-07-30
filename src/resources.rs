use std::collections::VecDeque;

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
