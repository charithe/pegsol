pub const BOARD_LEN: usize = 7;
pub const PADDING_TOP: f32 = 110.0;
pub const PADDING_LEFT: f32 = 10.0;
pub const PEG_COUNT: u8 = 32;
pub const TILE_SIZE: f32 = 97.0;
pub const LAYOUT: [[char; BOARD_LEN]; BOARD_LEN] = [
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'O', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
];
