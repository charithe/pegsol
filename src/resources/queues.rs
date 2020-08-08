use specs::Entity;
use std::collections::VecDeque;

// Generic queue implementation
#[derive(Default)]
pub struct Queue<T>(VecDeque<T>);

impl<T> Queue<T> {
    pub fn enqueue(&mut self, item: T) {
        self.0.push_back(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn drain(&mut self) -> std::collections::vec_deque::Drain<T> {
        self.0.drain(..)
    }
}

// Queue for handling input events
#[derive(Debug)]
pub enum InputEvent {
    MouseClick { x: f32, y: f32 },
    Up,
    Down,
    Left,
    Right,
    Enter,
}

pub type InputEventQueue = Queue<InputEvent>;

impl Default for InputEventQueue {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

// Queue for handling game moves
pub struct Move {
    pub prev: Entity,
    pub curr: Entity,
}

pub type MoveQueue = Queue<Move>;

impl Default for MoveQueue {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

// Queue for handling game events
#[derive(Debug)]
pub enum GameEvent {
    CorrectMove,
    IncorrectMove,
    GameOver,
}

pub type GameEventQueue = Queue<GameEvent>;

impl Default for GameEventQueue {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}
