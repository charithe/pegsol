pub const PADDING: f32 = 10.0;
pub const TILE_SIZE: f32 = 97.0;
pub const BOARD_LEN: usize = 7;
pub const LAYOUT: [[char; BOARD_LEN]; BOARD_LEN] = [
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'O', 'X', 'X', 'X'],
    ['X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
    ['.', '.', 'X', 'X', 'X', '.', '.'],
];
