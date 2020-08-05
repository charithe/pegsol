use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Selected;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Highlighted;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Occupied;

#[derive(Debug, Component, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Slot {
    pub x: usize,
    pub y: usize,
}

pub fn register_components(world: &mut World) {
    world.register::<Selected>();
    world.register::<Highlighted>();
    world.register::<Occupied>();
    world.register::<Slot>();
}
